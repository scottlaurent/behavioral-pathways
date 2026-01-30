//! Memory tags for categorizing memories.
//!
//! Tags allow memories to be filtered and retrieved by category.

use std::fmt;

/// Categorization tags for memories.
///
/// Tags are used to categorize memories and enable efficient retrieval
/// by category (e.g., finding all violence-related memories).
///
/// # Examples
///
/// ```
/// use behavioral_pathways::memory::MemoryTag;
///
/// let tag = MemoryTag::Betrayal;
/// assert_eq!(tag.name(), "Betrayal");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryTag {
    /// Mission-related events.
    Mission,
    /// Personal matters.
    Personal,
    /// Violence-related events.
    Violence,
    /// Betrayal events.
    Betrayal,
    /// Injustice events.
    Injustice,
    /// Ceremonial events.
    Ceremony,
    /// Resource scarcity events.
    Scarcity,
    /// Death-related events.
    Death,
    /// Crisis situations.
    Crisis,
    /// Relationship breakdowns.
    RelationshipBreakdown,
    /// Therapeutic experiences.
    Therapy,
    /// Supportive experiences.
    Support,
    /// Achievement events.
    Achievement,
    /// Loss events.
    Loss,
    /// Conflict events.
    Conflict,
    /// Cooperation events.
    Cooperation,
    /// Milestone events (required for Legacy promotion).
    Milestone,
}

impl MemoryTag {
    /// Returns all memory tag variants.
    #[must_use]
    pub fn all() -> [MemoryTag; 17] {
        [
            MemoryTag::Mission,
            MemoryTag::Personal,
            MemoryTag::Violence,
            MemoryTag::Betrayal,
            MemoryTag::Injustice,
            MemoryTag::Ceremony,
            MemoryTag::Scarcity,
            MemoryTag::Death,
            MemoryTag::Crisis,
            MemoryTag::RelationshipBreakdown,
            MemoryTag::Therapy,
            MemoryTag::Support,
            MemoryTag::Achievement,
            MemoryTag::Loss,
            MemoryTag::Conflict,
            MemoryTag::Cooperation,
            MemoryTag::Milestone,
        ]
    }

    /// Returns the name of this tag as a string.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            MemoryTag::Mission => "Mission",
            MemoryTag::Personal => "Personal",
            MemoryTag::Violence => "Violence",
            MemoryTag::Betrayal => "Betrayal",
            MemoryTag::Injustice => "Injustice",
            MemoryTag::Ceremony => "Ceremony",
            MemoryTag::Scarcity => "Scarcity",
            MemoryTag::Death => "Death",
            MemoryTag::Crisis => "Crisis",
            MemoryTag::RelationshipBreakdown => "RelationshipBreakdown",
            MemoryTag::Therapy => "Therapy",
            MemoryTag::Support => "Support",
            MemoryTag::Achievement => "Achievement",
            MemoryTag::Loss => "Loss",
            MemoryTag::Conflict => "Conflict",
            MemoryTag::Cooperation => "Cooperation",
            MemoryTag::Milestone => "Milestone",
        }
    }

    /// Returns whether this tag indicates a negative experience.
    #[must_use]
    pub fn is_negative(&self) -> bool {
        matches!(
            self,
            MemoryTag::Violence
                | MemoryTag::Betrayal
                | MemoryTag::Injustice
                | MemoryTag::Death
                | MemoryTag::Crisis
                | MemoryTag::RelationshipBreakdown
                | MemoryTag::Loss
                | MemoryTag::Conflict
                | MemoryTag::Scarcity
        )
    }

    /// Returns whether this tag indicates a positive experience.
    #[must_use]
    pub fn is_positive(&self) -> bool {
        matches!(
            self,
            MemoryTag::Ceremony
                | MemoryTag::Therapy
                | MemoryTag::Support
                | MemoryTag::Achievement
                | MemoryTag::Cooperation
        )
    }

    /// Returns whether this tag indicates a traumatic experience.
    ///
    /// Trauma tags are: Violence, Death, Crisis, Betrayal.
    /// These memories receive enhanced salience at encoding (per flashbulb
    /// memory research) and affect consolidation differently.
    #[must_use]
    pub fn is_trauma(&self) -> bool {
        matches!(
            self,
            MemoryTag::Violence | MemoryTag::Death | MemoryTag::Crisis | MemoryTag::Betrayal
        )
    }

    /// Returns whether this tag is neutral (neither positive nor negative).
    ///
    /// Neutral tags are: Mission, Personal, Milestone.
    /// These tags provide context but don't inherently indicate positive or negative experiences.
    #[must_use]
    pub fn is_neutral(&self) -> bool {
        matches!(
            self,
            MemoryTag::Mission | MemoryTag::Personal | MemoryTag::Milestone
        )
    }
}

