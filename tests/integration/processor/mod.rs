//! Targeted coverage tests for processor module edge cases.
//!
//! These tests specifically target uncovered code regions identified by llvm-cov:
//! - processor/event.rs: 7 missed regions, 2 missed lines
//! - processor/emotions.rs: 1 missed region
//! - processor/state_evolution.rs: 1 missed region, 1 missed line

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::{
    Attribution, AttributionStability, DispositionPath, EventCategory, EventPayload, EventType,
    LifeDomain, MentalHealthPath, MoodPath, NeedsPath, SocialCognitionPath, Species, StatePath,
    SupportType, RealizationType,
};
use behavioral_pathways::event::EventBuilder;
use behavioral_pathways::processor::{
    interpret_event, apply_interpreted_event, derive_emotion, process_event,
};
use behavioral_pathways::state::{IndividualState, Hexaco};
use behavioral_pathways::types::EntityId;

// ============================================================================
// processor/emotions.rs coverage
// ============================================================================

/// Test that disgust can be zero when flag is 0.0 (missing region: line 145)
#[test]
fn emotions_disgust_zero_when_flag_zero() {
    let emotions = derive_emotion(-0.8, 0.9, 0.7, 0.0);
    assert_eq!(emotions.disgust, 0.0);
}

/// Test that disgust is nonzero when flag is 1.0
#[test]
fn emotions_disgust_nonzero_when_flag_one() {
    let emotions = derive_emotion(-0.8, 0.9, 0.7, 1.0);
    assert!(emotions.disgust > 0.0);
}

/// Test derive_emotion with edge values for all octants
#[test]
fn emotions_hostile_octant() {
    let emotions = derive_emotion(-0.8, 0.8, 0.8, 0.0);
    assert!(emotions.hostile > 0.0);
}

/// Test disdainful octant
#[test]
fn emotions_disdainful_octant() {
    let emotions = derive_emotion(-0.8, -0.8, 0.8, 0.0);
    assert!(emotions.disdainful > 0.0);
}

/// Test intensity method with neutral emotion
#[test]
fn emotions_neutral_intensity_zero() {
    let emotions = derive_emotion(0.5, 0.5, 0.5, 0.0);
    assert_eq!(emotions.intensity(behavioral_pathways::enums::Emotion::Neutral), 0.0);
}

// ============================================================================
// processor/event.rs coverage - Missing regions in interpret_event
// ============================================================================

/// Test Support event with Empty payload (uses blueprint, not payload processing)
#[test]
fn event_support_with_empty_payload_uses_blueprint() {
    let entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    let event = EventBuilder::new(EventType::Support)
        .severity(0.6)
        .payload(EventPayload::Empty)
        .build()
        .unwrap();

    let interpreted = interpret_event(&event, &entity);

    // Blueprint provides: valence +0.08, loneliness -0.20, liability -0.10
    assert!(interpreted.valence_delta > 0.0);
    assert!(interpreted.loneliness_delta < 0.0);
    assert!(interpreted.perceived_liability_delta < 0.0);
}

/// Test Interaction event with payload that has no duration_minutes to process
#[test]
fn event_interaction_with_normal_payload() {
    let entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    let event = EventBuilder::new(EventType::Interaction)
        .severity(0.5)
        .payload(EventPayload::Interaction {
            topic: Some(behavioral_pathways::enums::InteractionTopic::Casual),
            duration_minutes: 30,
        })
        .build()
        .unwrap();

    let interpreted = interpret_event(&event, &entity);

    // Interaction reduces loneliness proportional to duration
    assert!(interpreted.loneliness_delta < 0.0);
}

/// Test Betray event that reduces PRC
#[test]
fn event_betrayal_reduces_prc_with_payload() {
    let entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    let event = EventBuilder::new(EventType::Betrayal)
        .severity(0.7)
        .payload(EventPayload::Betrayal {
            confidence_violated: 0.9,
        })
        .build()
        .unwrap();

    let interpreted = interpret_event(&event, &entity);

    assert!(interpreted.prc_delta < 0.0);
    assert!(interpreted.valence_delta < 0.0);
}

/// Test Conflict event in Social category that triggers blueprint path
#[test]
fn event_conflict_in_social_uses_blueprint() {
    let entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    let event = EventBuilder::new(EventType::Conflict)
        .severity(0.6)
        .payload(EventPayload::Conflict {
            verbal: true,
            physical: false,
            resolved: false,
        })
        .build()
        .unwrap();

    let interpreted = interpret_event(&event, &entity);

    // Conflict blueprint: valence -0.10, arousal +0.12, dominance -0.12
    assert!(interpreted.valence_delta < 0.0);
    assert!(interpreted.arousal_delta > 0.0);
    assert!(interpreted.dominance_delta < 0.0);

    // Conflict also adds grievance
    let has_grievance = interpreted.state_deltas.iter().any(|(path, _)| {
        matches!(path, StatePath::Disposition(DispositionPath::Grievance))
    });
    assert!(has_grievance);
}

