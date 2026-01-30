//! Memory source types indicating how the memory was acquired.
//!
//! The source of a memory affects its reliability and confidence weight.

use std::fmt;

/// How an entity learned about an event.
///
/// The memory source determines the base confidence in the memory's accuracy.
/// First-hand experience is most reliable, while rumors are least reliable.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::memory::MemorySource;
///
/// let source = MemorySource::Self_;
/// assert!((source.confidence() - 1.0).abs() < f32::EPSILON);
///
/// let witness = MemorySource::Witness;
/// assert!((witness.confidence() - 0.7).abs() < f32::EPSILON);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MemorySource {
    /// Entity directly experienced the event.
    /// Confidence: 1.0
    #[default]
    Self_,

    /// Entity witnessed the event happen.
    /// Confidence: 0.7
    Witness,

    /// Entity heard about the event secondhand.
    /// Confidence: 0.4
    Rumor,
}

impl MemorySource {
    /// Confidence weight for Self_ source.
    pub const SELF_CONFIDENCE: f32 = 1.0;

    /// Confidence weight for Witness source.
    pub const WITNESS_CONFIDENCE: f32 = 0.7;

    /// Confidence weight for Rumor source.
    pub const RUMOR_CONFIDENCE: f32 = 0.4;

    /// Returns the confidence weight for this memory source.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::memory::MemorySource;
    ///
    /// assert!((MemorySource::Self_.confidence() - 1.0).abs() < f32::EPSILON);
    /// assert!((MemorySource::Witness.confidence() - 0.7).abs() < f32::EPSILON);
    /// assert!((MemorySource::Rumor.confidence() - 0.4).abs() < f32::EPSILON);
    /// ```
    #[must_use]
    pub fn confidence(&self) -> f32 {
        match self {
            MemorySource::Self_ => Self::SELF_CONFIDENCE,
            MemorySource::Witness => Self::WITNESS_CONFIDENCE,
            MemorySource::Rumor => Self::RUMOR_CONFIDENCE,
        }
    }

    /// Returns all memory source variants.
    #[must_use]
    pub fn all() -> [MemorySource; 3] {
        [
            MemorySource::Self_,
            MemorySource::Witness,
            MemorySource::Rumor,
        ]
    }

    /// Returns the name of this memory source as a string.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            MemorySource::Self_ => "Self",
            MemorySource::Witness => "Witness",
            MemorySource::Rumor => "Rumor",
        }
    }
}

impl fmt::Display for MemorySource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_source_self_confidence_1_0() {
        let source = MemorySource::Self_;
        assert!((source.confidence() - 1.0).abs() < f32::EPSILON);
        assert!((MemorySource::SELF_CONFIDENCE - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn memory_source_witness_confidence_0_7() {
        let source = MemorySource::Witness;
        assert!((source.confidence() - 0.7).abs() < f32::EPSILON);
        assert!((MemorySource::WITNESS_CONFIDENCE - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn memory_source_rumor_confidence_0_4() {
        let source = MemorySource::Rumor;
        assert!((source.confidence() - 0.4).abs() < f32::EPSILON);
        assert!((MemorySource::RUMOR_CONFIDENCE - 0.4).abs() < f32::EPSILON);
    }

    #[test]
    fn all_returns_all_variants() {
        let all = MemorySource::all();
        assert_eq!(all.len(), 3);
        assert!(all.contains(&MemorySource::Self_));
        assert!(all.contains(&MemorySource::Witness));
        assert!(all.contains(&MemorySource::Rumor));
    }

    #[test]
    fn name_returns_correct_string() {
        assert_eq!(MemorySource::Self_.name(), "Self");
        assert_eq!(MemorySource::Witness.name(), "Witness");
        assert_eq!(MemorySource::Rumor.name(), "Rumor");
    }

    #[test]
    fn display_format() {
        assert_eq!(format!("{}", MemorySource::Self_), "Self");
        assert_eq!(format!("{}", MemorySource::Witness), "Witness");
        assert_eq!(format!("{}", MemorySource::Rumor), "Rumor");
    }

    #[test]
    fn debug_format() {
        let debug = format!("{:?}", MemorySource::Self_);
        assert!(debug.contains("Self_"));
    }

    #[test]
    fn default_is_self() {
        let source = MemorySource::default();
        assert_eq!(source, MemorySource::Self_);
    }

    #[test]
    fn clone_and_copy() {
        let source = MemorySource::Witness;
        let cloned = source.clone();
        let copied = source;
        assert_eq!(source, cloned);
        assert_eq!(source, copied);
    }

    #[test]
    fn equality() {
        assert_eq!(MemorySource::Self_, MemorySource::Self_);
        assert_ne!(MemorySource::Self_, MemorySource::Witness);
        assert_ne!(MemorySource::Witness, MemorySource::Rumor);
    }

    #[test]
    fn hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(MemorySource::Self_);
        set.insert(MemorySource::Witness);
        set.insert(MemorySource::Rumor);
        assert_eq!(set.len(), 3);

        // Inserting duplicate should not increase size
        set.insert(MemorySource::Self_);
        assert_eq!(set.len(), 3);
    }
}
