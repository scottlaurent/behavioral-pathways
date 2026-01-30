//! Event payload enums for type-specific event data.
//!
//! Each event type has an associated payload that contains
//! specific details about the event. All field types use
//! typed enums (no magic strings).

use crate::types::{EntityId, GroupId, MicrosystemId, RelationshipId};
use serde::{Deserialize, Serialize};

/// Type-specific event data.
///
/// Each variant corresponds to an `EventType` and contains the
/// specific details for that event.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::enums::{EventPayload, LifeDomain, SupportType};
///
/// let payload = EventPayload::Support {
///     support_type: SupportType::Emotional,
///     effectiveness: 0.8,
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EventPayload {
    /// Empty payload for events without specific data.
    ///
    /// Use when the event type itself carries all necessary information.
    Empty,

    // Social events
    /// General social exchange.
    Interaction {
        /// Topic of interaction.
        topic: Option<InteractionTopic>,
        /// Duration in minutes.
        duration_minutes: u32,
    },
    /// Social rejection/exclusion.
    SocialExclusion {
        /// Group entity was excluded from.
        group_id: Option<GroupId>,
        /// Whether exclusion was explicit (vs. subtle).
        explicit: bool,
    },
    /// Social acceptance/inclusion.
    SocialInclusion {
        /// Group entity was included in.
        group_id: Option<GroupId>,
    },
    /// Feedback perceived as indicating burden.
    BurdenFeedback {
        /// Relationship source of feedback.
        source_relationship: Option<RelationshipId>,
        /// Whether feedback was verbal (vs. behavioral).
        verbal: bool,
    },
    /// Trust violation.
    Betrayal {
        /// Degree of confidence violated (0.0-1.0).
        confidence_violated: f64,
    },
    /// Received help or support.
    Support {
        /// Type of support received.
        support_type: SupportType,
        /// How effective the support was (0.0-1.0).
        effectiveness: f64,
    },
    /// Interpersonal disagreement.
    Conflict {
        /// Whether conflict was verbal.
        verbal: bool,
        /// Whether conflict was physical.
        physical: bool,
        /// Whether conflict was resolved.
        resolved: bool,
    },
    /// Physical aggression.
    Violence {
        /// Weapon involved, if any.
        weapon: Option<WeaponType>,
        /// Severity of injury (0.0-1.0).
        injury_severity: f64,
    },

    // Control events
    /// Loss of control/status.
    Humiliation {
        /// Whether humiliation was public.
        public: bool,
        /// Entity who caused the humiliation.
        perpetrator: Option<EntityId>,
    },
    /// Gain of control/status.
    Empowerment {
        /// Life domain where empowerment occurred.
        domain: LifeDomain,
    },

    // Achievement events
    /// Success or accomplishment.
    Achievement {
        /// Life domain of achievement.
        domain: LifeDomain,
        /// Magnitude of achievement (0.0-1.0).
        magnitude: f64,
    },
    /// Goal failure.
    Failure {
        /// Life domain of failure.
        domain: LifeDomain,
        /// Whether failure was public.
        public: bool,
    },
    /// Loss of person/resource.
    Loss {
        /// Type of loss.
        loss_type: LossType,
    },

    // Environmental events
    /// Institutional/policy change.
    PolicyChange {
        /// Area affected by policy.
        policy_area: PolicyArea,
        /// How favorable the change is (-1.0 to 1.0).
        favorability: f64,
    },
    /// Movement between microsystems.
    ContextTransition {
        /// Microsystem transitioning from.
        from: MicrosystemId,
        /// Microsystem transitioning to.
        to: MicrosystemId,
    },
    /// Chronosystem-level event.
    HistoricalEvent {
        /// Type of historical event.
        event_type: HistoricalEventType,
        /// Geographic/social scope.
        scope: HistoricalScope,
    },

    // Internal events
    /// Insight or realization.
    Realization {
        /// Type of realization.
        realization_type: RealizationType,
    },
    /// Exposure to painful/frightening stimulus.
    TraumaticExposure {
        /// Type of trauma.
        trauma_type: TraumaType,
        /// Proximity to the trauma (0.0 distant to 1.0 direct).
        proximity: f64,
    },
}

/// Topic of a social interaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InteractionTopic {
    /// Work-related discussion.
    Work,
    /// Personal matters.
    Personal,
    /// Casual conversation.
    Casual,
    /// Deep, meaningful conversation.
    DeepConversation,
    /// Conflict or disagreement.
    Conflict,
    /// Providing or receiving support.
    Support,
}

