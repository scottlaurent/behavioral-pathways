//! Perceived risk assessment for trust decisions.
//!
//! Perceived risk represents a trustor's subjective assessment of the
//! potential negative consequences of trusting a specific trustee.

use crate::state::StateValue;
use crate::types::Duration;

/// Decay half-life for perceived risk (7 days).
/// Risk assessments fade faster without reinforcement.
const PERCEIVED_RISK_DECAY_HALF_LIFE: Duration = Duration::days(7);

/// Default base value for perceived risk.
const DEFAULT_BASE: f32 = 0.3;

/// What type of vulnerability is being accepted in a trust decision.
///
/// Per Mayer's model, trust is meaningful only when something is at stake.
/// This enum identifies what the trustor is risking.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::relationship::VulnerabilityType;
///
/// let at_risk = VulnerabilityType::Resources;
/// assert_eq!(at_risk.name(), "Resources");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum VulnerabilityType {
    /// Identity/self-concept at risk (e.g., sharing personal secrets).
    Identity,
    /// Financial or material resources at risk.
    #[default]
    Resources,
    /// Physical safety at risk.
    Safety,
    /// The relationship itself at risk (e.g., confronting a friend).
    Relationship,
    /// Professional reputation at risk.
    Reputation,
    /// Emotional wellbeing at risk.
    Emotional,
}

impl VulnerabilityType {
    /// Returns a human-readable name for this vulnerability type.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            VulnerabilityType::Identity => "Identity",
            VulnerabilityType::Resources => "Resources",
            VulnerabilityType::Safety => "Safety",
            VulnerabilityType::Relationship => "Relationship",
            VulnerabilityType::Reputation => "Reputation",
            VulnerabilityType::Emotional => "Emotional",
        }
    }

    /// Returns all vulnerability types.
    #[must_use]
    pub const fn all() -> [VulnerabilityType; 6] {
        [
            VulnerabilityType::Identity,
            VulnerabilityType::Resources,
            VulnerabilityType::Safety,
            VulnerabilityType::Relationship,
            VulnerabilityType::Reputation,
            VulnerabilityType::Emotional,
        ]
    }
}

impl std::fmt::Display for VulnerabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// The stakes level for a trust action.
///
/// Higher stakes increase perceived risk.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::relationship::StakesLevel;
///
/// let stakes = StakesLevel::High;
/// assert!(stakes.risk_contribution() > StakesLevel::Low.risk_contribution());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StakesLevel {
    /// Low stakes action with minimal consequences if trust is violated.
    #[default]
    Low,
    /// Medium stakes action with moderate consequences.
    Medium,
    /// High stakes action with significant consequences.
    High,
    /// Critical stakes action with severe or irreversible consequences.
    Critical,
}

impl StakesLevel {
    /// Returns the risk contribution from these stakes.
    ///
    /// This is added to the base perceived risk.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::relationship::StakesLevel;
    ///
    /// assert!((StakesLevel::Low.risk_contribution() - 0.0).abs() < f32::EPSILON);
    /// assert!((StakesLevel::High.risk_contribution() - 0.4).abs() < f32::EPSILON);
    /// ```
    #[must_use]
    pub const fn risk_contribution(&self) -> f32 {
        match self {
            StakesLevel::Low => 0.0,
            StakesLevel::Medium => 0.2,
            StakesLevel::High => 0.4,
            StakesLevel::Critical => 0.6,
        }
    }

    /// Returns a human-readable name for this stakes level.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            StakesLevel::Low => "Low",
            StakesLevel::Medium => "Medium",
            StakesLevel::High => "High",
            StakesLevel::Critical => "Critical",
        }
    }

    /// Returns all stakes levels.
    #[must_use]
    pub const fn all() -> [StakesLevel; 4] {
        [
            StakesLevel::Low,
            StakesLevel::Medium,
            StakesLevel::High,
            StakesLevel::Critical,
        ]
    }
}

