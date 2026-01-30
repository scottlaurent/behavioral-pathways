use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::Species;
use behavioral_pathways::state::StateInterpreter;
use behavioral_pathways::simulation::Simulation;
use behavioral_pathways::types::{EntityId, Timestamp};

fn create_baseline_entity() -> (Simulation, EntityId, Timestamp) {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);
    let entity = EntityBuilder::new().id("test").species(Species::Human).build().unwrap();
    let entity_id = EntityId::new("test").unwrap();
    sim.add_entity(entity, reference);
    (sim, entity_id, reference)
}

#[test]
fn delta_valence_increase() {
    let (sim, entity_id, reference) = create_baseline_entity();
    let handle = sim.entity(&entity_id).unwrap();
    let baseline_state = handle.state_at(reference).individual_state;

    let mut modified = baseline_state.clone();
    modified.mood_mut().add_valence_delta(0.8);

    let interpreter = StateInterpreter::from_state_with_baseline(&modified, &baseline_state);
    assert!(interpreter.delta_summary().unwrap().contains("happier"));
}

#[test]
fn delta_valence_decrease() {
    let (sim, entity_id, reference) = create_baseline_entity();
    let handle = sim.entity(&entity_id).unwrap();
    let baseline_state = handle.state_at(reference).individual_state;

    let mut modified = baseline_state.clone();
    modified.mood_mut().add_valence_delta(-0.8);

    let interpreter = StateInterpreter::from_state_with_baseline(&modified, &baseline_state);
    assert!(interpreter.delta_summary().unwrap().contains("sadder"));
}

#[test]
fn delta_arousal_increase() {
    let (sim, entity_id, reference) = create_baseline_entity();
    let handle = sim.entity(&entity_id).unwrap();
    let baseline_state = handle.state_at(reference).individual_state;

    let mut modified = baseline_state.clone();
    modified.mood_mut().add_arousal_delta(0.8);

    let interpreter = StateInterpreter::from_state_with_baseline(&modified, &baseline_state);
    assert!(interpreter.delta_summary().unwrap().contains("more") && interpreter.delta_summary().unwrap().contains("energized"));
}

#[test]
fn delta_arousal_decrease() {
    let (sim, entity_id, reference) = create_baseline_entity();
    let handle = sim.entity(&entity_id).unwrap();
    let baseline_state = handle.state_at(reference).individual_state;

    let mut modified = baseline_state.clone();
    modified.mood_mut().add_arousal_delta(-0.8);

    let interpreter = StateInterpreter::from_state_with_baseline(&modified, &baseline_state);
    assert!(interpreter.delta_summary().unwrap().contains("less") && interpreter.delta_summary().unwrap().contains("energized"));
}

#[test]
fn delta_dominance_increase() {
    let (sim, entity_id, reference) = create_baseline_entity();
    let handle = sim.entity(&entity_id).unwrap();
    let baseline_state = handle.state_at(reference).individual_state;

    let mut modified = baseline_state.clone();
    modified.mood_mut().add_dominance_delta(0.8);

    let interpreter = StateInterpreter::from_state_with_baseline(&modified, &baseline_state);
    assert!(interpreter.delta_summary().unwrap().contains("more in control"));
}

#[test]
fn delta_dominance_decrease() {
    let (sim, entity_id, reference) = create_baseline_entity();
    let handle = sim.entity(&entity_id).unwrap();
    let baseline_state = handle.state_at(reference).individual_state;

    let mut modified = baseline_state.clone();
    modified.mood_mut().add_dominance_delta(-0.8);

    let interpreter = StateInterpreter::from_state_with_baseline(&modified, &baseline_state);
    assert!(interpreter.delta_summary().unwrap().contains("less in control"));
}

