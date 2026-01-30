//! Alert generation for threshold crossings.
//!
//! This module provides functions that check entity state against
//! dangerous thresholds and generate alerts when crossed.
//!
//! Note: All functions in this module are test-only. They are used to test
//! the alert generation logic but are not part of the public API.

#[cfg(test)]
use crate::enums::{AlertTrigger, MentalHealthPath, SpiralType, StatePath};
#[cfg(test)]
use crate::processor::ItsFactors;
#[cfg(test)]
use crate::state::IndividualState;
#[cfg(test)]
use crate::types::{Alert, Duration};

/// Threshold for suicidal desire to trigger a warning.
#[cfg(test)]
const DESIRE_WARNING_THRESHOLD: f32 = 0.5;

/// Threshold for suicidal desire to trigger a critical alert.
#[cfg(test)]
const DESIRE_CRITICAL_THRESHOLD: f32 = 0.7;

/// Threshold for attempt risk to trigger a warning.
#[cfg(test)]
const RISK_WARNING_THRESHOLD: f32 = 0.4;

/// Threshold for attempt risk to trigger a critical alert.
#[cfg(test)]
const RISK_CRITICAL_THRESHOLD: f32 = 0.6;

/// Threshold for stress spiral to generate an alert.
#[cfg(test)]
const STRESS_SPIRAL_ALERT_THRESHOLD: f32 = 0.6;

/// Threshold for depression spiral to generate an alert.
#[cfg(test)]
const DEPRESSION_SPIRAL_ALERT_THRESHOLD: f32 = 0.4;

/// Checks ITS factors against thresholds and generates alerts.
///
/// This function checks:
/// - Suicidal desire thresholds (warning at 0.5, critical at 0.7)
/// - Attempt risk thresholds (warning at 0.4, critical at 0.6)
///
/// # Arguments
///
/// * `factors` - Computed ITS factors from entity state
/// * `timestamp` - Current simulation time for alert timestamp
///
/// # Returns
///
/// Vector of alerts generated. May be empty if no thresholds crossed.
///
/// # Examples
///
/// ```ignore
/// use behavioral_pathways::processor::{check_its_thresholds, compute_its_factors};
/// use behavioral_pathways::state::IndividualState;
/// use behavioral_pathways::types::Duration;
///
/// let state = IndividualState::new();
/// let factors = compute_its_factors(&state);
/// let alerts = check_its_thresholds(&factors, Duration::days(100));
///
/// // Healthy state should have no alerts
/// assert!(alerts.is_empty());
/// ```
#[cfg(test)]
#[must_use]
pub(crate) fn check_its_thresholds(factors: &ItsFactors, timestamp: Duration) -> Vec<Alert> {
    let mut alerts = Vec::new();

    // Check suicidal desire
    if factors.suicidal_desire >= DESIRE_CRITICAL_THRESHOLD {
        alerts.push(Alert::critical(
            AlertTrigger::threshold(
                StatePath::MentalHealth(MentalHealthPath::SuicidalDesire),
                f64::from(factors.suicidal_desire),
            ),
            timestamp,
            format!(
                "Critical suicidal desire level: {:.2}",
                factors.suicidal_desire
            ),
        ));
    } else if factors.suicidal_desire >= DESIRE_WARNING_THRESHOLD {
        alerts.push(Alert::warning(
            AlertTrigger::threshold(
                StatePath::MentalHealth(MentalHealthPath::SuicidalDesire),
                f64::from(factors.suicidal_desire),
            ),
            timestamp,
            format!(
                "Elevated suicidal desire level: {:.2}",
                factors.suicidal_desire
            ),
        ));
    }

    // Check attempt risk
    if factors.attempt_risk >= RISK_CRITICAL_THRESHOLD {
        alerts.push(Alert::critical(
            AlertTrigger::threshold(
                StatePath::MentalHealth(MentalHealthPath::AttemptRisk),
                f64::from(factors.attempt_risk),
            ),
            timestamp,
            format!("Critical attempt risk level: {:.2}", factors.attempt_risk),
        ));
    } else if factors.attempt_risk >= RISK_WARNING_THRESHOLD {
        alerts.push(Alert::warning(
            AlertTrigger::threshold(
                StatePath::MentalHealth(MentalHealthPath::AttemptRisk),
                f64::from(factors.attempt_risk),
            ),
            timestamp,
            format!("Elevated attempt risk level: {:.2}", factors.attempt_risk),
        ));
    }

    alerts
}

