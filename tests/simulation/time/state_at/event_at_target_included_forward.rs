//! Test: Events at query timestamp ARE included during forward projection.
//!
//! When projecting forward from anchor to query timestamp, events at the
//! query timestamp should be included. The forward range is (anchor, target].

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::{EventType, MoodPath, Species, StatePath};
use behavioral_pathways::event::EventBuilder;
use behavioral_pathways::simulation::Simulation;
use behavioral_pathways::types::{Duration, EntityId, Timestamp};

/// Events occurring exactly at query timestamp ARE applied during forward projection.
#[test]
fn event_at_target_included_forward() {
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

    // Query time is 1 hour after anchor
    let query_time = anchor + Duration::hours(1);

    // Add event exactly at the query timestamp
    let event = EventBuilder::new(EventType::SocialExclusion)
        .target(entity_id.clone())
        .severity(0.8)
        .build()
        .unwrap();
    sim.add_event(event, query_time);

    // Query state at the target time
    let handle = sim.entity(&entity_id).unwrap();
    let state = handle.state_at(query_time);
    // get_effective now returns f64 directly
    let valence = state.get_effective(StatePath::Mood(MoodPath::Valence));

    // The event at query_time should be included in forward projection.
    // Social exclusion with severity 0.8 should decrease valence significantly.
    assert!(
        valence < -0.1,
        "Forward projection should include event at query time, got valence {}",
        valence
    );
}

/// Event at exactly one second after anchor is included.
#[test]
fn event_one_second_after_anchor_included() {
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

    // Event one second after anchor
    let event_time = anchor + Duration::seconds(1);
    let event = EventBuilder::new(EventType::Achievement)
        .target(entity_id.clone())
        .severity(0.8)
        .build()
        .unwrap();
    sim.add_event(event, event_time);

    // Query at the event time
    let handle = sim.entity(&entity_id).unwrap();
    let state = handle.state_at(event_time);
    // get_effective now returns f64 directly
    let valence = state.get_effective(StatePath::Mood(MoodPath::Valence));

    // Event should be applied
    assert!(
        valence > 0.1,
        "Event one second after anchor should be included, got valence {}",
        valence
    );
}
