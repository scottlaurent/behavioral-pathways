//! Developmental category enum for sensitive period processing.
//!
//! Maps event types to developmental domains based on Erikson's
//! psychosocial stages. Used internally by developmental processing
//! to determine sensitive period amplification.

use crate::enums::EventType;

/// Developmental category based on Erikson's psychosocial stages.
///
/// Each category represents a developmental domain that may be
/// amplified during specific life stages (sensitive periods).
///
/// # Internal Use Only
///
/// This enum is used internally by developmental processing and is
/// not exposed in the public API. Consumers interact with developmental
/// effects implicitly through `state_at()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DevelopmentalCategory {
    /// Trust vs Mistrust (infancy) - attachment and trust-building events.
    Attachment,

    /// Autonomy vs Shame (toddler) - independence and self-control events.
    /// No EventType currently maps here; retained for Erikson stage completeness.
    #[allow(dead_code)]
    Autonomy,

    /// Initiative vs Guilt (preschool) - purpose and exploration events.
    /// No EventType currently maps here; retained for Erikson stage completeness.
    #[allow(dead_code)]
    Initiative,

    /// Industry vs Inferiority (school age) - competence and achievement events.
    Industry,

    /// Identity vs Role Confusion (adolescence) - identity formation events.
    Identity,

    /// Intimacy vs Isolation (young adult) - close relationship events.
    Intimacy,

    /// Generativity vs Stagnation (middle adult) - nurturing and mentoring events.
    Generativity,

    /// Integrity vs Despair (late adult) - meaning-making and life review events.
    Integrity,

    /// No developmental amplification - cross-stage or neutral events.
    Neutral,
}

impl DevelopmentalCategory {
    /// Returns all developmental category variants.
    #[must_use]
    #[allow(dead_code)]
    pub const fn all() -> [DevelopmentalCategory; 9] {
        [
            DevelopmentalCategory::Attachment,
            DevelopmentalCategory::Autonomy,
            DevelopmentalCategory::Initiative,
            DevelopmentalCategory::Industry,
            DevelopmentalCategory::Identity,
            DevelopmentalCategory::Intimacy,
            DevelopmentalCategory::Generativity,
            DevelopmentalCategory::Integrity,
            DevelopmentalCategory::Neutral,
        ]
    }

    /// Returns a human-readable name for this category.
    #[must_use]
    #[allow(dead_code)]
    pub const fn name(&self) -> &'static str {
        match self {
            DevelopmentalCategory::Attachment => "Attachment",
            DevelopmentalCategory::Autonomy => "Autonomy",
            DevelopmentalCategory::Initiative => "Initiative",
            DevelopmentalCategory::Industry => "Industry",
            DevelopmentalCategory::Identity => "Identity",
            DevelopmentalCategory::Intimacy => "Intimacy",
            DevelopmentalCategory::Generativity => "Generativity",
            DevelopmentalCategory::Integrity => "Integrity",
            DevelopmentalCategory::Neutral => "Neutral",
        }
    }
}