/// Test Processing of Social event with empty payload (default case in match)
#[test]
fn event_social_with_empty_payload_no_effect() {
    let entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    let mut event = EventBuilder::new(EventType::Interaction)
        .severity(0.5)
        .payload(EventPayload::Empty)
        .build()
        .unwrap();

    event.set_category_for_test(EventCategory::Social);
    let interpreted = interpret_event(&event, &entity);

    // Empty payload for social event should not change state
    assert!(interpreted.valence_delta.abs() < f32::EPSILON);
}

/// Test applying interpreted event when path is not in the match arms
#[test]
fn event_apply_unhandled_state_path_ignored() {
    let mut entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    let event = EventBuilder::new(EventType::Interaction)
        .severity(0.5)
        .build()
        .unwrap();

    let mut interpreted = interpret_event(&event, &entity);

    // Add a path that's not handled in apply_interpreted_event
    interpreted.state_deltas.push((
        StatePath::Hexaco(behavioral_pathways::enums::HexacoPath::Openness),
        0.1,
    ));

    // Should not panic, unhandled path is silently ignored
    apply_interpreted_event(&interpreted, &mut entity);
}

/// Test compute_attribution with source present
#[test]
fn event_attribution_source_creates_other_attribution() {
    let entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    let source = EntityId::new("aggressor").unwrap();
    let event = EventBuilder::new(EventType::Humiliation)
        .severity(0.5)
        .source(source.clone())
        .build()
        .unwrap();

    let interpreted = interpret_event(&event, &entity);

    assert!(interpreted.attribution.is_other());
    assert_eq!(interpreted.attribution.other_entity(), Some(&source));
}

/// Test compute_attribution stability based on severity > 0.7
#[test]
fn event_attribution_high_severity_is_stable() {
    let entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    let source = EntityId::new("source").unwrap();
    let high_severity_event = EventBuilder::new(EventType::Failure)
        .severity(0.75)
        .source(source.clone())
        .build()
        .unwrap();

    let interpreted = interpret_event(&high_severity_event, &entity);

    assert!(interpreted.attribution.is_stable());
}

/// Test compute_attribution low severity is unstable
#[test]
fn event_attribution_low_severity_is_unstable() {
    let entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    let source = EntityId::new("source").unwrap();
    let low_severity_event = EventBuilder::new(EventType::Failure)
        .severity(0.3)
        .source(source.clone())
        .build()
        .unwrap();

    let interpreted = interpret_event(&low_severity_event, &entity);

    assert!(!interpreted.attribution.is_stable());
}

/// Test Achievement event domain filtering for purpose boost
#[test]
fn event_achievement_creative_domain_adds_purpose() {
    let entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    let event = EventBuilder::new(EventType::Achievement)
        .severity(0.7)
        .payload(EventPayload::Achievement {
            domain: LifeDomain::Creative,
            magnitude: 0.8,
        })
        .build()
        .unwrap();

    let interpreted = interpret_event(&event, &entity);

    let has_purpose = interpreted.state_deltas.iter().any(|(path, _)| {
        matches!(path, StatePath::Needs(NeedsPath::Purpose))
    });
    assert!(has_purpose);
}

/// Test SocialInclusion with group_id adds extra loneliness reduction
#[test]
fn event_social_inclusion_with_group_id_extra_loneliness_reduction() {
    let entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    let group_id = behavioral_pathways::types::GroupId::new("group1").unwrap();
    let event = EventBuilder::new(EventType::SocialInclusion)
        .severity(0.6)
        .payload(EventPayload::SocialInclusion {
            group_id: Some(group_id),
        })
        .build()
        .unwrap();

    let interpreted = interpret_event(&event, &entity);

    // Has extra -0.08 loneliness reduction on top of base -0.20 (total -0.28)
    assert!(interpreted.loneliness_delta < -0.2);
}

