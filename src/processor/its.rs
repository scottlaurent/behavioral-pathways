//! ITS (Interpersonal Theory of Suicide) processor.
//!
//! This module orchestrates the computation of ITS factors using the
//! formulas defined in MentalHealth (Phase 2). It does NOT reimplement
//! the formulas - it calls the MentalHealth compute methods.
//!
//! # ITS Components
//!
//! - **Thwarted Belongingness (TB)**: Feeling disconnected from others
//! - **Perceived Burdensomeness (PB)**: Believing oneself to be a burden
//! - **Acquired Capability (AC)**: Habituation to pain/fear of death
//! - **Suicidal Desire**: Requires TB AND PB AND interpersonal hopelessness > 0.5
//! - **Attempt Risk**: Desire * Acquired Capability
//!
//! # Convergence Model (Joiner's Risk Matrix)
//!
//! The three-factor convergence model distinguishes:
//! - Single-factor elevation (TB only, PB only, AC only)
//! - Dual-factor convergence (desire without capability, or capability without desire)
//! - Three-factor convergence (TB + PB + AC all elevated = highest risk)

use crate::state::{IndividualState, MentalHealth, SocialCognition, TB_PRESENT_THRESHOLD, PB_PRESENT_THRESHOLD};
use serde::{Deserialize, Serialize};

/// Threshold for Acquired Capability to be considered elevated.
pub const AC_ELEVATED_THRESHOLD: f32 = 0.3;

/// The three proximal factors in the ITS model.
///
/// These are the immediate causes of suicidal ideation and behavior:
/// - TB (Thwarted Belongingness): Unmet need to belong
/// - PB (Perceived Burdensomeness): Belief of being a burden
/// - AC (Acquired Capability): Habituation to pain and reduced fear of death
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItsProximalFactor {
    /// Thwarted Belongingness - feeling disconnected from others.
    ThwartedBelongingness,
    /// Perceived Burdensomeness - believing oneself to be a burden.
    PerceivedBurdensomeness,
    /// Acquired Capability - habituation to pain/fear of death.
    AcquiredCapability,
}

impl ItsProximalFactor {
    /// Returns a human-readable name for this factor.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            ItsProximalFactor::ThwartedBelongingness => "Thwarted Belongingness",
            ItsProximalFactor::PerceivedBurdensomeness => "Perceived Burdensomeness",
            ItsProximalFactor::AcquiredCapability => "Acquired Capability",
        }
    }

    /// Returns all proximal factor variants.
    #[must_use]
    pub const fn all() -> [ItsProximalFactor; 3] {
        [
            ItsProximalFactor::ThwartedBelongingness,
            ItsProximalFactor::PerceivedBurdensomeness,
            ItsProximalFactor::AcquiredCapability,
        ]
    }

    /// Returns the short code for this factor.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            ItsProximalFactor::ThwartedBelongingness => "TB",
            ItsProximalFactor::PerceivedBurdensomeness => "PB",
            ItsProximalFactor::AcquiredCapability => "AC",
        }
    }
}

impl std::fmt::Display for ItsProximalFactor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Convergence status tracking which ITS factors are elevated.
///
/// Per Joiner's ITS, the critical clinical question is which factors
/// have converged. Three-factor convergence (TB + PB + AC all elevated)
/// represents the highest risk state.
///
/// # Risk Matrix
///
/// | TB | PB | AC | Risk Level |
/// |----|----|----|------------|
/// | X  |    |    | Low - passive ideation possible |
/// |    | X  |    | Low - passive ideation possible |
/// |    |    | X  | Low - capability without desire |
/// | X  | X  |    | Moderate - desire without capability |
/// | X  |    | X  | Moderate - belongingness + capability |
/// |    | X  | X  | Moderate - burdensomeness + capability |
/// | X  | X  | X  | HIGH - three-factor convergence |
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ConvergenceStatus {
    /// True if all three factors (TB, PB, AC) are elevated.
    pub is_three_factor_convergent: bool,

    /// The factor with the highest elevation above threshold.
    pub highest_factor: Option<ItsProximalFactor>,

    /// Count of factors currently elevated (0-3).
    pub elevated_factor_count: u8,

    /// True if TB is above its threshold.
    pub tb_elevated: bool,

    /// True if PB is above its threshold.
    pub pb_elevated: bool,

    /// True if AC is above its threshold.
    pub ac_elevated: bool,
}