/// Checks for active feedback spirals and generates alerts.
///
/// This function checks:
/// - Stress spiral (stress > 0.6)
/// - Depression spiral (depression > 0.4, Human only)
///
/// # Arguments
///
/// * `state` - The entity's individual state
/// * `is_human` - Whether the entity is human (affects depression spiral)
/// * `timestamp` - Current simulation time for alert timestamp
///
/// # Returns
///
/// Vector of alerts generated. May be empty if no spirals detected.
///
/// # Examples
///
/// ```ignore
/// use behavioral_pathways::processor::check_spiral_alerts;
/// use behavioral_pathways::state::IndividualState;
/// use behavioral_pathways::types::Duration;
///
/// let mut state = IndividualState::new();
/// state.needs_mut().stress_mut().set_base(0.8);
///
/// let alerts = check_spiral_alerts(&state, true, Duration::days(100));
///
/// assert!(!alerts.is_empty());
/// ```
#[cfg(test)]
#[must_use]
pub(crate) fn check_spiral_alerts(
    state: &IndividualState,
    is_human: bool,
    timestamp: Duration,
) -> Vec<Alert> {
    let mut alerts = Vec::new();

    // Check stress spiral
    let stress = state.needs().stress_effective();
    if stress > STRESS_SPIRAL_ALERT_THRESHOLD {
        alerts.push(Alert::warning(
            AlertTrigger::spiral(SpiralType::Stress),
            timestamp,
            format!("Stress spiral active (stress: {:.2})", stress),
        ));
    }

    // Check depression spiral (Human only)
    if is_human {
        let depression = state.mental_health().depression_effective();
        if depression > DEPRESSION_SPIRAL_ALERT_THRESHOLD {
            alerts.push(Alert::warning(
                AlertTrigger::spiral(SpiralType::Depression),
                timestamp,
                format!("Depression spiral active (depression: {:.2})", depression),
            ));
        }
    }

    alerts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::AlertSeverity;
    use crate::processor::compute_its_factors;

    fn has_threshold_alert(alerts: &[Alert], expected_path: StatePath) -> bool {
        let mut found = false;
        for alert in alerts {
            match alert.trigger() {
                AlertTrigger::ThresholdExceeded(path, _) => {
                    if *path == expected_path {
                        found = true;
                    }
                }
                _ => {}
            }
        }
        found
    }

    fn has_spiral_alert(alerts: &[Alert], expected_spiral: SpiralType) -> bool {
        let mut found = false;
        for alert in alerts {
            match alert.trigger() {
                AlertTrigger::SpiralDetected(spiral_type) => {
                    if *spiral_type == expected_spiral {
                        found = true;
                    }
                }
                _ => {}
            }
        }
        found
    }

    // --- Tests from phase-4.md ---

    #[test]
    fn high_suicidal_desire_triggers_alert() {
        // Test name from phase-4.md
        let factors = ItsFactors {
            suicidal_desire: 0.75,
            ..Default::default()
        };

        let alerts = check_its_thresholds(&factors, Duration::days(100));

        assert!(!alerts.is_empty());

        let alert = &alerts[0];
        assert!(alert.is_critical());
        assert!(alert.trigger().is_threshold());
        assert!(has_threshold_alert(
            &alerts,
            StatePath::MentalHealth(MentalHealthPath::SuicidalDesire)
        ));
    }

    #[test]
    fn alert_includes_severity_level() {
        // Test name from phase-4.md

        // Warning level
        let warning_factors = ItsFactors {
            suicidal_desire: 0.55,
            ..Default::default()
        };
        let warning_alerts = check_its_thresholds(&warning_factors, Duration::days(100));
        assert!(!warning_alerts.is_empty());
        assert_eq!(warning_alerts[0].severity(), AlertSeverity::Warning);

        // Critical level
        let critical_factors = ItsFactors {
            suicidal_desire: 0.75,
            ..Default::default()
        };
        let critical_alerts = check_its_thresholds(&critical_factors, Duration::days(100));
        assert!(!critical_alerts.is_empty());
        assert_eq!(critical_alerts[0].severity(), AlertSeverity::Critical);
    }

    // --- Additional tests ---

    #[test]
    fn no_alerts_for_healthy_state() {
        let state = IndividualState::new();
        let factors = compute_its_factors(&state);
        let alerts = check_its_thresholds(&factors, Duration::days(100));

        assert!(alerts.is_empty());
    }

    #[test]
    fn desire_warning_threshold() {
        let factors = ItsFactors {
            suicidal_desire: 0.5,
            ..Default::default()
        };

        let alerts = check_its_thresholds(&factors, Duration::days(100));

        assert!(!alerts.is_empty());
        assert_eq!(alerts[0].severity(), AlertSeverity::Warning);
    }

    #[test]
    fn desire_below_warning_no_alert() {
        let factors = ItsFactors {
            suicidal_desire: 0.49,
            attempt_risk: 0.7,
            ..Default::default()
        };

        let mut alerts = check_its_thresholds(&factors, Duration::days(100));

        let mut state = IndividualState::new();
        state.needs_mut().stress_mut().set_base(0.8);
        alerts.extend(check_spiral_alerts(&state, true, Duration::days(100)));

        assert!(!has_threshold_alert(
            &alerts,
            StatePath::MentalHealth(MentalHealthPath::SuicidalDesire)
        ));
    }

    #[test]
    fn risk_warning_threshold() {
        let factors = ItsFactors {
            attempt_risk: 0.4,
            ..Default::default()
        };

        let alerts = check_its_thresholds(&factors, Duration::days(100));

        assert!(!alerts.is_empty());
        assert_eq!(alerts[0].severity(), AlertSeverity::Warning);
    }

    #[test]
    fn risk_critical_threshold() {
        let factors = ItsFactors {
            attempt_risk: 0.7,
            ..Default::default()
        };

        let alerts = check_its_thresholds(&factors, Duration::days(100));

        assert!(!alerts.is_empty());
        assert!(alerts[0].is_critical());
    }

    #[test]
    fn multiple_alerts_for_multiple_thresholds() {
        let factors = ItsFactors {
            suicidal_desire: 0.8,
            attempt_risk: 0.7,
            ..Default::default()
        };

        let alerts = check_its_thresholds(&factors, Duration::days(100));

        // Should have both desire and risk alerts
        assert!(alerts.len() >= 2);
    }

    #[test]
    fn stress_spiral_alert() {
        let mut state = IndividualState::new();
        state.needs_mut().stress_mut().set_base(0.8);

        let alerts = check_spiral_alerts(&state, true, Duration::days(100));

        assert!(!alerts.is_empty());
        assert!(alerts[0].trigger().is_spiral());
        assert!(has_spiral_alert(&alerts, SpiralType::Stress));
        assert!(!has_spiral_alert(&alerts, SpiralType::Depression));
    }

    #[test]
    fn depression_spiral_alert_human() {
        let mut state = IndividualState::new();
        state.mental_health_mut().depression_mut().set_base(0.6);

        let alerts = check_spiral_alerts(&state, true, Duration::days(100));

        assert!(!alerts.is_empty());
        assert!(has_spiral_alert(&alerts, SpiralType::Depression));
    }

    #[test]
    fn depression_spiral_no_alert_non_human() {
        let mut state = IndividualState::new();
        state.mental_health_mut().depression_mut().set_base(0.6);

        let factors = ItsFactors {
            suicidal_desire: 0.75,
            ..Default::default()
        };
        let mut alerts = check_its_thresholds(&factors, Duration::days(100));
        alerts.extend(check_spiral_alerts(&state, false, Duration::days(100)));

        assert!(!has_spiral_alert(&alerts, SpiralType::Depression));
    }

    #[test]
    fn no_spiral_alerts_below_threshold() {
        let state = IndividualState::new();

        let alerts = check_spiral_alerts(&state, true, Duration::days(100));

        assert!(alerts.is_empty());
    }

    #[test]
    fn alert_timestamp_preserved() {
        let factors = ItsFactors {
            suicidal_desire: 0.75,
            ..Default::default()
        };

        let timestamp = Duration::days(500);
        let alerts = check_its_thresholds(&factors, timestamp);

        assert!(!alerts.is_empty());
        assert_eq!(alerts[0].timestamp().as_days(), 500);
    }

    #[test]
    fn alert_message_contains_value() {
        let factors = ItsFactors {
            suicidal_desire: 0.75,
            ..Default::default()
        };

        let alerts = check_its_thresholds(&factors, Duration::days(100));

        assert!(!alerts.is_empty());
        assert!(alerts[0].message().contains("0.75"));
    }

    #[test]
    fn risk_below_warning_no_alert() {
        let factors = ItsFactors {
            attempt_risk: 0.39,
            suicidal_desire: 0.75,
            ..Default::default()
        };

        let alerts = check_its_thresholds(&factors, Duration::days(100));

        assert!(!has_threshold_alert(
            &alerts,
            StatePath::MentalHealth(MentalHealthPath::AttemptRisk)
        ));
    }

    #[test]
    fn stress_spiral_no_alert_at_threshold() {
        let mut state = IndividualState::new();
        state.needs_mut().stress_mut().set_base(0.6); // At threshold, not over

        let alerts = check_spiral_alerts(&state, true, Duration::days(100));

        // Should not trigger alert at exactly 0.6
        assert!(!has_spiral_alert(&alerts, SpiralType::Stress));
    }

    #[test]
    fn depression_spiral_no_alert_at_threshold() {
        let mut state = IndividualState::new();
        state.mental_health_mut().depression_mut().set_base(0.4); // At threshold, not over

        let alerts = check_spiral_alerts(&state, true, Duration::days(100));

        // Should not trigger alert at exactly 0.4
        assert!(!has_spiral_alert(&alerts, SpiralType::Depression));
    }

    #[test]
    fn both_spirals_active() {
        let mut state = IndividualState::new();
        state.needs_mut().stress_mut().set_base(0.8);
        state.mental_health_mut().depression_mut().set_base(0.6);

        let alerts = check_spiral_alerts(&state, true, Duration::days(100));

        // Should have both stress and depression spiral alerts
        assert!(alerts.len() >= 2);
    }
}
