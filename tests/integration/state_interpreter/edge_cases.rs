use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::Species;
use behavioral_pathways::state::StateInterpreter;
use behavioral_pathways::simulation::Simulation;
use behavioral_pathways::types::{EntityId, Timestamp};

#[test]
fn valence_exact_thresholds() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let mut entity = EntityBuilder::new().id("test").species(Species::Human).build().unwrap();
    entity.individual_state_mut().mood_mut().add_valence_delta(0.6);
    sim.add_entity(entity, reference);
    let handle = sim.entity(&EntityId::new("test").unwrap()).unwrap();
    let state = handle.state_at(reference);
    let interp = StateInterpreter::from_state(state.individual_state());
    assert!(interp.interpretations().get("valence").unwrap().contains("moderately positive"));
}

#[test]
fn valence_just_below_threshold() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let mut entity = EntityBuilder::new().id("test").species(Species::Human).build().unwrap();
    entity.individual_state_mut().mood_mut().add_valence_delta(0.59);
    sim.add_entity(entity, reference);
    let handle = sim.entity(&EntityId::new("test").unwrap()).unwrap();
    let state = handle.state_at(reference);
    let interp = StateInterpreter::from_state(state.individual_state());
    assert!(interp.interpretations().get("valence").unwrap().contains("moderately positive"));
}

#[test]
fn delta_with_small_change_reports_magnitude() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let entity = EntityBuilder::new().id("test").species(Species::Human).build().unwrap();
    let entity_id = EntityId::new("test").unwrap();
    sim.add_entity(entity, reference);
    let handle = sim.entity(&entity_id).unwrap();
    let baseline_state = handle.state_at(reference).individual_state;

    let mut modified = baseline_state.clone();
    // 0.1 delta = "somewhat happier"
    modified.mood_mut().add_valence_delta(0.1);

    let interpreter = StateInterpreter::from_state_with_baseline(&modified, &baseline_state);
    let delta = interpreter.delta_summary();
    assert!(
        delta.is_some() && delta.unwrap().contains("happier"),
        "Expected 'happier' in delta: {:?}", delta
    );
}

#[test]
fn delta_arousal_equal_comparison() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let entity = EntityBuilder::new().id("test").species(Species::Human).build().unwrap();
    let entity_id = EntityId::new("test").unwrap();
    sim.add_entity(entity, reference);
    let handle = sim.entity(&entity_id).unwrap();
    let state = handle.state_at(reference).individual_state;

    let interpreter = StateInterpreter::from_state_with_baseline(&state, &state);
    assert_eq!(interpreter.delta_summary(), None);
}

#[test]
fn delta_dominance_equal_comparison() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let entity = EntityBuilder::new().id("test").species(Species::Human).build().unwrap();
    let entity_id = EntityId::new("test").unwrap();
    sim.add_entity(entity, reference);
    let handle = sim.entity(&entity_id).unwrap();
    let state = handle.state_at(reference).individual_state;

    let interpreter = StateInterpreter::from_state_with_baseline(&state, &state);
    assert_eq!(interpreter.delta_summary(), None);
}

#[test]
fn delta_stress_equal_comparison() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let entity = EntityBuilder::new().id("test").species(Species::Human).build().unwrap();
    let entity_id = EntityId::new("test").unwrap();
    sim.add_entity(entity, reference);
    let handle = sim.entity(&entity_id).unwrap();
    let state = handle.state_at(reference).individual_state;

    let interpreter = StateInterpreter::from_state_with_baseline(&state, &state);
    assert_eq!(interpreter.delta_summary(), None);
}