impl From<&EventType> for DevelopmentalCategory {
    /// Maps event types to developmental categories.
    ///
    /// # Mapping Rationale
    ///
    /// - **Attachment**: Social support/conflict/rejection - trust building/breaking
    /// - **Industry**: Achievement/failure - competence development
    /// - **Identity**: Status change, role transition, identity crisis - role definition
    /// - **Intimacy**: Close relationship events - connection and betrayal
    /// - **Generativity**: Caregiving and mentoring - guiding next generation
    /// - **Integrity**: Life review and legacy - meaning-making
    /// - **Neutral**: Cross-stage events (violence, trauma, loss, routine)
    fn from(event_type: &EventType) -> Self {
        match event_type {
            // Attachment category - trust building/breaking
            EventType::Support => DevelopmentalCategory::Attachment,
            EventType::Conflict => DevelopmentalCategory::Attachment,
            EventType::SocialExclusion => DevelopmentalCategory::Attachment,
            EventType::SocialInclusion => DevelopmentalCategory::Attachment,
            // TB pathway events are attachment-related
            EventType::Rejection => DevelopmentalCategory::Attachment,
            EventType::SocialIsolation => DevelopmentalCategory::Attachment,
            EventType::GroupExclusion => DevelopmentalCategory::Attachment,

            // Industry category - competence
            EventType::Achievement => DevelopmentalCategory::Industry,
            EventType::Failure => DevelopmentalCategory::Industry,
            // Financial burden affects competence perception
            EventType::FinancialBurden => DevelopmentalCategory::Industry,
            EventType::JobLoss => DevelopmentalCategory::Industry,

            // Identity category - role definition
            EventType::Humiliation => DevelopmentalCategory::Identity,
            EventType::Empowerment => DevelopmentalCategory::Identity,
            EventType::ContextTransition => DevelopmentalCategory::Identity,
            // Shaming affects identity
            EventType::ShamingEvent => DevelopmentalCategory::Identity,
            EventType::ChronicIllnessOnset => DevelopmentalCategory::Identity,

            // Intimacy category - close relationships
            EventType::Betrayal => DevelopmentalCategory::Intimacy,
            EventType::RelationshipEnd => DevelopmentalCategory::Intimacy,
            EventType::Bereavement => DevelopmentalCategory::Intimacy,

            // Generativity category - guiding others
            EventType::BurdenFeedback => DevelopmentalCategory::Generativity,
            EventType::FamilyDiscord => DevelopmentalCategory::Generativity,

            // Integrity category - meaning-making
            EventType::Realization => DevelopmentalCategory::Integrity,
            EventType::HistoricalEvent => DevelopmentalCategory::Integrity,
            EventType::SuicidalLoss => DevelopmentalCategory::Integrity,

            // Neutral category - cross-stage events (trauma/violence)
            EventType::Violence => DevelopmentalCategory::Neutral,
            EventType::TraumaticExposure => DevelopmentalCategory::Neutral,
            EventType::Loss => DevelopmentalCategory::Neutral,
            EventType::Interaction => DevelopmentalCategory::Neutral,
            EventType::PolicyChange => DevelopmentalCategory::Neutral,
            // AC pathway events are generally cross-stage (trauma/violence)
            EventType::NonSuicidalSelfInjury => DevelopmentalCategory::Neutral,
            EventType::ChildhoodAbuse => DevelopmentalCategory::Neutral,
            EventType::CombatExposure => DevelopmentalCategory::Neutral,
            EventType::PhysicalInjury => DevelopmentalCategory::Neutral,
            EventType::ViolenceExposure => DevelopmentalCategory::Neutral,
            EventType::PriorSuicideAttempt => DevelopmentalCategory::Neutral,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_returns_nine_categories() {
        let all = DevelopmentalCategory::all();
        assert_eq!(all.len(), 9);
    }

    #[test]
    fn all_categories_have_names() {
        for category in DevelopmentalCategory::all() {
            assert!(!category.name().is_empty());
        }
    }

    #[test]
    fn category_equality() {
        assert_eq!(
            DevelopmentalCategory::Attachment,
            DevelopmentalCategory::Attachment
        );
        assert_ne!(
            DevelopmentalCategory::Attachment,
            DevelopmentalCategory::Identity
        );
    }

    #[test]
    fn category_is_copy() {
        let c1 = DevelopmentalCategory::Attachment;
        let c2 = c1; // Copy
        assert_eq!(c1, c2);
    }

    #[test]
    fn category_is_hashable() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(DevelopmentalCategory::Attachment);
        set.insert(DevelopmentalCategory::Attachment);
        assert_eq!(set.len(), 1);

        set.insert(DevelopmentalCategory::Identity);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn category_debug_format() {
        let category = DevelopmentalCategory::Attachment;
        let debug = format!("{:?}", category);
        assert!(debug.contains("Attachment"));
    }

    // --- EventType to DevelopmentalCategory mapping tests ---

    #[test]
    fn event_type_maps_to_category() {
        // Attachment events
        assert_eq!(
            DevelopmentalCategory::from(&EventType::Support),
            DevelopmentalCategory::Attachment
        );
        assert_eq!(
            DevelopmentalCategory::from(&EventType::Conflict),
            DevelopmentalCategory::Attachment
        );
        assert_eq!(
            DevelopmentalCategory::from(&EventType::SocialExclusion),
            DevelopmentalCategory::Attachment
        );
        assert_eq!(
            DevelopmentalCategory::from(&EventType::SocialInclusion),
            DevelopmentalCategory::Attachment
        );

        // Industry events
        assert_eq!(
            DevelopmentalCategory::from(&EventType::Achievement),
            DevelopmentalCategory::Industry
        );
        assert_eq!(
            DevelopmentalCategory::from(&EventType::Failure),
            DevelopmentalCategory::Industry
        );

        // Identity events
        assert_eq!(
            DevelopmentalCategory::from(&EventType::Humiliation),
            DevelopmentalCategory::Identity
        );
        assert_eq!(
            DevelopmentalCategory::from(&EventType::Empowerment),
            DevelopmentalCategory::Identity
        );
        assert_eq!(
            DevelopmentalCategory::from(&EventType::ContextTransition),
            DevelopmentalCategory::Identity
        );

        // Intimacy events
        assert_eq!(
            DevelopmentalCategory::from(&EventType::Betrayal),
            DevelopmentalCategory::Intimacy
        );

        // Generativity events
        assert_eq!(
            DevelopmentalCategory::from(&EventType::BurdenFeedback),
            DevelopmentalCategory::Generativity
        );

        // Integrity events
        assert_eq!(
            DevelopmentalCategory::from(&EventType::Realization),
            DevelopmentalCategory::Integrity
        );
        assert_eq!(
            DevelopmentalCategory::from(&EventType::HistoricalEvent),
            DevelopmentalCategory::Integrity
        );
    }

    #[test]
    fn neutral_events_map_to_neutral() {
        assert_eq!(
            DevelopmentalCategory::from(&EventType::Violence),
            DevelopmentalCategory::Neutral
        );
        assert_eq!(
            DevelopmentalCategory::from(&EventType::TraumaticExposure),
            DevelopmentalCategory::Neutral
        );
        assert_eq!(
            DevelopmentalCategory::from(&EventType::Loss),
            DevelopmentalCategory::Neutral
        );
        assert_eq!(
            DevelopmentalCategory::from(&EventType::Interaction),
            DevelopmentalCategory::Neutral
        );
        assert_eq!(
            DevelopmentalCategory::from(&EventType::PolicyChange),
            DevelopmentalCategory::Neutral
        );
    }

    #[test]
    fn all_event_types_have_category_mapping() {
        // Ensure every event type maps to a developmental category
        for event_type in EventType::all() {
            let _ = DevelopmentalCategory::from(&event_type);
        }
    }

    #[test]
    fn neutral_category_name_is_neutral() {
        assert_eq!(DevelopmentalCategory::Neutral.name(), "Neutral");
    }

    #[test]
    fn attachment_name() {
        assert_eq!(DevelopmentalCategory::Attachment.name(), "Attachment");
    }

    #[test]
    fn identity_name() {
        assert_eq!(DevelopmentalCategory::Identity.name(), "Identity");
    }

    #[test]
    fn intimacy_name() {
        assert_eq!(DevelopmentalCategory::Intimacy.name(), "Intimacy");
    }

    #[test]
    fn generativity_name() {
        assert_eq!(DevelopmentalCategory::Generativity.name(), "Generativity");
    }

    #[test]
    fn integrity_name() {
        assert_eq!(DevelopmentalCategory::Integrity.name(), "Integrity");
    }

    #[test]
    fn autonomy_name() {
        assert_eq!(DevelopmentalCategory::Autonomy.name(), "Autonomy");
    }

    #[test]
    fn initiative_name() {
        assert_eq!(DevelopmentalCategory::Initiative.name(), "Initiative");
    }

    #[test]
    fn industry_name() {
        assert_eq!(DevelopmentalCategory::Industry.name(), "Industry");
    }
}
