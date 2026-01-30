use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::Species;
use behavioral_pathways::state::StateInterpreter;
use behavioral_pathways::simulation::Simulation;
use behavioral_pathways::types::{EntityId, Timestamp};

fn create_entity_with_valence(valence_delta: f32) -> StateInterpreter {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);
    let mut entity = EntityBuilder::new().id("test").species(Species::Human).build().unwrap();
    entity.individual_state_mut().mood_mut().add_valence_delta(valence_delta);
    sim.add_entity(entity, reference);
    let handle = sim.entity(&EntityId::new("test").unwrap()).unwrap();
    let state = handle.state_at(reference);
    StateInterpreter::from_state(state.individual_state())
}

fn create_entity_with_arousal(arousal_delta: f32) -> StateInterpreter {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);
    let mut entity = EntityBuilder::new().id("test").species(Species::Human).build().unwrap();
    entity.individual_state_mut().mood_mut().add_arousal_delta(arousal_delta);
    sim.add_entity(entity, reference);
    let handle = sim.entity(&EntityId::new("test").unwrap()).unwrap();
    let state = handle.state_at(reference);
    StateInterpreter::from_state(state.individual_state())
}

fn create_entity_with_dominance(dominance_delta: f32) -> StateInterpreter {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);
    let mut entity = EntityBuilder::new().id("test").species(Species::Human).build().unwrap();
    entity.individual_state_mut().mood_mut().add_dominance_delta(dominance_delta);
    sim.add_entity(entity, reference);
    let handle = sim.entity(&EntityId::new("test").unwrap()).unwrap();
    let state = handle.state_at(reference);
    StateInterpreter::from_state(state.individual_state())
}

fn create_entity_with_stress(stress_delta: f32) -> StateInterpreter {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);
    let mut entity = EntityBuilder::new().id("test").species(Species::Human).build().unwrap();
    entity.individual_state_mut().needs_mut().add_stress_delta(stress_delta);
    sim.add_entity(entity, reference);
    let handle = sim.entity(&EntityId::new("test").unwrap()).unwrap();
    let state = handle.state_at(reference);
    StateInterpreter::from_state(state.individual_state())
}

fn create_entity_with_fatigue(fatigue_delta: f32) -> StateInterpreter {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);
    let mut entity = EntityBuilder::new().id("test").species(Species::Human).build().unwrap();
    entity.individual_state_mut().needs_mut().add_fatigue_delta(fatigue_delta);
    sim.add_entity(entity, reference);
    let handle = sim.entity(&EntityId::new("test").unwrap()).unwrap();
    let state = handle.state_at(reference);
    StateInterpreter::from_state(state.individual_state())
}

fn create_entity_with_purpose(purpose_delta: f32) -> StateInterpreter {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);
    let mut entity = EntityBuilder::new().id("test").species(Species::Human).build().unwrap();
    entity.individual_state_mut().needs_mut().add_purpose_delta(purpose_delta);
    sim.add_entity(entity, reference);
    let handle = sim.entity(&EntityId::new("test").unwrap()).unwrap();
    let state = handle.state_at(reference);
    StateInterpreter::from_state(state.individual_state())
}

fn create_entity_with_loneliness(loneliness_delta: f32) -> StateInterpreter {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);
    let mut entity = EntityBuilder::new().id("test").species(Species::Human).build().unwrap();
    entity.individual_state_mut().social_cognition_mut().add_loneliness_delta(loneliness_delta);
    sim.add_entity(entity, reference);
    let handle = sim.entity(&EntityId::new("test").unwrap()).unwrap();
    let state = handle.state_at(reference);
    StateInterpreter::from_state(state.individual_state())
}

fn create_entity_with_prc(prc_delta: f32) -> StateInterpreter {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);
    let mut entity = EntityBuilder::new().id("test").species(Species::Human).build().unwrap();
    entity.individual_state_mut().social_cognition_mut().add_perceived_reciprocal_caring_delta(prc_delta);
    sim.add_entity(entity, reference);
    let handle = sim.entity(&EntityId::new("test").unwrap()).unwrap();
    let state = handle.state_at(reference);
    StateInterpreter::from_state(state.individual_state())
}

fn create_entity_with_depression(depression_delta: f32) -> StateInterpreter {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);
    let mut entity = EntityBuilder::new().id("test").species(Species::Human).build().unwrap();
    entity.individual_state_mut().mental_health_mut().add_depression_delta(depression_delta);
    sim.add_entity(entity, reference);
    let handle = sim.entity(&EntityId::new("test").unwrap()).unwrap();
    let state = handle.state_at(reference);
    StateInterpreter::from_state(state.individual_state())
}

#[test]
fn valence_all_branches() {
    assert_eq!(create_entity_with_valence(0.7).interpretations().get("valence").unwrap(), "feeling very positive");
    assert_eq!(create_entity_with_valence(0.3).interpretations().get("valence").unwrap(), "feeling moderately positive");
    assert_eq!(create_entity_with_valence(0.0).interpretations().get("valence").unwrap(), "feeling neutral");
    assert_eq!(create_entity_with_valence(-0.3).interpretations().get("valence").unwrap(), "feeling moderately negative");
    assert_eq!(create_entity_with_valence(-0.7).interpretations().get("valence").unwrap(), "feeling very negative");
}

