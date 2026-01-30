//! Event type enums for compile-time validated event classification.
//!
//! These enums provide type-safe event categorization without magic strings.
//! `EventType` is the primary classification, `EventCategory` maps to theoretical
//! domains, and `EventTag` provides additional categorization.

use serde::{Deserialize, Serialize};

/// Primary event classification for compile-time validation.
///
/// Each event type maps to a specific `EventCategory` that links to
/// the theoretical framework (ITS pathways, PAD dimensions, etc.).
///
/// # Examples
///
/// ```
/// use behavioral_pathways::enums::{EventType, EventCategory};
///
/// let event_type = EventType::SocialExclusion;
/// assert_eq!(event_type.category(), EventCategory::SocialBelonging);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventType {
    // Social/Interpersonal
    /// General social exchange.
    Interaction,
    /// Social rejection affecting belonging (TB pathway).
    SocialExclusion,
    /// Social acceptance/belonging (reduces TB).
    SocialInclusion,
    /// Feedback perceived as indicating burden (PB pathway).
    BurdenFeedback,
    /// Trust violation.
    Betrayal,
    /// Received help or support.
    Support,
    /// Interpersonal disagreement.
    Conflict,
    /// Physical aggression.
    Violence,

    // Control/Power (Dominance dimension)
    /// Loss of control/status (reduces dominance).
    Humiliation,
    /// Gain of control/status (increases dominance).
    Empowerment,

    // Achievement/Loss
    /// Success or accomplishment.
    Achievement,
    /// Goal failure.
    Failure,
    /// Loss of person/resource.
    Loss,

    // Environmental
    /// Institutional/policy change.
    PolicyChange,
    /// Movement between microsystems.
    ContextTransition,
    /// Chronosystem-level event.
    HistoricalEvent,

    // Internal
    /// Insight or realization.
    Realization,
    /// Exposure to painful/frightening stimulus.
    TraumaticExposure,

    // ITS: Thwarted Belongingness (TB) pathway events
    /// Explicit rejection from group or relationship (TB pathway).
    Rejection,
    /// Social isolation or withdrawal from social contact (TB pathway).
    SocialIsolation,
    /// Death of a close person (TB + PB multi-pathway).
    Bereavement,
    /// Relationship breakup or divorce (TB pathway).
    RelationshipEnd,
    /// Feeling left out or excluded from group activities (TB pathway).
    GroupExclusion,

    // ITS: Perceived Burdensomeness (PB) pathway events
    /// Being shamed or criticized for being a burden (PB pathway).
    ShamingEvent,
    /// Financial strain causing feelings of being a burden (PB pathway).
    FinancialBurden,
    /// Termination of employment (TB + PB multi-pathway).
    JobLoss,
    /// Chronic illness onset affecting self-perception (PB pathway).
    ChronicIllnessOnset,
    /// Family discord/conflict about burden (PB pathway).
    FamilyDiscord,

    // ITS: Acquired Capability (AC) pathway events
    /// Non-suicidal self-injury (AC pathway - pain tolerance).
    NonSuicidalSelfInjury,
    /// History of childhood abuse or maltreatment (AC pathway).
    ChildhoodAbuse,
    /// Military combat experience (AC pathway).
    CombatExposure,
    /// Physical injury or accident with pain exposure (AC pathway).
    PhysicalInjury,
    /// Exposure to violence against others (AC pathway).
    ViolenceExposure,
    /// Previous suicide attempt by self (AC pathway - highest).
    PriorSuicideAttempt,
    /// Suicide of someone close (TB + AC multi-pathway).
    SuicidalLoss,
}