impl InteractionTopic {
    /// Returns a human-readable name for this topic.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            InteractionTopic::Work => "Work",
            InteractionTopic::Personal => "Personal",
            InteractionTopic::Casual => "Casual",
            InteractionTopic::DeepConversation => "Deep Conversation",
            InteractionTopic::Conflict => "Conflict",
            InteractionTopic::Support => "Support",
        }
    }

    /// Returns all topic variants.
    #[must_use]
    pub const fn all() -> [InteractionTopic; 6] {
        [
            InteractionTopic::Work,
            InteractionTopic::Personal,
            InteractionTopic::Casual,
            InteractionTopic::DeepConversation,
            InteractionTopic::Conflict,
            InteractionTopic::Support,
        ]
    }
}

/// Life domain for achievements, empowerment, and failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LifeDomain {
    /// Professional work.
    Work,
    /// Academic pursuits.
    Academic,
    /// Social relationships.
    Social,
    /// Athletic/physical activities.
    Athletic,
    /// Creative endeavors.
    Creative,
    /// Financial matters.
    Financial,
    /// Physical or mental health.
    Health,
    /// Romantic relationships.
    Relationship,
}

impl LifeDomain {
    /// Returns a human-readable name for this domain.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            LifeDomain::Work => "Work",
            LifeDomain::Academic => "Academic",
            LifeDomain::Social => "Social",
            LifeDomain::Athletic => "Athletic",
            LifeDomain::Creative => "Creative",
            LifeDomain::Financial => "Financial",
            LifeDomain::Health => "Health",
            LifeDomain::Relationship => "Relationship",
        }
    }

    /// Returns all domain variants.
    #[must_use]
    pub const fn all() -> [LifeDomain; 8] {
        [
            LifeDomain::Work,
            LifeDomain::Academic,
            LifeDomain::Social,
            LifeDomain::Athletic,
            LifeDomain::Creative,
            LifeDomain::Financial,
            LifeDomain::Health,
            LifeDomain::Relationship,
        ]
    }
}

/// Policy area affected by a policy change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PolicyArea {
    /// Economic policy.
    Economic,
    /// Social policy.
    Social,
    /// Environmental policy.
    Environmental,
    /// Healthcare policy.
    Healthcare,
    /// Education policy.
    Education,
    /// Housing policy.
    Housing,
}

impl PolicyArea {
    /// Returns a human-readable name for this area.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            PolicyArea::Economic => "Economic",
            PolicyArea::Social => "Social",
            PolicyArea::Environmental => "Environmental",
            PolicyArea::Healthcare => "Healthcare",
            PolicyArea::Education => "Education",
            PolicyArea::Housing => "Housing",
        }
    }

    /// Returns all policy area variants.
    #[must_use]
    pub const fn all() -> [PolicyArea; 6] {
        [
            PolicyArea::Economic,
            PolicyArea::Social,
            PolicyArea::Environmental,
            PolicyArea::Healthcare,
            PolicyArea::Education,
            PolicyArea::Housing,
        ]
    }
}

/// Type of historical/chronosystem event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HistoricalEventType {
    /// Armed conflict.
    War,
    /// Disease outbreak.
    Pandemic,
    /// Financial crisis.
    EconomicCrisis,
    /// Natural disaster.
    NaturalDisaster,
    /// Political upheaval.
    PoliticalChange,
    /// Technological shift.
    TechnologicalShift,
}

impl HistoricalEventType {
    /// Returns a human-readable name for this event type.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            HistoricalEventType::War => "War",
            HistoricalEventType::Pandemic => "Pandemic",
            HistoricalEventType::EconomicCrisis => "Economic Crisis",
            HistoricalEventType::NaturalDisaster => "Natural Disaster",
            HistoricalEventType::PoliticalChange => "Political Change",
            HistoricalEventType::TechnologicalShift => "Technological Shift",
        }
    }

    /// Returns all historical event type variants.
    #[must_use]
    pub const fn all() -> [HistoricalEventType; 6] {
        [
            HistoricalEventType::War,
            HistoricalEventType::Pandemic,
            HistoricalEventType::EconomicCrisis,
            HistoricalEventType::NaturalDisaster,
            HistoricalEventType::PoliticalChange,
            HistoricalEventType::TechnologicalShift,
        ]
    }
}

/// Type of insight or realization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RealizationType {
    /// Insight about oneself.
    SelfInsight,
    /// Insight about a relationship.
    RelationshipInsight,
    /// Insight about morality or ethics.
    MoralInsight,
    /// Insight about existence or meaning.
    ExistentialInsight,
}

