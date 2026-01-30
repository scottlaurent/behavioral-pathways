//! Integration test: Mood-congruent recall uses entity's current mood.
//!
//! Validates that recall_mood_congruent correctly queries the entity's
//! current mood state to find matching memories.

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::Species;
use behavioral_pathways::memory::{EmotionalSnapshot, MemoryEntry, MemoryLayer};
use behavioral_pathways::types::Duration;

/// Tests that mood-congruent retrieval uses the entity's current mood.
#[test]
fn mood_congruent_recall_uses_entity_mood() {
    let mut entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    // Add a happy memory
    let happy_memory = MemoryEntry::new(Duration::days(5), "A joyful celebration")
        .with_emotional_snapshot(EmotionalSnapshot::new(0.9, 0.7, 0.5))
        .with_salience(0.8);
    entity
        .memories_mut()
        .add(MemoryLayer::ShortTerm, happy_memory);

    // Add a sad memory
    let sad_memory = MemoryEntry::new(Duration::days(10), "A difficult goodbye")
        .with_emotional_snapshot(EmotionalSnapshot::new(-0.8, 0.3, -0.4))
        .with_salience(0.7);
    entity
        .memories_mut()
        .add(MemoryLayer::ShortTerm, sad_memory);

    // Set entity to happy mood
    entity
        .individual_state_mut()
        .mood_mut()
        .add_valence_delta(0.8);
    entity
        .individual_state_mut()
        .mood_mut()
        .add_arousal_delta(0.6);
    entity
        .individual_state_mut()
        .mood_mut()
        .add_dominance_delta(0.4);

    // Should preferentially recall happy memory
    let congruent = entity.recall_mood_congruent(0.7);
    assert!(!congruent.is_empty());
    assert!(congruent[0].emotional_snapshot().valence() > 0.5);

    // Change to sad mood
    entity.individual_state_mut().mood_mut().reset_deltas();
    entity
        .individual_state_mut()
        .mood_mut()
        .add_valence_delta(-0.7);
    entity
        .individual_state_mut()
        .mood_mut()
        .add_arousal_delta(0.2);
    entity
        .individual_state_mut()
        .mood_mut()
        .add_dominance_delta(-0.3);

    // Should now preferentially recall sad memory
    let congruent_sad = entity.recall_mood_congruent(0.7);
    assert!(!congruent_sad.is_empty());
    assert!(congruent_sad[0].emotional_snapshot().valence() < 0.0);
}