impl EventType {
    /// Returns the theoretical domain category for this event type.
    ///
    /// This mapping links events to ITS pathways, PAD dimensions, and other
    /// theoretical constructs for proper state modification.
    ///
    /// Note: Some events affect multiple pathways (multi-pathway events).
    /// Use `its_pathways()` for the full list of affected ITS pathways.
    #[must_use]
    pub const fn category(&self) -> EventCategory {
        match self {
            // SocialBelonging maps to TB (Thwarted Belongingness)
            EventType::SocialExclusion => EventCategory::SocialBelonging,
            EventType::SocialInclusion => EventCategory::SocialBelonging,
            EventType::Rejection => EventCategory::SocialBelonging,
            EventType::SocialIsolation => EventCategory::SocialBelonging,
            EventType::RelationshipEnd => EventCategory::SocialBelonging,
            EventType::GroupExclusion => EventCategory::SocialBelonging,

            // BurdenPerception maps to PB (Perceived Burdensomeness)
            EventType::BurdenFeedback => EventCategory::BurdenPerception,
            EventType::ShamingEvent => EventCategory::BurdenPerception,
            EventType::FinancialBurden => EventCategory::BurdenPerception,
            EventType::ChronicIllnessOnset => EventCategory::BurdenPerception,
            EventType::FamilyDiscord => EventCategory::BurdenPerception,

            // Trauma maps to AC (Acquired Capability)
            EventType::Violence => EventCategory::Trauma,
            EventType::TraumaticExposure => EventCategory::Trauma,
            EventType::NonSuicidalSelfInjury => EventCategory::Trauma,
            EventType::ChildhoodAbuse => EventCategory::Trauma,
            EventType::CombatExposure => EventCategory::Trauma,
            EventType::PhysicalInjury => EventCategory::Trauma,
            EventType::ViolenceExposure => EventCategory::Trauma,
            EventType::PriorSuicideAttempt => EventCategory::Trauma,

            // Multi-pathway events: primary category listed here
            // Use its_pathways() for full pathway mapping
            EventType::Bereavement => EventCategory::SocialBelonging, // TB + PB
            EventType::JobLoss => EventCategory::BurdenPerception,    // TB + PB
            EventType::SuicidalLoss => EventCategory::Trauma,         // TB + AC

            // Control maps to Dominance dimension (PAD)
            EventType::Humiliation => EventCategory::Control,
            EventType::Empowerment => EventCategory::Control,

            // Achievement affects self-worth
            EventType::Achievement => EventCategory::Achievement,
            EventType::Failure => EventCategory::Achievement,
            EventType::Loss => EventCategory::Achievement,

            // General social events
            EventType::Interaction => EventCategory::Social,
            EventType::Betrayal => EventCategory::Social,
            EventType::Support => EventCategory::Social,
            EventType::Conflict => EventCategory::Social,

            // Contextual events
            EventType::PolicyChange => EventCategory::Contextual,
            EventType::ContextTransition => EventCategory::Contextual,
            EventType::HistoricalEvent => EventCategory::Contextual,
            EventType::Realization => EventCategory::Contextual,
        }
    }

    /// Returns all ITS pathways affected by this event type.
    ///
    /// Multi-pathway events (like job loss, bereavement) affect multiple
    /// ITS factors simultaneously, which is clinically significant for
    /// risk assessment.
    ///
    /// Returns a tuple of (affects_tb, affects_pb, affects_ac).
    #[must_use]
    pub const fn its_pathways(&self) -> (bool, bool, bool) {
        match self {
            // Pure TB events
            EventType::SocialExclusion => (true, false, false),
            EventType::SocialInclusion => (true, false, false), // Reduces TB
            EventType::Rejection => (true, false, false),
            EventType::SocialIsolation => (true, false, false),
            EventType::RelationshipEnd => (true, false, false),
            EventType::GroupExclusion => (true, false, false),

            // Pure PB events
            EventType::BurdenFeedback => (false, true, false),
            EventType::ShamingEvent => (false, true, false),
            EventType::FinancialBurden => (false, true, false),
            EventType::ChronicIllnessOnset => (false, true, false),
            EventType::FamilyDiscord => (false, true, false),

            // Pure AC events
            EventType::Violence => (false, false, true),
            EventType::TraumaticExposure => (false, false, true),
            EventType::NonSuicidalSelfInjury => (false, false, true),
            EventType::ChildhoodAbuse => (false, false, true),
            EventType::CombatExposure => (false, false, true),
            EventType::PhysicalInjury => (false, false, true),
            EventType::ViolenceExposure => (false, false, true),
            EventType::PriorSuicideAttempt => (false, false, true),

            // Multi-pathway events
            EventType::Bereavement => (true, true, false),  // TB + PB
            EventType::JobLoss => (true, true, false),      // TB + PB
            EventType::SuicidalLoss => (true, false, true), // TB + AC

            // Non-ITS events
            _ => (false, false, false),
        }
    }

    /// Returns true if this event affects the TB (Thwarted Belongingness) pathway.
    #[must_use]
    pub const fn affects_tb(&self) -> bool {
        self.its_pathways().0
    }

    /// Returns true if this event affects the PB (Perceived Burdensomeness) pathway.
    #[must_use]
    pub const fn affects_pb(&self) -> bool {
        self.its_pathways().1
    }

