//! Bond type definitions for relationship classification.
//!
//! Bond types describe the nature of relationships between entities.
//! A relationship can have multiple bond types (e.g., both Family and Friend).

/// The type of bond in a relationship between two entities.
///
/// Bonds describe the nature and role structure of relationships.
/// A relationship can have multiple bond types simultaneously.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::enums::BondType;
///
/// let bonds = vec![BondType::Family, BondType::Friend];
/// assert!(bonds.contains(&BondType::Family));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BondType {
    /// Equal standing relationship with no hierarchy.
    Peer,

    /// Guidance relationship where this entity provides mentorship.
    Mentor,

    /// Guidance relationship where this entity receives mentorship.
    Mentee,

    /// Family relationship (parent, sibling, child, etc.).
    Family,

    /// Friendship bond based on mutual affection.
    Friend,

    /// Work-related relationship.
    Colleague,

    /// Romantic partnership.
    Romantic,

    /// Competitive or adversarial relationship.
    Rival,

    /// This entity has authority over the other.
    Authority,

    /// This entity is subordinate to the other.
    Subordinate,

    /// Parent of the other entity.
    Parent,

    /// Child of the other entity.
    Child,

    /// Sibling of the other entity.
    Sibling,
}

impl BondType {
    /// Returns a human-readable name for this bond type.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            BondType::Peer => "Peer",
            BondType::Mentor => "Mentor",
            BondType::Mentee => "Mentee",
            BondType::Family => "Family",
            BondType::Friend => "Friend",
            BondType::Colleague => "Colleague",
            BondType::Romantic => "Romantic",
            BondType::Rival => "Rival",
            BondType::Authority => "Authority",
            BondType::Subordinate => "Subordinate",
            BondType::Parent => "Parent",
            BondType::Child => "Child",
            BondType::Sibling => "Sibling",
        }
    }

    /// Returns true if this bond type implies a hierarchical relationship.
    ///
    /// Hierarchical bonds have an implicit power differential.
    #[must_use]
    pub const fn is_hierarchical(&self) -> bool {
        matches!(
            self,
            BondType::Mentor
                | BondType::Mentee
                | BondType::Authority
                | BondType::Subordinate
                | BondType::Parent
                | BondType::Child
        )
    }

    /// Returns true if this bond type is a family relationship.
    #[must_use]
    pub const fn is_family(&self) -> bool {
        matches!(
            self,
            BondType::Family | BondType::Parent | BondType::Child | BondType::Sibling
        )
    }

    /// Returns the reciprocal bond type, if any.
    ///
    /// For asymmetric bonds, returns what the other entity would have.
    /// For symmetric bonds, returns the same type.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::enums::BondType;
    ///
    /// assert_eq!(BondType::Parent.reciprocal(), BondType::Child);
    /// assert_eq!(BondType::Friend.reciprocal(), BondType::Friend);
    /// ```
    #[must_use]
    pub const fn reciprocal(&self) -> BondType {
        match self {
            BondType::Mentor => BondType::Mentee,
            BondType::Mentee => BondType::Mentor,
            BondType::Authority => BondType::Subordinate,
            BondType::Subordinate => BondType::Authority,
            BondType::Parent => BondType::Child,
            BondType::Child => BondType::Parent,
            // Symmetric bonds
            BondType::Peer => BondType::Peer,
            BondType::Family => BondType::Family,
            BondType::Friend => BondType::Friend,
            BondType::Colleague => BondType::Colleague,
            BondType::Romantic => BondType::Romantic,
            BondType::Rival => BondType::Rival,
            BondType::Sibling => BondType::Sibling,
        }
    }

    /// Returns all bond types.
    #[must_use]
    pub const fn all() -> [BondType; 13] {
        [
            BondType::Peer,
            BondType::Mentor,
            BondType::Mentee,
            BondType::Family,
            BondType::Friend,
            BondType::Colleague,
            BondType::Romantic,
            BondType::Rival,
            BondType::Authority,
            BondType::Subordinate,
            BondType::Parent,
            BondType::Child,
            BondType::Sibling,
        ]
    }
}

impl std::fmt::Display for BondType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn bond_type_includes_parent_child_sibling_friend() {
        let all = BondType::all();
        let set: HashSet<_> = all.iter().collect();

