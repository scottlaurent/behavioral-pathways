//! Bond management integration tests.
//!
//! Tests for adding and removing bonds from relationships,
//! exercising the library crate's bond management code.

use behavioral_pathways::enums::BondType;
use behavioral_pathways::relationship::Relationship;
use behavioral_pathways::types::EntityId;

#[test]
fn remove_bond_from_multiple_bonds() {
    let alice = EntityId::new("alice").unwrap();
    let bob = EntityId::new("bob").unwrap();

    let mut rel = Relationship::try_between(alice, bob).unwrap();

    // Add multiple bonds
    rel.add_bond(BondType::Friend);
    rel.add_bond(BondType::Colleague);
    assert!(rel.has_bond(BondType::Friend));
    assert!(rel.has_bond(BondType::Colleague));

    // Remove one, other remains
    rel.remove_bond(BondType::Friend);
    assert!(!rel.has_bond(BondType::Friend));
    assert!(rel.has_bond(BondType::Colleague));

    // Remove the other
    rel.remove_bond(BondType::Colleague);
    assert!(!rel.has_bond(BondType::Colleague));
    assert!(rel.bonds().is_empty());
}

#[test]
fn remove_bond_that_does_not_exist() {
    let alice = EntityId::new("alice").unwrap();
    let bob = EntityId::new("bob").unwrap();

    let mut rel = Relationship::try_between(alice, bob).unwrap();

    rel.add_bond(BondType::Friend);

    // Remove a bond that was never added - should be no-op
    rel.remove_bond(BondType::Romantic);

    // Original bond still present
    assert!(rel.has_bond(BondType::Friend));
    assert_eq!(rel.bonds().len(), 1);
}
