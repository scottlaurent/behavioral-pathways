//! Alert trigger types for threshold notifications.
//!
//! This module defines what conditions can trigger an alert.
//!
//! # ITS Risk Matrix Alerts
//!
//! The `ItsAlert` enum implements Joiner's Interpersonal Theory of Suicide
//! risk matrix, which distinguishes different combinations of factor elevation:
//!
//! | TB | PB | AC | ItsAlert Variant |
//! |----|----|----|------------------|
//! | X  |    |    | SingleFactorTb |
//! |    | X  |    | SingleFactorPb |
//! |    |    | X  | SingleFactorAc |
//! | X  | X  |    | DesireWithoutCapability |
//! | X  |    | X  | TbWithCapability |
//! |    | X  | X  | PbWithCapability |
//! | X  | X  | X  | ThreeFactorConvergence |

use crate::enums::StatePath;
use crate::processor::{ConvergenceStatus, ItsProximalFactor};
use serde::{Deserialize, Serialize};

/// Type of feedback spiral detected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpiralType {
    /// Stress-fatigue-impulse control spiral.
    Stress,

    /// Depression-loneliness spiral (Human only).
    Depression,
}

impl SpiralType {
    /// Returns a human-readable name for this spiral type.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            SpiralType::Stress => "Stress Spiral",
            SpiralType::Depression => "Depression Spiral",
        }
    }
}

impl std::fmt::Display for SpiralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// ITS-specific alert based on Joiner's risk matrix.
///
/// This enum represents the clinically meaningful combinations of ITS factor
/// elevation, enabling targeted intervention recommendations.
///
/// # Risk Levels
///
/// - Single factor elevations: Generally low risk, passive ideation possible
/// - Two-factor combinations: Moderate risk, intervention recommended
/// - Three-factor convergence: HIGH RISK, immediate intervention needed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItsAlert {
    /// Only Thwarted Belongingness is elevated.
    /// Low risk - monitor for belonging needs, social connection.
    SingleFactorTb,

    /// Only Perceived Burdensomeness is elevated.
    /// Low risk - address self-worth, role contribution.
    SingleFactorPb,

    /// Only Acquired Capability is elevated (dormant capability).
    /// Low risk currently, but capability persists.
    /// Often seen in veterans, healthcare workers, etc.
    SingleFactorAc,

    /// Both TB and PB elevated, but not AC.
    /// MODERATE RISK - suicidal desire present without capability.
    /// This is the "desire without means" state.
    DesireWithoutCapability,

    /// TB and AC elevated, but not PB.
    /// Moderate risk - disconnection plus capability.
    TbWithCapability,

    /// PB and AC elevated, but not TB.
    /// Moderate risk - burdensomeness plus capability.
    PbWithCapability,

    /// All three factors elevated: TB, PB, and AC.
    /// HIGH RISK - suicidal desire plus capability for lethal action.
    /// This is the highest risk state in the ITS model.
    ThreeFactorConvergence,
}

impl ItsAlert {
    /// Creates an ItsAlert from a ConvergenceStatus, if any factors are elevated.
    ///
    /// Returns None if no factors are elevated.
    #[must_use]
    pub fn from_convergence(status: &ConvergenceStatus) -> Option<Self> {
        match (status.tb_elevated, status.pb_elevated, status.ac_elevated) {
            (true, true, true) => Some(ItsAlert::ThreeFactorConvergence),
            (true, true, false) => Some(ItsAlert::DesireWithoutCapability),
            (true, false, true) => Some(ItsAlert::TbWithCapability),
            (false, true, true) => Some(ItsAlert::PbWithCapability),
            (true, false, false) => Some(ItsAlert::SingleFactorTb),
            (false, true, false) => Some(ItsAlert::SingleFactorPb),
            (false, false, true) => Some(ItsAlert::SingleFactorAc),
            (false, false, false) => None,
        }
    }