impl RealizationType {
    /// Returns a human-readable name for this realization type.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            RealizationType::SelfInsight => "Self Insight",
            RealizationType::RelationshipInsight => "Relationship Insight",
            RealizationType::MoralInsight => "Moral Insight",
            RealizationType::ExistentialInsight => "Existential Insight",
        }
    }

    /// Returns all realization type variants.
    #[must_use]
    pub const fn all() -> [RealizationType; 4] {
        [
            RealizationType::SelfInsight,
            RealizationType::RelationshipInsight,
            RealizationType::MoralInsight,
            RealizationType::ExistentialInsight,
        ]
    }
}

/// Type of support received.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SupportType {
    /// Emotional support (listening, empathy).
    Emotional,
    /// Instrumental support (practical help).
    Instrumental,
    /// Informational support (advice, information).
    Informational,
    /// Companionship support (presence, togetherness).
    Companionship,
}

impl SupportType {
    /// Returns a human-readable name for this support type.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            SupportType::Emotional => "Emotional",
            SupportType::Instrumental => "Instrumental",
            SupportType::Informational => "Informational",
            SupportType::Companionship => "Companionship",
        }
    }

    /// Returns all support type variants.
    #[must_use]
    pub const fn all() -> [SupportType; 4] {
        [
            SupportType::Emotional,
            SupportType::Instrumental,
            SupportType::Informational,
            SupportType::Companionship,
        ]
    }
}

/// Type of weapon involved in violence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponType {
    /// No weapon (unarmed).
    None,
    /// Blunt object.
    Blunt,
    /// Sharp/edged weapon.
    Sharp,
    /// Firearm.
    Firearm,
}

impl WeaponType {
    /// Returns a human-readable name for this weapon type.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            WeaponType::None => "None",
            WeaponType::Blunt => "Blunt",
            WeaponType::Sharp => "Sharp",
            WeaponType::Firearm => "Firearm",
        }
    }

    /// Returns all weapon type variants.
    #[must_use]
    pub const fn all() -> [WeaponType; 4] {
        [
            WeaponType::None,
            WeaponType::Blunt,
            WeaponType::Sharp,
            WeaponType::Firearm,
        ]
    }
}

/// Type of loss experienced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LossType {
    /// Loss of a person (death, separation).
    Person,
    /// Loss of a resource (money, property).
    Resource,
    /// Loss of status or position.
    Status,
    /// Loss of an opportunity.
    Opportunity,
}

impl LossType {
    /// Returns a human-readable name for this loss type.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            LossType::Person => "Person",
            LossType::Resource => "Resource",
            LossType::Status => "Status",
            LossType::Opportunity => "Opportunity",
        }
    }

    /// Returns all loss type variants.
    #[must_use]
    pub const fn all() -> [LossType; 4] {
        [
            LossType::Person,
            LossType::Resource,
            LossType::Status,
            LossType::Opportunity,
        ]
    }
}

/// Type of trauma experienced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TraumaType {
    /// Physical trauma (injury, pain).
    Physical,
    /// Emotional trauma (psychological harm).
    Emotional,
    /// Witnessing trauma to others.
    Witnessing,
}

impl TraumaType {
    /// Returns a human-readable name for this trauma type.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            TraumaType::Physical => "Physical",
            TraumaType::Emotional => "Emotional",
            TraumaType::Witnessing => "Witnessing",
        }
    }

    /// Returns all trauma type variants.
    #[must_use]
    pub const fn all() -> [TraumaType; 3] {
        [
            TraumaType::Physical,
            TraumaType::Emotional,
            TraumaType::Witnessing,
        ]
    }
}

/// Geographic/social scope of a historical event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HistoricalScope {
    /// Local community impact.
    Local,
    /// Regional impact.
    Regional,
    /// National impact.
    National,
    /// Global impact.
    Global,
}

