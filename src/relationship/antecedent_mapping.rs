//! Mapping of event types to trust antecedents.
//!
//! This table links observable events to trust antecedent types
//! with directional valence and base magnitudes.

use crate::event::Event;
use crate::enums::{
    EventPayload, EventTag, EventType, InteractionTopic, LifeDomain, LossType, RealizationType,
    SupportType, TraumaType,
};
use crate::relationship::{AntecedentDirection, AntecedentType};
use crate::enums::TrustDomain;

/// A mapping entry from an event type to a trust antecedent.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AntecedentMapping {
    /// The trust dimension affected.
    pub antecedent_type: AntecedentType,
    /// Whether the antecedent is positive or negative.
    pub direction: AntecedentDirection,
    /// Base magnitude multiplier (0-1) applied to event severity.
    pub base_magnitude: f32,
    /// Narrative context string.
    pub context: &'static str,
    /// Trust domain affected by the antecedent.
    pub domain: TrustDomain,
}

impl AntecedentMapping {
    /// Creates a new mapping entry.
    pub const fn new(
        antecedent_type: AntecedentType,
        direction: AntecedentDirection,
        base_magnitude: f32,
        context: &'static str,
    ) -> Self {
        AntecedentMapping {
            antecedent_type,
            direction,
            base_magnitude,
            context,
            domain: antecedent_type.trust_domain(),
        }
    }
}

const ACHIEVEMENT_ANTECEDENTS: [AntecedentMapping; 1] = [AntecedentMapping::new(
    AntecedentType::Ability,
    AntecedentDirection::Positive,
    0.2,
    "task_completed_well",
)];

const FAILURE_ANTECEDENTS: [AntecedentMapping; 1] = [AntecedentMapping::new(
    AntecedentType::Ability,
    AntecedentDirection::Negative,
    0.2,
    "task_failed",
)];

const SUPPORT_ANTECEDENTS: [AntecedentMapping; 2] = [
    AntecedentMapping::new(
        AntecedentType::Benevolence,
        AntecedentDirection::Positive,
        0.3,
        "support",
    ),
    AntecedentMapping::new(
        AntecedentType::Ability,
        AntecedentDirection::Positive,
        0.15,
        "support_competence",
    ),
];

const BETRAYAL_ANTECEDENTS: [AntecedentMapping; 2] = [
    AntecedentMapping::new(
        AntecedentType::Integrity,
        AntecedentDirection::Negative,
        0.4,
        "betrayed_confidence",
    ),
    AntecedentMapping::new(
        AntecedentType::Benevolence,
        AntecedentDirection::Negative,
        0.2,
        "betrayal",
    ),
];

const CONFLICT_ANTECEDENTS: [AntecedentMapping; 2] = [
    AntecedentMapping::new(
        AntecedentType::Benevolence,
        AntecedentDirection::Negative,
        0.2,
        "conflict",
    ),
    AntecedentMapping::new(
        AntecedentType::Ability,
        AntecedentDirection::Negative,
        0.2,
        "public_disagreement",
    ),
];

const INTERACTION_ANTECEDENTS: [AntecedentMapping; 2] = [
    AntecedentMapping::new(
        AntecedentType::Ability,
        AntecedentDirection::Positive,
        0.01,
        "daily_interaction",
    ),
    AntecedentMapping::new(
        AntecedentType::Benevolence,
        AntecedentDirection::Positive,
        0.15,
        "shared_vulnerability",
    ),
];

const SOCIAL_INCLUSION_ANTECEDENTS: [AntecedentMapping; 1] = [AntecedentMapping::new(
    AntecedentType::Benevolence,
    AntecedentDirection::Positive,
    0.2,
    "social_inclusion",
)];

const SOCIAL_EXCLUSION_ANTECEDENTS: [AntecedentMapping; 1] = [AntecedentMapping::new(
    AntecedentType::Benevolence,
    AntecedentDirection::Negative,
    0.2,
    "social_exclusion",
)];

const BURDEN_FEEDBACK_ANTECEDENTS: [AntecedentMapping; 1] = [AntecedentMapping::new(
    AntecedentType::Benevolence,
    AntecedentDirection::Negative,
    0.25,
    "burden_feedback",
)];

const VIOLENCE_ANTECEDENTS: [AntecedentMapping; 2] = [
    AntecedentMapping::new(
        AntecedentType::Integrity,
        AntecedentDirection::Negative,
        0.5,
        "violence",
    ),
    AntecedentMapping::new(
        AntecedentType::Benevolence,
        AntecedentDirection::Negative,
        0.4,
        "violence",
    ),
];

const HUMILIATION_ANTECEDENTS: [AntecedentMapping; 2] = [
    AntecedentMapping::new(
        AntecedentType::Integrity,
        AntecedentDirection::Negative,
        0.25,
        "humiliation",
    ),
    AntecedentMapping::new(
        AntecedentType::Ability,
        AntecedentDirection::Negative,
        0.15,
        "public_disagreement",
    ),
];

const EMPOWERMENT_ANTECEDENTS: [AntecedentMapping; 1] = [AntecedentMapping::new(
    AntecedentType::Ability,
    AntecedentDirection::Positive,
    0.15,
    "empowerment",
)];

