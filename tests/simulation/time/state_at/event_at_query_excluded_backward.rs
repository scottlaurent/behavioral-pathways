//! Test: Events at query timestamp are excluded during backward regression.
//!
//! When regressing backward from anchor to query timestamp, events at the
//! query timestamp should NOT be included. The regression computes what
//! the state WAS at that time, which includes any events at that exact instant.
//!
//! Backward range is (target, anchor] - exclusive of target, inclusive of anchor.

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::{EventType, MoodPath, Species, StatePath};
use behavioral_pathways::event::EventBuilder;
use behavioral_pathways::simulation::Simulation;
use behavioral_pathways::types::{Duration, EntityId, Timestamp};

/// Events occurring exactly at query timestamp should NOT be reversed during
/// backward regression - they represent the state AT that time.
#[test]
fn event_at_query_excluded_backward() {
    // Setup: Create simulation with reference date
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    // Create entity with known initial state
    let entity = EntityBuilder::new()
        .id("person")
        .species(Species::Human)
        .build()
        .unwrap();

    let entity_id = EntityId::new("person").unwrap();
    let anchor = reference;
    sim.add_entity(entity, anchor);

    // Set up a scenario: anchor is at reference, query is 1 hour before
    let query_time = anchor - Duration::hours(1);

    // Add an event exactly at the query timestamp
    let event_at_query = EventBuilder::new(EventType::SocialExclusion)
        .target(entity_id.clone())
        .severity(0.8)
        .build()
        .unwrap();
    sim.add_event(event_at_query, query_time);

    // Query state at anchor (should be unaffected - event is before anchor)
    let handle = sim.entity(&entity_id).unwrap();
    let state_at_anchor = handle.state_at(anchor);
    // get_effective now returns f64 directly
    let valence_at_anchor = state_at_anchor.get_effective(StatePath::Mood(MoodPath::Valence));

    // Query state at query_time (backward regression)
    let state_backward = handle.state_at(query_time);
    let valence_backward = state_backward.get_effective(StatePath::Mood(MoodPath::Valence));

    // Anchor state should have near-zero valence
    assert!(
        valence_at_anchor.abs() < 0.05,
        "Anchor state should have near-zero valence, got {}",
        valence_at_anchor
    );

    // Backward regression to query_time should NOT reverse the event at query_time
    // because the backward range is (query, anchor], excluding query.
    // The state at query_time already has this event applied.
    // So valence at query time should also be near zero (event excluded from reversal).
    assert!(
        valence_backward.abs() < 0.05,
        "Backward state should NOT reverse event at query time (event excluded), got {}",
        valence_backward
    );
}

/// Events exactly at query timestamp are NOT in the range (query, anchor]
/// during backward regression - the range excludes query.
#[test]
fn event_at_query_is_excluded_from_backward_range() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 12, 0, 0);
    let mut sim = Simulation::new(reference);

    let entity = EntityBuilder::new()
        .id("person")
        .species(Species::Human)
        .build()
        .unwrap();

    let entity_id = EntityId::new("person").unwrap();
    let anchor = reference;
    sim.add_entity(entity, anchor);

    let query_time = anchor - Duration::hours(2);

    // Event at query time
    let event = EventBuilder::new(EventType::Achievement)
        .target(entity_id.clone())
        .severity(0.7)
        .build()
        .unwrap();
    sim.add_event(event, query_time);

    let handle = sim.entity(&entity_id).unwrap();

    // State at anchor - get_effective now returns f64 directly
    let anchor_state = handle.state_at(anchor);
    let anchor_valence = anchor_state.get_effective(StatePath::Mood(MoodPath::Valence));

    // State at query (backward regression)
    let query_state = handle.state_at(query_time);
    let query_valence = query_state.get_effective(StatePath::Mood(MoodPath::Valence));

    // Anchor should be near zero
    assert!(anchor_valence.abs() < 0.05);

    // Query state should NOT reverse the event at query time.
    // The event is at query_time, which is excluded from (query, anchor].
    // So valence should be near zero.
    assert!(
        query_valence.abs() < 0.05,
        "Event at query time should be excluded from backward reversal, got {}",
        query_valence
    );
}

/// Events BETWEEN query and anchor are reversed during backward regression.
#[test]
fn events_between_query_and_anchor_reversed() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 12, 0, 0);
    let mut sim = Simulation::new(reference);

    let entity = EntityBuilder::new()
        .id("person")
        .species(Species::Human)
        .build()
        .unwrap();

    let entity_id = EntityId::new("person").unwrap();
    let anchor = reference;
    sim.add_entity(entity, anchor);

    let query_time = anchor - Duration::hours(2);
    let event_time = anchor - Duration::hours(1); // Between query and anchor

    // Event between query and anchor
    let event = EventBuilder::new(EventType::SocialExclusion)
        .target(entity_id.clone())
        .severity(0.8)
        .build()
        .unwrap();
    sim.add_event(event, event_time);

    let handle = sim.entity(&entity_id).unwrap();

    // State at anchor (should be near zero - event is before anchor)
    // get_effective now returns f64 directly
    let anchor_state = handle.state_at(anchor);
    let anchor_valence = anchor_state.get_effective(StatePath::Mood(MoodPath::Valence));
    assert!(
        anchor_valence.abs() < 0.05,
        "Anchor valence should be near zero, got {}",
        anchor_valence
    );

    // State at query (backward regression) - event should be reversed
    let query_state = handle.state_at(query_time);
    let query_valence = query_state.get_effective(StatePath::Mood(MoodPath::Valence));

    // The event is in (query, anchor], so it should be reversed.
    // Reversing a social exclusion (negative) means valence goes positive.
    assert!(
        query_valence > 0.1,
        "Backward regression should reverse event between query and anchor, got {}",
        query_valence
    );
}