    /// Returns the risk level of this alert (1-3).
    ///
    /// - 1: Low risk (single factor)
    /// - 2: Moderate risk (two factors)
    /// - 3: High risk (three-factor convergence)
    #[must_use]
    pub const fn risk_level(&self) -> u8 {
        match self {
            ItsAlert::SingleFactorTb
            | ItsAlert::SingleFactorPb
            | ItsAlert::SingleFactorAc => 1,

            ItsAlert::DesireWithoutCapability
            | ItsAlert::TbWithCapability
            | ItsAlert::PbWithCapability => 2,

            ItsAlert::ThreeFactorConvergence => 3,
        }
    }

    /// Returns true if this alert represents high risk (three-factor convergence).
    #[must_use]
    pub const fn is_high_risk(&self) -> bool {
        matches!(self, ItsAlert::ThreeFactorConvergence)
    }

    /// Returns true if suicidal desire is present (TB + PB elevated).
    #[must_use]
    pub const fn has_desire(&self) -> bool {
        matches!(
            self,
            ItsAlert::DesireWithoutCapability | ItsAlert::ThreeFactorConvergence
        )
    }

    /// Returns true if acquired capability is elevated.
    #[must_use]
    pub const fn has_capability(&self) -> bool {
        matches!(
            self,
            ItsAlert::SingleFactorAc
                | ItsAlert::TbWithCapability
                | ItsAlert::PbWithCapability
                | ItsAlert::ThreeFactorConvergence
        )
    }

    /// Returns the elevated factors for this alert.
    #[must_use]
    pub fn elevated_factors(&self) -> Vec<ItsProximalFactor> {
        match self {
            ItsAlert::SingleFactorTb => vec![ItsProximalFactor::ThwartedBelongingness],
            ItsAlert::SingleFactorPb => vec![ItsProximalFactor::PerceivedBurdensomeness],
            ItsAlert::SingleFactorAc => vec![ItsProximalFactor::AcquiredCapability],
            ItsAlert::DesireWithoutCapability => vec![
                ItsProximalFactor::ThwartedBelongingness,
                ItsProximalFactor::PerceivedBurdensomeness,
            ],
            ItsAlert::TbWithCapability => vec![
                ItsProximalFactor::ThwartedBelongingness,
                ItsProximalFactor::AcquiredCapability,
            ],
            ItsAlert::PbWithCapability => vec![
                ItsProximalFactor::PerceivedBurdensomeness,
                ItsProximalFactor::AcquiredCapability,
            ],
            ItsAlert::ThreeFactorConvergence => vec![
                ItsProximalFactor::ThwartedBelongingness,
                ItsProximalFactor::PerceivedBurdensomeness,
                ItsProximalFactor::AcquiredCapability,
            ],
        }
    }

    /// Returns a human-readable name for this alert.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            ItsAlert::SingleFactorTb => "Thwarted Belongingness Elevated",
            ItsAlert::SingleFactorPb => "Perceived Burdensomeness Elevated",
            ItsAlert::SingleFactorAc => "Acquired Capability Elevated",
            ItsAlert::DesireWithoutCapability => "Suicidal Desire Without Capability",
            ItsAlert::TbWithCapability => "Thwarted Belongingness With Capability",
            ItsAlert::PbWithCapability => "Perceived Burdensomeness With Capability",
            ItsAlert::ThreeFactorConvergence => "Three-Factor Convergence (High Risk)",
        }
    }

    /// Returns clinical intervention recommendations for this alert.
    #[must_use]
    pub const fn intervention_focus(&self) -> &'static str {
        match self {
            ItsAlert::SingleFactorTb => "Focus on social connection and belonging needs",
            ItsAlert::SingleFactorPb => "Address self-worth and perceived contribution",
            ItsAlert::SingleFactorAc => "Monitor for desire development; capability persists",
            ItsAlert::DesireWithoutCapability => {
                "Address both TB and PB urgently; prevent capability acquisition"
            }
            ItsAlert::TbWithCapability => {
                "Priority: social connection; monitor for PB development"
            }
            ItsAlert::PbWithCapability => {
                "Priority: self-worth; monitor for TB development"
            }
            ItsAlert::ThreeFactorConvergence => {
                "IMMEDIATE: Safety planning, means restriction, crisis intervention"
            }
        }
    }
}