#[test]
fn delta_stress_increase() {
    let (sim, entity_id, reference) = create_baseline_entity();
    let handle = sim.entity(&entity_id).unwrap();
    let baseline_state = handle.state_at(reference).individual_state;

    let mut modified = baseline_state.clone();
    modified.needs_mut().add_stress_delta(0.7);

    let interpreter = StateInterpreter::from_state_with_baseline(&modified, &baseline_state);
    assert!(interpreter.delta_summary().unwrap().contains("much more stressed"));
}

#[test]
fn delta_stress_decrease() {
    let (sim, entity_id, reference) = create_baseline_entity();
    let handle = sim.entity(&entity_id).unwrap();
    let mut baseline_state = handle.state_at(reference).individual_state;
    // Start with elevated stress so we can decrease
    baseline_state.needs_mut().add_stress_delta(0.5);

    let mut modified = baseline_state.clone();
    modified.needs_mut().add_stress_delta(-0.2);

    let interpreter = StateInterpreter::from_state_with_baseline(&modified, &baseline_state);
    assert!(interpreter.delta_summary().unwrap().contains("less stressed"));
}

#[test]
fn delta_fatigue_increase() {
    let (sim, entity_id, reference) = create_baseline_entity();
    let handle = sim.entity(&entity_id).unwrap();
    let baseline_state = handle.state_at(reference).individual_state;

    let mut modified = baseline_state.clone();
    modified.needs_mut().add_fatigue_delta(0.7);

    let interpreter = StateInterpreter::from_state_with_baseline(&modified, &baseline_state);
    assert!(interpreter.delta_summary().unwrap().contains("more fatigued"));
}

#[test]
fn delta_fatigue_decrease() {
    let (sim, entity_id, reference) = create_baseline_entity();
    let handle = sim.entity(&entity_id).unwrap();
    let mut baseline_state = handle.state_at(reference).individual_state;
    // Start with elevated fatigue so we can decrease
    baseline_state.needs_mut().add_fatigue_delta(0.5);

    let mut modified = baseline_state.clone();
    modified.needs_mut().add_fatigue_delta(-0.2);

    let interpreter = StateInterpreter::from_state_with_baseline(&modified, &baseline_state);
    assert!(interpreter.delta_summary().unwrap().contains("less fatigued"));
}

#[test]
fn delta_purpose_increase() {
    let (sim, entity_id, reference) = create_baseline_entity();
    let handle = sim.entity(&entity_id).unwrap();
    let baseline_state = handle.state_at(reference).individual_state;

    let mut modified = baseline_state.clone();
    modified.needs_mut().add_purpose_delta(0.3);

    let interpreter = StateInterpreter::from_state_with_baseline(&modified, &baseline_state);
    assert!(interpreter.delta_summary().unwrap().contains("stronger sense of purpose"));
}

#[test]
fn delta_purpose_decrease() {
    let (sim, entity_id, reference) = create_baseline_entity();
    let handle = sim.entity(&entity_id).unwrap();
    let baseline_state = handle.state_at(reference).individual_state;

    let mut modified = baseline_state.clone();
    modified.needs_mut().add_purpose_delta(-0.3);

    let interpreter = StateInterpreter::from_state_with_baseline(&modified, &baseline_state);
    assert!(interpreter.delta_summary().unwrap().contains("weaker sense of purpose"));
}

#[test]
fn delta_loneliness_increase() {
    let (sim, entity_id, reference) = create_baseline_entity();
    let handle = sim.entity(&entity_id).unwrap();
    let baseline_state = handle.state_at(reference).individual_state;

    let mut modified = baseline_state.clone();
    modified.social_cognition_mut().add_loneliness_delta(0.7);

    let interpreter = StateInterpreter::from_state_with_baseline(&modified, &baseline_state);
    assert!(interpreter.delta_summary().unwrap().contains("lonelier"));
}