    /// Returns true if this event affects the AC (Acquired Capability) pathway.
    #[must_use]
    pub const fn affects_ac(&self) -> bool {
        self.its_pathways().2
    }

    /// Returns true if this event affects multiple ITS pathways.
    #[must_use]
    pub const fn is_multi_pathway(&self) -> bool {
        let (tb, pb, ac) = self.its_pathways();
        (tb as u8 + pb as u8 + ac as u8) > 1
    }

    /// Returns a human-readable name for this event type.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            EventType::Interaction => "Interaction",
            EventType::SocialExclusion => "Social Exclusion",
            EventType::SocialInclusion => "Social Inclusion",
            EventType::BurdenFeedback => "Burden Feedback",
            EventType::Betrayal => "Betrayal",
            EventType::Support => "Support",
            EventType::Conflict => "Conflict",
            EventType::Violence => "Violence",
            EventType::Humiliation => "Humiliation",
            EventType::Empowerment => "Empowerment",
            EventType::Achievement => "Achievement",
            EventType::Failure => "Failure",
            EventType::Loss => "Loss",
            EventType::PolicyChange => "Policy Change",
            EventType::ContextTransition => "Context Transition",
            EventType::HistoricalEvent => "Historical Event",
            EventType::Realization => "Realization",
            EventType::TraumaticExposure => "Traumatic Exposure",
            // TB pathway events
            EventType::Rejection => "Rejection",
            EventType::SocialIsolation => "Social Isolation",
            EventType::Bereavement => "Bereavement",
            EventType::RelationshipEnd => "Relationship End",
            EventType::GroupExclusion => "Group Exclusion",
            // PB pathway events
            EventType::ShamingEvent => "Shaming Event",
            EventType::FinancialBurden => "Financial Burden",
            EventType::JobLoss => "Job Loss",
            EventType::ChronicIllnessOnset => "Chronic Illness Onset",
            EventType::FamilyDiscord => "Family Discord",
            // AC pathway events
            EventType::NonSuicidalSelfInjury => "Non-Suicidal Self-Injury",
            EventType::ChildhoodAbuse => "Childhood Abuse",
            EventType::CombatExposure => "Combat Exposure",
            EventType::PhysicalInjury => "Physical Injury",
            EventType::ViolenceExposure => "Violence Exposure",
            EventType::PriorSuicideAttempt => "Prior Suicide Attempt",
            EventType::SuicidalLoss => "Suicidal Loss",
        }
    }

    /// Returns all event type variants.
    #[must_use]
    pub const fn all() -> [EventType; 35] {
        [
            EventType::Interaction,
            EventType::SocialExclusion,
            EventType::SocialInclusion,
            EventType::BurdenFeedback,
            EventType::Betrayal,
            EventType::Support,
            EventType::Conflict,
            EventType::Violence,
            EventType::Humiliation,
            EventType::Empowerment,
            EventType::Achievement,
            EventType::Failure,
            EventType::Loss,
            EventType::PolicyChange,
            EventType::ContextTransition,
            EventType::HistoricalEvent,
            EventType::Realization,
            EventType::TraumaticExposure,
            // TB pathway events
            EventType::Rejection,
            EventType::SocialIsolation,
            EventType::Bereavement,
            EventType::RelationshipEnd,
            EventType::GroupExclusion,
            // PB pathway events
            EventType::ShamingEvent,
            EventType::FinancialBurden,
            EventType::JobLoss,
            EventType::ChronicIllnessOnset,
            EventType::FamilyDiscord,
            // AC pathway events
            EventType::NonSuicidalSelfInjury,
            EventType::ChildhoodAbuse,
            EventType::CombatExposure,
            EventType::PhysicalInjury,
            EventType::ViolenceExposure,
            EventType::PriorSuicideAttempt,
            EventType::SuicidalLoss,
        ]
    }
}

/// Theoretical domain category for events.
///
/// Maps events to ITS pathways, PAD dimensions, and other theoretical
/// constructs. This enables compile-time linkage between events and
/// their effects on psychological state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventCategory {
    /// Events affecting belonging (maps to TB - Thwarted Belongingness).
    SocialBelonging,
    /// Events affecting burdensomeness (maps to PB - Perceived Burdensomeness).
    BurdenPerception,
    /// Events affecting acquired capability (maps to AC).
    Trauma,
    /// Events affecting dominance dimension (PAD completeness).
    Control,
    /// Events affecting self-worth.
    Achievement,
    /// General interpersonal events.
    Social,
    /// Environmental/ecological events.
    Contextual,
}