const LOSS_ANTECEDENTS: [AntecedentMapping; 1] = [AntecedentMapping::new(
    AntecedentType::Integrity,
    AntecedentDirection::Negative,
    0.2,
    "loss_caused",
)];

const REALIZATION_ANTECEDENTS: [AntecedentMapping; 1] = [AntecedentMapping::new(
    AntecedentType::Integrity,
    AntecedentDirection::Positive,
    0.2,
    "honest_difficult_truth",
)];

const TRAUMATIC_EXPOSURE_ANTECEDENTS: [AntecedentMapping; 2] = [
    AntecedentMapping::new(
        AntecedentType::Integrity,
        AntecedentDirection::Negative,
        0.25,
        "traumatic_exposure",
    ),
    AntecedentMapping::new(
        AntecedentType::Benevolence,
        AntecedentDirection::Negative,
        0.2,
        "traumatic_exposure",
    ),
];

/// Trust antecedent lookup table.
pub const TRUST_ANTECEDENT_TABLE: &[(EventType, &[AntecedentMapping])] = &[
    (EventType::Achievement, &ACHIEVEMENT_ANTECEDENTS),
    (EventType::Failure, &FAILURE_ANTECEDENTS),
    (EventType::Support, &SUPPORT_ANTECEDENTS),
    (EventType::Betrayal, &BETRAYAL_ANTECEDENTS),
    (EventType::Conflict, &CONFLICT_ANTECEDENTS),
    (EventType::Interaction, &INTERACTION_ANTECEDENTS),
    (EventType::SocialInclusion, &SOCIAL_INCLUSION_ANTECEDENTS),
    (EventType::SocialExclusion, &SOCIAL_EXCLUSION_ANTECEDENTS),
    (EventType::BurdenFeedback, &BURDEN_FEEDBACK_ANTECEDENTS),
    (EventType::Violence, &VIOLENCE_ANTECEDENTS),
    (EventType::Humiliation, &HUMILIATION_ANTECEDENTS),
    (EventType::Empowerment, &EMPOWERMENT_ANTECEDENTS),
    (EventType::Loss, &LOSS_ANTECEDENTS),
    (EventType::Realization, &REALIZATION_ANTECEDENTS),
    (EventType::TraumaticExposure, &TRAUMATIC_EXPOSURE_ANTECEDENTS),
];

fn clamp01(value: f32) -> f32 {
    value.clamp(0.0, 1.0)
}

fn witness_weight(event: &Event) -> f32 {
    if event.has_tag(EventTag::Witnessed) {
        0.5
    } else {
        1.0
    }
}

fn stakes_weight(event: &Event) -> f32 {
    if event.has_tag(EventTag::HighStakes) {
        1.3
    } else if event.has_tag(EventTag::LowStakes) {
        0.8
    } else {
        1.0
    }
}

fn duration_factor(minutes: u32) -> f32 {
    (minutes as f32 / 60.0).clamp(0.0, 1.0)
}

