//! Integration test: Entity.set_context(ContextPath, value) modifies context state.
//!
//! Validates that set_context correctly modifies context values via typed paths.

use behavioral_pathways::context::{EcologicalContext, Microsystem, WorkContext};
use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::{
    ContextPath, ExosystemPath, MacrosystemPath, MicrosystemPath, Species, WorkPath,
};
use behavioral_pathways::types::MicrosystemId;

/// Tests that Entity.set_context modifies macrosystem values correctly.
#[test]
fn entity_set_context_modifies_value() {
    let mut entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    // Set initial value via set_context
    let path = ContextPath::Macrosystem(MacrosystemPath::CulturalStress);
    let result = entity.set_context(&path, 0.4);
    assert!(result, "set_context should return true for valid path");

    // Verify value changed
    let value = entity.get_context(&path).unwrap();
    assert!(
        (value - 0.4).abs() < f64::EPSILON,
        "Expected 0.4, got {}",
        value
    );

    // Set different value
    let result2 = entity.set_context(&path, 0.8);
    assert!(result2, "set_context should return true for valid path");

    // Verify new value
    let new_value = entity.get_context(&path).unwrap();
    assert!(
        (new_value - 0.8).abs() < f64::EPSILON,
        "Expected 0.8, got {}",
        new_value
    );
}

/// Tests that set_context modifies exosystem values correctly.
#[test]
fn entity_set_context_modifies_exosystem_value() {
    let mut entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    let path = ContextPath::Exosystem(ExosystemPath::InstitutionalSupport);

    // Set initial value
    entity.set_context(&path, 0.3);
    assert!((entity.get_context(&path).unwrap() - 0.3).abs() < f64::EPSILON);

    // Modify to new value
    entity.set_context(&path, 0.9);
    assert!((entity.get_context(&path).unwrap() - 0.9).abs() < f64::EPSILON);
}

/// Tests that set_context modifies microsystem values correctly.
#[test]
fn entity_set_context_modifies_microsystem_value() {
    let mut context = EcologicalContext::default();
    let work_id = MicrosystemId::new("work_primary").unwrap();
    context.add_microsystem(
        work_id.clone(),
        Microsystem::new_work(WorkContext::default()),
    );

    let mut entity = EntityBuilder::new()
        .species(Species::Human)
        .with_context(context)
        .build()
        .unwrap();

    let path = ContextPath::Microsystem(work_id, MicrosystemPath::Work(WorkPath::WorkloadStress));

    // Set initial value
    entity.set_context(&path, 0.5);
    assert!((entity.get_context(&path).unwrap() - 0.5).abs() < f64::EPSILON);

    // Modify to new value
    entity.set_context(&path, 0.2);
    assert!((entity.get_context(&path).unwrap() - 0.2).abs() < f64::EPSILON);
}

/// Tests that set_context returns false for nonexistent microsystem.
#[test]
fn entity_set_context_returns_false_for_nonexistent_microsystem() {
    let mut entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    let nonexistent_id = MicrosystemId::new("nonexistent").unwrap();
    let path = ContextPath::Microsystem(
        nonexistent_id,
        MicrosystemPath::Work(WorkPath::WorkloadStress),
    );

    let result = entity.set_context(&path, 0.5);
    assert!(
        !result,
        "set_context should return false for nonexistent microsystem"
    );
}