#[test]
fn arousal_all_branches() {
    assert_eq!(create_entity_with_arousal(0.7).interpretations().get("arousal").unwrap(), "highly energized");
    assert_eq!(create_entity_with_arousal(0.3).interpretations().get("arousal").unwrap(), "moderately energized");
    assert_eq!(create_entity_with_arousal(0.0).interpretations().get("arousal").unwrap(), "neutral energy level");
    assert_eq!(create_entity_with_arousal(-0.3).interpretations().get("arousal").unwrap(), "low energy");
    assert_eq!(create_entity_with_arousal(-0.7).interpretations().get("arousal").unwrap(), "very low energy");
}

#[test]
fn dominance_all_branches() {
    assert_eq!(create_entity_with_dominance(0.7).interpretations().get("dominance").unwrap(), "feeling very in control");
    assert_eq!(create_entity_with_dominance(0.3).interpretations().get("dominance").unwrap(), "feeling somewhat in control");
    assert_eq!(create_entity_with_dominance(0.0).interpretations().get("dominance").unwrap(), "feeling neutral control");
    assert_eq!(create_entity_with_dominance(-0.3).interpretations().get("dominance").unwrap(), "feeling somewhat out of control");
    assert_eq!(create_entity_with_dominance(-0.7).interpretations().get("dominance").unwrap(), "feeling very out of control");
}

#[test]
fn stress_all_branches() {
    assert_eq!(create_entity_with_stress(0.6).interpretations().get("stress").unwrap(), "experiencing severe stress");
    assert_eq!(create_entity_with_stress(0.4).interpretations().get("stress").unwrap(), "experiencing elevated stress");
    assert_eq!(create_entity_with_stress(0.1).interpretations().get("stress").unwrap(), "experiencing mild stress");
    assert_eq!(create_entity_with_stress(-0.2).interpretations().get("stress").unwrap(), "feeling calm");
}

#[test]
fn fatigue_all_branches() {
    assert_eq!(create_entity_with_fatigue(0.6).interpretations().get("fatigue").unwrap(), "extremely fatigued");
    assert_eq!(create_entity_with_fatigue(0.4).interpretations().get("fatigue").unwrap(), "moderately fatigued");
    assert_eq!(create_entity_with_fatigue(0.1).interpretations().get("fatigue").unwrap(), "mildly fatigued");
    assert_eq!(create_entity_with_fatigue(-0.2).interpretations().get("fatigue").unwrap(), "well-rested");
}

#[test]
fn purpose_all_branches() {
    assert_eq!(create_entity_with_purpose(0.1).interpretations().get("purpose").unwrap(), "has strong sense of purpose");
    assert_eq!(create_entity_with_purpose(-0.1).interpretations().get("purpose").unwrap(), "has moderate sense of purpose");
    assert_eq!(create_entity_with_purpose(-0.4).interpretations().get("purpose").unwrap(), "has weak sense of purpose");
    assert_eq!(create_entity_with_purpose(-0.7).interpretations().get("purpose").unwrap(), "lacks sense of purpose");
}

#[test]
fn loneliness_all_branches() {
    assert_eq!(create_entity_with_loneliness(0.6).interpretations().get("loneliness").unwrap(), "feeling very lonely");
    assert_eq!(create_entity_with_loneliness(0.4).interpretations().get("loneliness").unwrap(), "feeling moderately lonely");
    assert_eq!(create_entity_with_loneliness(0.1).interpretations().get("loneliness").unwrap(), "feeling mildly lonely");
    assert_eq!(create_entity_with_loneliness(-0.2).interpretations().get("loneliness").unwrap(), "feeling well-connected");
}

#[test]
fn perceived_reciprocal_caring_all_branches() {
    assert_eq!(create_entity_with_prc(0.2).interpretations().get("perceived_reciprocal_caring").unwrap(), "feels deeply cared for by others");
    assert_eq!(create_entity_with_prc(0.0).interpretations().get("perceived_reciprocal_caring").unwrap(), "feels moderately cared for by others");
    assert_eq!(create_entity_with_prc(-0.3).interpretations().get("perceived_reciprocal_caring").unwrap(), "feels somewhat cared for by others");
    assert_eq!(create_entity_with_prc(-0.6).interpretations().get("perceived_reciprocal_caring").unwrap(), "feels uncared for by others");
}

#[test]
fn depression_all_branches() {
    assert_eq!(create_entity_with_depression(0.7).interpretations().get("depression").unwrap(), "experiencing severe depression");
    assert_eq!(create_entity_with_depression(0.5).interpretations().get("depression").unwrap(), "experiencing moderate depression");
    assert_eq!(create_entity_with_depression(0.2).interpretations().get("depression").unwrap(), "experiencing mild depression");
    assert_eq!(create_entity_with_depression(0.0).interpretations().get("depression").unwrap(), "not depressed");
}
