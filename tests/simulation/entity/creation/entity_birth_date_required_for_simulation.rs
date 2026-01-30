//! Test: Entity birth_date is important for accurate temporal simulation.
//!
//! When simulating entity state at different timestamps, the birth_date
//! is used to compute the entity's age at each point in time. Without
//! birth_date, age remains fixed at the anchor age regardless of query time.
//!
//! While birth_date is not strictly required to add an entity to simulation,
//! it should be set for accurate age computation in temporal queries.

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::Species;
use behavioral_pathways::simulation::Simulation;
use behavioral_pathways::types::{Duration, EntityId, Timestamp};

/// Entity with birth_date has correct age at different timestamps.
#[test]
fn entity_with_birth_date_has_correct_age_at_timestamps() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    // Entity born on 1994-01-01, making them 30 at reference date
    let birth_date = Timestamp::from_ymd_hms(1994, 1, 1, 0, 0, 0);
    let entity = EntityBuilder::new()
        .id("person")
        .species(Species::Human)
        .birth_date(birth_date)
        .age(Duration::years(30))
        .build()
        .unwrap();

    let entity_id = EntityId::new("person").unwrap();
    sim.add_entity(entity, reference);

    let handle = sim.entity(&entity_id).unwrap();

    // At anchor (2024-01-01), age is 30
    let state_at_anchor = handle.state_at(reference);
    assert_eq!(state_at_anchor.age_at_timestamp().as_years(), 30);

    // 10 years in future (2034-01-01), age is 40
    let future = reference + Duration::years(10);
    let state_at_future = handle.state_at(future);
    assert_eq!(state_at_future.age_at_timestamp().as_years(), 40);

    // 10 years in past (2014-01-01), age is 20
    let past = reference - Duration::years(10);
    let state_at_past = handle.state_at(past);
    assert_eq!(state_at_past.age_at_timestamp().as_years(), 20);
}

/// Entity without birth_date has constant age at all timestamps.
#[test]
fn entity_without_birth_date_has_constant_age() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    // Entity without birth_date
    let entity = EntityBuilder::new()
        .id("person")
        .species(Species::Human)
        .age(Duration::years(30))
        .build()
        .unwrap();

    let entity_id = EntityId::new("person").unwrap();
    sim.add_entity(entity, reference);

    let handle = sim.entity(&entity_id).unwrap();

    // At anchor, age is 30
    let state_at_anchor = handle.state_at(reference);
    assert_eq!(state_at_anchor.age_at_timestamp().as_years(), 30);

    // 10 years in future, age is STILL 30 (no birth_date to compute from)
    let future = reference + Duration::years(10);
    let state_at_future = handle.state_at(future);
    assert_eq!(state_at_future.age_at_timestamp().as_years(), 30);

    // 10 years in past, age is STILL 30
    let past = reference - Duration::years(10);
    let state_at_past = handle.state_at(past);
    assert_eq!(state_at_past.age_at_timestamp().as_years(), 30);
}

/// Birth_date enables accurate life stage transitions over time.
#[test]
fn birth_date_enables_life_stage_transitions() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    // Entity born 2010-01-01, age 14 at reference (adolescent)
    let birth_date = Timestamp::from_ymd_hms(2010, 1, 1, 0, 0, 0);
    let entity = EntityBuilder::new()
        .id("person")
        .species(Species::Human)
        .birth_date(birth_date)
        .age(Duration::years(14))
        .build()
        .unwrap();

    let entity_id = EntityId::new("person").unwrap();
    sim.add_entity(entity, reference);

    let handle = sim.entity(&entity_id).unwrap();

    // At anchor (age 14), should be adolescent
    let state_at_anchor = handle.state_at(reference);
    assert!(matches!(
        state_at_anchor.life_stage(),
        behavioral_pathways::enums::LifeStage::Adolescent
    ));

    // 10 years later (age 24), should be young adult
    let future = reference + Duration::years(10);
    let state_at_future = handle.state_at(future);
    assert!(matches!(
        state_at_future.life_stage(),
        behavioral_pathways::enums::LifeStage::YoungAdult
    ));

    // 6 years earlier (age 8), should be childhood
    let past = reference - Duration::years(6);
    let state_at_past = handle.state_at(past);
    assert!(matches!(
        state_at_past.life_stage(),
        behavioral_pathways::enums::LifeStage::Child
    ));
}
