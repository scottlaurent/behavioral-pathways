use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::Species;
use behavioral_pathways::state::StateInterpreter;
use behavioral_pathways::simulation::Simulation;
use behavioral_pathways::types::{EntityId, Timestamp};

#[test]
fn mock_b_person_in_crisis() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let mut entity = EntityBuilder::new()
        .id("mock_b")
        .species(Species::Human)
        .build()
        .unwrap();

    entity.individual_state_mut().mood_mut().add_valence_delta(-0.8);
    entity.individual_state_mut().mood_mut().add_arousal_delta(-0.8);
    entity.individual_state_mut().mood_mut().add_dominance_delta(-0.7);
    entity.individual_state_mut().needs_mut().add_stress_delta(0.6);
    entity.individual_state_mut().needs_mut().add_fatigue_delta(0.6);
    entity.individual_state_mut().needs_mut().add_purpose_delta(-0.7);
    entity.individual_state_mut().social_cognition_mut().add_loneliness_delta(0.6);
    entity.individual_state_mut().social_cognition_mut().add_perceived_reciprocal_caring_delta(-0.6);
    entity.individual_state_mut().mental_health_mut().add_depression_delta(0.7);

    let entity_id = EntityId::new("mock_b").unwrap();
    sim.add_entity(entity, reference);

    let handle = sim.entity(&entity_id).unwrap();
    let state = handle.state_at(reference);
    let interpreter = StateInterpreter::from_state(state.individual_state());

    assert_eq!(interpreter.interpretations().get("valence").unwrap(), "feeling very negative");
    assert_eq!(interpreter.interpretations().get("arousal").unwrap(), "very low energy");
    assert_eq!(interpreter.interpretations().get("dominance").unwrap(), "feeling very out of control");
    assert_eq!(interpreter.interpretations().get("stress").unwrap(), "experiencing severe stress");
    assert_eq!(interpreter.interpretations().get("fatigue").unwrap(), "extremely fatigued");
    assert_eq!(interpreter.interpretations().get("purpose").unwrap(), "lacks sense of purpose");
    assert_eq!(interpreter.interpretations().get("loneliness").unwrap(), "feeling very lonely");
    assert_eq!(interpreter.interpretations().get("perceived_reciprocal_caring").unwrap(), "feels uncared for by others");
    assert_eq!(interpreter.interpretations().get("depression").unwrap(), "experiencing severe depression");

    let expected_summary = "Feeling very negative. Very low energy. Feeling very out of control. Experiencing severe stress. Extremely fatigued. Lacks sense of purpose. Feeling very lonely. Feels uncared for by others. Experiencing severe depression.";
    assert_eq!(interpreter.summary(), expected_summary);
}