impl std::fmt::Display for ItsAlert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// The condition that triggered an alert.
///
/// Alerts can be triggered by thresholds being exceeded, feedback loops
/// activating, or custom application-specific conditions.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::enums::{AlertTrigger, StatePath, MentalHealthPath};
///
/// let trigger = AlertTrigger::ThresholdExceeded(
///     StatePath::MentalHealth(MentalHealthPath::SuicidalDesire),
///     0.7,
/// );
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum AlertTrigger {
    /// A state dimension exceeded a threshold value.
    ThresholdExceeded(StatePath, f64),

    /// A feedback spiral was detected as active.
    SpiralDetected(SpiralType),

    /// A custom trigger with a description.
    Custom(String),
}

impl AlertTrigger {
    /// Creates a threshold exceeded trigger.
    #[must_use]
    pub const fn threshold(path: StatePath, value: f64) -> Self {
        AlertTrigger::ThresholdExceeded(path, value)
    }

    /// Creates a spiral detected trigger.
    #[must_use]
    pub const fn spiral(spiral_type: SpiralType) -> Self {
        AlertTrigger::SpiralDetected(spiral_type)
    }

    /// Creates a custom trigger.
    #[must_use]
    pub fn custom(description: impl Into<String>) -> Self {
        AlertTrigger::Custom(description.into())
    }

    /// Returns true if this is a threshold trigger.
    #[must_use]
    pub const fn is_threshold(&self) -> bool {
        matches!(self, AlertTrigger::ThresholdExceeded(_, _))
    }

    /// Returns true if this is a spiral trigger.
    #[must_use]
    pub const fn is_spiral(&self) -> bool {
        matches!(self, AlertTrigger::SpiralDetected(_))
    }

    /// Returns true if this is a custom trigger.
    #[must_use]
    pub const fn is_custom(&self) -> bool {
        matches!(self, AlertTrigger::Custom(_))
    }
}

