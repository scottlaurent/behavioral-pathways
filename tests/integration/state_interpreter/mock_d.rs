use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::Species;
use behavioral_pathways::simulation::Simulation;
use behavioral_pathways::types::{EntityId, Timestamp};

#[test]
fn mock_d_computed_state_includes_interpretations() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let mut entity = EntityBuilder::new()
        .id("mock_d")
        .species(Species::Human)
        .build()
        .unwrap();

    entity.individual_state_mut().mood_mut().add_valence_delta(0.8);
    entity.individual_state_mut().needs_mut().add_stress_delta(0.7);

    let entity_id = EntityId::new("mock_d").unwrap();
    sim.add_entity(entity, reference);

    let handle = sim.entity(&entity_id).unwrap();
    let computed = handle.state_at(reference);

    assert_eq!(computed.interpretations.get("valence").unwrap(), "feeling very positive");
    assert_eq!(computed.interpretations.get("stress").unwrap(), "experiencing severe stress");

    assert!(computed.summary.contains("Feeling very positive"));
    assert!(computed.summary.contains("Experiencing severe stress"));

    let expected_summary = "Feeling very positive. Neutral energy level. Feeling neutral control. Experiencing severe stress. Well-rested. Has moderate sense of purpose. Feeling well-connected. Feels moderately cared for by others. Not depressed.";
    assert_eq!(computed.summary, expected_summary);
}