/// Returns the antecedent mappings for an event.
#[must_use]
pub fn get_antecedent_for_event(event: &Event) -> Vec<AntecedentMapping> {
    let witness = witness_weight(event);
    let stakes = stakes_weight(event);

    match event.event_type() {
        EventType::Achievement => {
            let magnitude = match event.payload() {
                EventPayload::Achievement { magnitude, .. } => *magnitude as f32,
                _ => 0.6,
            };
            let base = ACHIEVEMENT_ANTECEDENTS[0].base_magnitude;
            vec![AntecedentMapping::new(
                AntecedentType::Ability,
                AntecedentDirection::Positive,
                clamp01(base * magnitude * stakes * witness),
                ACHIEVEMENT_ANTECEDENTS[0].context,
            )]
        }
        EventType::Failure => {
            let public = match event.payload() {
                EventPayload::Failure { public, .. } => *public,
                _ => false,
            };
            let public_factor = if public { 1.2 } else { 1.0 };
            let base = FAILURE_ANTECEDENTS[0].base_magnitude;
            vec![AntecedentMapping::new(
                AntecedentType::Ability,
                AntecedentDirection::Negative,
                clamp01(base * public_factor * stakes * witness),
                FAILURE_ANTECEDENTS[0].context,
            )]
        }
        EventType::Support => {
            let (support_type, effectiveness) = match event.payload() {
                EventPayload::Support {
                    support_type,
                    effectiveness,
                } => (*support_type, *effectiveness as f32),
                _ => (SupportType::Emotional, 0.7),
            };
            let context = if event.has_tag(EventTag::HighStakes) {
                "helped_in_crisis"
            } else {
                SUPPORT_ANTECEDENTS[0].context
            };
            let base_benevolence = SUPPORT_ANTECEDENTS[0].base_magnitude;
            let mut mappings = vec![AntecedentMapping::new(
                AntecedentType::Benevolence,
                AntecedentDirection::Positive,
                clamp01(base_benevolence * effectiveness * stakes * witness),
                context,
            )];

            if matches!(
                support_type,
                SupportType::Instrumental | SupportType::Informational
            ) {
                let ability_context = if event.has_tag(EventTag::HighStakes) {
                    "helped_in_crisis"
                } else {
                    SUPPORT_ANTECEDENTS[1].context
                };
                let base_ability = SUPPORT_ANTECEDENTS[1].base_magnitude;
                mappings.push(AntecedentMapping::new(
                    AntecedentType::Ability,
                    AntecedentDirection::Positive,
                    clamp01(base_ability * effectiveness * stakes * witness),
                    ability_context,
                ));
            }

            mappings
        }
        EventType::Betrayal => {
            let confidence = match event.payload() {
                EventPayload::Betrayal {
                    confidence_violated,
                } => *confidence_violated as f32,
                _ => 0.7,
            };
            let moral_factor = if event.has_tag(EventTag::MoralViolation) {
                1.2
            } else {
                1.0
            };
            let integrity_context = if event.has_tag(EventTag::MoralViolation) {
                "lied_to"
            } else {
                BETRAYAL_ANTECEDENTS[0].context
            };
            let integrity_base = BETRAYAL_ANTECEDENTS[0].base_magnitude;
            let benevolence_base = BETRAYAL_ANTECEDENTS[1].base_magnitude;
            vec![
                AntecedentMapping::new(
                    AntecedentType::Integrity,
                    AntecedentDirection::Negative,
                    clamp01(integrity_base * confidence * moral_factor * stakes * witness),
                    integrity_context,
                ),
                AntecedentMapping::new(
                    AntecedentType::Benevolence,
                    AntecedentDirection::Negative,
                    clamp01(benevolence_base * confidence * stakes * witness),
                    BETRAYAL_ANTECEDENTS[1].context,
                ),
            ]
        }
        EventType::Conflict => {
            let (physical, verbal, resolved) = match event.payload() {
                EventPayload::Conflict {
                    physical,
                    verbal,
                    resolved,
                } => (*physical, *verbal, *resolved),
                _ => (false, true, false),
            };
            let conflict_factor = if physical {
                1.5
            } else if verbal {
                1.0
            } else {
                0.7
            };
            let resolution_factor = if resolved { 0.6 } else { 1.0 };
            let benevolence_base = CONFLICT_ANTECEDENTS[0].base_magnitude;
            let mut mappings = vec![AntecedentMapping::new(
                AntecedentType::Benevolence,
                AntecedentDirection::Negative,
                clamp01(benevolence_base * conflict_factor * resolution_factor * stakes * witness),
                CONFLICT_ANTECEDENTS[0].context,
            )];

            if event.has_tag(EventTag::HighStakes) || event.has_tag(EventTag::Work) {
                let ability_base = CONFLICT_ANTECEDENTS[1].base_magnitude;
                mappings.push(AntecedentMapping::new(
                    AntecedentType::Ability,
                    AntecedentDirection::Negative,
                    clamp01(
                        ability_base * conflict_factor * resolution_factor * stakes * witness,
                    ),
                    CONFLICT_ANTECEDENTS[1].context,
                ));
            }

            mappings
        }
        EventType::Interaction => {
            let (topic, duration_minutes) = match event.payload() {
                EventPayload::Interaction {
                    topic,
                    duration_minutes,
                } => (*topic, *duration_minutes),
                _ => (None, 10),
            };
            let duration = duration_factor(duration_minutes);
            let ability_base = INTERACTION_ANTECEDENTS[0].base_magnitude;
            let mut mappings = vec![AntecedentMapping::new(
                AntecedentType::Ability,
                AntecedentDirection::Positive,
                clamp01((ability_base + 0.04 * duration) * witness),
                INTERACTION_ANTECEDENTS[0].context,
            )];

            if matches!(
                topic,
                Some(
                    InteractionTopic::DeepConversation
                        | InteractionTopic::Personal
                        | InteractionTopic::Support
                )
            ) {
                let benevolence_base = INTERACTION_ANTECEDENTS[1].base_magnitude;
                mappings.push(AntecedentMapping::new(
                    AntecedentType::Benevolence,
                    AntecedentDirection::Positive,
                    clamp01(benevolence_base * duration * witness),
                    INTERACTION_ANTECEDENTS[1].context,
                ));
            }

            mappings
        }
        EventType::SocialInclusion => {
            let base = SOCIAL_INCLUSION_ANTECEDENTS[0].base_magnitude;
            vec![AntecedentMapping::new(
                AntecedentType::Benevolence,
                AntecedentDirection::Positive,
                clamp01(base * stakes * witness),
                SOCIAL_INCLUSION_ANTECEDENTS[0].context,
            )]
        }
        EventType::SocialExclusion => {
            let base = SOCIAL_EXCLUSION_ANTECEDENTS[0].base_magnitude;
            vec![AntecedentMapping::new(
                AntecedentType::Benevolence,
                AntecedentDirection::Negative,
                clamp01(base * stakes * witness),
                SOCIAL_EXCLUSION_ANTECEDENTS[0].context,
            )]
        }
        EventType::BurdenFeedback => {
            let verbal = match event.payload() {
                EventPayload::BurdenFeedback { verbal, .. } => *verbal,
                _ => false,
            };
            let verbal_factor = if verbal { 1.1 } else { 1.0 };
            let base = BURDEN_FEEDBACK_ANTECEDENTS[0].base_magnitude;
            vec![AntecedentMapping::new(
                AntecedentType::Benevolence,
                AntecedentDirection::Negative,
                clamp01(base * verbal_factor * stakes * witness),
                BURDEN_FEEDBACK_ANTECEDENTS[0].context,
            )]
        }
        EventType::Violence => {
            let injury_severity = match event.payload() {
                EventPayload::Violence { injury_severity, .. } => *injury_severity as f32,
                _ => 0.7,
            };
            let integrity_base = VIOLENCE_ANTECEDENTS[0].base_magnitude;
            let benevolence_base = VIOLENCE_ANTECEDENTS[1].base_magnitude;
            vec![
                AntecedentMapping::new(
                    AntecedentType::Integrity,
                    AntecedentDirection::Negative,
                    clamp01(integrity_base * injury_severity * witness),
                    VIOLENCE_ANTECEDENTS[0].context,
                ),
                AntecedentMapping::new(
                    AntecedentType::Benevolence,
                    AntecedentDirection::Negative,
                    clamp01(benevolence_base * injury_severity * witness),
                    VIOLENCE_ANTECEDENTS[1].context,
                ),
            ]
        }
        EventType::Humiliation => {
            let public = match event.payload() {
                EventPayload::Humiliation { public, .. } => *public,
                _ => false,
            };
            let public_factor = if public { 1.3 } else { 1.0 };
            let integrity_base = HUMILIATION_ANTECEDENTS[0].base_magnitude;
            let mut mappings = vec![AntecedentMapping::new(
                AntecedentType::Integrity,
                AntecedentDirection::Negative,
                clamp01(integrity_base * public_factor * stakes * witness),
                HUMILIATION_ANTECEDENTS[0].context,
            )];

            if public {
                let ability_base = HUMILIATION_ANTECEDENTS[1].base_magnitude;
                mappings.push(AntecedentMapping::new(
                    AntecedentType::Ability,
                    AntecedentDirection::Negative,
                    clamp01(ability_base * public_factor * stakes * witness),
                    HUMILIATION_ANTECEDENTS[1].context,
                ));
            }

            mappings
        }
        EventType::Empowerment => {
            let domain = match event.payload() {
                EventPayload::Empowerment { domain } => *domain,
                _ => LifeDomain::Work,
            };
            let domain_factor = match domain {
                LifeDomain::Work | LifeDomain::Academic | LifeDomain::Financial => 1.2,
                _ => 1.0,
            };
            let base = EMPOWERMENT_ANTECEDENTS[0].base_magnitude;
            vec![AntecedentMapping::new(
                AntecedentType::Ability,
                AntecedentDirection::Positive,
                clamp01(base * domain_factor * stakes * witness),
                EMPOWERMENT_ANTECEDENTS[0].context,
            )]
        }
        EventType::Loss => {
            let loss_type = match event.payload() {
                EventPayload::Loss { loss_type } => *loss_type,
                _ => LossType::Resource,
            };
            let loss_factor = match loss_type {
                LossType::Person => 1.3,
                LossType::Status => 1.1,
                LossType::Resource | LossType::Opportunity => 1.0,
            };
            let base = LOSS_ANTECEDENTS[0].base_magnitude;
            vec![AntecedentMapping::new(
                AntecedentType::Integrity,
                AntecedentDirection::Negative,
                clamp01(base * loss_factor * stakes * witness),
                LOSS_ANTECEDENTS[0].context,
            )]
        }
        EventType::Realization => {
            let realization_type = match event.payload() {
                EventPayload::Realization { realization_type } => *realization_type,
                _ => RealizationType::MoralInsight,
            };
            match realization_type {
                RealizationType::MoralInsight => {
                    let base = REALIZATION_ANTECEDENTS[0].base_magnitude;
                    vec![AntecedentMapping::new(
                        AntecedentType::Integrity,
                        AntecedentDirection::Positive,
                        clamp01(base * stakes * witness),
                        REALIZATION_ANTECEDENTS[0].context,
                    )]
                }
                RealizationType::RelationshipInsight => {
                    let base = REALIZATION_ANTECEDENTS[0].base_magnitude;
                    vec![AntecedentMapping::new(
                        AntecedentType::Benevolence,
                        AntecedentDirection::Positive,
                        clamp01(base * stakes * witness),
                        "shared_vulnerability",
                    )]
                }
                _ => Vec::new(),
            }
        }
        EventType::TraumaticExposure => {
            let (trauma_type, proximity) = match event.payload() {
                EventPayload::TraumaticExposure {
                    trauma_type,
                    proximity,
                } => (*trauma_type, *proximity as f32),
                _ => (TraumaType::Emotional, 0.5),
            };
            let trauma_factor = if trauma_type == TraumaType::Witnessing {
                0.7
            } else {
                1.0
            };
            let integrity_base = TRAUMATIC_EXPOSURE_ANTECEDENTS[0].base_magnitude;
            let benevolence_base = TRAUMATIC_EXPOSURE_ANTECEDENTS[1].base_magnitude;
            vec![
                AntecedentMapping::new(
                    AntecedentType::Integrity,
                    AntecedentDirection::Negative,
                    clamp01(integrity_base * proximity * trauma_factor * witness),
                    TRAUMATIC_EXPOSURE_ANTECEDENTS[0].context,
                ),
                AntecedentMapping::new(
                    AntecedentType::Benevolence,
                    AntecedentDirection::Negative,
                    clamp01(benevolence_base * proximity * trauma_factor * witness),
                    TRAUMATIC_EXPOSURE_ANTECEDENTS[1].context,
                ),
            ]
        }
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::EventBuilder;
    use crate::enums::{EventPayload, EventTag, InteractionTopic, SupportType, TrustDomain};

    #[test]
    fn antecedent_mapping_returns_entries_for_known_event() {
        let event = EventBuilder::new(EventType::Betrayal)
            .payload(EventPayload::Betrayal {
                confidence_violated: 0.8,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);

        assert!(mappings
            .iter()
            .any(|m| m.antecedent_type == AntecedentType::Integrity));
        assert!(mappings
            .iter()
            .any(|m| m.antecedent_type == AntecedentType::Benevolence));
        assert!(mappings
            .iter()
            .any(|m| m.direction == AntecedentDirection::Negative));
    }

    #[test]
    fn antecedent_mapping_returns_empty_for_unmapped_event() {
        let event = EventBuilder::new(EventType::PolicyChange).build().unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert!(mappings.is_empty());
    }

    #[test]
    fn antecedent_mapping_table_expands_event_types() {
        assert!(TRUST_ANTECEDENT_TABLE.len() >= 15);
    }

    #[test]
    fn antecedent_mapping_table_covers_core_contexts() {
        assert_eq!(ACHIEVEMENT_ANTECEDENTS[0].context, "task_completed_well");
        assert_eq!(SUPPORT_ANTECEDENTS[0].context, "support");
        assert_eq!(SUPPORT_ANTECEDENTS[1].context, "support_competence");
        assert_eq!(CONFLICT_ANTECEDENTS[1].context, "public_disagreement");
        assert_eq!(INTERACTION_ANTECEDENTS[1].context, "shared_vulnerability");
    }

    #[test]
    fn antecedent_mapping_support_in_crisis_adds_multi_antecedents() {
        let event = EventBuilder::new(EventType::Support)
            .tag(EventTag::HighStakes)
            .payload(EventPayload::Support {
                support_type: SupportType::Instrumental,
                effectiveness: 0.9,
            })
            .build()
            .unwrap();

        let mappings = get_antecedent_for_event(&event);
        let benevolence = mappings
            .iter()
            .find(|m| m.antecedent_type == AntecedentType::Benevolence)
            .expect("expected benevolence mapping");
        assert!(benevolence.context.contains("helped_in_crisis"));
        assert!(mappings
            .iter()
            .any(|m| m.antecedent_type == AntecedentType::Ability));
    }

    #[test]
    fn antecedent_mapping_interaction_deep_conversation_adds_benevolence() {
        let event = EventBuilder::new(EventType::Interaction)
            .payload(EventPayload::Interaction {
                topic: Some(InteractionTopic::DeepConversation),
                duration_minutes: 45,
            })
            .build()
            .unwrap();

        let mappings = get_antecedent_for_event(&event);
        assert!(mappings
            .iter()
            .any(|m| m.antecedent_type == AntecedentType::Benevolence));
    }

    #[test]
    fn antecedent_mapping_new_builds_struct() {
        let mapping = AntecedentMapping::new(
            AntecedentType::Integrity,
            AntecedentDirection::Negative,
            0.42,
            "test",
        );

        assert_eq!(mapping.antecedent_type, AntecedentType::Integrity);
        assert_eq!(mapping.direction, AntecedentDirection::Negative);
        assert!((mapping.base_magnitude - 0.42).abs() < f32::EPSILON);
        assert_eq!(mapping.context, "test");
        assert_eq!(mapping.domain, TrustDomain::Disclosure);
    }

    #[test]
    fn achievement_with_magnitude() {
        let event = EventBuilder::new(EventType::Achievement)
            .payload(EventPayload::Achievement {
                magnitude: 0.9,
                domain: LifeDomain::Work,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 1);
        assert_eq!(mappings[0].antecedent_type, AntecedentType::Ability);
        assert_eq!(mappings[0].direction, AntecedentDirection::Positive);
    }

    #[test]
    fn achievement_with_high_stakes() {
        let event = EventBuilder::new(EventType::Achievement)
            .tag(EventTag::HighStakes)
            .payload(EventPayload::Achievement {
                magnitude: 0.8,
                domain: LifeDomain::Work,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 1);
        assert!(mappings[0].base_magnitude > 0.0);
    }

    #[test]
    fn achievement_witnessed_reduces_magnitude() {
        let event = EventBuilder::new(EventType::Achievement)
            .tag(EventTag::Witnessed)
            .payload(EventPayload::Achievement {
                magnitude: 0.8,
                domain: LifeDomain::Work,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 1);
    }

    #[test]
    fn failure_public() {
        let event = EventBuilder::new(EventType::Failure)
            .payload(EventPayload::Failure {
                public: true,
                domain: LifeDomain::Work,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 1);
        assert_eq!(mappings[0].antecedent_type, AntecedentType::Ability);
        assert_eq!(mappings[0].direction, AntecedentDirection::Negative);
    }

    #[test]
    fn failure_private() {
        let event = EventBuilder::new(EventType::Failure)
            .payload(EventPayload::Failure {
                public: false,
                domain: LifeDomain::Work,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 1);
    }

    #[test]
    fn failure_with_low_stakes() {
        let event = EventBuilder::new(EventType::Failure)
            .tag(EventTag::LowStakes)
            .payload(EventPayload::Failure {
                public: false,
                domain: LifeDomain::Work,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 1);
    }

    #[test]
    fn support_emotional() {
        let event = EventBuilder::new(EventType::Support)
            .payload(EventPayload::Support {
                support_type: SupportType::Emotional,
                effectiveness: 0.8,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 1);
        assert_eq!(mappings[0].antecedent_type, AntecedentType::Benevolence);
    }

    #[test]
    fn support_instrumental() {
        let event = EventBuilder::new(EventType::Support)
            .payload(EventPayload::Support {
                support_type: SupportType::Instrumental,
                effectiveness: 0.8,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 2);
        assert!(mappings
            .iter()
            .any(|m| m.antecedent_type == AntecedentType::Benevolence));
        assert!(mappings
            .iter()
            .any(|m| m.antecedent_type == AntecedentType::Ability));
    }

    #[test]
    fn support_informational() {
        let event = EventBuilder::new(EventType::Support)
            .payload(EventPayload::Support {
                support_type: SupportType::Informational,
                effectiveness: 0.7,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 2);
    }

    #[test]
    fn betrayal_with_moral_violation() {
        let event = EventBuilder::new(EventType::Betrayal)
            .tag(EventTag::MoralViolation)
            .payload(EventPayload::Betrayal {
                confidence_violated: 0.9,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 2);
        let integrity = mappings
            .iter()
            .find(|m| m.antecedent_type == AntecedentType::Integrity)
            .unwrap();
        assert_eq!(integrity.context, "lied_to");
    }

    #[test]
    fn conflict_physical() {
        let event = EventBuilder::new(EventType::Conflict)
            .payload(EventPayload::Conflict {
                physical: true,
                verbal: false,
                resolved: false,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert!(!mappings.is_empty());
        assert!(mappings
            .iter()
            .any(|m| m.antecedent_type == AntecedentType::Benevolence));
    }

    #[test]
    fn conflict_verbal() {
        let event = EventBuilder::new(EventType::Conflict)
            .payload(EventPayload::Conflict {
                physical: false,
                verbal: true,
                resolved: false,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert!(!mappings.is_empty());
    }

    #[test]
    fn conflict_resolved() {
        let event = EventBuilder::new(EventType::Conflict)
            .payload(EventPayload::Conflict {
                physical: false,
                verbal: true,
                resolved: true,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert!(!mappings.is_empty());
    }

    #[test]
    fn conflict_neither_physical_nor_verbal() {
        let event = EventBuilder::new(EventType::Conflict)
            .payload(EventPayload::Conflict {
                physical: false,
                verbal: false,
                resolved: false,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 1);
        assert_eq!(mappings[0].antecedent_type, AntecedentType::Benevolence);
    }

    #[test]
    fn conflict_with_work_tag() {
        let event = EventBuilder::new(EventType::Conflict)
            .tag(EventTag::Work)
            .payload(EventPayload::Conflict {
                physical: false,
                verbal: true,
                resolved: false,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 2);
        assert!(mappings
            .iter()
            .any(|m| m.antecedent_type == AntecedentType::Ability));
    }

    #[test]
    fn conflict_high_stakes() {
        let event = EventBuilder::new(EventType::Conflict)
            .tag(EventTag::HighStakes)
            .payload(EventPayload::Conflict {
                physical: false,
                verbal: true,
                resolved: false,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 2);
    }

    #[test]
    fn interaction_casual() {
        let event = EventBuilder::new(EventType::Interaction)
            .payload(EventPayload::Interaction {
                topic: Some(InteractionTopic::Casual),
                duration_minutes: 10,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 1);
        assert_eq!(mappings[0].antecedent_type, AntecedentType::Ability);
    }

    #[test]
    fn interaction_personal() {
        let event = EventBuilder::new(EventType::Interaction)
            .payload(EventPayload::Interaction {
                topic: Some(InteractionTopic::Personal),
                duration_minutes: 30,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 2);
        assert!(mappings
            .iter()
            .any(|m| m.antecedent_type == AntecedentType::Benevolence));
    }

    #[test]
    fn interaction_support_topic() {
        let event = EventBuilder::new(EventType::Interaction)
            .payload(EventPayload::Interaction {
                topic: Some(InteractionTopic::Support),
                duration_minutes: 45,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 2);
    }

    #[test]
    fn social_inclusion() {
        let event = EventBuilder::new(EventType::SocialInclusion)
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 1);
        assert_eq!(mappings[0].antecedent_type, AntecedentType::Benevolence);
        assert_eq!(mappings[0].direction, AntecedentDirection::Positive);
    }

    #[test]
    fn social_exclusion() {
        let event = EventBuilder::new(EventType::SocialExclusion)
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 1);
        assert_eq!(mappings[0].antecedent_type, AntecedentType::Benevolence);
        assert_eq!(mappings[0].direction, AntecedentDirection::Negative);
    }

    #[test]
    fn burden_feedback_verbal() {
        let event = EventBuilder::new(EventType::BurdenFeedback)
            .payload(EventPayload::BurdenFeedback {
                source_relationship: None,
                verbal: true,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 1);
        assert_eq!(mappings[0].antecedent_type, AntecedentType::Benevolence);
    }

    #[test]
    fn burden_feedback_nonverbal() {
        let event = EventBuilder::new(EventType::BurdenFeedback)
            .payload(EventPayload::BurdenFeedback {
                source_relationship: None,
                verbal: false,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 1);
    }

    #[test]
    fn violence_with_injury() {
        let event = EventBuilder::new(EventType::Violence)
            .payload(EventPayload::Violence {
                weapon: None,
                injury_severity: 0.9,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 2);
        assert!(mappings
            .iter()
            .any(|m| m.antecedent_type == AntecedentType::Integrity));
        assert!(mappings
            .iter()
            .any(|m| m.antecedent_type == AntecedentType::Benevolence));
    }

    #[test]
    fn humiliation_private() {
        let event = EventBuilder::new(EventType::Humiliation)
            .payload(EventPayload::Humiliation {
                public: false,
                perpetrator: None,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 1);
        assert_eq!(mappings[0].antecedent_type, AntecedentType::Integrity);
    }

    #[test]
    fn humiliation_public() {
        let event = EventBuilder::new(EventType::Humiliation)
            .payload(EventPayload::Humiliation {
                public: true,
                perpetrator: None,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 2);
        assert!(mappings
            .iter()
            .any(|m| m.antecedent_type == AntecedentType::Ability));
    }

    #[test]
    fn empowerment_work_domain() {
        let event = EventBuilder::new(EventType::Empowerment)
            .payload(EventPayload::Empowerment {
                domain: LifeDomain::Work,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 1);
        assert_eq!(mappings[0].antecedent_type, AntecedentType::Ability);
    }

    #[test]
    fn empowerment_academic_domain() {
        let event = EventBuilder::new(EventType::Empowerment)
            .payload(EventPayload::Empowerment {
                domain: LifeDomain::Academic,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 1);
    }

    #[test]
    fn empowerment_financial_domain() {
        let event = EventBuilder::new(EventType::Empowerment)
            .payload(EventPayload::Empowerment {
                domain: LifeDomain::Financial,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 1);
    }

    #[test]
    fn empowerment_social_domain() {
        let event = EventBuilder::new(EventType::Empowerment)
            .payload(EventPayload::Empowerment {
                domain: LifeDomain::Social,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 1);
    }

    #[test]
    fn loss_person() {
        let event = EventBuilder::new(EventType::Loss)
            .payload(EventPayload::Loss {
                loss_type: LossType::Person,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 1);
        assert_eq!(mappings[0].antecedent_type, AntecedentType::Integrity);
    }

    #[test]
    fn loss_status() {
        let event = EventBuilder::new(EventType::Loss)
            .payload(EventPayload::Loss {
                loss_type: LossType::Status,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 1);
    }

    #[test]
    fn loss_resource() {
        let event = EventBuilder::new(EventType::Loss)
            .payload(EventPayload::Loss {
                loss_type: LossType::Resource,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 1);
    }

    #[test]
    fn loss_opportunity() {
        let event = EventBuilder::new(EventType::Loss)
            .payload(EventPayload::Loss {
                loss_type: LossType::Opportunity,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 1);
    }

    #[test]
    fn realization_moral_insight() {
        let event = EventBuilder::new(EventType::Realization)
            .payload(EventPayload::Realization {
                realization_type: RealizationType::MoralInsight,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 1);
        assert_eq!(mappings[0].antecedent_type, AntecedentType::Integrity);
    }

    #[test]
    fn realization_relationship_insight() {
        let event = EventBuilder::new(EventType::Realization)
            .payload(EventPayload::Realization {
                realization_type: RealizationType::RelationshipInsight,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 1);
        assert_eq!(mappings[0].antecedent_type, AntecedentType::Benevolence);
    }

    #[test]
    fn realization_self_insight() {
        let event = EventBuilder::new(EventType::Realization)
            .payload(EventPayload::Realization {
                realization_type: RealizationType::SelfInsight,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 0);
    }

    #[test]
    fn realization_existential_insight() {
        let event = EventBuilder::new(EventType::Realization)
            .payload(EventPayload::Realization {
                realization_type: RealizationType::ExistentialInsight,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 0);
    }

    #[test]
    fn traumatic_exposure_physical() {
        let event = EventBuilder::new(EventType::TraumaticExposure)
            .payload(EventPayload::TraumaticExposure {
                trauma_type: TraumaType::Physical,
                proximity: 0.9,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 2);
    }

    #[test]
    fn traumatic_exposure_witnessing() {
        let event = EventBuilder::new(EventType::TraumaticExposure)
            .payload(EventPayload::TraumaticExposure {
                trauma_type: TraumaType::Witnessing,
                proximity: 0.7,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 2);
    }

    #[test]
    fn traumatic_exposure_emotional_not_witnessing() {
        let event = EventBuilder::new(EventType::TraumaticExposure)
            .payload(EventPayload::TraumaticExposure {
                trauma_type: TraumaType::Emotional,
                proximity: 0.8,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 2);
        // Trauma factor should be 1.0 (not 0.7 for witnessing)
        assert!(mappings.iter().all(|m| m.base_magnitude > 0.0));
    }

    #[test]
    fn traumatic_exposure_emotional() {
        let event = EventBuilder::new(EventType::TraumaticExposure)
            .payload(EventPayload::TraumaticExposure {
                trauma_type: TraumaType::Emotional,
                proximity: 0.5,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 2);
    }

    #[test]
    fn achievement_no_payload() {
        let event = EventBuilder::new(EventType::Achievement).build().unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 1);
    }

    #[test]
    fn failure_no_payload() {
        let event = EventBuilder::new(EventType::Failure).build().unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 1);
    }

    #[test]
    fn support_no_payload() {
        let event = EventBuilder::new(EventType::Support).build().unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert!(!mappings.is_empty());
    }

    #[test]
    fn betrayal_no_payload() {
        let event = EventBuilder::new(EventType::Betrayal).build().unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 2);
    }

    #[test]
    fn conflict_no_payload() {
        let event = EventBuilder::new(EventType::Conflict).build().unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert!(!mappings.is_empty());
    }

    #[test]
    fn interaction_no_payload() {
        let event = EventBuilder::new(EventType::Interaction).build().unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert!(!mappings.is_empty());
    }

    #[test]
    fn burden_feedback_no_payload() {
        let event = EventBuilder::new(EventType::BurdenFeedback)
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert!(!mappings.is_empty());
    }

    #[test]
    fn violence_no_payload() {
        let event = EventBuilder::new(EventType::Violence).build().unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 2);
    }

    #[test]
    fn humiliation_no_payload() {
        let event = EventBuilder::new(EventType::Humiliation).build().unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert!(!mappings.is_empty());
    }

    #[test]
    fn empowerment_no_payload() {
        let event = EventBuilder::new(EventType::Empowerment).build().unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 1);
    }

    #[test]
    fn loss_no_payload() {
        let event = EventBuilder::new(EventType::Loss).build().unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 1);
    }

    #[test]
    fn realization_no_payload() {
        let event = EventBuilder::new(EventType::Realization).build().unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert!(!mappings.is_empty());
    }

    #[test]
    fn traumatic_exposure_no_payload() {
        let event = EventBuilder::new(EventType::TraumaticExposure)
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert_eq!(mappings.len(), 2);
    }

    #[test]
    fn clamp01_clamps_values() {
        assert_eq!(clamp01(-0.1), 0.0);
        assert_eq!(clamp01(0.5), 0.5);
        assert_eq!(clamp01(1.5), 1.0);
    }

    #[test]
    fn witness_weight_full_for_direct() {
        let event = EventBuilder::new(EventType::Achievement).build().unwrap();
        assert_eq!(witness_weight(&event), 1.0);
    }

    #[test]
    fn witness_weight_reduced_for_witnessed() {
        let event = EventBuilder::new(EventType::Achievement)
            .tag(EventTag::Witnessed)
            .build()
            .unwrap();
        assert_eq!(witness_weight(&event), 0.5);
    }

    #[test]
    fn stakes_weight_normal() {
        let event = EventBuilder::new(EventType::Achievement).build().unwrap();
        assert_eq!(stakes_weight(&event), 1.0);
    }

    #[test]
    fn stakes_weight_high() {
        let event = EventBuilder::new(EventType::Achievement)
            .tag(EventTag::HighStakes)
            .build()
            .unwrap();
        assert_eq!(stakes_weight(&event), 1.3);
    }

    #[test]
    fn stakes_weight_low() {
        let event = EventBuilder::new(EventType::Achievement)
            .tag(EventTag::LowStakes)
            .build()
            .unwrap();
        assert_eq!(stakes_weight(&event), 0.8);
    }

    #[test]
    fn duration_factor_zero() {
        assert_eq!(duration_factor(0), 0.0);
    }

    #[test]
    fn duration_factor_thirty_minutes() {
        assert!((duration_factor(30) - 0.5).abs() < 0.01);
    }

    #[test]
    fn duration_factor_sixty_minutes() {
        assert_eq!(duration_factor(60), 1.0);
    }

    #[test]
    fn duration_factor_over_sixty_minutes() {
        assert_eq!(duration_factor(120), 1.0);
    }

    #[test]
    fn realization_unmapped_type_returns_empty() {
        // Realization types that don't map (SelfInsight, ExistentialInsight, etc)
        // should return empty vector to avoid spurious trust impacts
        let event = EventBuilder::new(EventType::Realization)
            .payload(EventPayload::Realization {
                realization_type: RealizationType::SelfInsight,
            })
            .build()
            .unwrap();
        let mappings = get_antecedent_for_event(&event);
        assert!(mappings.is_empty());
    }

    #[test]
    fn stakes_weight_no_tags() {
        let event = EventBuilder::new(EventType::Achievement)
            .build()
            .unwrap();
        assert_eq!(stakes_weight(&event), 1.0);
    }
}