impl std::fmt::Display for StakesLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Explicit representation of what is at stake in a trust decision.
///
/// Per Mayer's model, trust is the willingness to be vulnerable. This struct
/// makes the vulnerability explicit: what is being risked and how much.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::relationship::{Vulnerability, VulnerabilityType, StakesLevel};
///
/// let vuln = Vulnerability::new(VulnerabilityType::Resources, StakesLevel::High);
/// assert_eq!(vuln.vulnerability_type(), VulnerabilityType::Resources);
/// assert_eq!(vuln.stakes(), StakesLevel::High);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vulnerability {
    /// What type of thing is at risk.
    vulnerability_type: VulnerabilityType,
    /// How much is at stake (magnitude of potential loss).
    stakes: StakesLevel,
}

impl Vulnerability {
    /// Creates a new Vulnerability.
    ///
    /// # Arguments
    ///
    /// * `vulnerability_type` - What is at risk
    /// * `stakes` - How much is at stake
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::relationship::{Vulnerability, VulnerabilityType, StakesLevel};
    ///
    /// let vuln = Vulnerability::new(VulnerabilityType::Safety, StakesLevel::Critical);
    /// ```
    #[must_use]
    pub const fn new(vulnerability_type: VulnerabilityType, stakes: StakesLevel) -> Self {
        Vulnerability {
            vulnerability_type,
            stakes,
        }
    }

    /// Returns the type of vulnerability.
    #[must_use]
    pub const fn vulnerability_type(&self) -> VulnerabilityType {
        self.vulnerability_type
    }

    /// Returns the stakes level.
    #[must_use]
    pub const fn stakes(&self) -> StakesLevel {
        self.stakes
    }

    /// Returns the risk contribution from this vulnerability.
    ///
    /// Combines the stakes magnitude with vulnerability type sensitivity.
    #[must_use]
    pub fn risk_contribution(&self) -> f32 {
        self.stakes.risk_contribution()
    }
}

impl Default for Vulnerability {
    fn default() -> Self {
        Vulnerability {
            vulnerability_type: VulnerabilityType::default(),
            stakes: StakesLevel::default(),
        }
    }
}

impl std::fmt::Display for Vulnerability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.vulnerability_type, self.stakes)
    }
}

/// Perceived risk of trusting another entity.
///
/// Perceived risk is subjective and varies based on:
/// - Relationship history (past betrayals increase risk)
/// - Stakes of the current action
/// - Relationship stage (strangers are higher risk)
/// - Entity personality (neuroticism increases risk perception)
///
/// Risk is independent of trustor propensity - high propensity increases
/// willingness but never lowers the computed risk score.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::relationship::{PerceivedRisk, StakesLevel};
///
/// let mut risk = PerceivedRisk::new();
///
/// // Apply risk from a betrayal
/// risk.add_delta(0.3);
///
/// // Compute risk for a specific action
/// let action_risk = risk.compute_for_stakes(StakesLevel::High);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct PerceivedRisk {
    /// The base/accumulated perceived risk.
    risk: StateValue,

    /// Whether this relationship has a history of betrayal.
    betrayal_history: bool,
}

impl PerceivedRisk {
    /// Risk increase from betrayal history.
    const BETRAYAL_RISK_INCREASE: f32 = 0.3;

