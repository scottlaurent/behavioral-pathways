//! Relationship schema definitions for structural relationship types.
//!
//! Schemas define the overall structure and expected patterns
//! of relationships, particularly for macrosystem constraints.

/// The structural schema of a relationship.
///
/// Schemas define the expected patterns and constraints for relationships.
/// The macrosystem can constrain which schemas are culturally available.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::enums::RelationshipSchema;
///
/// let schema = RelationshipSchema::Peer;
/// assert!(!schema.is_hierarchical());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RelationshipSchema {
    /// Equal standing relationship with no hierarchy.
    #[default]
    Peer,

    /// Guidance relationship where one entity mentors another.
    Mentor,

    /// Authority relationship with clear power differential.
    Subordinate,

    /// Romantic partnership.
    Romantic,

    /// Family bond (general).
    Family,

    /// Nuclear family structure (parents + children).
    Nuclear,

    /// Extended family structure (grandparents, aunts, uncles, cousins).
    Extended,

    /// Competitive or adversarial relationship.
    Rival,
}

impl RelationshipSchema {
    /// Returns a human-readable name for this schema.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            RelationshipSchema::Peer => "Peer",
            RelationshipSchema::Mentor => "Mentor",
            RelationshipSchema::Subordinate => "Subordinate",
            RelationshipSchema::Romantic => "Romantic",
            RelationshipSchema::Family => "Family",
            RelationshipSchema::Nuclear => "Nuclear Family",
            RelationshipSchema::Extended => "Extended Family",
            RelationshipSchema::Rival => "Rival",
        }
    }

    /// Returns true if this schema implies a hierarchical relationship.
    #[must_use]
    pub const fn is_hierarchical(&self) -> bool {
        matches!(
            self,
            RelationshipSchema::Mentor | RelationshipSchema::Subordinate
        )
    }

    /// Returns true if this schema is a family relationship.
    #[must_use]
    pub const fn is_family(&self) -> bool {
        matches!(
            self,
            RelationshipSchema::Family | RelationshipSchema::Nuclear | RelationshipSchema::Extended
        )
    }

    /// Returns a description of this schema.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        match self {
            RelationshipSchema::Peer => "Equal standing, mutual relationship",
            RelationshipSchema::Mentor => "Guidance relationship with teaching role",
            RelationshipSchema::Subordinate => "Authority relationship with power differential",
            RelationshipSchema::Romantic => "Romantic or intimate partnership",
            RelationshipSchema::Family => "General family bond",
            RelationshipSchema::Nuclear => "Core family unit (parents and children)",
            RelationshipSchema::Extended => "Extended family network",
            RelationshipSchema::Rival => "Competitive or adversarial dynamic",
        }
    }

    /// Returns all relationship schemas.
    #[must_use]
    pub const fn all() -> [RelationshipSchema; 8] {
        [
            RelationshipSchema::Peer,
            RelationshipSchema::Mentor,
            RelationshipSchema::Subordinate,
            RelationshipSchema::Romantic,
            RelationshipSchema::Family,
            RelationshipSchema::Nuclear,
            RelationshipSchema::Extended,
            RelationshipSchema::Rival,
        ]
    }
}

impl std::fmt::Display for RelationshipSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn relationship_schema_includes_nuclear_extended() {
        let all = RelationshipSchema::all();
        let set: HashSet<_> = all.iter().collect();

        assert!(set.contains(&RelationshipSchema::Nuclear));
        assert!(set.contains(&RelationshipSchema::Extended));
    }

    #[test]
    fn all_schemas() {
        let all = RelationshipSchema::all();
        assert_eq!(all.len(), 8);
    }

    #[test]
    fn hierarchical_schemas() {
        assert!(RelationshipSchema::Mentor.is_hierarchical());
        assert!(RelationshipSchema::Subordinate.is_hierarchical());

        assert!(!RelationshipSchema::Peer.is_hierarchical());
        assert!(!RelationshipSchema::Family.is_hierarchical());
        assert!(!RelationshipSchema::Romantic.is_hierarchical());
    }

    #[test]
    fn family_schemas() {
        assert!(RelationshipSchema::Family.is_family());
        assert!(RelationshipSchema::Nuclear.is_family());
        assert!(RelationshipSchema::Extended.is_family());

        assert!(!RelationshipSchema::Peer.is_family());
        assert!(!RelationshipSchema::Romantic.is_family());
    }

    #[test]
    fn display_format() {
        assert_eq!(format!("{}", RelationshipSchema::Peer), "Peer");
        assert_eq!(format!("{}", RelationshipSchema::Nuclear), "Nuclear Family");
    }

    #[test]
    fn default_is_peer() {
        assert_eq!(RelationshipSchema::default(), RelationshipSchema::Peer);
    }

    #[test]
    fn descriptions_not_empty() {
        for schema in RelationshipSchema::all() {
            assert!(!schema.description().is_empty());
        }
    }

    #[test]
    fn equality_and_hash() {
        assert_eq!(RelationshipSchema::Peer, RelationshipSchema::Peer);
        assert_ne!(RelationshipSchema::Peer, RelationshipSchema::Rival);

        let mut set = HashSet::new();
        set.insert(RelationshipSchema::Peer);
        set.insert(RelationshipSchema::Peer);
        assert_eq!(set.len(), 1);

        set.insert(RelationshipSchema::Family);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn copy_and_clone() {
        let original = RelationshipSchema::Nuclear;
        let copied = original;
        let cloned = original.clone();

        assert_eq!(original, copied);
        assert_eq!(original, cloned);
    }

    #[test]
    fn name_returns_correct_string() {
        assert_eq!(RelationshipSchema::Peer.name(), "Peer");
        assert_eq!(RelationshipSchema::Extended.name(), "Extended Family");
    }

    #[test]
    fn all_schemas_have_names() {
        for schema in RelationshipSchema::all() {
            assert!(!schema.name().is_empty());
        }
    }

    #[test]
    fn mentor_schema() {
        let schema = RelationshipSchema::Mentor;
        assert!(schema.is_hierarchical());
        assert!(!schema.is_family());
    }

    #[test]
    fn subordinate_schema() {
        let schema = RelationshipSchema::Subordinate;
        assert!(schema.is_hierarchical());
        assert!(!schema.is_family());
    }

    #[test]
    fn romantic_schema() {
        let schema = RelationshipSchema::Romantic;
        assert!(!schema.is_hierarchical());
        assert!(!schema.is_family());
    }

    #[test]
    fn rival_schema() {
        let schema = RelationshipSchema::Rival;
        assert!(!schema.is_hierarchical());
        assert!(!schema.is_family());
    }

    #[test]
    fn debug_format() {
        let schema = RelationshipSchema::Nuclear;
        let debug = format!("{:?}", schema);
        assert!(debug.contains("Nuclear"));
    }
}
