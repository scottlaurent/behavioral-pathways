//! Test: Querying at anchor timestamp returns the anchor state unchanged.
//!
//! When state_at() is called with the exact anchor timestamp, it should
//! return the entity's anchor state without any decay or event processing.
//! This is a short-circuit case.

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::{EventType, MoodPath, Species, StatePath};
use behavioral_pathways::event::EventBuilder;
use behavioral_pathways::simulation::{RegressionQuality, Simulation};
use behavioral_pathways::types::{Duration, EntityId, Timestamp};

/// Query at exactly anchor timestamp returns anchor state unchanged.
#[test]
fn state_at_anchor_returns_anchor_state() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let entity = EntityBuilder::new()
        .id("person")
        .species(Species::Human)
        .build()
        .unwrap();

    let entity_id = EntityId::new("person").unwrap();
    let anchor = reference;
    sim.add_entity(entity, anchor);

    // Add events before and after anchor (should not affect anchor query)
    let before_event = EventBuilder::new(EventType::SocialExclusion)
        .target(entity_id.clone())
        .severity(0.9)
        .build()
        .unwrap();
    sim.add_event(before_event, anchor - Duration::hours(1));

    let after_event = EventBuilder::new(EventType::Achievement)
        .target(entity_id.clone())
        .severity(0.9)
        .build()
        .unwrap();
    sim.add_event(after_event, anchor + Duration::hours(1));

    // Query exactly at anchor
    let handle = sim.entity(&entity_id).unwrap();
    let state = handle.state_at(anchor);

    // Should have default valence (near 0) - get_effective now returns f64 directly
    let valence = state.get_effective(StatePath::Mood(MoodPath::Valence));

    assert!(
        valence.abs() < 0.01,
        "Anchor state should have default valence, got {}",
        valence
    );

    // Should be exact regression quality (no uncertainty)
    assert_eq!(state.regression_quality(), RegressionQuality::Exact);
}

/// Multiple queries at anchor timestamp produce identical results.
#[test]
fn multiple_queries_at_anchor_identical() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let entity = EntityBuilder::new()
        .id("person")
        .species(Species::Human)
        .build()
        .unwrap();

    let entity_id = EntityId::new("person").unwrap();
    let anchor = reference;
    sim.add_entity(entity, anchor);

    let handle = sim.entity(&entity_id).unwrap();

    let state1 = handle.state_at(anchor);
    let state2 = handle.state_at(anchor);
    let state3 = handle.state_at(anchor);

    // get_effective now returns f64 directly
    let v1 = state1.get_effective(StatePath::Mood(MoodPath::Valence));
    let v2 = state2.get_effective(StatePath::Mood(MoodPath::Valence));
    let v3 = state3.get_effective(StatePath::Mood(MoodPath::Valence));

    assert!((v1 - v2).abs() < f64::EPSILON);
    assert!((v2 - v3).abs() < f64::EPSILON);
}
