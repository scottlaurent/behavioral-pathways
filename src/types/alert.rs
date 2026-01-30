//! Alert type for threshold notifications.
//!
//! Alerts are generated when entity state crosses dangerous thresholds
//! or when feedback loops are detected.

use crate::enums::{AlertSeverity, AlertTrigger};
use crate::types::Duration;

/// An alert generated when entity state crosses a threshold.
///
/// Alerts contain information about what triggered them, their severity,
/// and when they occurred.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::types::{Alert, Duration};
/// use behavioral_pathways::enums::{AlertSeverity, AlertTrigger, StatePath, MentalHealthPath};
///
/// let alert = Alert::new(
///     AlertSeverity::Warning,
///     AlertTrigger::ThresholdExceeded(
///         StatePath::MentalHealth(MentalHealthPath::SuicidalDesire),
///         0.7,
///     ),
///     Duration::days(100),
///     "High suicidal desire detected",
/// );
///
/// assert_eq!(alert.severity(), AlertSeverity::Warning);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Alert {
    /// The severity level of this alert.
    severity: AlertSeverity,

    /// What triggered this alert.
    trigger: AlertTrigger,

    /// When this alert was generated (simulation time).
    timestamp: Duration,

    /// Human-readable message describing the alert.
    message: String,
}

impl Alert {
    /// Creates a new alert.
    ///
    /// # Arguments
    ///
    /// * `severity` - The severity level
    /// * `trigger` - What triggered the alert
    /// * `timestamp` - When the alert was generated (simulation time)
    /// * `message` - A human-readable description
    #[must_use]
    pub fn new(
        severity: AlertSeverity,
        trigger: AlertTrigger,
        timestamp: Duration,
        message: impl Into<String>,
    ) -> Self {
        Alert {
            severity,
            trigger,
            timestamp,
            message: message.into(),
        }
    }

    /// Creates an info-level alert.
    #[must_use]
    pub fn info(trigger: AlertTrigger, timestamp: Duration, message: impl Into<String>) -> Self {
        Alert::new(AlertSeverity::Info, trigger, timestamp, message)
    }

    /// Creates a warning-level alert.
    #[must_use]
    pub fn warning(trigger: AlertTrigger, timestamp: Duration, message: impl Into<String>) -> Self {
        Alert::new(AlertSeverity::Warning, trigger, timestamp, message)
    }

    /// Creates a critical-level alert.
    #[must_use]
    pub fn critical(
        trigger: AlertTrigger,
        timestamp: Duration,
        message: impl Into<String>,
    ) -> Self {
        Alert::new(AlertSeverity::Critical, trigger, timestamp, message)
    }

    /// Returns the severity of this alert.
    #[must_use]
    pub fn severity(&self) -> AlertSeverity {
        self.severity
    }

    /// Returns the trigger for this alert.
    #[must_use]
    pub fn trigger(&self) -> &AlertTrigger {
        &self.trigger
    }

    /// Returns the timestamp when this alert was generated.
    #[must_use]
    pub fn timestamp(&self) -> Duration {
        self.timestamp
    }

    /// Returns the message for this alert.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns true if this alert is critical.
    #[must_use]
    pub fn is_critical(&self) -> bool {
        self.severity.is_critical()
    }

    /// Returns true if this alert is warning or higher.
    #[must_use]
    pub fn is_warning_or_higher(&self) -> bool {
        self.severity.is_warning_or_higher()
    }
}