impl ConvergenceStatus {
    /// Creates a new convergence status from factor values.
    #[must_use]
    pub fn from_factors(tb: f32, pb: f32, ac: f32) -> Self {
        let tb_elevated = tb >= TB_PRESENT_THRESHOLD;
        let pb_elevated = pb >= PB_PRESENT_THRESHOLD;
        let ac_elevated = ac >= AC_ELEVATED_THRESHOLD;

        let elevated_factor_count = u8::from(tb_elevated) + u8::from(pb_elevated) + u8::from(ac_elevated);
        let is_three_factor_convergent = elevated_factor_count == 3;

        // Determine highest factor (by how much it exceeds its threshold)
        let tb_excess = if tb_elevated { tb - TB_PRESENT_THRESHOLD } else { -1.0 };
        let pb_excess = if pb_elevated { pb - PB_PRESENT_THRESHOLD } else { -1.0 };
        let ac_excess = if ac_elevated { ac - AC_ELEVATED_THRESHOLD } else { -1.0 };

        let highest_factor = if tb_excess >= pb_excess && tb_excess >= ac_excess && tb_elevated {
            Some(ItsProximalFactor::ThwartedBelongingness)
        } else if pb_excess >= tb_excess && pb_excess >= ac_excess && pb_elevated {
            Some(ItsProximalFactor::PerceivedBurdensomeness)
        } else if ac_elevated {
            Some(ItsProximalFactor::AcquiredCapability)
        } else {
            None
        };

        ConvergenceStatus {
            is_three_factor_convergent,
            highest_factor,
            elevated_factor_count,
            tb_elevated,
            pb_elevated,
            ac_elevated,
        }
    }

    /// Returns true if suicidal desire is present (TB + PB elevated).
    #[must_use]
    pub fn has_desire(&self) -> bool {
        self.tb_elevated && self.pb_elevated
    }

    /// Returns true if only capability is elevated (no desire).
    #[must_use]
    pub fn is_dormant_capability(&self) -> bool {
        self.ac_elevated && !self.has_desire()
    }

    /// Returns true if desire is present without capability.
    #[must_use]
    pub fn has_desire_without_capability(&self) -> bool {
        self.has_desire() && !self.ac_elevated
    }

    /// Returns a list of currently elevated factors.
    #[must_use]
    pub fn elevated_factors(&self) -> Vec<ItsProximalFactor> {
        let mut factors = Vec::with_capacity(3);
        if self.tb_elevated {
            factors.push(ItsProximalFactor::ThwartedBelongingness);
        }
        if self.pb_elevated {
            factors.push(ItsProximalFactor::PerceivedBurdensomeness);
        }
        if self.ac_elevated {
            factors.push(ItsProximalFactor::AcquiredCapability);
        }
        factors
    }
}

impl Default for ConvergenceStatus {
    fn default() -> Self {
        ConvergenceStatus {
            is_three_factor_convergent: false,
            highest_factor: None,
            elevated_factor_count: 0,
            tb_elevated: false,
            pb_elevated: false,
            ac_elevated: false,
        }
    }
}

/// Computed ITS factors from entity state.
///
/// This struct holds the computed values for all ITS components.
/// These are derived values - they are computed from the underlying
/// state dimensions, not stored directly.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ItsFactors {
    /// Thwarted Belongingness: (loneliness + (1 - reciprocal_caring)) / 2
    pub thwarted_belongingness: f32,

    /// Perceived Burdensomeness: liability * self_hate
    pub perceived_burdensomeness: f32,

    /// Acquired Capability: habituation to pain/death (never decreases)
    pub acquired_capability: f32,

    /// Suicidal Desire: TB * PB when hopelessness threshold met
    pub suicidal_desire: f32,

    /// Attempt Risk: Desire * Acquired Capability
    pub attempt_risk: f32,

    /// Whether passive ideation is present (TB > 0.3 OR PB > 0.3)
    pub passive_ideation_present: bool,

    /// Convergence status tracking which factors are elevated.
    /// This enables API consumers to detect three-factor convergence
    /// and understand the risk matrix without manual threshold checking.
    pub convergence_status: ConvergenceStatus,
}