/// Test Support event with effectiveness reduces liability and increases worth
#[test]
fn event_support_emotional_reduces_liability_and_worth() {
    let entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    let event = EventBuilder::new(EventType::Support)
        .severity(0.8)
        .payload(EventPayload::Support {
            support_type: SupportType::Emotional,
            effectiveness: 0.85,
        })
        .build()
        .unwrap();

    let interpreted = interpret_event(&event, &entity);

    assert!(interpreted.perceived_liability_delta < 0.0);
    assert!(interpreted.self_hate_delta < 0.0);

    let has_worth = interpreted.state_deltas.iter().any(|(path, _)| {
        matches!(path, StatePath::MentalHealth(MentalHealthPath::SelfWorth))
    });
    assert!(has_worth);
}

/// Test Realization with ExistentialInsight adds purpose and reduces burden
#[test]
fn event_realization_existential_adds_purpose() {
    let entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    let event = EventBuilder::new(EventType::Realization)
        .severity(0.7)
        .payload(EventPayload::Realization {
            realization_type: RealizationType::ExistentialInsight,
        })
        .build()
        .unwrap();

    let interpreted = interpret_event(&event, &entity);

    let has_purpose = interpreted.state_deltas.iter().any(|(path, _)| {
        matches!(path, StatePath::Needs(NeedsPath::Purpose))
    });
    assert!(has_purpose);
    assert!(interpreted.perceived_liability_delta < 0.0);
    assert!(interpreted.self_hate_delta < 0.0);
}

/// Test Loss event adds both self_worth and grievance
#[test]
fn event_loss_adds_self_worth_and_grievance() {
    let entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    let event = EventBuilder::new(EventType::Loss)
        .severity(0.8)
        .build()
        .unwrap();

    let interpreted = interpret_event(&event, &entity);

    let has_self_worth = interpreted.state_deltas.iter().any(|(path, _)| {
        matches!(path, StatePath::MentalHealth(MentalHealthPath::SelfWorth))
    });
    let has_grievance = interpreted.state_deltas.iter().any(|(path, _)| {
        matches!(path, StatePath::Disposition(DispositionPath::Grievance))
    });

    assert!(has_self_worth);
    assert!(has_grievance);
}

/// Test that support_type filtering works for non-Emotional support types
#[test]
fn event_support_non_emotional_type_no_self_worth_boost() {
    let entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    let event = EventBuilder::new(EventType::Support)
        .severity(0.8)
        .payload(EventPayload::Support {
            support_type: SupportType::Financial,
            effectiveness: 0.9,
        })
        .build()
        .unwrap();

    let interpreted = interpret_event(&event, &entity);

    // Financial support shouldn't add self_worth delta
    let has_self_worth = interpreted.state_deltas.iter().any(|(path, _)| {
        matches!(path, StatePath::MentalHealth(MentalHealthPath::SelfWorth))
    });
    assert!(!has_self_worth);
}

/// Test apply_interpreted_event with chronic pattern tag on loneliness
#[test]
fn event_apply_chronic_loneliness_uses_chronic_delta() {
    let mut entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    let event = EventBuilder::new(EventType::SocialExclusion)
        .severity(0.6)
        .tag(behavioral_pathways::enums::EventTag::ChronicPattern)
        .build()
        .unwrap();

    let interpreted = interpret_event(&event, &entity);
    apply_interpreted_event(&interpreted, &mut entity);

    // Should have increased loneliness via chronic pattern
    let loneliness = entity
        .get_effective(StatePath::SocialCognition(SocialCognitionPath::Loneliness))
        .unwrap_or(0.0);
    assert!(loneliness > 0.0);
}

/// Test apply_interpreted_event with non-chronic pattern
#[test]
fn event_apply_non_chronic_loneliness_uses_normal_delta() {
    let mut entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    let event = EventBuilder::new(EventType::SocialInclusion)
        .severity(0.6)
        .build()
        .unwrap();

    let interpreted = interpret_event(&event, &entity);
    apply_interpreted_event(&interpreted, &mut entity);

    // Should have decreased loneliness
    let loneliness = entity
        .get_effective(StatePath::SocialCognition(SocialCognitionPath::Loneliness))
        .unwrap_or(0.0);
    assert!(loneliness < 0.0);
}

// ============================================================================
// processor/state_evolution.rs coverage - edge case regions
// ============================================================================

/// Test state_value near zero delta application
#[test]
fn state_evolution_tiny_delta_not_applied() {
    let entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    let event = EventBuilder::new(EventType::PolicyChange)
        .severity(0.001) // Very small severity
        .build()
        .unwrap();

    let interpreted = interpret_event(&event, &entity);

    // Should have minimal state deltas due to epsilon check
    assert!(interpreted.state_deltas.is_empty() || interpreted.state_deltas.len() < 2);
}
