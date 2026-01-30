//! Test: state_at childhood regresses from adult anchor
//!
//! Scenario: Adult entity at anchor, query state in childhood
//! Expected: Age and life stage correctly regressed

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::{LifeStage, Species};
use behavioral_pathways::simulation::Simulation;
use behavioral_pathways::types::{Duration, EntityId, Timestamp};

/// Querying childhood state from adult anchor correctly regresses.
///
/// Stage 1: Create adult entity (30 years old) with birth date
/// Stage 2: Query state 20 years in the past (age 10 = childhood)
/// Stage 3: Verify age and life stage are correct for childhood
#[test]
fn state_at_childhood_regresses_from_adult() {
    // Stage 1: Setup - adult entity with birth date
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let birth = Timestamp::from_ymd_hms(1994, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let entity = EntityBuilder::new()
        .id("person_001")
        .species(Species::Human)
        .birth_date(birth)
        .build()
        .unwrap();

    sim.add_entity(entity, reference);

    let entity_id = EntityId::new("person_001").unwrap();
    let handle = sim.entity(&entity_id).unwrap();

    // Verify initial state is Adult (30 years old)
    let initial_state = handle.state_at(reference);
    assert_eq!(initial_state.life_stage(), LifeStage::YoungAdult); // 30 is YoungAdult (18-30)

    // Stage 2: Query state 20 years in the past (childhood)
    let childhood = reference - Duration::years(20);
    let childhood_state = handle.state_at(childhood);

    // Stage 3: Verify childhood state
    // At 10 years old, should be Child life stage
    let age = childhood_state.age_at_timestamp();
    assert_eq!(age.as_years(), 10);
    assert_eq!(childhood_state.life_stage(), LifeStage::Child);

    // Also verify an intermediate state
    let adolescence = reference - Duration::years(15);
    let adolescent_state = handle.state_at(adolescence);
    assert_eq!(adolescent_state.age_at_timestamp().as_years(), 15);
    assert_eq!(adolescent_state.life_stage(), LifeStage::Adolescent);
}
