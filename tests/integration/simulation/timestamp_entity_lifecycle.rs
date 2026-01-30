//! Integration tests for entity lifecycle with timestamps.
//!
//! Tests the full lifecycle of entities in a simulation with timestamp-based operations.

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::Species;
use behavioral_pathways::simulation::Simulation;
use behavioral_pathways::types::{Duration, EntityId, Timestamp};

/// Entity can be added with anchor timestamp and queried.
#[test]
fn entity_added_with_anchor_can_be_queried() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let entity = EntityBuilder::new()
        .id("person_001")
        .species(Species::Human)
        .age(Duration::years(30))
        .build()
        .unwrap();

    let anchor = Timestamp::from_ymd_hms(2024, 1, 1, 9, 0, 0);
    sim.add_entity(entity, anchor);

    let entity_id = EntityId::new("person_001").unwrap();
    let handle = sim.entity(&entity_id);

    assert!(handle.is_some());
}

/// Entity with birth date has correct age at different timestamps.
#[test]
fn entity_birth_date_computes_age_correctly() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    // Entity born on 1990-06-15
    let birth = Timestamp::from_ymd_hms(1990, 6, 15, 0, 0, 0);
    let entity = EntityBuilder::new()
        .id("person_001")
        .species(Species::Human)
        .birth_date(birth)
        .build()
        .unwrap();

    sim.add_entity(entity, reference);

    let entity_id = EntityId::new("person_001").unwrap();
    let handle = sim.entity(&entity_id).unwrap();

    // At reference date (2024-01-01), age should be ~33.5 years
    let state_at_reference = handle.state_at(reference);
    assert!(state_at_reference.age_at_timestamp().as_years() >= 33);
    assert!(state_at_reference.age_at_timestamp().as_years() <= 34);

    // 10 years later, age should be ~43.5 years
    let future = reference + Duration::years(10);
    let state_at_future = handle.state_at(future);
    assert!(state_at_future.age_at_timestamp().as_years() >= 43);
    assert!(state_at_future.age_at_timestamp().as_years() <= 44);

    // 10 years before, age should be ~23.5 years
    let past = reference - Duration::years(10);
    let state_at_past = handle.state_at(past);
    assert!(state_at_past.age_at_timestamp().as_years() >= 23);
    assert!(state_at_past.age_at_timestamp().as_years() <= 24);
}

/// Entity's life stage is computed correctly at different ages.
#[test]
fn entity_life_stage_computed_from_age() {
    use behavioral_pathways::enums::LifeStage;

    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    // Entity born 35 years ago (adult at reference)
    let birth = Timestamp::from_ymd_hms(1989, 1, 1, 0, 0, 0);
    let entity = EntityBuilder::new()
        .id("person_001")
        .species(Species::Human)
        .birth_date(birth)
        .build()
        .unwrap();

    sim.add_entity(entity, reference);

    let entity_id = EntityId::new("person_001").unwrap();
    let handle = sim.entity(&entity_id).unwrap();

    // At reference, should be Adult (35 years old, falls in 31-55 range)
    let state = handle.state_at(reference);
    assert_eq!(state.life_stage(), LifeStage::Adult);

    // 30 years in future, should be MatureAdult (65 years old, falls in 56-70 range)
    let future = reference + Duration::years(30);
    let future_state = handle.state_at(future);
    assert_eq!(future_state.life_stage(), LifeStage::MatureAdult);

    // 20 years in past, should be Adolescent (15 years old, falls in 13-17 range)
    let past = reference - Duration::years(20);
    let past_state = handle.state_at(past);
    assert_eq!(past_state.life_stage(), LifeStage::Adolescent);
}

/// Multiple entities can be added and queried independently.
#[test]
fn multiple_entities_queried_independently() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    // Add two entities with different ages
    let entity1 = EntityBuilder::new()
        .id("person_001")
        .species(Species::Human)
        .age(Duration::years(30))
        .build()
        .unwrap();

    let entity2 = EntityBuilder::new()
        .id("person_002")
        .species(Species::Human)
        .age(Duration::years(50))
        .build()
        .unwrap();

    sim.add_entity(entity1, reference);
    sim.add_entity(entity2, reference);

    // Query both entities
    let id1 = EntityId::new("person_001").unwrap();
    let id2 = EntityId::new("person_002").unwrap();

    let handle1 = sim.entity(&id1).unwrap();
    let handle2 = sim.entity(&id2).unwrap();

    let state1 = handle1.state_at(reference);
    let state2 = handle2.state_at(reference);

    // Ages should match what was set
    assert_eq!(state1.age_at_timestamp().as_years(), 30);
    assert_eq!(state2.age_at_timestamp().as_years(), 50);
}

/// Entity count is tracked correctly.
#[test]
fn entity_count_tracked() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    assert_eq!(sim.entity_count(), 0);

    let entity = EntityBuilder::new()
        .id("person_001")
        .species(Species::Human)
        .build()
        .unwrap();

    sim.add_entity(entity, reference);

    assert_eq!(sim.entity_count(), 1);
}