#[test]
fn delta_loneliness_decrease() {
    let (sim, entity_id, reference) = create_baseline_entity();
    let handle = sim.entity(&entity_id).unwrap();
    let mut baseline_state = handle.state_at(reference).individual_state;
    // Start with elevated loneliness so we can decrease
    baseline_state.social_cognition_mut().add_loneliness_delta(0.5);

    let mut modified = baseline_state.clone();
    modified.social_cognition_mut().add_loneliness_delta(-0.2);

    let interpreter = StateInterpreter::from_state_with_baseline(&modified, &baseline_state);
    assert!(interpreter.delta_summary().unwrap().contains("less lonely"));
}

#[test]
fn delta_perceived_reciprocal_caring_increase() {
    let (sim, entity_id, reference) = create_baseline_entity();
    let handle = sim.entity(&entity_id).unwrap();
    let baseline_state = handle.state_at(reference).individual_state;

    let mut modified = baseline_state.clone();
    modified.social_cognition_mut().add_perceived_reciprocal_caring_delta(0.3);

    let interpreter = StateInterpreter::from_state_with_baseline(&modified, &baseline_state);
    assert!(interpreter.delta_summary().unwrap().contains("more cared for"));
}

#[test]
fn delta_perceived_reciprocal_caring_decrease() {
    let (sim, entity_id, reference) = create_baseline_entity();
    let handle = sim.entity(&entity_id).unwrap();
    let baseline_state = handle.state_at(reference).individual_state;

    // Clone and decrease PRC significantly
    let mut modified = baseline_state.clone();
    modified.social_cognition_mut().add_perceived_reciprocal_caring_delta(-0.3);

    let interpreter = StateInterpreter::from_state_with_baseline(&modified, &baseline_state);
    let delta = interpreter.delta_summary();
    assert!(
        delta.is_some() && delta.unwrap().contains("less cared for"),
        "Expected 'less cared for' in delta: {:?}", delta
    );
}

#[test]
fn delta_depression_increase() {
    let (sim, entity_id, reference) = create_baseline_entity();
    let handle = sim.entity(&entity_id).unwrap();
    let baseline_state = handle.state_at(reference).individual_state;

    let mut modified = baseline_state.clone();
    modified.mental_health_mut().add_depression_delta(0.7);

    let interpreter = StateInterpreter::from_state_with_baseline(&modified, &baseline_state);
    assert!(interpreter.delta_summary().unwrap().contains("more depressed"));
}

#[test]
fn delta_depression_decrease() {
    let (sim, entity_id, reference) = create_baseline_entity();
    let handle = sim.entity(&entity_id).unwrap();
    let mut baseline_state = handle.state_at(reference).individual_state;
    // Start with elevated depression so we can decrease
    baseline_state.mental_health_mut().add_depression_delta(0.5);

    let mut modified = baseline_state.clone();
    modified.mental_health_mut().add_depression_delta(-0.2);

    let interpreter = StateInterpreter::from_state_with_baseline(&modified, &baseline_state);
    assert!(interpreter.delta_summary().unwrap().contains("less depressed"));
}

#[test]
fn delta_no_changes() {
    let (sim, entity_id, reference) = create_baseline_entity();
    let handle = sim.entity(&entity_id).unwrap();
    let baseline_state = handle.state_at(reference).individual_state;

    let interpreter = StateInterpreter::from_state_with_baseline(&baseline_state, &baseline_state);
    assert!(interpreter.delta_summary().is_none());
}

#[test]
fn delta_multiple_changes() {
    let (sim, entity_id, reference) = create_baseline_entity();
    let handle = sim.entity(&entity_id).unwrap();
    let baseline_state = handle.state_at(reference).individual_state;

    let mut modified = baseline_state.clone();
    modified.mood_mut().add_valence_delta(0.8);
    modified.needs_mut().add_stress_delta(0.7);
    modified.social_cognition_mut().add_loneliness_delta(0.6);

    let interpreter = StateInterpreter::from_state_with_baseline(&modified, &baseline_state);
    let delta = interpreter.delta_summary().unwrap();
    assert!(delta.contains("happier"));
    assert!(delta.contains("stressed"));
    assert!(delta.contains("lonelier"));
}
