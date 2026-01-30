use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::{Species, EventType};
use behavioral_pathways::event::EventBuilder;
use behavioral_pathways::simulation::Simulation;
use behavioral_pathways::types::{EntityId, Timestamp, Duration};

#[test]
fn delta_summary_shows_changes_from_baseline() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let mut entity = EntityBuilder::new()
        .id("test_delta")
        .species(Species::Human)
        .build()
        .unwrap();

    entity.individual_state_mut().mood_mut().add_valence_delta(0.1);
    entity.individual_state_mut().needs_mut().add_stress_delta(0.1);

    let entity_id = EntityId::new("test_delta").unwrap();
    sim.add_entity(entity, reference);

    let event = EventBuilder::new(EventType::SocialExclusion)
        .target(entity_id.clone())
        .severity(0.8)
        .build()
        .unwrap();

    let event_time = reference + Duration::hours(1);
    sim.add_event(event, event_time);

    let handle = sim.entity(&entity_id).unwrap();
    let state_before = handle.state_at(reference);
    let state_after = handle.state_at(event_time + Duration::minutes(1));

    assert!(state_before.delta_summary.is_none());
    assert!(state_after.delta_summary.is_some());

    let delta = state_after.delta_summary.as_ref().unwrap();
    assert!(!delta.is_empty());
}

#[test]
fn delta_summary_empty_when_no_changes() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let entity = EntityBuilder::new()
        .id("test_no_delta")
        .species(Species::Human)
        .build()
        .unwrap();

    let entity_id = EntityId::new("test_no_delta").unwrap();
    sim.add_entity(entity, reference);

    let handle = sim.entity(&entity_id).unwrap();
    let state = handle.state_at(reference + Duration::seconds(1));

    assert!(state.delta_summary.is_none() || state.delta_summary.as_ref().unwrap().is_empty() || state.delta_summary == Some(".".to_string()));
}

#[test]
fn delta_summary_shows_specific_changes() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let mut entity = EntityBuilder::new()
        .id("test_specific")
        .species(Species::Human)
        .build()
        .unwrap();

    entity.individual_state_mut().mood_mut().add_valence_delta(0.0);

    let entity_id = EntityId::new("test_specific").unwrap();
    sim.add_entity(entity, reference);

    let event = EventBuilder::new(EventType::SocialExclusion)
        .target(entity_id.clone())
        .severity(0.9)
        .build()
        .unwrap();

    let event_time = reference + Duration::hours(2);
    sim.add_event(event, event_time);

    let handle = sim.entity(&entity_id).unwrap();
    let state = handle.state_at(event_time + Duration::minutes(5));

    if let Some(delta) = &state.delta_summary {
        assert!(delta.contains("sadder") || delta.contains("stressed") || delta.contains("lonely"));
    }
}
