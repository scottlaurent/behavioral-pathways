//! Integration tests for event processing with timestamps.
//!
//! Tests that events are applied at correct timestamps during state_at queries.

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::{EventType, MoodPath, Species, StatePath};
use behavioral_pathways::event::EventBuilder;
use behavioral_pathways::simulation::Simulation;
use behavioral_pathways::types::{Duration, EntityId, Timestamp};

/// Event applied at timestamp affects state queried after that timestamp.
#[test]
fn event_affects_state_after_timestamp() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let entity = EntityBuilder::new()
        .id("person_001")
        .species(Species::Human)
        .age(Duration::years(30))
        .build()
        .unwrap();

    let entity_id = EntityId::new("person_001").unwrap();
    sim.add_entity(entity, reference);

    // Get baseline valence at reference - get_effective now returns f64 directly
    let handle = sim.entity(&entity_id).unwrap();
    let baseline_state = handle.state_at(reference);
    let baseline_valence = baseline_state.get_effective(StatePath::Mood(MoodPath::Valence));

    // Add negative event at a later time
    let event = EventBuilder::new(EventType::SocialExclusion)
        .target(entity_id.clone())
        .severity(0.8)
        .build()
        .unwrap();
    let event_time = reference + Duration::days(30);
    sim.add_event(event, event_time);

    // Query state after the event - valence should be lower
    let after_event = event_time + Duration::hours(1);
    let handle = sim.entity(&entity_id).unwrap();
    let state_after = handle.state_at(after_event);
    let valence_after = state_after.get_effective(StatePath::Mood(MoodPath::Valence));

    // Valence should have decreased due to social exclusion
    assert!(valence_after < baseline_valence);
}

/// Event does not affect state queried before its timestamp.
#[test]
fn event_does_not_affect_state_before_timestamp() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let entity = EntityBuilder::new()
        .id("person_001")
        .species(Species::Human)
        .age(Duration::years(30))
        .build()
        .unwrap();

    let entity_id = EntityId::new("person_001").unwrap();
    sim.add_entity(entity, reference);

    // Get baseline valence at reference - get_effective now returns f64 directly
    let handle = sim.entity(&entity_id).unwrap();
    let baseline_state = handle.state_at(reference);
    let baseline_valence = baseline_state.get_effective(StatePath::Mood(MoodPath::Valence));

    // Add event at a later time
    let event = EventBuilder::new(EventType::SocialExclusion)
        .target(entity_id.clone())
        .severity(0.8)
        .build()
        .unwrap();
    let event_time = reference + Duration::days(30);
    sim.add_event(event, event_time);

    // Query state before the event - should be same as baseline (no event effect)
    let before_event = reference + Duration::days(15);
    let handle = sim.entity(&entity_id).unwrap();
    let state_before = handle.state_at(before_event);
    let valence_before = state_before.get_effective(StatePath::Mood(MoodPath::Valence));

    // Valence should only differ by decay, not event
    // The event hasn't happened yet, so no event-based change
    // Decay should be minimal over 15 days with mood decay
    assert!((valence_before - baseline_valence).abs() < 0.1);
}

/// Multiple events are applied in chronological order.
#[test]
fn multiple_events_applied_in_order() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let entity = EntityBuilder::new()
        .id("person_001")
        .species(Species::Human)
        .age(Duration::years(30))
        .build()
        .unwrap();

    let entity_id = EntityId::new("person_001").unwrap();
    sim.add_entity(entity, reference);

    // Add a negative event first
    let negative_event = EventBuilder::new(EventType::SocialExclusion)
        .target(entity_id.clone())
        .severity(0.8)
        .build()
        .unwrap();
    let negative_time = reference + Duration::days(10);
    sim.add_event(negative_event, negative_time);

    // Add a positive event later
    let positive_event = EventBuilder::new(EventType::SocialInclusion)
        .target(entity_id.clone())
        .severity(0.8)
        .build()
        .unwrap();
    let positive_time = reference + Duration::days(20);
    sim.add_event(positive_event, positive_time);

    // Query at different times to see the progression
    let handle = sim.entity(&entity_id).unwrap();

    // After first event (negative) - get_effective now returns f64 directly
    let state_after_negative = handle.state_at(negative_time + Duration::hours(1));
    let valence_after_negative =
        state_after_negative.get_effective(StatePath::Mood(MoodPath::Valence));

    // After second event (positive)
    let state_after_positive = handle.state_at(positive_time + Duration::hours(1));
    let valence_after_positive =
        state_after_positive.get_effective(StatePath::Mood(MoodPath::Valence));

    // Valence should have recovered somewhat after the positive event
    assert!(valence_after_positive > valence_after_negative);
}

/// Decay continues to apply between events.
#[test]
fn decay_applied_between_events() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let entity = EntityBuilder::new()
        .id("person_001")
        .species(Species::Human)
        .age(Duration::years(30))
        .build()
        .unwrap();

    let entity_id = EntityId::new("person_001").unwrap();
    sim.add_entity(entity, reference);

    // Add a negative event
    let event = EventBuilder::new(EventType::SocialExclusion)
        .target(entity_id.clone())
        .severity(0.8)
        .build()
        .unwrap();
    let event_time = reference + Duration::days(1);
    sim.add_event(event, event_time);

    let handle = sim.entity(&entity_id).unwrap();

    // Query immediately after event - get_effective now returns f64 directly
    let state_immediate = handle.state_at(event_time + Duration::hours(1));
    let valence_immediate = state_immediate.get_effective(StatePath::Mood(MoodPath::Valence));

    // Query 1 week after event - decay should have occurred
    let state_later = handle.state_at(event_time + Duration::weeks(1));
    let valence_later = state_later.get_effective(StatePath::Mood(MoodPath::Valence));

    // Valence should have recovered toward baseline due to decay
    assert!(valence_later > valence_immediate);
}
