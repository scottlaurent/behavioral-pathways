//! Integration test: Entity.Disposition.trust_propensity flows to Relationship trust decision.
//!
//! This test validates that an Entity's dispositional trust propensity is correctly
//! extracted and passed to Relationship's trust computation.

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::{Direction, DispositionPath, Species, StatePath};
use behavioral_pathways::relationship::{Relationship, StakesLevel};
use behavioral_pathways::types::EntityId;

/// Entity's trust_propensity disposition integrates with Relationship trust computation.
///
/// Flow:
/// 1. Create Entity with specific trust_propensity in Disposition
/// 2. Create Relationship between entities
/// 3. Extract propensity from Entity via StatePath
/// 4. Pass propensity to Relationship's compute_trust_decision
/// 5. Verify propensity affects the result
#[test]
fn trust_propensity_flows_to_trust_decision() {
    // Step 1: Create entity with HIGH trust propensity
    let high_trust_entity = EntityBuilder::new()
        .id("high_trust_person")
        .species(Species::Human)
        .build()
        .unwrap();

    // Modify disposition to have high trust propensity
    let mut high_trust_entity = high_trust_entity;
    high_trust_entity
        .individual_state_mut()
        .disposition_mut()
        .trust_propensity_mut()
        .set_base(0.9);

    // Step 2: Create entity with LOW trust propensity
    let low_trust_entity = EntityBuilder::new()
        .id("low_trust_person")
        .species(Species::Human)
        .build()
        .unwrap();

    let mut low_trust_entity = low_trust_entity;
    low_trust_entity
        .individual_state_mut()
        .disposition_mut()
        .trust_propensity_mut()
        .set_base(0.1);

    // Step 3: Extract propensity values via StatePath
    let high_propensity = high_trust_entity
        .get_effective(StatePath::Disposition(DispositionPath::TrustPropensity))
        .expect("Trust propensity should be accessible");

    let low_propensity = low_trust_entity
        .get_effective(StatePath::Disposition(DispositionPath::TrustPropensity))
        .expect("Trust propensity should be accessible");

    // Verify extraction worked
    assert!(
        (high_propensity - 0.9).abs() < 0.01,
        "High propensity should be ~0.9"
    );
    assert!(
        (low_propensity - 0.1).abs() < 0.01,
        "Low propensity should be ~0.1"
    );

    // Step 4: Create a relationship and compute trust decisions with both propensities
    let alice = EntityId::new("alice").unwrap();
    let bob = EntityId::new("bob").unwrap();
    let relationship = Relationship::try_between(alice, bob).unwrap();

    // Compute trust decision with HIGH propensity
    let high_decision = relationship.compute_trust_decision(
        Direction::AToB,
        high_propensity as f32,
        StakesLevel::Medium,
    );

    // Compute trust decision with LOW propensity
    let low_decision = relationship.compute_trust_decision(
        Direction::AToB,
        low_propensity as f32,
        StakesLevel::Medium,
    );

    // Step 5: Verify propensity affects the result
    // Higher propensity should lead to higher willingness
    assert!(
        high_decision.task_willingness() > low_decision.task_willingness(),
        "High propensity ({}) should yield higher task willingness ({}) than low propensity ({}) yielding ({})",
        high_propensity,
        high_decision.task_willingness(),
        low_propensity,
        low_decision.task_willingness()
    );

    assert!(
        high_decision.support_willingness() > low_decision.support_willingness(),
        "High propensity should yield higher support willingness"
    );

    assert!(
        high_decision.disclosure_willingness() > low_decision.disclosure_willingness(),
        "High propensity should yield higher disclosure willingness"
    );
}

/// Propensity weight is higher for strangers than intimate relationships.
///
/// Per Mayer's model, dispositional trust propensity matters more when
/// you don't have experience with someone. As relationships develop,
/// observed trustworthiness matters more.
#[test]
fn propensity_weight_decreases_with_relationship_stage() {
    use behavioral_pathways::relationship::RelationshipStage;

    let alice = EntityId::new("alice").unwrap();
    let bob = EntityId::new("bob").unwrap();

    // Create stranger relationship (default)
    let stranger_rel = Relationship::try_between(alice.clone(), bob.clone()).unwrap();

    // Create intimate relationship
    let intimate_rel = Relationship::try_between(alice, bob)
        .unwrap()
        .with_stage(RelationshipStage::Intimate);

    // Use a high propensity value
    let high_propensity = 0.9_f32;
    let low_propensity = 0.2_f32;

    // For STRANGERS, propensity difference should matter a lot
    let stranger_high =
        stranger_rel.compute_trust_decision(Direction::AToB, high_propensity, StakesLevel::Medium);
    let stranger_low =
        stranger_rel.compute_trust_decision(Direction::AToB, low_propensity, StakesLevel::Medium);
    let stranger_diff = stranger_high.task_willingness() - stranger_low.task_willingness();

    // For INTIMATE relationships, propensity difference should matter less
    let intimate_high =
        intimate_rel.compute_trust_decision(Direction::AToB, high_propensity, StakesLevel::Medium);
    let intimate_low =
        intimate_rel.compute_trust_decision(Direction::AToB, low_propensity, StakesLevel::Medium);
    let intimate_diff = intimate_high.task_willingness() - intimate_low.task_willingness();

    // Propensity should have MORE impact on strangers than intimates
    assert!(
        stranger_diff > intimate_diff,
        "Propensity difference should have greater impact on strangers ({}) than intimates ({})",
        stranger_diff,
        intimate_diff
    );
}
