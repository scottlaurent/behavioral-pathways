//! Integration test: Memory layers respect capacity limits.
//!
//! Validates that each memory layer enforces its capacity limit
//! and evicts lowest-salience entries when full.

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::Species;
use behavioral_pathways::memory::{
    MemoryEntry, MemoryLayer, IMMEDIATE_CAPACITY, SHORT_TERM_CAPACITY,
};
use behavioral_pathways::types::Duration;

/// Tests that immediate layer evicts when at capacity.
#[test]
fn memory_layers_respect_capacity() {
    let mut entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    // Fill immediate layer to capacity with increasing salience
    for i in 0..IMMEDIATE_CAPACITY {
        let salience = (i as f32) / 10.0 + 0.1; // 0.1, 0.2, 0.3, ...
        let entry = MemoryEntry::new(Duration::days(i as u64), format!("Memory {i}"))
            .with_salience(salience);
        entity.memories_mut().add(MemoryLayer::Immediate, entry);
    }

    assert_eq!(entity.memories().immediate_count(), IMMEDIATE_CAPACITY);

    // Verify all memories are present
    let before_ids: Vec<_> = entity
        .memories()
        .immediate()
        .iter()
        .map(|m| m.summary().to_string())
        .collect();
    assert!(before_ids.contains(&"Memory 0".to_string())); // Lowest salience (0.1)

    // Add one more with high salience
    let new_entry =
        MemoryEntry::new(Duration::days(100), "New important memory").with_salience(0.95);
    entity.memories_mut().add(MemoryLayer::Immediate, new_entry);

    // Should still be at capacity
    assert_eq!(entity.memories().immediate_count(), IMMEDIATE_CAPACITY);

    // The lowest-salience memory (Memory 0) should have been evicted
    let after_ids: Vec<_> = entity
        .memories()
        .immediate()
        .iter()
        .map(|m| m.summary().to_string())
        .collect();
    assert!(!after_ids.contains(&"Memory 0".to_string())); // Evicted
    assert!(after_ids.contains(&"New important memory".to_string())); // New entry present
}

/// Tests that short-term layer respects its capacity.
#[test]
fn short_term_layer_respects_capacity() {
    let mut entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    // Fill short-term layer to capacity
    for i in 0..SHORT_TERM_CAPACITY {
        let entry =
            MemoryEntry::new(Duration::days(i as u64), format!("ST Memory {i}")).with_salience(0.5);
        entity.memories_mut().add(MemoryLayer::ShortTerm, entry);
    }

    assert_eq!(entity.memories().short_term_count(), SHORT_TERM_CAPACITY);

    // Add one more
    let new_entry = MemoryEntry::new(Duration::days(100), "New ST memory").with_salience(0.5);
    entity.memories_mut().add(MemoryLayer::ShortTerm, new_entry);

    // Should still be at capacity (oldest evicted due to tie on salience)
    assert_eq!(entity.memories().short_term_count(), SHORT_TERM_CAPACITY);

    // Oldest (ST Memory 0) should have been evicted
    let summaries: Vec<_> = entity
        .memories()
        .short_term()
        .iter()
        .map(|m| m.summary().to_string())
        .collect();
    assert!(!summaries.contains(&"ST Memory 0".to_string()));
    assert!(summaries.contains(&"New ST memory".to_string()));
}

/// Tests that legacy layer has no capacity limit.
#[test]
fn legacy_layer_has_no_capacity_limit() {
    let mut entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    // Add many memories to legacy layer
    let large_count = 100;
    for i in 0..large_count {
        let entry =
            MemoryEntry::new(Duration::days(i as u64), format!("Legacy {i}")).with_salience(0.5);
        entity.memories_mut().add(MemoryLayer::Legacy, entry);
    }

    // All should be present
    assert_eq!(entity.memories().legacy_count(), large_count);
}
