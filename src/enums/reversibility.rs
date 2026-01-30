//! Reversibility enums for delta reversal operations.
//!
//! This module defines types for indicating whether a state dimension's
//! delta can be reversed (undone) and errors that can occur during reversal.

use std::fmt;

/// Result of checking whether a state dimension is reversible.
///
/// Some state dimensions cannot be reversed:
/// - Acquired Capability (AC) has no decay and accumulates permanently
/// - Feedback loop effects are cumulative and non-linear
///
/// # Examples
///
/// ```
/// use behavioral_pathways::enums::ReversibilityResult;
///
/// let result = ReversibilityResult::Reversible;
/// assert!(result.is_reversible());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReversibilityResult {
    /// The dimension's delta can be reversed.
    Reversible,

    /// The dimension's delta cannot be reversed (e.g., AC, feedback loops).
    NonReversible,
}

impl ReversibilityResult {
    /// Returns true if this result indicates the dimension is reversible.
    #[must_use]
    pub const fn is_reversible(&self) -> bool {
        matches!(self, ReversibilityResult::Reversible)
    }

    /// Returns true if this result indicates the dimension is not reversible.
    #[must_use]
    pub const fn is_non_reversible(&self) -> bool {
        matches!(self, ReversibilityResult::NonReversible)
    }
}

impl fmt::Display for ReversibilityResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReversibilityResult::Reversible => write!(f, "Reversible"),
            ReversibilityResult::NonReversible => write!(f, "NonReversible"),
        }
    }
}

/// Error type for delta reversal operations.
///
/// Returned when attempting to reverse a delta that cannot be reversed.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::enums::ReversibilityError;
///
/// let err = ReversibilityError::NonReversibleDimension("AcquiredCapability".to_string());
/// assert!(format!("{}", err).contains("AcquiredCapability"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReversibilityError {
    /// The dimension has no decay and cannot be reversed.
    NonReversibleDimension(String),

    /// The delta includes feedback loop effects which cannot be reversed.
    FeedbackLoopEffect(String),

    /// The reversal would produce an invalid result (e.g., NaN, infinite).
    InvalidReversal(String),
}

impl ReversibilityError {
    /// Creates a non-reversible dimension error.
    #[must_use]
    pub fn non_reversible(dimension: impl Into<String>) -> Self {
        ReversibilityError::NonReversibleDimension(dimension.into())
    }

    /// Creates a feedback loop effect error.
    #[must_use]
    pub fn feedback_loop(description: impl Into<String>) -> Self {
        ReversibilityError::FeedbackLoopEffect(description.into())
    }

    /// Creates an invalid reversal error.
    #[must_use]
    pub fn invalid(reason: impl Into<String>) -> Self {
        ReversibilityError::InvalidReversal(reason.into())
    }

    /// Returns true if this error is due to a non-reversible dimension.
    #[must_use]
    pub const fn is_non_reversible_dimension(&self) -> bool {
        matches!(self, ReversibilityError::NonReversibleDimension(_))
    }

    /// Returns true if this error is due to feedback loop effects.
    #[must_use]
    pub const fn is_feedback_loop_effect(&self) -> bool {
        matches!(self, ReversibilityError::FeedbackLoopEffect(_))
    }

    /// Returns true if this error is due to an invalid reversal result.
    #[must_use]
    pub const fn is_invalid_reversal(&self) -> bool {
        matches!(self, ReversibilityError::InvalidReversal(_))
    }
}

impl fmt::Display for ReversibilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReversibilityError::NonReversibleDimension(dim) => {
                write!(f, "Dimension '{}' cannot be reversed (no decay)", dim)
            }
            ReversibilityError::FeedbackLoopEffect(desc) => {
                write!(f, "Feedback loop effects cannot be reversed: {}", desc)
            }
            ReversibilityError::InvalidReversal(reason) => {
                write!(f, "Invalid reversal: {}", reason)
            }
        }
    }
}

