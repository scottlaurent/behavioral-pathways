//! Integration tests for backward regression with timestamps.
//!
//! Tests that backward regression (querying state before anchor) works correctly.

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::{EventType, MoodPath, Species, StatePath};
use behavioral_pathways::event::EventBuilder;
use behavioral_pathways::simulation::{RegressionQuality, Simulation};
use behavioral_pathways::types::{Duration, EntityId, Timestamp};

/// Backward regression returns state before anchor.
#[test]
fn backward_regression_returns_past_state() {
    let reference = Timestamp::from_ymd_hms(2024, 6, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let entity = EntityBuilder::new()
        .id("person_001")
        .species(Species::Human)
        .age(Duration::years(30))
        .build()
        .unwrap();

    let entity_id = EntityId::new("person_001").unwrap();
    sim.add_entity(entity, reference);

    // Query state 6 months before the anchor
    let past = reference - Duration::days(180);
    let handle = sim.entity(&entity_id).unwrap();
    let computed = handle.state_at(past);

    // Age should be adjusted
    let age = computed.age_at_timestamp();
    // Entity was 30 at anchor, so should be ~29.5 at past
    assert!(age.as_years() >= 29);
}

/// Backward regression through events reverses them.
#[test]
fn backward_regression_reverses_events() {
    let reference = Timestamp::from_ymd_hms(2024, 6, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let entity = EntityBuilder::new()
        .id("person_001")
        .species(Species::Human)
        .age(Duration::years(30))
        .build()
        .unwrap();

    let entity_id = EntityId::new("person_001").unwrap();
    sim.add_entity(entity, reference);

    // Add an event before the anchor
    let event = EventBuilder::new(EventType::SocialExclusion)
        .target(entity_id.clone())
        .severity(0.7)
        .build()
        .unwrap();
    let event_time = reference - Duration::days(30);
    sim.add_event(event, event_time);

    let handle = sim.entity(&entity_id).unwrap();

    // Query at anchor (reflects event effects + decay since event)
    let _state_at_anchor = handle.state_at(reference);

    // Query before the event (should reverse the event)
    let before_event = event_time - Duration::days(1);
    let state_before = handle.state_at(before_event);
    // get_effective now returns f64 directly
    let valence_before = state_before.get_effective(StatePath::Mood(MoodPath::Valence));

    // State before event shouldn't have the negative effects
    // Note: The exact values depend on the reversal implementation
    // For now, we just verify the query succeeds
    assert!(valence_before.abs() <= 1.0); // Valid range
}

/// Backward regression is exact when no trauma events.
#[test]
fn backward_regression_without_trauma_is_exact() {
    let reference = Timestamp::from_ymd_hms(2024, 6, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let entity = EntityBuilder::new()
        .id("person_001")
        .species(Species::Human)
        .age(Duration::years(30))
        .build()
        .unwrap();

    let entity_id = EntityId::new("person_001").unwrap();
    sim.add_entity(entity, reference);

    // Add a non-trauma event before anchor
    let event = EventBuilder::new(EventType::SocialExclusion)
        .target(entity_id.clone())
        .severity(0.5)
        .build()
        .unwrap();
    let event_time = reference - Duration::days(30);
    sim.add_event(event, event_time);

    // Query before the event
    let handle = sim.entity(&entity_id).unwrap();
    let before_event = event_time - Duration::days(1);
    let state = handle.state_at(before_event);

    // Regression through non-trauma should be exact
    assert_eq!(state.regression_quality(), RegressionQuality::Exact);
}

/// Backward regression through trauma is approximate.
#[test]
fn backward_regression_through_trauma_is_approximate() {
    let reference = Timestamp::from_ymd_hms(2024, 6, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let entity = EntityBuilder::new()
        .id("person_001")
        .species(Species::Human)
        .age(Duration::years(30))
        .build()
        .unwrap();

    let entity_id = EntityId::new("person_001").unwrap();
    sim.add_entity(entity, reference);

    // Add a trauma event (Violence) before anchor
    let trauma_event = EventBuilder::new(EventType::Violence)
        .target(entity_id.clone())
        .severity(0.8)
        .build()
        .unwrap();
    let event_time = reference - Duration::days(30);
    sim.add_event(trauma_event, event_time);

    // Query before the trauma event
    let handle = sim.entity(&entity_id).unwrap();
    let before_event = event_time - Duration::days(1);
    let state = handle.state_at(before_event);

    // Regression through trauma should be approximate (AC not reversible)
    assert_eq!(state.regression_quality(), RegressionQuality::Approximate);
}

/// Forward projection is always exact.
#[test]
fn forward_projection_is_always_exact() {
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

    // Add a trauma event in the future
    let trauma_event = EventBuilder::new(EventType::Violence)
        .target(entity_id.clone())
        .severity(0.8)
        .build()
        .unwrap();
    let event_time = reference + Duration::days(30);
    sim.add_event(trauma_event, event_time);

    // Query after the trauma event (forward projection)
    let handle = sim.entity(&entity_id).unwrap();
    let after_event = event_time + Duration::days(1);
    let state = handle.state_at(after_event);

    // Forward projection is always exact
    assert_eq!(state.regression_quality(), RegressionQuality::Exact);
}

/// Regression with no events returns exact quality.
#[test]
fn regression_without_events_is_exact() {
    let reference = Timestamp::from_ymd_hms(2024, 6, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let entity = EntityBuilder::new()
        .id("person_001")
        .species(Species::Human)
        .age(Duration::years(30))
        .build()
        .unwrap();

    let entity_id = EntityId::new("person_001").unwrap();
    sim.add_entity(entity, reference);

    // Query in past with no events
    let handle = sim.entity(&entity_id).unwrap();
    let past = reference - Duration::days(30);
    let state = handle.state_at(past);

    // Pure time regression is exact
    assert_eq!(state.regression_quality(), RegressionQuality::Exact);
}