impl HistoricalScope {
    /// Returns a human-readable name for this scope.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            HistoricalScope::Local => "Local",
            HistoricalScope::Regional => "Regional",
            HistoricalScope::National => "National",
            HistoricalScope::Global => "Global",
        }
    }

    /// Returns all scope variants.
    #[must_use]
    pub const fn all() -> [HistoricalScope; 4] {
        [
            HistoricalScope::Local,
            HistoricalScope::Regional,
            HistoricalScope::National,
            HistoricalScope::Global,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unpack_interaction(payload: EventPayload) -> (Option<InteractionTopic>, u32) {
        match payload {
            EventPayload::Interaction {
                topic,
                duration_minutes,
            } => (topic, duration_minutes),
            _ => panic!("Expected Interaction payload"),
        }
    }

    fn unpack_social_exclusion(payload: EventPayload) -> (Option<GroupId>, bool) {
        match payload {
            EventPayload::SocialExclusion { group_id, explicit } => (group_id, explicit),
            _ => panic!("Expected SocialExclusion payload"),
        }
    }

    fn unpack_support(payload: EventPayload) -> (SupportType, f64) {
        match payload {
            EventPayload::Support {
                support_type,
                effectiveness,
            } => (support_type, effectiveness),
            _ => panic!("Expected Support payload"),
        }
    }

    fn unpack_violence(payload: EventPayload) -> (Option<WeaponType>, f64) {
        match payload {
            EventPayload::Violence {
                weapon,
                injury_severity,
            } => (weapon, injury_severity),
            _ => panic!("Expected Violence payload"),
        }
    }

    #[test]
    fn event_payload_interaction_creation() {
        let payload = EventPayload::Interaction {
            topic: Some(InteractionTopic::DeepConversation),
            duration_minutes: 45,
        };

        let (topic, duration_minutes) = unpack_interaction(payload);
        assert_eq!(topic, Some(InteractionTopic::DeepConversation));
        assert_eq!(duration_minutes, 45);
    }

    #[test]
    #[should_panic(expected = "Expected Interaction payload")]
    fn event_payload_interaction_wrong_variant_panics() {
        let payload = EventPayload::Support {
            support_type: SupportType::Emotional,
            effectiveness: 0.4,
        };
        let _ = unpack_interaction(payload);
    }

    #[test]
    fn event_payload_social_exclusion_creation() {
        let group = GroupId::new("team_alpha").unwrap();
        let payload = EventPayload::SocialExclusion {
            group_id: Some(group.clone()),
            explicit: true,
        };

        let (group_id, explicit) = unpack_social_exclusion(payload);
        assert_eq!(group_id, Some(group));
        assert!(explicit);
    }

    #[test]
    #[should_panic(expected = "Expected SocialExclusion payload")]
    fn event_payload_social_exclusion_wrong_variant_panics() {
        let payload = EventPayload::Interaction {
            topic: None,
            duration_minutes: 5,
        };
        let _ = unpack_social_exclusion(payload);
    }

    #[test]
    fn event_payload_support_creation() {
        let payload = EventPayload::Support {
            support_type: SupportType::Emotional,
            effectiveness: 0.8,
        };

        let (support_type, effectiveness) = unpack_support(payload);
        assert_eq!(support_type, SupportType::Emotional);
        assert!((effectiveness - 0.8).abs() < f64::EPSILON);
    }

    #[test]
    #[should_panic(expected = "Expected Support payload")]
    fn event_payload_support_wrong_variant_panics() {
        let payload = EventPayload::Violence {
            weapon: None,
            injury_severity: 0.1,
        };
        let _ = unpack_support(payload);
    }

    #[test]
    fn event_payload_violence_creation() {
        let payload = EventPayload::Violence {
            weapon: Some(WeaponType::Blunt),
            injury_severity: 0.3,
        };

        let (weapon, injury_severity) = unpack_violence(payload);
        assert_eq!(weapon, Some(WeaponType::Blunt));
        assert!((injury_severity - 0.3).abs() < f64::EPSILON);
    }

    #[test]
    #[should_panic(expected = "Expected Violence payload")]
    fn event_payload_violence_wrong_variant_panics() {
        let payload = EventPayload::Support {
            support_type: SupportType::Instrumental,
            effectiveness: 0.6,
        };
        let _ = unpack_violence(payload);
    }

    #[test]
    fn event_payload_all_variants_constructible() {
        // Just verify all variants can be constructed
        let _ = EventPayload::Interaction {
            topic: None,
            duration_minutes: 0,
        };
        let _ = EventPayload::SocialExclusion {
            group_id: None,
            explicit: false,
        };
        let _ = EventPayload::SocialInclusion { group_id: None };
        let _ = EventPayload::BurdenFeedback {
            source_relationship: None,
            verbal: false,
        };
        let _ = EventPayload::Betrayal {
            confidence_violated: 0.5,
        };
        let _ = EventPayload::Support {
            support_type: SupportType::Emotional,
            effectiveness: 0.5,
        };
        let _ = EventPayload::Conflict {
            verbal: true,
            physical: false,
            resolved: false,
        };
        let _ = EventPayload::Violence {
            weapon: None,
            injury_severity: 0.0,
        };
        let _ = EventPayload::Humiliation {
            public: false,
            perpetrator: None,
        };
        let _ = EventPayload::Empowerment {
            domain: LifeDomain::Work,
        };
        let _ = EventPayload::Achievement {
            domain: LifeDomain::Academic,
            magnitude: 0.5,
        };
        let _ = EventPayload::Failure {
            domain: LifeDomain::Financial,
            public: false,
        };
        let _ = EventPayload::Loss {
            loss_type: LossType::Person,
        };
        let _ = EventPayload::PolicyChange {
            policy_area: PolicyArea::Healthcare,
            favorability: 0.5,
        };
        let from = MicrosystemId::new("home").unwrap();
        let to = MicrosystemId::new("work").unwrap();
        let _ = EventPayload::ContextTransition { from, to };
        let _ = EventPayload::HistoricalEvent {
            event_type: HistoricalEventType::Pandemic,
            scope: HistoricalScope::Global,
        };
        let _ = EventPayload::Realization {
            realization_type: RealizationType::SelfInsight,
        };
        let _ = EventPayload::TraumaticExposure {
            trauma_type: TraumaType::Witnessing,
            proximity: 0.5,
        };
    }

    #[test]
    fn interaction_topic_all() {
        let all = InteractionTopic::all();
        assert_eq!(all.len(), 6);
    }

    #[test]
    fn life_domain_all() {
        let all = LifeDomain::all();
        assert_eq!(all.len(), 8);
    }

    #[test]
    fn policy_area_all() {
        let all = PolicyArea::all();
        assert_eq!(all.len(), 6);
    }

    #[test]
    fn historical_event_type_all() {
        let all = HistoricalEventType::all();
        assert_eq!(all.len(), 6);
    }

    #[test]
    fn realization_type_all() {
        let all = RealizationType::all();
        assert_eq!(all.len(), 4);
    }

    #[test]
    fn support_type_all() {
        let all = SupportType::all();
        assert_eq!(all.len(), 4);
    }

    #[test]
    fn weapon_type_all() {
        let all = WeaponType::all();
        assert_eq!(all.len(), 4);
    }

    #[test]
    fn loss_type_all() {
        let all = LossType::all();
        assert_eq!(all.len(), 4);
    }

    #[test]
    fn trauma_type_all() {
        let all = TraumaType::all();
        assert_eq!(all.len(), 3);
    }

    #[test]
    fn historical_scope_all() {
        let all = HistoricalScope::all();
        assert_eq!(all.len(), 4);
    }

    #[test]
    fn all_supporting_enums_have_names() {
        for t in InteractionTopic::all() {
            assert!(!t.name().is_empty());
        }
        for d in LifeDomain::all() {
            assert!(!d.name().is_empty());
        }
        for a in PolicyArea::all() {
            assert!(!a.name().is_empty());
        }
        for e in HistoricalEventType::all() {
            assert!(!e.name().is_empty());
        }
        for r in RealizationType::all() {
            assert!(!r.name().is_empty());
        }
        for s in SupportType::all() {
            assert!(!s.name().is_empty());
        }
        for w in WeaponType::all() {
            assert!(!w.name().is_empty());
        }
        for l in LossType::all() {
            assert!(!l.name().is_empty());
        }
        for t in TraumaType::all() {
            assert!(!t.name().is_empty());
        }
        for s in HistoricalScope::all() {
            assert!(!s.name().is_empty());
        }
    }

    #[test]
    fn supporting_enums_are_copy() {
        let t1 = InteractionTopic::Work;
        let t2 = t1;
        assert_eq!(t1, t2);

        let d1 = LifeDomain::Academic;
        let d2 = d1;
        assert_eq!(d1, d2);
    }

    #[test]
    fn event_payload_debug_format() {
        let payload = EventPayload::Support {
            support_type: SupportType::Emotional,
            effectiveness: 0.8,
        };
        let debug = format!("{:?}", payload);
        assert!(debug.contains("Support"));
        assert!(debug.contains("Emotional"));
    }

    #[test]
    fn event_payload_clone() {
        let payload = EventPayload::Achievement {
            domain: LifeDomain::Work,
            magnitude: 0.9,
        };
        let cloned = payload.clone();
        assert_eq!(payload, cloned);
    }

    #[test]
    fn event_payload_empty_variant() {
        let payload = EventPayload::Empty;
        let debug = format!("{:?}", payload);
        assert!(debug.contains("Empty"));

        let cloned = payload.clone();
        assert_eq!(payload, cloned);
    }
}
