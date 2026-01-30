use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::Species;
use behavioral_pathways::state::StateInterpreter;
use behavioral_pathways::simulation::Simulation;
use behavioral_pathways::types::{EntityId, Timestamp};

#[test]
fn mock_a_positive_well_adjusted_person() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let mut entity = EntityBuilder::new()
        .id("mock_a")
        .species(Species::Human)
        .build()
        .unwrap();

    entity.individual_state_mut().mood_mut().add_valence_delta(0.7);
    entity.individual_state_mut().mood_mut().add_arousal_delta(0.3);
    entity.individual_state_mut().mood_mut().add_dominance_delta(0.25);
    entity.individual_state_mut().needs_mut().add_stress_delta(-0.1);
    entity.individual_state_mut().needs_mut().add_fatigue_delta(-0.2);
    entity.individual_state_mut().needs_mut().add_purpose_delta(0.1);
    entity.individual_state_mut().social_cognition_mut().add_loneliness_delta(-0.2);
    entity.individual_state_mut().social_cognition_mut().add_perceived_reciprocal_caring_delta(0.2);

    let entity_id = EntityId::new("mock_a").unwrap();
    sim.add_entity(entity, reference);

    let handle = sim.entity(&entity_id).unwrap();
    let state = handle.state_at(reference);
    let interpreter = StateInterpreter::from_state(state.individual_state());

    assert_eq!(interpreter.interpretations().get("valence").unwrap(), "feeling very positive");
    assert_eq!(interpreter.interpretations().get("arousal").unwrap(), "moderately energized");
    assert_eq!(interpreter.interpretations().get("stress").unwrap(), "feeling calm");
    assert_eq!(interpreter.interpretations().get("loneliness").unwrap(), "feeling well-connected");
    assert_eq!(interpreter.interpretations().get("depression").unwrap(), "not depressed");

    let expected_summary = "Feeling very positive. Moderately energized. Feeling somewhat in control. Feeling calm. Well-rested. Has strong sense of purpose. Feeling well-connected. Feels deeply cared for by others. Not depressed.";
    assert_eq!(interpreter.summary(), expected_summary);
}