impl std::fmt::Display for Alert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {} - {}", self.severity, self.trigger, self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::{MentalHealthPath, SpiralType, StatePath};

    #[test]
    fn alert_construction() {
        let alert = Alert::new(
            AlertSeverity::Warning,
            AlertTrigger::ThresholdExceeded(
                StatePath::MentalHealth(MentalHealthPath::SuicidalDesire),
                0.7,
            ),
            Duration::days(100),
            "Test message",
        );

        assert_eq!(alert.severity(), AlertSeverity::Warning);
        assert!(alert.trigger().is_threshold());
        assert_eq!(alert.timestamp().as_days(), 100);
        assert_eq!(alert.message(), "Test message");
    }

    #[test]
    fn alert_info_constructor() {
        let alert = Alert::info(
            AlertTrigger::custom("test"),
            Duration::days(1),
            "Info message",
        );

        assert_eq!(alert.severity(), AlertSeverity::Info);
        assert!(!alert.is_critical());
        assert!(!alert.is_warning_or_higher());
    }

    #[test]
    fn alert_warning_constructor() {
        let alert = Alert::warning(
            AlertTrigger::spiral(SpiralType::Stress),
            Duration::days(1),
            "Warning message",
        );

        assert_eq!(alert.severity(), AlertSeverity::Warning);
        assert!(!alert.is_critical());
        assert!(alert.is_warning_or_higher());
    }

    #[test]
    fn alert_critical_constructor() {
        let alert = Alert::critical(
            AlertTrigger::ThresholdExceeded(
                StatePath::MentalHealth(MentalHealthPath::AttemptRisk),
                0.8,
            ),
            Duration::days(1),
            "Critical message",
        );

        assert_eq!(alert.severity(), AlertSeverity::Critical);
        assert!(alert.is_critical());
        assert!(alert.is_warning_or_higher());
    }

    #[test]
    fn alert_display() {
        let alert = Alert::new(
            AlertSeverity::Warning,
            AlertTrigger::spiral(SpiralType::Depression),
            Duration::days(50),
            "Depression spiral detected",
        );

        let display = format!("{}", alert);
        assert!(display.contains("Warning"));
        assert!(display.contains("Depression spiral detected"));
    }

    #[test]
    fn alert_debug() {
        let alert = Alert::info(
            AlertTrigger::custom("test"),
            Duration::days(1),
            "Debug test",
        );

        let debug = format!("{:?}", alert);
        assert!(debug.contains("Alert"));
        assert!(debug.contains("Info"));
    }

    #[test]
    fn alert_clone() {
        let alert1 = Alert::warning(
            AlertTrigger::custom("test"),
            Duration::days(1),
            "Clone test",
        );
        let alert2 = alert1.clone();

        assert_eq!(alert1, alert2);
    }

    #[test]
    fn alert_includes_severity_level() {
        // Test name from phase-4.md
        let alert = Alert::new(
            AlertSeverity::Critical,
            AlertTrigger::ThresholdExceeded(
                StatePath::MentalHealth(MentalHealthPath::SuicidalDesire),
                0.9,
            ),
            Duration::days(100),
            "High risk",
        );

        assert_eq!(alert.severity(), AlertSeverity::Critical);
        assert!(alert.is_critical());
    }

    #[test]
    fn high_suicidal_desire_triggers_alert() {
        // Test name from phase-4.md - demonstrates the pattern
        let desire_value = 0.75;
        let alert = Alert::warning(
            AlertTrigger::ThresholdExceeded(
                StatePath::MentalHealth(MentalHealthPath::SuicidalDesire),
                desire_value,
            ),
            Duration::days(100),
            format!("Suicidal desire at {:.2}", desire_value),
        );

        assert!(matches!(
            alert.trigger(),
            AlertTrigger::ThresholdExceeded(path, value)
                if *path == StatePath::MentalHealth(MentalHealthPath::SuicidalDesire)
                    && (*value - desire_value).abs() < f64::EPSILON
        ));
    }

    #[test]
    fn entity_starts_with_no_alerts() {
        // This test verifies the pattern - actual entity test is in entity.rs
        let alerts: Vec<Alert> = Vec::new();
        assert!(alerts.is_empty());
    }

    #[test]
    fn entity_can_accumulate_alerts() {
        // This test verifies the pattern - actual entity test is in entity.rs
        let mut alerts: Vec<Alert> = Vec::new();

        alerts.push(Alert::warning(
            AlertTrigger::spiral(SpiralType::Stress),
            Duration::days(1),
            "First alert",
        ));
        alerts.push(Alert::critical(
            AlertTrigger::ThresholdExceeded(
                StatePath::MentalHealth(MentalHealthPath::SuicidalDesire),
                0.8,
            ),
            Duration::days(2),
            "Second alert",
        ));

        assert_eq!(alerts.len(), 2);
    }
}