impl fmt::Display for MemoryTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_returns_17_variants() {
        let all = MemoryTag::all();
        assert_eq!(all.len(), 17);
    }

    #[test]
    fn all_variants_unique() {
        use std::collections::HashSet;
        let all = MemoryTag::all();
        let set: HashSet<_> = all.iter().collect();
        assert_eq!(set.len(), 17);
    }

    #[test]
    fn name_returns_correct_string() {
        assert_eq!(MemoryTag::Mission.name(), "Mission");
        assert_eq!(MemoryTag::Personal.name(), "Personal");
        assert_eq!(MemoryTag::Violence.name(), "Violence");
        assert_eq!(MemoryTag::Betrayal.name(), "Betrayal");
        assert_eq!(MemoryTag::Injustice.name(), "Injustice");
        assert_eq!(MemoryTag::Ceremony.name(), "Ceremony");
        assert_eq!(MemoryTag::Scarcity.name(), "Scarcity");
        assert_eq!(MemoryTag::Death.name(), "Death");
        assert_eq!(MemoryTag::Crisis.name(), "Crisis");
        assert_eq!(
            MemoryTag::RelationshipBreakdown.name(),
            "RelationshipBreakdown"
        );
        assert_eq!(MemoryTag::Therapy.name(), "Therapy");
        assert_eq!(MemoryTag::Support.name(), "Support");
        assert_eq!(MemoryTag::Achievement.name(), "Achievement");
        assert_eq!(MemoryTag::Loss.name(), "Loss");
        assert_eq!(MemoryTag::Conflict.name(), "Conflict");
        assert_eq!(MemoryTag::Cooperation.name(), "Cooperation");
        assert_eq!(MemoryTag::Milestone.name(), "Milestone");
    }

    #[test]
    fn display_format() {
        assert_eq!(format!("{}", MemoryTag::Betrayal), "Betrayal");
        assert_eq!(format!("{}", MemoryTag::Achievement), "Achievement");
    }

    #[test]
    fn debug_format() {
        let debug = format!("{:?}", MemoryTag::Violence);
        assert!(debug.contains("Violence"));
    }

    #[test]
    fn clone_and_copy() {
        let tag = MemoryTag::Personal;
        let cloned = tag.clone();
        let copied = tag;
        assert_eq!(tag, cloned);
        assert_eq!(tag, copied);
    }

    #[test]
    fn equality() {
        assert_eq!(MemoryTag::Mission, MemoryTag::Mission);
        assert_ne!(MemoryTag::Mission, MemoryTag::Personal);
    }

    #[test]
    fn hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(MemoryTag::Mission);
        set.insert(MemoryTag::Personal);
        assert_eq!(set.len(), 2);

        // Duplicate should not increase size
        set.insert(MemoryTag::Mission);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn is_negative_correct() {
        // Negative tags
        assert!(MemoryTag::Violence.is_negative());
        assert!(MemoryTag::Betrayal.is_negative());
        assert!(MemoryTag::Injustice.is_negative());
        assert!(MemoryTag::Death.is_negative());
        assert!(MemoryTag::Crisis.is_negative());
        assert!(MemoryTag::RelationshipBreakdown.is_negative());
        assert!(MemoryTag::Loss.is_negative());
        assert!(MemoryTag::Conflict.is_negative());
        assert!(MemoryTag::Scarcity.is_negative());

        // Non-negative tags
        assert!(!MemoryTag::Mission.is_negative());
        assert!(!MemoryTag::Personal.is_negative());
        assert!(!MemoryTag::Ceremony.is_negative());
        assert!(!MemoryTag::Therapy.is_negative());
        assert!(!MemoryTag::Support.is_negative());
        assert!(!MemoryTag::Achievement.is_negative());
        assert!(!MemoryTag::Cooperation.is_negative());
        assert!(!MemoryTag::Milestone.is_negative());
    }

    #[test]
    fn is_positive_correct() {
        // Positive tags
        assert!(MemoryTag::Ceremony.is_positive());
        assert!(MemoryTag::Therapy.is_positive());
        assert!(MemoryTag::Support.is_positive());
        assert!(MemoryTag::Achievement.is_positive());
        assert!(MemoryTag::Cooperation.is_positive());

        // Non-positive tags
        assert!(!MemoryTag::Mission.is_positive());
        assert!(!MemoryTag::Personal.is_positive());
        assert!(!MemoryTag::Violence.is_positive());
        assert!(!MemoryTag::Betrayal.is_positive());
        assert!(!MemoryTag::Injustice.is_positive());
        assert!(!MemoryTag::Scarcity.is_positive());
        assert!(!MemoryTag::Death.is_positive());
        assert!(!MemoryTag::Crisis.is_positive());
        assert!(!MemoryTag::RelationshipBreakdown.is_positive());
        assert!(!MemoryTag::Loss.is_positive());
        assert!(!MemoryTag::Conflict.is_positive());
        assert!(!MemoryTag::Milestone.is_positive());
    }

    #[test]
    fn neutral_tags_are_neither_positive_nor_negative() {
        // Mission, Personal, and Milestone are neutral
        assert!(!MemoryTag::Mission.is_positive());
        assert!(!MemoryTag::Mission.is_negative());
        assert!(!MemoryTag::Personal.is_positive());
        assert!(!MemoryTag::Personal.is_negative());
        assert!(!MemoryTag::Milestone.is_positive());
        assert!(!MemoryTag::Milestone.is_negative());
    }

    #[test]
    fn is_trauma_identifies_trauma_tags() {
        // Trauma tags: Violence, Death, Crisis, Betrayal
        assert!(MemoryTag::Violence.is_trauma());
        assert!(MemoryTag::Death.is_trauma());
        assert!(MemoryTag::Crisis.is_trauma());
        assert!(MemoryTag::Betrayal.is_trauma());
    }

    #[test]
    fn is_trauma_false_for_non_trauma() {
        // All non-trauma tags should return false
        assert!(!MemoryTag::Mission.is_trauma());
        assert!(!MemoryTag::Personal.is_trauma());
        assert!(!MemoryTag::Injustice.is_trauma());
        assert!(!MemoryTag::Ceremony.is_trauma());
        assert!(!MemoryTag::Scarcity.is_trauma());
        assert!(!MemoryTag::RelationshipBreakdown.is_trauma());
        assert!(!MemoryTag::Therapy.is_trauma());
        assert!(!MemoryTag::Support.is_trauma());
        assert!(!MemoryTag::Achievement.is_trauma());
        assert!(!MemoryTag::Loss.is_trauma());
        assert!(!MemoryTag::Conflict.is_trauma());
        assert!(!MemoryTag::Cooperation.is_trauma());
        assert!(!MemoryTag::Milestone.is_trauma());
    }

    #[test]
    fn is_neutral_identifies_neutral_tags() {
        // Neutral tags: Mission, Personal, Milestone
        assert!(MemoryTag::Mission.is_neutral());
        assert!(MemoryTag::Personal.is_neutral());
        assert!(MemoryTag::Milestone.is_neutral());
    }

    #[test]
    fn is_neutral_false_for_non_neutral() {
        // All non-neutral tags should return false
        assert!(!MemoryTag::Violence.is_neutral());
        assert!(!MemoryTag::Betrayal.is_neutral());
        assert!(!MemoryTag::Injustice.is_neutral());
        assert!(!MemoryTag::Ceremony.is_neutral());
        assert!(!MemoryTag::Scarcity.is_neutral());
        assert!(!MemoryTag::Death.is_neutral());
        assert!(!MemoryTag::Crisis.is_neutral());
        assert!(!MemoryTag::RelationshipBreakdown.is_neutral());
        assert!(!MemoryTag::Therapy.is_neutral());
        assert!(!MemoryTag::Support.is_neutral());
        assert!(!MemoryTag::Achievement.is_neutral());
        assert!(!MemoryTag::Loss.is_neutral());
        assert!(!MemoryTag::Conflict.is_neutral());
        assert!(!MemoryTag::Cooperation.is_neutral());
    }
}