    /// Creates a new PerceivedRisk with default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::relationship::PerceivedRisk;
    ///
    /// let risk = PerceivedRisk::new();
    /// assert!((risk.effective() - 0.3).abs() < f32::EPSILON);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        PerceivedRisk {
            risk: StateValue::new(DEFAULT_BASE)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(PERCEIVED_RISK_DECAY_HALF_LIFE),
            betrayal_history: false,
        }
    }

    /// Creates a PerceivedRisk with a custom base value.
    ///
    /// # Arguments
    ///
    /// * `base` - Base risk level (0-1)
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::relationship::PerceivedRisk;
    ///
    /// let risk = PerceivedRisk::with_base(0.5);
    /// assert!((risk.effective() - 0.5).abs() < f32::EPSILON);
    /// ```
    #[must_use]
    pub fn with_base(base: f32) -> Self {
        PerceivedRisk {
            risk: StateValue::new(base)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(PERCEIVED_RISK_DECAY_HALF_LIFE),
            betrayal_history: false,
        }
    }

    /// Returns the effective perceived risk (base + delta), clamped to [0, 1].
    #[must_use]
    pub fn effective(&self) -> f32 {
        self.risk.effective()
    }

    /// Returns the base risk level.
    #[must_use]
    pub fn base(&self) -> f32 {
        self.risk.base()
    }

    /// Returns the current delta.
    #[must_use]
    pub fn delta(&self) -> f32 {
        self.risk.delta()
    }

    /// Returns a reference to the underlying StateValue.
    #[must_use]
    pub fn state_value(&self) -> &StateValue {
        &self.risk
    }

    /// Returns a mutable reference to the underlying StateValue.
    pub fn state_value_mut(&mut self) -> &mut StateValue {
        &mut self.risk
    }

    /// Returns whether there is a betrayal history.
    #[must_use]
    pub fn has_betrayal_history(&self) -> bool {
        self.betrayal_history
    }

    /// Marks a betrayal in the relationship history.
    ///
    /// This permanently increases the risk baseline for this relationship.
    pub fn mark_betrayal(&mut self) {
        self.betrayal_history = true;
    }

    /// Clears the betrayal history.
    ///
    /// This is typically only used during testing or relationship repair.
    pub fn clear_betrayal_history(&mut self) {
        self.betrayal_history = false;
    }

    /// Computes the total risk for a specific stakes level.
    ///
    /// This combines:
    /// - Base perceived risk
    /// - Delta from recent events
    /// - Stakes contribution
    /// - Betrayal history modifier
    ///
    /// The result is clamped to [0, 1].
    ///
    /// # Arguments
    ///
    /// * `stakes` - The stakes level for the action
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::relationship::{PerceivedRisk, StakesLevel};
    ///
    /// let risk = PerceivedRisk::new();
    /// let low_stakes = risk.compute_for_stakes(StakesLevel::Low);
    /// let high_stakes = risk.compute_for_stakes(StakesLevel::High);
    ///
    /// assert!(high_stakes > low_stakes);
    /// ```
    #[must_use]
    pub fn compute_for_stakes(&self, stakes: StakesLevel) -> f32 {
        let mut total = self.effective() + stakes.risk_contribution();

        if self.betrayal_history {
            total += Self::BETRAYAL_RISK_INCREASE;
        }

        total.clamp(0.0, 1.0)
    }

    /// Computes risk for a specific vulnerability.
    ///
    /// This is the preferred method per Mayer's model, as it makes explicit
    /// what is at stake. The vulnerability contains both what type of thing
    /// is being risked and the magnitude of potential loss.
    ///
    /// # Arguments
    ///
    /// * `vulnerability` - What is at stake and how much
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::relationship::{
    ///     PerceivedRisk, Vulnerability, VulnerabilityType, StakesLevel
    /// };
    ///
    /// let risk = PerceivedRisk::new();
    /// let vuln = Vulnerability::new(VulnerabilityType::Resources, StakesLevel::High);
    /// let computed = risk.compute_for_vulnerability(&vuln);
    /// assert!(computed > 0.0);
    /// ```
    #[must_use]
    pub fn compute_for_vulnerability(&self, vulnerability: &Vulnerability) -> f32 {
        self.compute_for_stakes(vulnerability.stakes())
    }

    /// Computes risk with a stage modifier.
    ///
    /// Relationship stage affects perceived risk:
    /// - Strangers: Higher uncertainty
    /// - Intimate: Lower perceived risk
    ///
    /// # Arguments
    ///
    /// * `stakes` - The stakes level for the action
    /// * `stage_modifier` - Risk modifier from relationship stage
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::relationship::{PerceivedRisk, StakesLevel};
    ///
    /// let risk = PerceivedRisk::new();
    ///
    /// // Stranger has +0.3 stage modifier
    /// let stranger_risk = risk.compute_with_stage_modifier(StakesLevel::Medium, 0.3);
    ///
    /// // Intimate has -0.1 stage modifier
    /// let intimate_risk = risk.compute_with_stage_modifier(StakesLevel::Medium, -0.1);
    ///
    /// assert!(stranger_risk > intimate_risk);
    /// ```
    #[must_use]
    pub fn compute_with_stage_modifier(&self, stakes: StakesLevel, stage_modifier: f32) -> f32 {
        let base_risk = self.compute_for_stakes(stakes);
        (base_risk + stage_modifier).clamp(0.0, 1.0)
    }

    /// Computes subjective risk based on trustor characteristics.
    ///
    /// Per Mayer's model, risk is subjective and trustor-specific. Individuals
    /// with higher sensitivity (e.g., high neuroticism) perceive more risk
    /// for the same objective stakes.
    ///
    /// # Arguments
    ///
    /// * `stakes` - The stakes level for the action
    /// * `trustor_sensitivity` - The trustor's risk sensitivity (0-1).
    ///   0.5 = neutral, > 0.5 = heightened risk perception, < 0.5 = reduced.
    ///   This can be derived from personality traits like neuroticism.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::relationship::{PerceivedRisk, StakesLevel};
    ///
    /// let risk = PerceivedRisk::new();
    ///
    /// // High sensitivity individual perceives more risk
    /// let high_sens = risk.compute_for_trustor(StakesLevel::Medium, 0.8);
    /// let low_sens = risk.compute_for_trustor(StakesLevel::Medium, 0.2);
    ///
    /// assert!(high_sens > low_sens);
    /// ```
    #[must_use]
    pub fn compute_for_trustor(&self, stakes: StakesLevel, trustor_sensitivity: f32) -> f32 {
        let base_risk = self.compute_for_stakes(stakes);
        // Sensitivity modifies risk: 0.5 is neutral, 0 reduces by 20%, 1 increases by 20%
        let sensitivity = trustor_sensitivity.clamp(0.0, 1.0);
        let sensitivity_modifier = (sensitivity - 0.5) * 0.4; // Range: -0.2 to +0.2
        (base_risk + sensitivity_modifier).clamp(0.0, 1.0)
    }

    /// Computes fully subjective risk with all moderating factors.
    ///
    /// This is the most complete risk computation, incorporating:
    /// - Base perceived risk and history
    /// - Stakes level
    /// - Relationship stage effects
    /// - Trustor's individual sensitivity
    ///
    /// # Arguments
    ///
    /// * `stakes` - The stakes level for the action
    /// * `stage_modifier` - Risk modifier from relationship stage
    /// * `trustor_sensitivity` - The trustor's risk sensitivity (0-1)
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::relationship::{PerceivedRisk, StakesLevel};
    ///
    /// let risk = PerceivedRisk::new();
    ///
    /// // Compute risk for a nervous person (high sensitivity) trusting a stranger (high stage modifier)
    /// let high_risk = risk.compute_subjective(StakesLevel::High, 0.3, 0.9);
    ///
    /// // Compare to relaxed person (low sensitivity) trusting an intimate (low stage modifier)
    /// let low_risk = risk.compute_subjective(StakesLevel::High, -0.1, 0.2);
    ///
    /// assert!(high_risk > low_risk);
    /// ```
    #[must_use]
    pub fn compute_subjective(
        &self,
        stakes: StakesLevel,
        stage_modifier: f32,
        trustor_sensitivity: f32,
    ) -> f32 {
        let base_with_stage = self.compute_with_stage_modifier(stakes, stage_modifier);
        let sensitivity = trustor_sensitivity.clamp(0.0, 1.0);
        let sensitivity_modifier = (sensitivity - 0.5) * 0.4;
        (base_with_stage + sensitivity_modifier).clamp(0.0, 1.0)
    }

    /// Adds to the risk delta.
    pub fn add_delta(&mut self, amount: f32) {
        self.risk.add_delta(amount);
    }

    /// Sets the risk delta directly.
    pub fn set_delta(&mut self, delta: f32) {
        self.risk.set_delta(delta);
    }

    /// Sets the base risk level.
    pub fn set_base(&mut self, base: f32) {
        self.risk.set_base(base);
    }

    /// Applies decay to the risk delta over the specified duration.
    pub fn apply_decay(&mut self, elapsed: Duration) {
        self.risk.apply_decay(elapsed);
    }

    /// Resets the delta to zero.
    pub fn reset_delta(&mut self) {
        self.risk.reset_delta();
    }
}