impl std::fmt::Display for AlertTrigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlertTrigger::ThresholdExceeded(path, value) => {
                write!(f, "Threshold exceeded: {} = {:.2}", path, value)
            }
            AlertTrigger::SpiralDetected(spiral) => {
                write!(f, "Spiral detected: {}", spiral)
            }
            AlertTrigger::Custom(desc) => {
                write!(f, "Custom: {}", desc)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::MentalHealthPath;

    fn matches_threshold(
        trigger: &AlertTrigger,
        expected_path: StatePath,
        expected_value: f64,
    ) -> bool {
        match trigger {
            AlertTrigger::ThresholdExceeded(path, value) => {
                let path_matches = *path == expected_path;
                let value_matches = (*value - expected_value).abs() < f64::EPSILON;
                path_matches && value_matches
            }
            _ => false,
        }
    }

    fn matches_spiral(trigger: &AlertTrigger, expected: SpiralType) -> bool {
        match trigger {
            AlertTrigger::SpiralDetected(spiral_type) => *spiral_type == expected,
            _ => false,
        }
    }

    fn matches_custom(trigger: &AlertTrigger, expected: &str) -> bool {
        match trigger {
            AlertTrigger::Custom(desc) => desc == expected,
            _ => false,
        }
    }

    #[test]
    fn alert_trigger_variants_exist() {
        // Verify all variants can be constructed
        let threshold = AlertTrigger::ThresholdExceeded(
            StatePath::MentalHealth(MentalHealthPath::SuicidalDesire),
            0.7,
        );
        let spiral = AlertTrigger::SpiralDetected(SpiralType::Stress);
        let custom = AlertTrigger::Custom("Test trigger".to_string());

        assert!(threshold.is_threshold());
        assert!(spiral.is_spiral());
        assert!(custom.is_custom());
    }

    #[test]
    fn threshold_trigger_constructor() {
        let trigger =
            AlertTrigger::threshold(StatePath::MentalHealth(MentalHealthPath::Depression), 0.8);

        assert!(matches_threshold(
            &trigger,
            StatePath::MentalHealth(MentalHealthPath::Depression),
            0.8
        ));
        assert!(!matches_threshold(
            &AlertTrigger::spiral(SpiralType::Stress),
            StatePath::MentalHealth(MentalHealthPath::Depression),
            0.8
        ));
    }

    #[test]
    fn spiral_trigger_constructor() {
        let trigger = AlertTrigger::spiral(SpiralType::Depression);

        assert!(matches_spiral(&trigger, SpiralType::Depression));
        assert!(!matches_spiral(
            &AlertTrigger::threshold(StatePath::MentalHealth(MentalHealthPath::Depression), 0.5),
            SpiralType::Depression
        ));
    }

    #[test]
    fn custom_trigger_constructor() {
        let trigger = AlertTrigger::custom("High risk detected");

        assert!(matches_custom(&trigger, "High risk detected"));
        assert!(!matches_custom(
            &AlertTrigger::spiral(SpiralType::Stress),
            "High risk detected"
        ));
    }

    #[test]
    fn spiral_type_names() {
        assert_eq!(SpiralType::Stress.name(), "Stress Spiral");
        assert_eq!(SpiralType::Depression.name(), "Depression Spiral");
    }

    #[test]
    fn spiral_type_display() {
        assert_eq!(format!("{}", SpiralType::Stress), "Stress Spiral");
        assert_eq!(format!("{}", SpiralType::Depression), "Depression Spiral");
    }

    #[test]
    fn trigger_display() {
        let threshold = AlertTrigger::ThresholdExceeded(
            StatePath::MentalHealth(MentalHealthPath::SuicidalDesire),
            0.75,
        );
        let display = format!("{}", threshold);
        assert!(display.contains("Threshold exceeded"));
        assert!(display.contains("0.75"));

        let spiral = AlertTrigger::SpiralDetected(SpiralType::Stress);
        let spiral_display = format!("{}", spiral);
        assert!(spiral_display.contains("Spiral detected"));
        assert!(spiral_display.contains("Stress"));

        let custom = AlertTrigger::Custom("Test".to_string());
        let custom_display = format!("{}", custom);
        assert!(custom_display.contains("Custom"));
        assert!(custom_display.contains("Test"));
    }

    #[test]
    fn trigger_debug() {
        let trigger = AlertTrigger::SpiralDetected(SpiralType::Depression);
        let debug = format!("{:?}", trigger);
        assert!(debug.contains("SpiralDetected"));
    }

    #[test]
    fn trigger_clone() {
        let original = AlertTrigger::Custom("Test".to_string());
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn spiral_type_clone_copy() {
        let s1 = SpiralType::Stress;
        let s2 = s1; // Copy
        let s3 = s1.clone();
        assert_eq!(s1, s2);
        assert_eq!(s1, s3);
    }

    #[test]
    fn spiral_type_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(SpiralType::Stress);
        set.insert(SpiralType::Stress);
        assert_eq!(set.len(), 1);

        set.insert(SpiralType::Depression);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn is_methods_exclusive() {
        let threshold =
            AlertTrigger::threshold(StatePath::MentalHealth(MentalHealthPath::Depression), 0.5);
        assert!(threshold.is_threshold());
        assert!(!threshold.is_spiral());
        assert!(!threshold.is_custom());

        let spiral = AlertTrigger::spiral(SpiralType::Stress);
        assert!(!spiral.is_threshold());
        assert!(spiral.is_spiral());
        assert!(!spiral.is_custom());

        let custom = AlertTrigger::custom("test");
        assert!(!custom.is_threshold());
        assert!(!custom.is_spiral());
        assert!(custom.is_custom());
    }

    #[test]
    fn threshold_trigger_display_format() {
        let trigger = AlertTrigger::ThresholdExceeded(
            StatePath::MentalHealth(MentalHealthPath::SuicidalDesire),
            0.75,
        );
        let display = format!("{}", trigger);
        assert!(display.contains("Threshold exceeded"));
        assert!(display.contains("0.75"));
        assert!(display.contains("MentalHealth"));
    }

    #[test]
    fn spiral_trigger_display_format() {
        let trigger = AlertTrigger::SpiralDetected(SpiralType::Depression);
        let display = format!("{}", trigger);
        assert!(display.contains("Spiral detected"));
        assert!(display.contains("Depression"));
    }

    #[test]
    fn custom_trigger_display_format() {
        let trigger = AlertTrigger::Custom("High risk intervention needed".to_string());
        let display = format!("{}", trigger);
        assert!(display.contains("Custom"));
        assert!(display.contains("High risk intervention needed"));
    }

    // --- ItsAlert tests ---

    #[test]
    fn its_alert_from_convergence_none() {
        let status = ConvergenceStatus::from_factors(0.3, 0.3, 0.1);
        assert!(ItsAlert::from_convergence(&status).is_none());
    }

    #[test]
    fn its_alert_from_convergence_single_tb() {
        let status = ConvergenceStatus::from_factors(0.7, 0.3, 0.1);
        let alert = ItsAlert::from_convergence(&status);
        assert_eq!(alert, Some(ItsAlert::SingleFactorTb));
    }

    #[test]
    fn its_alert_from_convergence_single_pb() {
        let status = ConvergenceStatus::from_factors(0.3, 0.7, 0.1);
        let alert = ItsAlert::from_convergence(&status);
        assert_eq!(alert, Some(ItsAlert::SingleFactorPb));
    }

    #[test]
    fn its_alert_from_convergence_single_ac() {
        let status = ConvergenceStatus::from_factors(0.3, 0.3, 0.5);
        let alert = ItsAlert::from_convergence(&status);
        assert_eq!(alert, Some(ItsAlert::SingleFactorAc));
    }

    #[test]
    fn its_alert_from_convergence_desire_only() {
        let status = ConvergenceStatus::from_factors(0.7, 0.7, 0.1);
        let alert = ItsAlert::from_convergence(&status);
        assert_eq!(alert, Some(ItsAlert::DesireWithoutCapability));
    }

    #[test]
    fn its_alert_from_convergence_tb_with_ac() {
        let status = ConvergenceStatus::from_factors(0.7, 0.3, 0.5);
        let alert = ItsAlert::from_convergence(&status);
        assert_eq!(alert, Some(ItsAlert::TbWithCapability));
    }

    #[test]
    fn its_alert_from_convergence_pb_with_ac() {
        let status = ConvergenceStatus::from_factors(0.3, 0.7, 0.5);
        let alert = ItsAlert::from_convergence(&status);
        assert_eq!(alert, Some(ItsAlert::PbWithCapability));
    }

    #[test]
    fn its_alert_from_convergence_three_factor() {
        let status = ConvergenceStatus::from_factors(0.7, 0.7, 0.5);
        let alert = ItsAlert::from_convergence(&status);
        assert_eq!(alert, Some(ItsAlert::ThreeFactorConvergence));
    }

    #[test]
    fn its_alert_risk_levels() {
        assert_eq!(ItsAlert::SingleFactorTb.risk_level(), 1);
        assert_eq!(ItsAlert::SingleFactorPb.risk_level(), 1);
        assert_eq!(ItsAlert::SingleFactorAc.risk_level(), 1);
        assert_eq!(ItsAlert::DesireWithoutCapability.risk_level(), 2);
        assert_eq!(ItsAlert::TbWithCapability.risk_level(), 2);
        assert_eq!(ItsAlert::PbWithCapability.risk_level(), 2);
        assert_eq!(ItsAlert::ThreeFactorConvergence.risk_level(), 3);
    }

    #[test]
    fn its_alert_is_high_risk() {
        assert!(!ItsAlert::SingleFactorTb.is_high_risk());
        assert!(!ItsAlert::DesireWithoutCapability.is_high_risk());
        assert!(ItsAlert::ThreeFactorConvergence.is_high_risk());
    }

    #[test]
    fn its_alert_has_desire() {
        assert!(!ItsAlert::SingleFactorTb.has_desire());
        assert!(!ItsAlert::SingleFactorPb.has_desire());
        assert!(!ItsAlert::SingleFactorAc.has_desire());
        assert!(ItsAlert::DesireWithoutCapability.has_desire());
        assert!(!ItsAlert::TbWithCapability.has_desire());
        assert!(!ItsAlert::PbWithCapability.has_desire());
        assert!(ItsAlert::ThreeFactorConvergence.has_desire());
    }

    #[test]
    fn its_alert_has_capability() {
        assert!(!ItsAlert::SingleFactorTb.has_capability());
        assert!(!ItsAlert::SingleFactorPb.has_capability());
        assert!(ItsAlert::SingleFactorAc.has_capability());
        assert!(!ItsAlert::DesireWithoutCapability.has_capability());
        assert!(ItsAlert::TbWithCapability.has_capability());
        assert!(ItsAlert::PbWithCapability.has_capability());
        assert!(ItsAlert::ThreeFactorConvergence.has_capability());
    }

    #[test]
    fn its_alert_elevated_factors_count() {
        assert_eq!(ItsAlert::SingleFactorTb.elevated_factors().len(), 1);
        assert_eq!(ItsAlert::DesireWithoutCapability.elevated_factors().len(), 2);
        assert_eq!(ItsAlert::ThreeFactorConvergence.elevated_factors().len(), 3);
    }

    #[test]
    fn its_alert_names() {
        assert_eq!(ItsAlert::SingleFactorTb.name(), "Thwarted Belongingness Elevated");
        assert_eq!(
            ItsAlert::ThreeFactorConvergence.name(),
            "Three-Factor Convergence (High Risk)"
        );
    }

    #[test]
    fn its_alert_intervention_focus() {
        let focus = ItsAlert::ThreeFactorConvergence.intervention_focus();
        assert!(focus.contains("IMMEDIATE"));
        assert!(focus.contains("Safety planning"));
    }

    #[test]
    fn its_alert_display() {
        assert_eq!(
            format!("{}", ItsAlert::DesireWithoutCapability),
            "Suicidal Desire Without Capability"
        );
    }

    #[test]
    fn its_alert_copy_clone() {
        let a1 = ItsAlert::SingleFactorTb;
        let a2 = a1; // Copy
        let a3 = a1.clone();
        assert_eq!(a1, a2);
        assert_eq!(a1, a3);
    }

    #[test]
    fn its_alert_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(ItsAlert::SingleFactorTb);
        set.insert(ItsAlert::SingleFactorTb);
        assert_eq!(set.len(), 1);

        set.insert(ItsAlert::ThreeFactorConvergence);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn its_alert_debug() {
        let debug = format!("{:?}", ItsAlert::ThreeFactorConvergence);
        assert!(debug.contains("ThreeFactorConvergence"));
    }

    // --- Additional ItsAlert tests for full coverage ---

    #[test]
    fn its_alert_elevated_factors_single_tb() {
        let factors = ItsAlert::SingleFactorTb.elevated_factors();
        assert_eq!(factors.len(), 1);
        assert!(factors.contains(&ItsProximalFactor::ThwartedBelongingness));
    }

    #[test]
    fn its_alert_elevated_factors_single_pb() {
        let factors = ItsAlert::SingleFactorPb.elevated_factors();
        assert_eq!(factors.len(), 1);
        assert!(factors.contains(&ItsProximalFactor::PerceivedBurdensomeness));
    }

    #[test]
    fn its_alert_elevated_factors_single_ac() {
        let factors = ItsAlert::SingleFactorAc.elevated_factors();
        assert_eq!(factors.len(), 1);
        assert!(factors.contains(&ItsProximalFactor::AcquiredCapability));
    }

    #[test]
    fn its_alert_elevated_factors_desire() {
        let factors = ItsAlert::DesireWithoutCapability.elevated_factors();
        assert_eq!(factors.len(), 2);
        assert!(factors.contains(&ItsProximalFactor::ThwartedBelongingness));
        assert!(factors.contains(&ItsProximalFactor::PerceivedBurdensomeness));
    }

    #[test]
    fn its_alert_elevated_factors_tb_with_ac() {
        let factors = ItsAlert::TbWithCapability.elevated_factors();
        assert_eq!(factors.len(), 2);
        assert!(factors.contains(&ItsProximalFactor::ThwartedBelongingness));
        assert!(factors.contains(&ItsProximalFactor::AcquiredCapability));
    }

    #[test]
    fn its_alert_elevated_factors_pb_with_ac() {
        let factors = ItsAlert::PbWithCapability.elevated_factors();
        assert_eq!(factors.len(), 2);
        assert!(factors.contains(&ItsProximalFactor::PerceivedBurdensomeness));
        assert!(factors.contains(&ItsProximalFactor::AcquiredCapability));
    }

    #[test]
    fn its_alert_elevated_factors_three_factor() {
        let factors = ItsAlert::ThreeFactorConvergence.elevated_factors();
        assert_eq!(factors.len(), 3);
        assert!(factors.contains(&ItsProximalFactor::ThwartedBelongingness));
        assert!(factors.contains(&ItsProximalFactor::PerceivedBurdensomeness));
        assert!(factors.contains(&ItsProximalFactor::AcquiredCapability));
    }

    #[test]
    fn its_alert_all_names() {
        assert_eq!(ItsAlert::SingleFactorPb.name(), "Perceived Burdensomeness Elevated");
        assert_eq!(ItsAlert::SingleFactorAc.name(), "Acquired Capability Elevated");
        assert_eq!(
            ItsAlert::DesireWithoutCapability.name(),
            "Suicidal Desire Without Capability"
        );
        assert_eq!(
            ItsAlert::TbWithCapability.name(),
            "Thwarted Belongingness With Capability"
        );
        assert_eq!(
            ItsAlert::PbWithCapability.name(),
            "Perceived Burdensomeness With Capability"
        );
    }

    #[test]
    fn its_alert_all_intervention_focuses() {
        let tb_focus = ItsAlert::SingleFactorTb.intervention_focus();
        assert!(tb_focus.contains("social connection"));

        let pb_focus = ItsAlert::SingleFactorPb.intervention_focus();
        assert!(pb_focus.contains("self-worth"));

        let ac_focus = ItsAlert::SingleFactorAc.intervention_focus();
        assert!(ac_focus.contains("Monitor"));

        let desire_focus = ItsAlert::DesireWithoutCapability.intervention_focus();
        assert!(desire_focus.contains("urgently"));

        let tb_ac_focus = ItsAlert::TbWithCapability.intervention_focus();
        assert!(tb_ac_focus.contains("social connection"));

        let pb_ac_focus = ItsAlert::PbWithCapability.intervention_focus();
        assert!(pb_ac_focus.contains("self-worth"));
    }

    #[test]
    fn its_alert_serialize_deserialize() {
        let alert = ItsAlert::ThreeFactorConvergence;
        let json = serde_json::to_string(&alert).unwrap();
        let deserialized: ItsAlert = serde_json::from_str(&json).unwrap();
        assert_eq!(alert, deserialized);
    }

    #[test]
    fn its_alert_all_display() {
        assert_eq!(
            format!("{}", ItsAlert::SingleFactorTb),
            "Thwarted Belongingness Elevated"
        );
        assert_eq!(
            format!("{}", ItsAlert::SingleFactorPb),
            "Perceived Burdensomeness Elevated"
        );
        assert_eq!(
            format!("{}", ItsAlert::SingleFactorAc),
            "Acquired Capability Elevated"
        );
        assert_eq!(
            format!("{}", ItsAlert::TbWithCapability),
            "Thwarted Belongingness With Capability"
        );
        assert_eq!(
            format!("{}", ItsAlert::PbWithCapability),
            "Perceived Burdensomeness With Capability"
        );
        assert_eq!(
            format!("{}", ItsAlert::ThreeFactorConvergence),
            "Three-Factor Convergence (High Risk)"
        );
    }
}