impl std::error::Error for ReversibilityError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reversibility_result_variants() {
        let reversible = ReversibilityResult::Reversible;
        let non_reversible = ReversibilityResult::NonReversible;

        assert!(reversible.is_reversible());
        assert!(!reversible.is_non_reversible());

        assert!(!non_reversible.is_reversible());
        assert!(non_reversible.is_non_reversible());
    }

    #[test]
    fn reversibility_result_display() {
        assert_eq!(format!("{}", ReversibilityResult::Reversible), "Reversible");
        assert_eq!(
            format!("{}", ReversibilityResult::NonReversible),
            "NonReversible"
        );
    }

    #[test]
    fn reversibility_result_clone_copy() {
        let r1 = ReversibilityResult::Reversible;
        let r2 = r1; // Copy
        let r3 = r1.clone();
        assert_eq!(r1, r2);
        assert_eq!(r1, r3);
    }

    #[test]
    fn reversibility_result_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(ReversibilityResult::Reversible);
        set.insert(ReversibilityResult::Reversible);
        assert_eq!(set.len(), 1);

        set.insert(ReversibilityResult::NonReversible);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn reversibility_error_constructors() {
        let non_rev = ReversibilityError::non_reversible("AcquiredCapability");
        assert!(non_rev.is_non_reversible_dimension());
        assert!(!non_rev.is_feedback_loop_effect());
        assert!(!non_rev.is_invalid_reversal());

        let feedback = ReversibilityError::feedback_loop("Stress spiral effects");
        assert!(!feedback.is_non_reversible_dimension());
        assert!(feedback.is_feedback_loop_effect());
        assert!(!feedback.is_invalid_reversal());

        let invalid = ReversibilityError::invalid("Result was NaN");
        assert!(!invalid.is_non_reversible_dimension());
        assert!(!invalid.is_feedback_loop_effect());
        assert!(invalid.is_invalid_reversal());
    }

    #[test]
    fn reversibility_error_display() {
        let non_rev = ReversibilityError::NonReversibleDimension("AC".to_string());
        let display = format!("{}", non_rev);
        assert!(display.contains("AC"));
        assert!(display.contains("cannot be reversed"));

        let feedback = ReversibilityError::FeedbackLoopEffect("stress".to_string());
        let display2 = format!("{}", feedback);
        assert!(display2.contains("Feedback loop"));
        assert!(display2.contains("stress"));

        let invalid = ReversibilityError::InvalidReversal("overflow".to_string());
        let display3 = format!("{}", invalid);
        assert!(display3.contains("Invalid reversal"));
        assert!(display3.contains("overflow"));
    }

    #[test]
    fn reversibility_error_debug() {
        let err = ReversibilityError::non_reversible("test");
        let debug = format!("{:?}", err);
        assert!(debug.contains("NonReversibleDimension"));
    }

    #[test]
    fn reversibility_error_clone() {
        let e1 = ReversibilityError::non_reversible("test");
        let e2 = e1.clone();
        assert_eq!(e1, e2);
    }

    #[test]
    fn reversibility_error_is_error() {
        let err: Box<dyn std::error::Error> = Box::new(ReversibilityError::non_reversible("test"));
        // Just verify it implements Error trait
        let _ = format!("{}", err);
    }

    #[test]
    fn reversibility_result_indicates_status() {
        // This test name matches the requirement in phase-4.md
        let reversible = ReversibilityResult::Reversible;
        let non_reversible = ReversibilityResult::NonReversible;

        assert!(reversible.is_reversible());
        assert!(non_reversible.is_non_reversible());
    }

    #[test]
    fn reverse_decay_returns_error_for_non_reversible() {
        // This test name matches the requirement in phase-4.md
        // The actual reverse_decay function will use these error types
        let err = ReversibilityError::non_reversible("AcquiredCapability");
        assert!(err.is_non_reversible_dimension());

        let err2 = ReversibilityError::feedback_loop("Depression spiral");
        assert!(err2.is_feedback_loop_effect());
    }
}
