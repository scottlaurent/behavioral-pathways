//! Integration test: Entity provides propensity, Relationship computes trust decision.
//!
//! This test validates the parameter passing between Entity and Relationship
//! systems for trust computation.

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::{Direction, DispositionPath, Species, StatePath};
use behavioral_pathways::relationship::{Relationship, RelationshipStage, StakesLevel};
use behavioral_pathways::types::EntityId;

/// Complete integration: Entity disposition flows through to trust decision.
///
/// This test shows the full pattern a consumer would use:
/// 1. Have an Entity with disposition
/// 2. Have a Relationship between that entity and another
/// 3. Extract the trust propensity from Entity
/// 4. Pass it to Relationship for trust computation
/// 5. Get back a TrustDecision with willingness values
#[test]
fn entity_relationship_trust_decision_integration() {
    // === Setup: Create two entities with different trust dispositions ===

    // Alice: high trust propensity (0.8)
    let mut alice_entity = EntityBuilder::new()
        .id("alice")
        .species(Species::Human)
        .build()
        .unwrap();
    alice_entity
        .individual_state_mut()
        .disposition_mut()
        .trust_propensity_mut()
        .set_base(0.8);

    // Bob: moderate trust propensity (0.5)
    let mut bob_entity = EntityBuilder::new()
        .id("bob")
        .species(Species::Human)
        .build()
        .unwrap();
    bob_entity
        .individual_state_mut()
        .disposition_mut()
        .trust_propensity_mut()
        .set_base(0.5);

    // === Create Relationship ===

    let alice_id = EntityId::new("alice").unwrap();
    let bob_id = EntityId::new("bob").unwrap();

    let relationship = Relationship::try_between(alice_id.clone(), bob_id.clone())
        .unwrap()
        .with_stage(RelationshipStage::Acquaintance);

    // === Extract propensities from entities ===

    let alice_propensity = alice_entity
        .get_effective(StatePath::Disposition(DispositionPath::TrustPropensity))
        .expect("Should get Alice's trust propensity") as f32;

    let bob_propensity = bob_entity
        .get_effective(StatePath::Disposition(DispositionPath::TrustPropensity))
        .expect("Should get Bob's trust propensity") as f32;

    // === Compute trust decisions ===

    // Alice deciding whether to trust Bob (AToB, using Alice's propensity)
    let alice_trusts_bob =
        relationship.compute_trust_decision(Direction::AToB, alice_propensity, StakesLevel::Medium);

    // Bob deciding whether to trust Alice (BToA, using Bob's propensity)
    let bob_trusts_alice =
        relationship.compute_trust_decision(Direction::BToA, bob_propensity, StakesLevel::Medium);

    // === Verify: Alice's higher propensity leads to higher willingness ===

    // Alice should be more willing to trust than Bob (higher propensity)
    assert!(
        alice_trusts_bob.task_willingness() > bob_trusts_alice.task_willingness(),
        "Alice (propensity {}) should have higher task willingness ({}) than Bob (propensity {}, willingness {})",
        alice_propensity,
        alice_trusts_bob.task_willingness(),
        bob_propensity,
        bob_trusts_alice.task_willingness()
    );

    // Both should have reasonable willingness values
    // Note: Low propensity with stranger relationship may yield very low willingness
    assert!(alice_trusts_bob.task_willingness() > 0.0);
    assert!(alice_trusts_bob.task_willingness() < 1.0);
    assert!(bob_trusts_alice.task_willingness() >= 0.0);
    assert!(bob_trusts_alice.task_willingness() < 1.0);

    // Confidence should reflect acquaintance stage
    assert!(
        alice_trusts_bob.confidence() > 0.2,
        "Should have some confidence at acquaintance stage"
    );
    assert!(
        alice_trusts_bob.confidence() < 0.7,
        "Should not have full confidence at acquaintance stage"
    );
}

/// Stakes level affects trust decision alongside propensity.
#[test]
fn stakes_level_interacts_with_propensity() {
    // Create entity with moderate propensity
    let mut entity = EntityBuilder::new()
        .id("person")
        .species(Species::Human)
        .build()
        .unwrap();
    entity
        .individual_state_mut()
        .disposition_mut()
        .trust_propensity_mut()
        .set_base(0.6);

    let propensity = entity
        .get_effective(StatePath::Disposition(DispositionPath::TrustPropensity))
        .unwrap() as f32;

    // Create relationship
    let person = EntityId::new("person").unwrap();
    let other = EntityId::new("other").unwrap();
    let relationship = Relationship::try_between(person, other).unwrap();

    // Compute decisions at different stakes levels
    let low_stakes =
        relationship.compute_trust_decision(Direction::AToB, propensity, StakesLevel::Low);
    let medium_stakes =
        relationship.compute_trust_decision(Direction::AToB, propensity, StakesLevel::Medium);
    let high_stakes =
        relationship.compute_trust_decision(Direction::AToB, propensity, StakesLevel::High);

    // Higher stakes should lead to lower willingness (more caution)
    assert!(
        low_stakes.task_willingness() > medium_stakes.task_willingness(),
        "Low stakes ({}) should yield higher willingness than medium ({})",
        low_stakes.task_willingness(),
        medium_stakes.task_willingness()
    );

    assert!(
        medium_stakes.task_willingness() > high_stakes.task_willingness(),
        "Medium stakes ({}) should yield higher willingness than high ({})",
        medium_stakes.task_willingness(),
        high_stakes.task_willingness()
    );
}

/// TrustDecision provides all three willingness domains.
#[test]
#[allow(deprecated)]
fn trust_decision_provides_all_domains() {
    let entity = EntityBuilder::new()
        .id("person")
        .species(Species::Human)
        .build()
        .unwrap();

    let propensity = entity
        .get_effective(StatePath::Disposition(DispositionPath::TrustPropensity))
        .unwrap() as f32;

    let person = EntityId::new("person").unwrap();
    let other = EntityId::new("other").unwrap();
    let relationship = Relationship::try_between(person, other).unwrap();

    let decision =
        relationship.compute_trust_decision(Direction::AToB, propensity, StakesLevel::Medium);

    // All willingness values should be valid (0-1 range)
    assert!(decision.task_willingness() >= 0.0 && decision.task_willingness() <= 1.0);
    assert!(decision.support_willingness() >= 0.0 && decision.support_willingness() <= 1.0);
    assert!(decision.disclosure_willingness() >= 0.0 && decision.disclosure_willingness() <= 1.0);
    assert!(decision.confidence() >= 0.0 && decision.confidence() <= 1.0);

    // Overall willingness should be average of the three
    let expected_overall = (decision.task_willingness()
        + decision.support_willingness()
        + decision.disclosure_willingness())
        / 3.0;
    assert!((decision.overall_willingness() - expected_overall).abs() < f32::EPSILON);
}
