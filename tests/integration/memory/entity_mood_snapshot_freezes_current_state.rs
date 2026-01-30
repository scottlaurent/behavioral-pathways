//! Integration test: Entity mood snapshot freezes current state.
//!
//! Validates that EmotionalSnapshot captures the mood at the moment
//! of snapshot creation and does not change when mood later changes.

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::Species;

/// Tests that mood snapshot captures mood at moment of creation
/// and remains unchanged when mood later changes.
#[test]
fn entity_mood_snapshot_freezes_current_state() {
    let mut entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    // Set initial mood
    entity
        .individual_state_mut()
        .mood_mut()
        .add_valence_delta(0.5);
    entity
        .individual_state_mut()
        .mood_mut()
        .add_arousal_delta(0.3);
    entity
        .individual_state_mut()
        .mood_mut()
        .add_dominance_delta(-0.2);

    // Take snapshot
    let snapshot = entity.mood_snapshot();

    // Verify snapshot captured current values
    assert!((snapshot.valence() - 0.5).abs() < 0.01);
    assert!((snapshot.arousal() - 0.3).abs() < 0.01);
    assert!((snapshot.dominance() - (-0.2)).abs() < 0.01);

    // Change mood significantly
    entity.individual_state_mut().mood_mut().reset_deltas();
    entity
        .individual_state_mut()
        .mood_mut()
        .add_valence_delta(-0.8);
    entity
        .individual_state_mut()
        .mood_mut()
        .add_arousal_delta(-0.5);
    entity
        .individual_state_mut()
        .mood_mut()
        .add_dominance_delta(0.7);

    // Original snapshot should be unchanged
    assert!((snapshot.valence() - 0.5).abs() < 0.01);
    assert!((snapshot.arousal() - 0.3).abs() < 0.01);
    assert!((snapshot.dominance() - (-0.2)).abs() < 0.01);

    // New snapshot should reflect current mood
    let new_snapshot = entity.mood_snapshot();
    assert!((new_snapshot.valence() - (-0.8)).abs() < 0.01);
    assert!((new_snapshot.arousal() - (-0.5)).abs() < 0.01);
    assert!((new_snapshot.dominance() - 0.7).abs() < 0.01);
}
