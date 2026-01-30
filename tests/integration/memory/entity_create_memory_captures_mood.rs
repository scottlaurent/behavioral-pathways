//! Integration test: Entity.create_memory captures current mood.
//!
//! Validates that when creating a memory via Entity.create_memory(),
//! the current mood is automatically captured as an EmotionalSnapshot.

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::Species;
use behavioral_pathways::memory::{MemoryLayer, MemoryTag};
use behavioral_pathways::types::EntityId;

/// Tests that create_memory auto-captures the entity's current mood.
#[test]
fn entity_create_memory_captures_mood() {
    let mut entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    // Set specific mood state
    entity
        .individual_state_mut()
        .mood_mut()
        .add_valence_delta(0.7);
    entity
        .individual_state_mut()
        .mood_mut()
        .add_arousal_delta(0.4);
    entity
        .individual_state_mut()
        .mood_mut()
        .add_dominance_delta(-0.1);

    // Create a memory using explicit layer selection
    let participant = EntityId::new("friend_001").unwrap();
    entity.create_memory_in_layer(
        MemoryLayer::ShortTerm,
        "Shared a meaningful experience",
        vec![participant],
        vec![MemoryTag::Personal, MemoryTag::Support],
        0.8,
        None,
    );

    // Retrieve the memory
    let memories = entity.memories().short_term();
    assert_eq!(memories.len(), 1);

    // Verify the emotional snapshot matches the mood at creation time
    let snapshot = memories[0].emotional_snapshot();
    assert!((snapshot.valence() - 0.7).abs() < 0.01);
    assert!((snapshot.arousal() - 0.4).abs() < 0.01);
    assert!((snapshot.dominance() - (-0.1)).abs() < 0.01);

    // Change mood after memory creation
    entity.individual_state_mut().mood_mut().reset_deltas();
    entity
        .individual_state_mut()
        .mood_mut()
        .add_valence_delta(-0.5);

    // Memory snapshot should still have original values
    let memories_after = entity.memories().short_term();
    let snapshot_after = memories_after[0].emotional_snapshot();
    assert!((snapshot_after.valence() - 0.7).abs() < 0.01);
}