impl ItsFactors {
    /// Returns true if active suicidal desire is present.
    ///
    /// Active desire requires TB AND PB AND hopelessness above threshold.
    #[must_use]
    #[allow(dead_code)]
    pub fn has_active_desire(&self) -> bool {
        self.suicidal_desire > 0.0
    }

    /// Returns true if there is significant attempt risk.
    ///
    /// Significant risk requires both desire and acquired capability.
    #[must_use]
    #[allow(dead_code)]
    pub fn has_significant_risk(&self) -> bool {
        self.attempt_risk > 0.3
    }
}

impl Default for ItsFactors {
    fn default() -> Self {
        ItsFactors {
            thwarted_belongingness: 0.0,
            perceived_burdensomeness: 0.0,
            acquired_capability: 0.0,
            suicidal_desire: 0.0,
            attempt_risk: 0.0,
            passive_ideation_present: false,
            convergence_status: ConvergenceStatus::default(),
        }
    }
}

/// Threshold for TB or PB to indicate passive ideation presence.
#[allow(dead_code)]
const PASSIVE_IDEATION_THRESHOLD: f32 = 0.3;

/// Computes ITS factors from individual state.
///
/// This function orchestrates the ITS computation by calling the
/// MentalHealth compute methods. It does NOT reimplement the formulas -
/// those are defined in MentalHealth (Phase 2).
///
/// # Arguments
///
/// * `state` - The entity's individual state containing social cognition and mental health
///
/// # Returns
///
/// Computed ITS factors including TB, PB, AC, desire, and risk.
///
/// # Examples
///
/// ```ignore
/// use behavioral_pathways::processor::compute_its_factors;
/// use behavioral_pathways::state::IndividualState;
///
/// let state = IndividualState::new();
/// let factors = compute_its_factors(&state);
///
/// // Default healthy state has low ITS factors
/// assert!(factors.thwarted_belongingness < 0.5);
/// assert!(factors.perceived_burdensomeness < 0.5);
/// assert!(factors.suicidal_desire < 0.1);
/// ```
#[must_use]
#[allow(dead_code)]
pub fn compute_its_factors(state: &IndividualState) -> ItsFactors {
    let social = state.social_cognition();
    let mental_health = state.mental_health();

    compute_its_factors_from_components(social, mental_health)
}