impl EventCategory {
    /// Returns a human-readable name for this category.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            EventCategory::SocialBelonging => "Social Belonging",
            EventCategory::BurdenPerception => "Burden Perception",
            EventCategory::Trauma => "Trauma",
            EventCategory::Control => "Control",
            EventCategory::Achievement => "Achievement",
            EventCategory::Social => "Social",
            EventCategory::Contextual => "Contextual",
        }
    }

    /// Returns all category variants.
    #[must_use]
    pub const fn all() -> [EventCategory; 7] {
        [
            EventCategory::SocialBelonging,
            EventCategory::BurdenPerception,
            EventCategory::Trauma,
            EventCategory::Control,
            EventCategory::Achievement,
            EventCategory::Social,
            EventCategory::Contextual,
        ]
    }
}

/// Additional categorization tags for events.
///
/// Multiple tags can be applied to a single event for fine-grained
/// filtering and retrieval.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventTag {
    /// Event relates to personal matters.
    Personal,
    /// Event is social in nature.
    Social,
    /// Event relates to work/career.
    Work,
    /// Event relates to family.
    Family,
    /// Event was witnessed, not directly experienced.
    Witnessed,
    /// Event was directly experienced.
    DirectExperience,
    /// Event has positive valence.
    Positive,
    /// Event has negative valence.
    Negative,
    /// Event involves a moral violation.
    MoralViolation,
    /// Event is neutral.
    Neutral,
    /// Event has high stakes/consequences.
    HighStakes,
    /// Event has low stakes/consequences.
    LowStakes,
    /// Acute event with fast-decaying impact.
    AcuteEvent,
    /// Chronic pattern with slow-decaying impact.
    ChronicPattern,
}