        assert!(set.contains(&BondType::Parent));
        assert!(set.contains(&BondType::Child));
        assert!(set.contains(&BondType::Sibling));
        assert!(set.contains(&BondType::Friend));
    }

    #[test]
    fn all_bond_types() {
        let all = BondType::all();
        assert_eq!(all.len(), 13);
    }

    #[test]
    fn hierarchical_bonds() {
        assert!(BondType::Parent.is_hierarchical());
        assert!(BondType::Child.is_hierarchical());
        assert!(BondType::Mentor.is_hierarchical());
        assert!(BondType::Mentee.is_hierarchical());
        assert!(BondType::Authority.is_hierarchical());
        assert!(BondType::Subordinate.is_hierarchical());

        assert!(!BondType::Peer.is_hierarchical());
        assert!(!BondType::Friend.is_hierarchical());
        assert!(!BondType::Sibling.is_hierarchical());
    }

    #[test]
    fn family_bonds() {
        assert!(BondType::Family.is_family());
        assert!(BondType::Parent.is_family());
        assert!(BondType::Child.is_family());
        assert!(BondType::Sibling.is_family());

        assert!(!BondType::Friend.is_family());
        assert!(!BondType::Colleague.is_family());
    }

    #[test]
    fn reciprocal_relationships() {
        assert_eq!(BondType::Parent.reciprocal(), BondType::Child);
        assert_eq!(BondType::Child.reciprocal(), BondType::Parent);
        assert_eq!(BondType::Mentor.reciprocal(), BondType::Mentee);
        assert_eq!(BondType::Mentee.reciprocal(), BondType::Mentor);
        assert_eq!(BondType::Authority.reciprocal(), BondType::Subordinate);
        assert_eq!(BondType::Subordinate.reciprocal(), BondType::Authority);

        // Symmetric bonds
        assert_eq!(BondType::Friend.reciprocal(), BondType::Friend);
        assert_eq!(BondType::Sibling.reciprocal(), BondType::Sibling);
        assert_eq!(BondType::Peer.reciprocal(), BondType::Peer);
    }

    #[test]
    fn display_format() {
        assert_eq!(format!("{}", BondType::Parent), "Parent");
        assert_eq!(format!("{}", BondType::Friend), "Friend");
    }

    #[test]
    fn equality_and_hash() {
        assert_eq!(BondType::Friend, BondType::Friend);
        assert_ne!(BondType::Friend, BondType::Rival);

        let mut set = HashSet::new();
        set.insert(BondType::Friend);
        set.insert(BondType::Friend);
        assert_eq!(set.len(), 1);

        set.insert(BondType::Family);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn copy_and_clone() {
        let original = BondType::Parent;
        let copied = original;
        let cloned = original.clone();

        assert_eq!(original, copied);
        assert_eq!(original, cloned);
    }

    #[test]
    fn name_returns_correct_string() {
        assert_eq!(BondType::Peer.name(), "Peer");
        assert_eq!(BondType::Romantic.name(), "Romantic");
        assert_eq!(BondType::Colleague.name(), "Colleague");
    }

    #[test]
    fn all_bond_names() {
        // Verify all bond types have expected names
        for bond in BondType::all() {
            assert!(!bond.name().is_empty());
        }
    }

    #[test]
    fn all_reciprocals() {
        // Verify all bond types have valid reciprocals
        for bond in BondType::all() {
            let reciprocal = bond.reciprocal();
            // Reciprocal of reciprocal should be original
            assert_eq!(reciprocal.reciprocal(), bond);
        }
    }

    #[test]
    fn mentor_mentee_bond() {
        assert!(BondType::Mentor.is_hierarchical());
        assert!(BondType::Mentee.is_hierarchical());
        assert!(!BondType::Mentor.is_family());
        assert!(!BondType::Mentee.is_family());
    }

    #[test]
    fn authority_subordinate_bond() {
        assert!(BondType::Authority.is_hierarchical());
        assert!(BondType::Subordinate.is_hierarchical());
        assert_eq!(BondType::Authority.reciprocal(), BondType::Subordinate);
        assert_eq!(BondType::Subordinate.reciprocal(), BondType::Authority);
    }

    #[test]
    fn romantic_and_colleague_bond() {
        assert!(!BondType::Romantic.is_hierarchical());
        assert!(!BondType::Romantic.is_family());
        assert!(!BondType::Colleague.is_hierarchical());
        assert!(!BondType::Colleague.is_family());
    }

    #[test]
    fn rival_bond() {
        assert!(!BondType::Rival.is_hierarchical());
        assert!(!BondType::Rival.is_family());
        assert_eq!(BondType::Rival.reciprocal(), BondType::Rival);
    }

    #[test]
    fn debug_format() {
        let bond = BondType::Parent;
        let debug = format!("{:?}", bond);
        assert!(debug.contains("Parent"));
    }
}
