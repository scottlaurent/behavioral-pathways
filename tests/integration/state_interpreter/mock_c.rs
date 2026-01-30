use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::Species;
use behavioral_pathways::state::StateInterpreter;
use behavioral_pathways::simulation::Simulation;
use behavioral_pathways::types::{EntityId, Timestamp};

#[test]
fn mock_c_neutral_baseline_person() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let entity = EntityBuilder::new()
        .id("mock_c")
        .species(Species::Human)
        .build()
        .unwrap();

    let entity_id = EntityId::new("mock_c").unwrap();
    sim.add_entity(entity, reference);

    let handle = sim.entity(&entity_id).unwrap();
    let state = handle.state_at(reference);
    let interpreter = StateInterpreter::from_state(state.individual_state());

    assert_eq!(interpreter.interpretations().get("valence").unwrap(), "feeling neutral");
    assert_eq!(interpreter.interpretations().get("arousal").unwrap(), "neutral energy level");
    assert_eq!(interpreter.interpretations().get("dominance").unwrap(), "feeling neutral control");

    let expected_summary = "Feeling neutral. Neutral energy level. Feeling neutral control. Feeling calm. Well-rested. Has moderate sense of purpose. Feeling well-connected. Feels moderately cared for by others. Not depressed.";
    assert_eq!(interpreter.summary(), expected_summary);
}