impl EventTag {
    /// Returns a human-readable name for this tag.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            EventTag::Personal => "Personal",
            EventTag::Social => "Social",
            EventTag::Work => "Work",
            EventTag::Family => "Family",
            EventTag::Witnessed => "Witnessed",
            EventTag::DirectExperience => "Direct Experience",
            EventTag::Positive => "Positive",
            EventTag::Negative => "Negative",
            EventTag::MoralViolation => "Moral Violation",
            EventTag::Neutral => "Neutral",
            EventTag::HighStakes => "High Stakes",
            EventTag::LowStakes => "Low Stakes",
            EventTag::AcuteEvent => "Acute Event",
            EventTag::ChronicPattern => "Chronic Pattern",
        }
    }

    /// Returns all tag variants.
    #[must_use]
    pub const fn all() -> [EventTag; 14] {
        [
            EventTag::Personal,
            EventTag::Social,
            EventTag::Work,
            EventTag::Family,
            EventTag::Witnessed,
            EventTag::DirectExperience,
            EventTag::Positive,
            EventTag::Negative,
            EventTag::MoralViolation,
            EventTag::Neutral,
            EventTag::HighStakes,
            EventTag::LowStakes,
            EventTag::AcuteEvent,
            EventTag::ChronicPattern,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_type_category_mapping() {
        assert_eq!(
            EventType::SocialExclusion.category(),
            EventCategory::SocialBelonging
        );
        assert_eq!(
            EventType::SocialInclusion.category(),
            EventCategory::SocialBelonging
        );
        assert_eq!(
            EventType::BurdenFeedback.category(),
            EventCategory::BurdenPerception
        );
        assert_eq!(EventType::Violence.category(), EventCategory::Trauma);
        assert_eq!(
            EventType::TraumaticExposure.category(),
            EventCategory::Trauma
        );
        assert_eq!(EventType::Humiliation.category(), EventCategory::Control);
        assert_eq!(EventType::Empowerment.category(), EventCategory::Control);
        assert_eq!(
            EventType::Achievement.category(),
            EventCategory::Achievement
        );
        assert_eq!(EventType::Failure.category(), EventCategory::Achievement);
    }

    #[test]
    fn event_type_all_returns_all_variants() {
        let all = EventType::all();
        assert_eq!(all.len(), 35);
    }

    #[test]
    fn event_type_names_not_empty() {
        for event_type in EventType::all() {
            assert!(!event_type.name().is_empty());
        }
    }

    #[test]
    fn event_category_all_returns_all_variants() {
        let all = EventCategory::all();
        assert_eq!(all.len(), 7);
    }

    #[test]
    fn event_category_names_not_empty() {
        for category in EventCategory::all() {
            assert!(!category.name().is_empty());
        }
    }

    #[test]
    fn event_tag_all_returns_all_variants() {
        let all = EventTag::all();
        assert_eq!(all.len(), 14);
    }

    #[test]
    fn event_tag_names_not_empty() {
        for tag in EventTag::all() {
            assert!(!tag.name().is_empty());
        }
    }

    #[test]
    fn event_type_is_copy() {
        let t1 = EventType::Violence;
        let t2 = t1; // Copy
        assert_eq!(t1, t2);
    }

    #[test]
    fn event_category_is_copy() {
        let c1 = EventCategory::Trauma;
        let c2 = c1; // Copy
        assert_eq!(c1, c2);
    }

    #[test]
    fn event_tag_is_copy() {
        let t1 = EventTag::Personal;
        let t2 = t1; // Copy
        assert_eq!(t1, t2);
    }

    #[test]
    fn event_type_is_hashable() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(EventType::Violence);
        set.insert(EventType::Violence);
        assert_eq!(set.len(), 1);

        set.insert(EventType::Betrayal);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn event_type_debug_format() {
        let event_type = EventType::SocialExclusion;
        let debug = format!("{:?}", event_type);
        assert!(debug.contains("SocialExclusion"));
    }

    #[test]
    fn event_category_debug_format() {
        let category = EventCategory::SocialBelonging;
        let debug = format!("{:?}", category);
        assert!(debug.contains("SocialBelonging"));
    }

    #[test]
    fn event_tag_debug_format() {
        let tag = EventTag::HighStakes;
        let debug = format!("{:?}", tag);
        assert!(debug.contains("HighStakes"));
    }

    #[test]
    fn all_event_types_have_category() {
        for event_type in EventType::all() {
            // All event types must map to a valid category
            let _ = event_type.category();
        }
    }

    #[test]
    fn social_events_map_correctly() {
        assert_eq!(EventType::Interaction.category(), EventCategory::Social);
        assert_eq!(EventType::Betrayal.category(), EventCategory::Social);
        assert_eq!(EventType::Support.category(), EventCategory::Social);
        assert_eq!(EventType::Conflict.category(), EventCategory::Social);
    }

    #[test]
    fn achievement_events_map_correctly() {
        assert_eq!(
            EventType::Achievement.category(),
            EventCategory::Achievement
        );
        assert_eq!(EventType::Failure.category(), EventCategory::Achievement);
        assert_eq!(EventType::Loss.category(), EventCategory::Achievement);
    }

    #[test]
    fn contextual_events_map_correctly() {
        assert_eq!(
            EventType::PolicyChange.category(),
            EventCategory::Contextual
        );
        assert_eq!(
            EventType::ContextTransition.category(),
            EventCategory::Contextual
        );
        assert_eq!(
            EventType::HistoricalEvent.category(),
            EventCategory::Contextual
        );
        assert_eq!(EventType::Realization.category(), EventCategory::Contextual);
    }

    // --- ITS pathway tests ---

    #[test]
    fn tb_events_affect_tb_pathway() {
        assert!(EventType::SocialExclusion.affects_tb());
        assert!(EventType::SocialInclusion.affects_tb());
        assert!(EventType::Rejection.affects_tb());
        assert!(EventType::SocialIsolation.affects_tb());
        assert!(EventType::RelationshipEnd.affects_tb());
        assert!(EventType::GroupExclusion.affects_tb());
    }

    #[test]
    fn tb_events_category_is_social_belonging() {
        assert_eq!(EventType::Rejection.category(), EventCategory::SocialBelonging);
        assert_eq!(EventType::SocialIsolation.category(), EventCategory::SocialBelonging);
        assert_eq!(EventType::RelationshipEnd.category(), EventCategory::SocialBelonging);
        assert_eq!(EventType::GroupExclusion.category(), EventCategory::SocialBelonging);
    }

    #[test]
    fn pb_events_affect_pb_pathway() {
        assert!(EventType::BurdenFeedback.affects_pb());
        assert!(EventType::ShamingEvent.affects_pb());
        assert!(EventType::FinancialBurden.affects_pb());
        assert!(EventType::ChronicIllnessOnset.affects_pb());
        assert!(EventType::FamilyDiscord.affects_pb());
    }

    #[test]
    fn pb_events_category_is_burden_perception() {
        assert_eq!(EventType::ShamingEvent.category(), EventCategory::BurdenPerception);
        assert_eq!(EventType::FinancialBurden.category(), EventCategory::BurdenPerception);
        assert_eq!(EventType::ChronicIllnessOnset.category(), EventCategory::BurdenPerception);
        assert_eq!(EventType::FamilyDiscord.category(), EventCategory::BurdenPerception);
    }

    #[test]
    fn ac_events_affect_ac_pathway() {
        assert!(EventType::Violence.affects_ac());
        assert!(EventType::TraumaticExposure.affects_ac());
        assert!(EventType::NonSuicidalSelfInjury.affects_ac());
        assert!(EventType::ChildhoodAbuse.affects_ac());
        assert!(EventType::CombatExposure.affects_ac());
        assert!(EventType::PhysicalInjury.affects_ac());
        assert!(EventType::ViolenceExposure.affects_ac());
        assert!(EventType::PriorSuicideAttempt.affects_ac());
    }

    #[test]
    fn ac_events_category_is_trauma() {
        assert_eq!(EventType::NonSuicidalSelfInjury.category(), EventCategory::Trauma);
        assert_eq!(EventType::ChildhoodAbuse.category(), EventCategory::Trauma);
        assert_eq!(EventType::CombatExposure.category(), EventCategory::Trauma);
        assert_eq!(EventType::PhysicalInjury.category(), EventCategory::Trauma);
        assert_eq!(EventType::ViolenceExposure.category(), EventCategory::Trauma);
        assert_eq!(EventType::PriorSuicideAttempt.category(), EventCategory::Trauma);
    }

    #[test]
    fn multi_pathway_events_identified() {
        assert!(EventType::Bereavement.is_multi_pathway());
        assert!(EventType::JobLoss.is_multi_pathway());
        assert!(EventType::SuicidalLoss.is_multi_pathway());

        // Single pathway events are not multi-pathway
        assert!(!EventType::Rejection.is_multi_pathway());
        assert!(!EventType::ShamingEvent.is_multi_pathway());
        assert!(!EventType::Violence.is_multi_pathway());
    }

    #[test]
    fn bereavement_affects_tb_and_pb() {
        let (tb, pb, ac) = EventType::Bereavement.its_pathways();
        assert!(tb);
        assert!(pb);
        assert!(!ac);
    }

    #[test]
    fn job_loss_affects_tb_and_pb() {
        let (tb, pb, ac) = EventType::JobLoss.its_pathways();
        assert!(tb);
        assert!(pb);
        assert!(!ac);
    }

    #[test]
    fn suicidal_loss_affects_tb_and_ac() {
        let (tb, pb, ac) = EventType::SuicidalLoss.its_pathways();
        assert!(tb);
        assert!(!pb);
        assert!(ac);
    }

    #[test]
    fn non_its_events_dont_affect_pathways() {
        let (tb, pb, ac) = EventType::Interaction.its_pathways();
        assert!(!tb);
        assert!(!pb);
        assert!(!ac);

        let (tb, pb, ac) = EventType::PolicyChange.its_pathways();
        assert!(!tb);
        assert!(!pb);
        assert!(!ac);
    }

    #[test]
    fn new_event_type_names_not_empty() {
        assert!(!EventType::Rejection.name().is_empty());
        assert!(!EventType::SocialIsolation.name().is_empty());
        assert!(!EventType::Bereavement.name().is_empty());
        assert!(!EventType::NonSuicidalSelfInjury.name().is_empty());
        assert!(!EventType::CombatExposure.name().is_empty());
        assert!(!EventType::PriorSuicideAttempt.name().is_empty());
        assert!(!EventType::SuicidalLoss.name().is_empty());
    }

    #[test]
    fn all_new_event_types_have_category() {
        // TB events
        let _ = EventType::Rejection.category();
        let _ = EventType::SocialIsolation.category();
        let _ = EventType::Bereavement.category();
        let _ = EventType::RelationshipEnd.category();
        let _ = EventType::GroupExclusion.category();

        // PB events
        let _ = EventType::ShamingEvent.category();
        let _ = EventType::FinancialBurden.category();
        let _ = EventType::JobLoss.category();
        let _ = EventType::ChronicIllnessOnset.category();
        let _ = EventType::FamilyDiscord.category();

        // AC events
        let _ = EventType::NonSuicidalSelfInjury.category();
        let _ = EventType::ChildhoodAbuse.category();
        let _ = EventType::CombatExposure.category();
        let _ = EventType::PhysicalInjury.category();
        let _ = EventType::ViolenceExposure.category();
        let _ = EventType::PriorSuicideAttempt.category();
        let _ = EventType::SuicidalLoss.category();
    }
}