/// Computes ITS factors from social cognition and mental health components.
///
/// This is the internal implementation that takes component references.
/// Use `compute_its_factors` for the public API.
#[must_use]
#[allow(dead_code)]
pub fn compute_its_factors_from_components(
    social: &SocialCognition,
    mental_health: &MentalHealth,
) -> ItsFactors {
    // Delegate to MentalHealth compute methods (Phase 2 formulas)
    let tb = mental_health.compute_thwarted_belongingness(social);
    let pb = mental_health.compute_perceived_burdensomeness(social);
    let ac = mental_health.acquired_capability_effective();

    // Check hopelessness threshold for active desire
    // Note: compute_suicidal_desire handles the hopelessness threshold internally
    let _hopelessness = mental_health.interpersonal_hopelessness_effective();

    // Suicidal desire requires TB AND PB AND hopelessness > threshold
    // Using Phase 2's compute_suicidal_desire which handles this logic
    let desire = mental_health.compute_suicidal_desire(social);

    // Attempt risk = desire * capability
    let risk = mental_health.compute_attempt_risk(social);

    // Passive ideation: TB > 0.3 OR PB > 0.3
    let passive_ideation = tb > PASSIVE_IDEATION_THRESHOLD || pb > PASSIVE_IDEATION_THRESHOLD;

    // Compute convergence status for risk matrix
    let convergence_status = ConvergenceStatus::from_factors(tb, pb, ac);

    ItsFactors {
        thwarted_belongingness: tb,
        perceived_burdensomeness: pb,
        acquired_capability: ac,
        suicidal_desire: desire,
        attempt_risk: risk,
        passive_ideation_present: passive_ideation,
        convergence_status,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create high-risk state
    fn high_risk_state() -> IndividualState {
        let mut state = IndividualState::new();

        // High TB components
        state.social_cognition_mut().loneliness_mut().set_base(0.9);
        state
            .social_cognition_mut()
            .perceived_reciprocal_caring_mut()
            .set_base(0.1);

        // High PB components
        state
            .social_cognition_mut()
            .perceived_liability_mut()
            .set_base(0.9);
        state.social_cognition_mut().self_hate_mut().set_base(0.9);

        // High hopelessness (required for active desire)
        state
            .mental_health_mut()
            .interpersonal_hopelessness_mut()
            .set_base(0.7);

        // High AC
        state
            .mental_health_mut()
            .acquired_capability_mut()
            .set_base(0.8);

        state
    }

    // Helper to create low hopelessness state with high TB/PB
    fn high_tb_pb_low_hopelessness() -> IndividualState {
        let mut state = IndividualState::new();

        // High TB
        state.social_cognition_mut().loneliness_mut().set_base(0.9);
        state
            .social_cognition_mut()
            .perceived_reciprocal_caring_mut()
            .set_base(0.1);

        // High PB
        state
            .social_cognition_mut()
            .perceived_liability_mut()
            .set_base(0.9);
        state.social_cognition_mut().self_hate_mut().set_base(0.9);

        // LOW hopelessness (below threshold)
        state
            .mental_health_mut()
            .interpersonal_hopelessness_mut()
            .set_base(0.3);

        state
    }

    // --- Tests from phase-4.md ---

    #[test]
    fn its_desire_formula() {
        // Test name from phase-4.md
        // Verify desire computation uses Phase 2 methods
        let state = high_risk_state();
        let factors = compute_its_factors(&state);

        // With high TB, high PB, and high hopelessness, desire should be significant
        assert!(factors.suicidal_desire > 0.0);

        // TB and PB should be high
        assert!(factors.thwarted_belongingness > 0.7);
        assert!(factors.perceived_burdensomeness > 0.7);
    }

    #[test]
    fn its_active_desire_requires_hopelessness_threshold() {
        // Test name from phase-4.md
        // Desire only when TB AND PB AND hopelessness > 0.5
        let state = high_risk_state();
        let factors = compute_its_factors(&state);

        // All conditions met - should have desire
        assert!(factors.has_active_desire());
    }

    #[test]
    fn its_desire_zero_below_hopelessness_threshold() {
        // Test name from phase-4.md
        // Desire is zero when hopelessness <= 0.5 even with high TB and PB
        let state = high_tb_pb_low_hopelessness();
        let factors = compute_its_factors(&state);

        // TB and PB are high
        assert!(factors.thwarted_belongingness > 0.7);
        assert!(factors.perceived_burdensomeness > 0.7);

        // But hopelessness is low, so desire should be zero or very low
        assert!(factors.suicidal_desire < 0.01);
    }

    #[test]
    fn its_passive_ideation_when_tb_or_pb_present() {
        // Test name from phase-4.md
        // Passive ideation when TB > 0.3 OR PB > 0.3

        // Just high TB
        let mut tb_state = IndividualState::new();
        tb_state
            .social_cognition_mut()
            .loneliness_mut()
            .set_base(0.8);
        let tb_factors = compute_its_factors(&tb_state);
        assert!(tb_factors.passive_ideation_present);

        // Just high PB
        let mut pb_state = IndividualState::new();
        pb_state
            .social_cognition_mut()
            .perceived_liability_mut()
            .set_base(0.8);
        pb_state
            .social_cognition_mut()
            .self_hate_mut()
            .set_base(0.8);
        let pb_factors = compute_its_factors(&pb_state);
        assert!(pb_factors.passive_ideation_present);

        // Neither high
        let low_state = IndividualState::new();
        let low_factors = compute_its_factors(&low_state);
        assert!(!low_factors.passive_ideation_present);
    }

    #[test]
    fn its_processor_reuses_mental_health_formulas() {
        // Test name from phase-4.md
        // ITS processor delegates to MentalHealth.compute_* (single source of truth)

        let state = high_risk_state();
        let factors = compute_its_factors(&state);

        // Compute directly using MentalHealth methods
        let social = state.social_cognition();
        let mental_health = state.mental_health();

        let direct_tb = mental_health.compute_thwarted_belongingness(social);
        let direct_pb = mental_health.compute_perceived_burdensomeness(social);
        let direct_desire = mental_health.compute_suicidal_desire(social);
        let direct_risk = mental_health.compute_attempt_risk(social);

        // Values should match exactly - processor uses same methods
        assert!((factors.thwarted_belongingness - direct_tb).abs() < f32::EPSILON);
        assert!((factors.perceived_burdensomeness - direct_pb).abs() < f32::EPSILON);
        assert!((factors.suicidal_desire - direct_desire).abs() < f32::EPSILON);
        assert!((factors.attempt_risk - direct_risk).abs() < f32::EPSILON);
    }

    // --- Additional tests ---

    #[test]
    fn its_factors_default() {
        let factors = ItsFactors::default();

        assert!(factors.thwarted_belongingness.abs() < f32::EPSILON);
        assert!(factors.perceived_burdensomeness.abs() < f32::EPSILON);
        assert!(factors.acquired_capability.abs() < f32::EPSILON);
        assert!(factors.suicidal_desire.abs() < f32::EPSILON);
        assert!(factors.attempt_risk.abs() < f32::EPSILON);
        assert!(!factors.passive_ideation_present);
    }

    #[test]
    fn its_factors_has_active_desire() {
        let mut factors = ItsFactors::default();
        assert!(!factors.has_active_desire());

        factors.suicidal_desire = 0.5;
        assert!(factors.has_active_desire());

        factors.suicidal_desire = 0.0;
        assert!(!factors.has_active_desire());
    }

    #[test]
    fn its_factors_has_significant_risk() {
        let mut factors = ItsFactors::default();
        assert!(!factors.has_significant_risk());

        factors.attempt_risk = 0.5;
        assert!(factors.has_significant_risk());

        factors.attempt_risk = 0.2;
        assert!(!factors.has_significant_risk());

        factors.attempt_risk = 0.3;
        assert!(!factors.has_significant_risk()); // Exactly at threshold, not over
    }

    #[test]
    fn healthy_state_has_low_factors() {
        let state = IndividualState::new();
        let factors = compute_its_factors(&state);

        assert!(factors.thwarted_belongingness < 0.5);
        assert!(factors.perceived_burdensomeness < 0.3);
        assert!(factors.suicidal_desire < 0.1);
        assert!(factors.attempt_risk < 0.1);
        assert!(!factors.passive_ideation_present);
    }

    #[test]
    fn acquired_capability_preserved() {
        let mut state = IndividualState::new();
        state
            .mental_health_mut()
            .acquired_capability_mut()
            .set_base(0.7);

        let factors = compute_its_factors(&state);
        assert!((factors.acquired_capability - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn its_factors_clone_and_equality() {
        let factors1 = ItsFactors {
            thwarted_belongingness: 0.5,
            perceived_burdensomeness: 0.4,
            acquired_capability: 0.3,
            suicidal_desire: 0.2,
            attempt_risk: 0.1,
            passive_ideation_present: true,
            convergence_status: ConvergenceStatus::from_factors(0.5, 0.4, 0.3),
        };

        let factors2 = factors1;
        assert_eq!(factors1, factors2);
    }

    #[test]
    fn its_factors_debug() {
        let factors = ItsFactors::default();
        let debug = format!("{:?}", factors);
        assert!(debug.contains("ItsFactors"));
    }

    #[test]
    fn its_factors_copy() {
        let factors1 = ItsFactors::default();
        let factors2 = factors1; // Copy
        assert_eq!(factors1, factors2);
    }

    #[test]
    fn its_factors_has_significant_risk_at_threshold() {
        let mut factors = ItsFactors::default();
        factors.attempt_risk = 0.31;
        assert!(factors.has_significant_risk());
    }

    #[test]
    fn compute_its_from_components_directly() {
        let mut state = IndividualState::new();
        state
            .social_cognition_mut()
            .loneliness_mut()
            .set_base(0.5);

        let social = state.social_cognition();
        let mental_health = state.mental_health();
        let factors = compute_its_factors_from_components(social, mental_health);

        // Should compute TB based on loneliness
        assert!(factors.thwarted_belongingness > 0.3);
    }

    // --- ItsProximalFactor tests ---

    #[test]
    fn proximal_factor_names() {
        assert_eq!(ItsProximalFactor::ThwartedBelongingness.name(), "Thwarted Belongingness");
        assert_eq!(ItsProximalFactor::PerceivedBurdensomeness.name(), "Perceived Burdensomeness");
        assert_eq!(ItsProximalFactor::AcquiredCapability.name(), "Acquired Capability");
    }

    #[test]
    fn proximal_factor_codes() {
        assert_eq!(ItsProximalFactor::ThwartedBelongingness.code(), "TB");
        assert_eq!(ItsProximalFactor::PerceivedBurdensomeness.code(), "PB");
        assert_eq!(ItsProximalFactor::AcquiredCapability.code(), "AC");
    }

    #[test]
    fn proximal_factor_all() {
        let all = ItsProximalFactor::all();
        assert_eq!(all.len(), 3);
        assert_eq!(all[0], ItsProximalFactor::ThwartedBelongingness);
        assert_eq!(all[1], ItsProximalFactor::PerceivedBurdensomeness);
        assert_eq!(all[2], ItsProximalFactor::AcquiredCapability);
    }

    #[test]
    fn proximal_factor_display() {
        assert_eq!(format!("{}", ItsProximalFactor::ThwartedBelongingness), "Thwarted Belongingness");
    }

    #[test]
    fn proximal_factor_copy_clone() {
        let f1 = ItsProximalFactor::ThwartedBelongingness;
        let f2 = f1; // Copy
        let f3 = f1.clone();
        assert_eq!(f1, f2);
        assert_eq!(f1, f3);
    }

    #[test]
    fn proximal_factor_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(ItsProximalFactor::ThwartedBelongingness);
        set.insert(ItsProximalFactor::ThwartedBelongingness);
        assert_eq!(set.len(), 1);
        set.insert(ItsProximalFactor::PerceivedBurdensomeness);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn proximal_factor_debug() {
        let debug = format!("{:?}", ItsProximalFactor::AcquiredCapability);
        assert!(debug.contains("AcquiredCapability"));
    }

    // --- ConvergenceStatus tests ---

    #[test]
    fn convergence_status_default() {
        let status = ConvergenceStatus::default();
        assert!(!status.is_three_factor_convergent);
        assert!(status.highest_factor.is_none());
        assert_eq!(status.elevated_factor_count, 0);
        assert!(!status.tb_elevated);
        assert!(!status.pb_elevated);
        assert!(!status.ac_elevated);
    }

    #[test]
    fn convergence_status_no_factors_elevated() {
        let status = ConvergenceStatus::from_factors(0.3, 0.3, 0.1);
        assert!(!status.is_three_factor_convergent);
        assert!(status.highest_factor.is_none());
        assert_eq!(status.elevated_factor_count, 0);
        assert!(!status.tb_elevated);
        assert!(!status.pb_elevated);
        assert!(!status.ac_elevated);
    }

    #[test]
    fn convergence_status_tb_only() {
        let status = ConvergenceStatus::from_factors(0.7, 0.3, 0.1);
        assert!(!status.is_three_factor_convergent);
        assert_eq!(status.highest_factor, Some(ItsProximalFactor::ThwartedBelongingness));
        assert_eq!(status.elevated_factor_count, 1);
        assert!(status.tb_elevated);
        assert!(!status.pb_elevated);
        assert!(!status.ac_elevated);
    }

    #[test]
    fn convergence_status_pb_only() {
        let status = ConvergenceStatus::from_factors(0.3, 0.7, 0.1);
        assert!(!status.is_three_factor_convergent);
        assert_eq!(status.highest_factor, Some(ItsProximalFactor::PerceivedBurdensomeness));
        assert_eq!(status.elevated_factor_count, 1);
        assert!(!status.tb_elevated);
        assert!(status.pb_elevated);
        assert!(!status.ac_elevated);
    }

    #[test]
    fn convergence_status_ac_only() {
        let status = ConvergenceStatus::from_factors(0.3, 0.3, 0.5);
        assert!(!status.is_three_factor_convergent);
        assert_eq!(status.highest_factor, Some(ItsProximalFactor::AcquiredCapability));
        assert_eq!(status.elevated_factor_count, 1);
        assert!(!status.tb_elevated);
        assert!(!status.pb_elevated);
        assert!(status.ac_elevated);
    }

    #[test]
    fn convergence_status_tb_pb_dual() {
        let status = ConvergenceStatus::from_factors(0.6, 0.7, 0.1);
        assert!(!status.is_three_factor_convergent);
        assert_eq!(status.elevated_factor_count, 2);
        assert!(status.tb_elevated);
        assert!(status.pb_elevated);
        assert!(!status.ac_elevated);
        assert!(status.has_desire());
        assert!(status.has_desire_without_capability());
        assert!(!status.is_dormant_capability());
    }

    #[test]
    fn convergence_status_three_factor() {
        let status = ConvergenceStatus::from_factors(0.7, 0.6, 0.5);
        assert!(status.is_three_factor_convergent);
        assert_eq!(status.elevated_factor_count, 3);
        assert!(status.tb_elevated);
        assert!(status.pb_elevated);
        assert!(status.ac_elevated);
        assert!(status.has_desire());
        assert!(!status.has_desire_without_capability());
        assert!(!status.is_dormant_capability());
    }

    #[test]
    fn convergence_status_dormant_capability() {
        let status = ConvergenceStatus::from_factors(0.3, 0.3, 0.5);
        assert!(status.is_dormant_capability());
        assert!(!status.has_desire());
    }

    #[test]
    fn convergence_status_elevated_factors_list() {
        let status = ConvergenceStatus::from_factors(0.6, 0.7, 0.5);
        let factors = status.elevated_factors();
        assert_eq!(factors.len(), 3);
        assert!(factors.contains(&ItsProximalFactor::ThwartedBelongingness));
        assert!(factors.contains(&ItsProximalFactor::PerceivedBurdensomeness));
        assert!(factors.contains(&ItsProximalFactor::AcquiredCapability));
    }

    #[test]
    fn convergence_status_elevated_factors_empty() {
        let status = ConvergenceStatus::default();
        let factors = status.elevated_factors();
        assert!(factors.is_empty());
    }

    #[test]
    fn convergence_status_highest_factor_by_excess() {
        // TB has highest excess (0.7 - 0.5 = 0.2)
        // PB has lower excess (0.55 - 0.5 = 0.05)
        let status = ConvergenceStatus::from_factors(0.7, 0.55, 0.1);
        assert_eq!(status.highest_factor, Some(ItsProximalFactor::ThwartedBelongingness));
    }

    #[test]
    fn convergence_status_copy_clone() {
        let s1 = ConvergenceStatus::from_factors(0.6, 0.6, 0.6);
        let s2 = s1; // Copy
        let s3 = s1.clone();
        assert_eq!(s1, s2);
        assert_eq!(s1, s3);
    }

    #[test]
    fn convergence_status_debug() {
        let status = ConvergenceStatus::default();
        let debug = format!("{:?}", status);
        assert!(debug.contains("ConvergenceStatus"));
    }

    // --- Integration tests with ItsFactors ---

    #[test]
    fn its_factors_includes_convergence_status() {
        let state = high_risk_state();
        let factors = compute_its_factors(&state);

        // High risk state should have three-factor convergence
        assert!(factors.convergence_status.is_three_factor_convergent);
        assert_eq!(factors.convergence_status.elevated_factor_count, 3);
    }

    #[test]
    fn its_factors_default_has_default_convergence() {
        let factors = ItsFactors::default();
        assert!(!factors.convergence_status.is_three_factor_convergent);
        assert_eq!(factors.convergence_status.elevated_factor_count, 0);
    }

    #[test]
    fn healthy_state_no_convergence() {
        let state = IndividualState::new();
        let factors = compute_its_factors(&state);

        assert!(!factors.convergence_status.is_three_factor_convergent);
        assert!(!factors.convergence_status.has_desire());
    }

    #[test]
    fn high_tb_pb_without_ac_shows_desire_without_capability() {
        let state = high_tb_pb_low_hopelessness();
        let factors = compute_its_factors(&state);

        // TB and PB are elevated, AC is not
        assert!(factors.convergence_status.tb_elevated);
        assert!(factors.convergence_status.pb_elevated);
        assert!(!factors.convergence_status.ac_elevated);
        assert!(factors.convergence_status.has_desire_without_capability());
    }

    #[test]
    fn ac_elevated_threshold_constant() {
        assert!((AC_ELEVATED_THRESHOLD - 0.3).abs() < f32::EPSILON);
    }
}
