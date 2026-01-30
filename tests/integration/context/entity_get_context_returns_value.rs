//! Integration test: Entity.get_context(ContextPath) retrieves values correctly.
//!
//! Validates that get_context correctly queries context values via typed paths.

use behavioral_pathways::context::{EcologicalContext, Microsystem, WorkContext};
use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::{
    ContextPath, ExosystemPath, MacrosystemPath, MicrosystemPath, Species, WorkPath,
};
use behavioral_pathways::types::MicrosystemId;

/// Tests that Entity.get_context retrieves macrosystem values correctly.
#[test]
fn entity_get_context_returns_value() {
    // Create entity with context
    let mut context = EcologicalContext::default();

    // Set a specific macrosystem value
    context
        .macrosystem_mut()
        .cultural_orientation
        .power_distance = 0.75;

    let entity = EntityBuilder::new()
        .species(Species::Human)
        .with_context(context)
        .build()
        .unwrap();

    // Query the value via get_context
    let path = ContextPath::Macrosystem(MacrosystemPath::PowerDistance);
    let value = entity.get_context(&path);

    // Verify the value is returned correctly
    assert!(value.is_some());
    assert!(
        (value.unwrap() - 0.75).abs() < f64::EPSILON,
        "Expected 0.75, got {}",
        value.unwrap()
    );
}

/// Tests that get_context retrieves exosystem values correctly.
#[test]
fn entity_get_context_returns_exosystem_value() {
    let mut context = EcologicalContext::default();
    context.exosystem_mut().resource_availability = 0.85;

    let entity = EntityBuilder::new()
        .species(Species::Human)
        .with_context(context)
        .build()
        .unwrap();

    let path = ContextPath::Exosystem(ExosystemPath::ResourceAvailability);
    let value = entity.get_context(&path);

    assert!(value.is_some());
    assert!(
        (value.unwrap() - 0.85).abs() < f64::EPSILON,
        "Expected 0.85, got {}",
        value.unwrap()
    );
}

/// Tests that get_context retrieves microsystem values correctly.
#[test]
fn entity_get_context_returns_microsystem_value() {
    let mut context = EcologicalContext::default();

    // Add a work microsystem with specific stress
    let work_id = MicrosystemId::new("work_primary").unwrap();
    let mut work = WorkContext::default();
    work.workload_stress = 0.65;
    context.add_microsystem(work_id.clone(), Microsystem::new_work(work));

    let entity = EntityBuilder::new()
        .species(Species::Human)
        .with_context(context)
        .build()
        .unwrap();

    let path = ContextPath::Microsystem(work_id, MicrosystemPath::Work(WorkPath::WorkloadStress));
    let value = entity.get_context(&path);

    assert!(value.is_some());
    assert!(
        (value.unwrap() - 0.65).abs() < f64::EPSILON,
        "Expected 0.65, got {}",
        value.unwrap()
    );
}

/// Tests that get_context returns None for nonexistent microsystem.
#[test]
fn entity_get_context_returns_none_for_nonexistent_microsystem() {
    let entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    let nonexistent_id = MicrosystemId::new("nonexistent").unwrap();
    let path = ContextPath::Microsystem(
        nonexistent_id,
        MicrosystemPath::Work(WorkPath::WorkloadStress),
    );
    let value = entity.get_context(&path);

    assert!(value.is_none());
}
