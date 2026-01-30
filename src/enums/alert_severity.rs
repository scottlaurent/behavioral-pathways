//! Alert severity levels for threshold notifications.
//!
//! This module defines severity levels for alerts generated when
//! entity state crosses dangerous thresholds.

/// Severity level for an alert.
///
/// Alerts are generated when entity state crosses dangerous thresholds.
/// The severity level indicates the urgency of the alert.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::enums::AlertSeverity;
///
/// let severity = AlertSeverity::Warning;
/// assert!(severity < AlertSeverity::Critical);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AlertSeverity {
    /// Informational alert - noteworthy but not concerning.
    Info,

    /// Warning alert - concerning threshold crossed, monitor closely.
    Warning,

    /// Critical alert - dangerous threshold crossed, intervention needed.
    Critical,
}

impl AlertSeverity {
    /// Returns a human-readable name for this severity level.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            AlertSeverity::Info => "Info",
            AlertSeverity::Warning => "Warning",
            AlertSeverity::Critical => "Critical",
        }
    }

    /// Returns true if this severity is at least Warning.
    #[must_use]
    pub const fn is_warning_or_higher(&self) -> bool {
        matches!(self, AlertSeverity::Warning | AlertSeverity::Critical)
    }

    /// Returns true if this severity is Critical.
    #[must_use]
    pub const fn is_critical(&self) -> bool {
        matches!(self, AlertSeverity::Critical)
    }
}

impl std::fmt::Display for AlertSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alert_severity_has_three_levels() {
        // Verify all three levels exist and are distinct
        let info = AlertSeverity::Info;
        let warning = AlertSeverity::Warning;
        let critical = AlertSeverity::Critical;

        assert_ne!(info, warning);
        assert_ne!(warning, critical);
        assert_ne!(info, critical);
    }

    #[test]
    fn severity_ordering() {
        assert!(AlertSeverity::Info < AlertSeverity::Warning);
        assert!(AlertSeverity::Warning < AlertSeverity::Critical);
        assert!(AlertSeverity::Info < AlertSeverity::Critical);
    }

    #[test]
    fn severity_names() {
        assert_eq!(AlertSeverity::Info.name(), "Info");
        assert_eq!(AlertSeverity::Warning.name(), "Warning");
        assert_eq!(AlertSeverity::Critical.name(), "Critical");
    }

    #[test]
    fn is_warning_or_higher() {
        assert!(!AlertSeverity::Info.is_warning_or_higher());
        assert!(AlertSeverity::Warning.is_warning_or_higher());
        assert!(AlertSeverity::Critical.is_warning_or_higher());
    }

    #[test]
    fn is_critical() {
        assert!(!AlertSeverity::Info.is_critical());
        assert!(!AlertSeverity::Warning.is_critical());
        assert!(AlertSeverity::Critical.is_critical());
    }

    #[test]
    fn display_format() {
        assert_eq!(format!("{}", AlertSeverity::Info), "Info");
        assert_eq!(format!("{}", AlertSeverity::Warning), "Warning");
        assert_eq!(format!("{}", AlertSeverity::Critical), "Critical");
    }

    #[test]
    fn debug_format() {
        let debug = format!("{:?}", AlertSeverity::Warning);
        assert!(debug.contains("Warning"));
    }

    #[test]
    fn clone_and_copy() {
        let s1 = AlertSeverity::Critical;
        let s2 = s1; // Copy
        let s3 = s1.clone();
        assert_eq!(s1, s2);
        assert_eq!(s1, s3);
    }

    #[test]
    fn hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(AlertSeverity::Info);
        set.insert(AlertSeverity::Info);
        assert_eq!(set.len(), 1);

        set.insert(AlertSeverity::Warning);
        assert_eq!(set.len(), 2);
    }
}
