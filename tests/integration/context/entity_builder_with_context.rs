//! Integration test: EntityBuilder initialization with context values.
//!
//! Validates that EntityBuilder.with_context() correctly initializes
//! the entity with a pre-populated EcologicalContext.

use behavioral_pathways::context::{
    EcologicalContext, FamilyContext, Microsystem, SocialContext, WorkContext,
};
use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::{
    ContextPath, MacrosystemPath, MicrosystemPath, Species, WorkPath,
};
use behavioral_pathways::types::MicrosystemId;

/// Tests that EntityBuilder.with_context() initializes entity with provided context.
#[test]
fn entity_builder_with_context() {
    // Create a populated EcologicalContext
    let mut context = EcologicalContext::default();

    // Add multiple microsystems
    let work_id = MicrosystemId::new("work_primary").unwrap();
    let mut work = WorkContext::default();
    work.workload_stress = 0.6;
    work.role_satisfaction = 0.8;
    context.add_microsystem(work_id.clone(), Microsystem::new_work(work));

    let family_id = MicrosystemId::new("family_primary").unwrap();
    let mut family = FamilyContext::default();
    family.warmth = 0.9;
    context.add_microsystem(family_id.clone(), Microsystem::new_family(family));

    let social_id = MicrosystemId::new("friends").unwrap();
    let mut social = SocialContext::default();
    social.warmth = 0.7;
    context.add_microsystem(social_id.clone(), Microsystem::new_social(social));

    // Modify other context layers
    context.macrosystem_mut().cultural_stress = 0.3;
    context.exosystem_mut().institutional_support = 0.7;

    // Create entity using with_context
    let entity = EntityBuilder::new()
        .species(Species::Human)
        .with_context(context)
        .build()
        .unwrap();

    // Verify entity has the provided context
    assert_eq!(
        entity.context().microsystem_count(),
        3,
        "Entity should have 3 microsystems"
    );

    // Verify work microsystem values
    let work_path = ContextPath::Microsystem(
        work_id.clone(),
        MicrosystemPath::Work(WorkPath::WorkloadStress),
    );
    let work_stress = entity.get_context(&work_path).unwrap();
    assert!(
        (work_stress - 0.6).abs() < f64::EPSILON,
        "Work stress should be 0.6"
    );

    // Verify macrosystem values
    let macro_path = ContextPath::Macrosystem(MacrosystemPath::CulturalStress);
    let cultural_stress = entity.get_context(&macro_path).unwrap();
    assert!(
        (cultural_stress - 0.3).abs() < f64::EPSILON,
        "Cultural stress should be 0.3"
    );

    // Verify microsystems are accessible by ID
    assert!(
        entity.context().get_microsystem(&work_id).is_some(),
        "Work microsystem should exist"
    );
    assert!(
        entity.context().get_microsystem(&family_id).is_some(),
        "Family microsystem should exist"
    );
    assert!(
        entity.context().get_microsystem(&social_id).is_some(),
        "Social microsystem should exist"
    );
}

/// Tests that entity without with_context gets default empty context.
#[test]
fn entity_builder_without_context_gets_default() {
    let entity = EntityBuilder::new()
        .species(Species::Human)
        .build()
        .unwrap();

    // Default context should have no microsystems
    assert_eq!(
        entity.context().microsystem_count(),
        0,
        "Default context should have no microsystems"
    );

    // Default macrosystem values should be accessible
    let macro_path = ContextPath::Macrosystem(MacrosystemPath::PowerDistance);
    let power_distance = entity.get_context(&macro_path);
    assert!(
        power_distance.is_some(),
        "Default macrosystem values should be accessible"
    );
}