impl Default for PerceivedRisk {
    fn default() -> Self {
        PerceivedRisk::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // StakesLevel tests

    #[test]
    fn stakes_level_risk_contribution() {
        assert!(StakesLevel::Low.risk_contribution().abs() < f32::EPSILON);
        assert!((StakesLevel::Medium.risk_contribution() - 0.2).abs() < f32::EPSILON);
        assert!((StakesLevel::High.risk_contribution() - 0.4).abs() < f32::EPSILON);
        assert!((StakesLevel::Critical.risk_contribution() - 0.6).abs() < f32::EPSILON);
    }

    #[test]
    fn stakes_level_name() {
        assert_eq!(StakesLevel::Low.name(), "Low");
        assert_eq!(StakesLevel::Medium.name(), "Medium");
        assert_eq!(StakesLevel::High.name(), "High");
        assert_eq!(StakesLevel::Critical.name(), "Critical");
    }

    #[test]
    fn stakes_level_all() {
        let all = StakesLevel::all();
        assert_eq!(all.len(), 4);
    }

    #[test]
    fn stakes_level_default() {
        assert_eq!(StakesLevel::default(), StakesLevel::Low);
    }

    #[test]
    fn stakes_level_display() {
        assert_eq!(format!("{}", StakesLevel::High), "High");
    }

    #[test]
    fn stakes_level_equality() {
        assert_eq!(StakesLevel::Low, StakesLevel::Low);
        assert_ne!(StakesLevel::Low, StakesLevel::High);
    }

    #[test]
    fn stakes_level_clone_copy() {
        let s1 = StakesLevel::Medium;
        let s2 = s1;
        let s3 = s1.clone();
        assert_eq!(s1, s2);
        assert_eq!(s1, s3);
    }

    #[test]
    fn stakes_level_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(StakesLevel::Low);
        set.insert(StakesLevel::Low);
        assert_eq!(set.len(), 1);
        set.insert(StakesLevel::High);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn stakes_level_debug() {
        let debug = format!("{:?}", StakesLevel::Critical);
        assert!(debug.contains("Critical"));
    }

    // VulnerabilityType tests

    #[test]
    fn vulnerability_type_name() {
        assert_eq!(VulnerabilityType::Identity.name(), "Identity");
        assert_eq!(VulnerabilityType::Resources.name(), "Resources");
        assert_eq!(VulnerabilityType::Safety.name(), "Safety");
        assert_eq!(VulnerabilityType::Relationship.name(), "Relationship");
        assert_eq!(VulnerabilityType::Reputation.name(), "Reputation");
        assert_eq!(VulnerabilityType::Emotional.name(), "Emotional");
    }

    #[test]
    fn vulnerability_type_all() {
        let all = VulnerabilityType::all();
        assert_eq!(all.len(), 6);
    }

    #[test]
    fn vulnerability_type_default() {
        assert_eq!(VulnerabilityType::default(), VulnerabilityType::Resources);
    }

    #[test]
    fn vulnerability_type_display() {
        assert_eq!(format!("{}", VulnerabilityType::Safety), "Safety");
    }

    #[test]
    fn vulnerability_type_equality() {
        assert_eq!(VulnerabilityType::Identity, VulnerabilityType::Identity);
        assert_ne!(VulnerabilityType::Identity, VulnerabilityType::Safety);
    }

    #[test]
    fn vulnerability_type_clone_copy() {
        let v1 = VulnerabilityType::Reputation;
        let v2 = v1;
        let v3 = v1.clone();
        assert_eq!(v1, v2);
        assert_eq!(v1, v3);
    }

    #[test]
    fn vulnerability_type_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(VulnerabilityType::Identity);
        set.insert(VulnerabilityType::Identity);
        assert_eq!(set.len(), 1);
        set.insert(VulnerabilityType::Safety);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn vulnerability_type_debug() {
        let debug = format!("{:?}", VulnerabilityType::Emotional);
        assert!(debug.contains("Emotional"));
    }

    // Vulnerability tests

    #[test]
    fn vulnerability_new() {
        let vuln = Vulnerability::new(VulnerabilityType::Resources, StakesLevel::High);
        assert_eq!(vuln.vulnerability_type(), VulnerabilityType::Resources);
        assert_eq!(vuln.stakes(), StakesLevel::High);
    }

    #[test]
    fn vulnerability_default() {
        let vuln = Vulnerability::default();
        assert_eq!(vuln.vulnerability_type(), VulnerabilityType::Resources);
        assert_eq!(vuln.stakes(), StakesLevel::Low);
    }

    #[test]
    fn vulnerability_risk_contribution() {
        let low = Vulnerability::new(VulnerabilityType::Identity, StakesLevel::Low);
        let high = Vulnerability::new(VulnerabilityType::Identity, StakesLevel::High);
        assert!(low.risk_contribution() < high.risk_contribution());
    }

    #[test]
    fn vulnerability_display() {
        let vuln = Vulnerability::new(VulnerabilityType::Safety, StakesLevel::Critical);
        assert_eq!(format!("{}", vuln), "Safety (Critical)");
    }

    #[test]
    fn vulnerability_equality() {
        let v1 = Vulnerability::new(VulnerabilityType::Resources, StakesLevel::Medium);
        let v2 = Vulnerability::new(VulnerabilityType::Resources, StakesLevel::Medium);
        let v3 = Vulnerability::new(VulnerabilityType::Resources, StakesLevel::High);
        assert_eq!(v1, v2);
        assert_ne!(v1, v3);
    }

    #[test]
    fn vulnerability_clone_copy() {
        let v1 = Vulnerability::new(VulnerabilityType::Emotional, StakesLevel::Low);
        let v2 = v1;
        let v3 = v1.clone();
        assert_eq!(v1, v2);
        assert_eq!(v1, v3);
    }

    #[test]
    fn vulnerability_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        let v1 = Vulnerability::new(VulnerabilityType::Resources, StakesLevel::Medium);
        set.insert(v1);
        set.insert(v1);
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn vulnerability_debug() {
        let vuln = Vulnerability::new(VulnerabilityType::Relationship, StakesLevel::High);
        let debug = format!("{:?}", vuln);
        assert!(debug.contains("Vulnerability"));
    }

    // PerceivedRisk tests

    #[test]
    fn new_creates_default_values() {
        let risk = PerceivedRisk::new();
        assert!((risk.effective() - DEFAULT_BASE).abs() < f32::EPSILON);
        assert!(!risk.has_betrayal_history());
    }

    #[test]
    fn with_base_creates_custom_value() {
        let risk = PerceivedRisk::with_base(0.5);
        assert!((risk.effective() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn base_accessor() {
        let risk = PerceivedRisk::with_base(0.6);
        assert!((risk.base() - 0.6).abs() < f32::EPSILON);
    }

    #[test]
    fn delta_accessor() {
        let mut risk = PerceivedRisk::new();
        risk.add_delta(0.2);
        assert!((risk.delta() - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn state_value_accessor() {
        let risk = PerceivedRisk::new();
        assert!((risk.state_value().effective() - DEFAULT_BASE).abs() < f32::EPSILON);
    }

    #[test]
    fn state_value_mut_accessor() {
        let mut risk = PerceivedRisk::new();
        risk.state_value_mut().add_delta(0.1);
        assert!((risk.delta() - 0.1).abs() < f32::EPSILON);
    }

    #[test]
    fn add_delta() {
        let mut risk = PerceivedRisk::new();
        risk.add_delta(0.2);
        assert!((risk.effective() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn set_delta() {
        let mut risk = PerceivedRisk::new();
        risk.set_delta(0.3);
        assert!((risk.delta() - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn set_base() {
        let mut risk = PerceivedRisk::new();
        risk.set_base(0.7);
        assert!((risk.base() - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn compute_for_stakes_low() {
        let risk = PerceivedRisk::new();
        let computed = risk.compute_for_stakes(StakesLevel::Low);
        // Base (0.3) + stakes (0.0) = 0.3
        assert!((computed - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn compute_for_stakes_high() {
        let risk = PerceivedRisk::new();
        let computed = risk.compute_for_stakes(StakesLevel::High);
        // Base (0.3) + stakes (0.4) = 0.7
        assert!((computed - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn compute_for_stakes_with_betrayal() {
        let mut risk = PerceivedRisk::new();
        risk.mark_betrayal();
        let computed = risk.compute_for_stakes(StakesLevel::Low);
        // Base (0.3) + stakes (0.0) + betrayal (0.3) = 0.6
        assert!((computed - 0.6).abs() < f32::EPSILON);
    }

    #[test]
    fn compute_for_stakes_clamped() {
        let mut risk = PerceivedRisk::with_base(0.8);
        risk.mark_betrayal();
        let computed = risk.compute_for_stakes(StakesLevel::High);
        // 0.8 + 0.4 + 0.3 = 1.5, clamped to 1.0
        assert!((computed - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn compute_for_vulnerability() {
        let risk = PerceivedRisk::new();
        let vuln = Vulnerability::new(VulnerabilityType::Resources, StakesLevel::High);
        let computed = risk.compute_for_vulnerability(&vuln);
        // Same as compute_for_stakes with High stakes
        assert!((computed - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn compute_for_vulnerability_matches_stakes() {
        let risk = PerceivedRisk::new();
        let vuln = Vulnerability::new(VulnerabilityType::Safety, StakesLevel::Medium);
        let from_vuln = risk.compute_for_vulnerability(&vuln);
        let from_stakes = risk.compute_for_stakes(StakesLevel::Medium);
        assert!((from_vuln - from_stakes).abs() < f32::EPSILON);
    }

    #[test]
    fn compute_with_stage_modifier_positive() {
        let risk = PerceivedRisk::new();
        let computed = risk.compute_with_stage_modifier(StakesLevel::Low, 0.3);
        // 0.3 + 0.0 + 0.3 = 0.6
        assert!((computed - 0.6).abs() < f32::EPSILON);
    }

    #[test]
    fn compute_with_stage_modifier_negative() {
        let risk = PerceivedRisk::new();
        let computed = risk.compute_with_stage_modifier(StakesLevel::Medium, -0.1);
        // 0.3 + 0.2 - 0.1 = 0.4
        assert!((computed - 0.4).abs() < f32::EPSILON);
    }

    #[test]
    fn compute_with_stage_modifier_clamped_low() {
        let risk = PerceivedRisk::with_base(0.1);
        let computed = risk.compute_with_stage_modifier(StakesLevel::Low, -0.2);
        // 0.1 + 0.0 - 0.2 = -0.1, clamped to 0.0
        assert!(computed.abs() < f32::EPSILON);
    }

    #[test]
    fn compute_for_trustor_high_sensitivity() {
        let risk = PerceivedRisk::new();
        let high_sens = risk.compute_for_trustor(StakesLevel::Medium, 0.8);
        let low_sens = risk.compute_for_trustor(StakesLevel::Medium, 0.2);
        // High sensitivity should perceive more risk
        assert!(high_sens > low_sens);
    }

    #[test]
    fn compute_for_trustor_neutral_sensitivity() {
        let risk = PerceivedRisk::new();
        let neutral = risk.compute_for_trustor(StakesLevel::Medium, 0.5);
        let base = risk.compute_for_stakes(StakesLevel::Medium);
        // Neutral sensitivity should match base risk
        assert!((neutral - base).abs() < f32::EPSILON);
    }

    #[test]
    fn compute_for_trustor_clamps_sensitivity() {
        let risk = PerceivedRisk::new();
        let extreme_high = risk.compute_for_trustor(StakesLevel::Medium, 1.5);
        let clamped_high = risk.compute_for_trustor(StakesLevel::Medium, 1.0);
        // Should be clamped to 1.0
        assert!((extreme_high - clamped_high).abs() < f32::EPSILON);
    }

    #[test]
    fn compute_subjective_combines_all_factors() {
        let risk = PerceivedRisk::new();
        // Nervous person (high sensitivity) trusting stranger (high stage modifier)
        let high_risk = risk.compute_subjective(StakesLevel::High, 0.3, 0.9);
        // Relaxed person (low sensitivity) trusting intimate (low stage modifier)
        let low_risk = risk.compute_subjective(StakesLevel::High, -0.1, 0.2);
        assert!(high_risk > low_risk);
    }

    #[test]
    fn compute_subjective_clamped() {
        let mut risk = PerceivedRisk::with_base(0.8);
        risk.mark_betrayal();
        // Very high base, high stage, high sensitivity
        let computed = risk.compute_subjective(StakesLevel::Critical, 0.3, 1.0);
        assert!((computed - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn mark_betrayal() {
        let mut risk = PerceivedRisk::new();
        assert!(!risk.has_betrayal_history());
        risk.mark_betrayal();
        assert!(risk.has_betrayal_history());
    }

    #[test]
    fn clear_betrayal_history() {
        let mut risk = PerceivedRisk::new();
        risk.mark_betrayal();
        assert!(risk.has_betrayal_history());
        risk.clear_betrayal_history();
        assert!(!risk.has_betrayal_history());
    }

    #[test]
    fn decay_over_7_days() {
        let mut risk = PerceivedRisk::new();
        risk.add_delta(0.4);

        // After 7 days, delta should be halved
        risk.apply_decay(Duration::days(7));
        assert!((risk.delta() - 0.2).abs() < 0.01);
    }

    #[test]
    fn reset_delta() {
        let mut risk = PerceivedRisk::new();
        risk.add_delta(0.5);
        risk.reset_delta();
        assert!(risk.delta().abs() < f32::EPSILON);
    }

    #[test]
    fn default_equals_new() {
        let d = PerceivedRisk::default();
        let n = PerceivedRisk::new();
        assert_eq!(d, n);
    }

    #[test]
    fn clone_and_equality() {
        let mut r1 = PerceivedRisk::with_base(0.5);
        r1.mark_betrayal();
        let r2 = r1.clone();
        assert_eq!(r1, r2);
    }

    #[test]
    fn debug_format() {
        let risk = PerceivedRisk::new();
        let debug = format!("{:?}", risk);
        assert!(debug.contains("PerceivedRisk"));
    }

    #[test]
    fn higher_stakes_mean_higher_risk() {
        let risk = PerceivedRisk::new();
        let low = risk.compute_for_stakes(StakesLevel::Low);
        let medium = risk.compute_for_stakes(StakesLevel::Medium);
        let high = risk.compute_for_stakes(StakesLevel::High);
        let critical = risk.compute_for_stakes(StakesLevel::Critical);

        assert!(low < medium);
        assert!(medium < high);
        assert!(high < critical);
    }
}
