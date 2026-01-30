//! Type-safe identifier wrappers for domain entities.
//!
//! Each ID type is a newtype wrapper around String, providing type safety
//! to prevent mixing different kinds of identifiers.

use std::fmt;
use std::hash::Hash;

use serde::{Deserialize, Serialize};

/// Error returned when an ID cannot be created from invalid input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdError {
    /// The type of ID that failed to create.
    pub id_type: &'static str,
    /// Description of why the ID is invalid.
    pub reason: String,
}

impl fmt::Display for IdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid {}: {}", self.id_type, self.reason)
    }
}

impl std::error::Error for IdError {}

/// Macro to generate ID newtypes with common functionality.
macro_rules! define_id {
    (
        $(#[$meta:meta])*
        $name:ident, $type_name:literal
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name(String);

        impl $name {
            /// Creates a new ID from a string.
            ///
            /// # Errors
            ///
            /// Returns an error if the string is empty.
            pub fn new(id: impl Into<String>) -> Result<Self, IdError> {
                let id = id.into();
                if id.is_empty() {
                    return Err(IdError {
                        id_type: $type_name,
                        reason: "ID cannot be empty".to_string(),
                    });
                }
                Ok(Self(id))
            }

            /// Returns the ID as a string slice.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Consumes the ID and returns the inner String.
            #[must_use]
            pub fn into_string(self) -> String {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = IdError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = IdError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

define_id!(
    /// Unique identifier for an entity.
    ///
    /// Entities are living individuals that have psychological states.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::EntityId;
    ///
    /// let id = EntityId::new("person_001").unwrap();
    /// assert_eq!(id.as_str(), "person_001");
    /// ```
    EntityId,
    "EntityId"
);

define_id!(
    /// Unique identifier for an event.
    ///
    /// Events are occurrences that affect entity state.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::EventId;
    ///
    /// let id = EventId::new("event_042").unwrap();
    /// assert_eq!(id.as_str(), "event_042");
    /// ```
    EventId,
    "EventId"
);

define_id!(
    /// Unique identifier for a relationship.
    ///
    /// Relationships represent connections between two entities.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::RelationshipId;
    ///
    /// let id = RelationshipId::new("rel_001_002").unwrap();
    /// assert_eq!(id.as_str(), "rel_001_002");
    /// ```
    RelationshipId,
    "RelationshipId"
);

define_id!(
    /// Unique identifier for a memory.
    ///
    /// Memories are stored experiences that affect behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::MemoryId;
    ///
    /// let id = MemoryId::new("mem_12345").unwrap();
    /// assert_eq!(id.as_str(), "mem_12345");
    /// ```
    MemoryId,
    "MemoryId"
);

define_id!(
    /// Unique identifier for a group.
    ///
    /// Groups are collections of entities with shared context.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::GroupId;
    ///
    /// let id = GroupId::new("group_alpha").unwrap();
    /// assert_eq!(id.as_str(), "group_alpha");
    /// ```
    GroupId,
    "GroupId"
);

define_id!(
    /// Unique identifier for a microsystem context.
    ///
    /// Microsystems represent immediate environments where an entity
    /// interacts directly (e.g., family, work, school). This is used
    /// for context-dependent memory retrieval.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::MicrosystemId;
    ///
    /// let id = MicrosystemId::new("work_001").unwrap();
    /// assert_eq!(id.as_str(), "work_001");
    /// ```
    MicrosystemId,
    "MicrosystemId"
);

define_id!(
    /// Unique identifier for an event bus subscription.
    ///
    /// Subscriptions are used to filter and receive events from the EventBus.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::SubscriptionId;
    ///
    /// let id = SubscriptionId::new("sub_001").unwrap();
    /// assert_eq!(id.as_str(), "sub_001");
    /// ```
    SubscriptionId,
    "SubscriptionId"
);

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn entity_id_from_string() {
        let id = EntityId::new("person_001").unwrap();
        assert_eq!(id.as_str(), "person_001");
        assert_eq!(id.to_string(), "person_001");
    }

    #[test]
    fn entity_id_from_empty_string_returns_error() {
        let result = EntityId::new("");
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.id_type, "EntityId");
        assert!(err.reason.contains("empty"));
    }

    #[test]
    fn entity_id_equality() {
        let id1 = EntityId::new("person_001").unwrap();
        let id2 = EntityId::new("person_001").unwrap();
        let id3 = EntityId::new("person_002").unwrap();

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn entity_id_hash() {
        let id1 = EntityId::new("person_001").unwrap();
        let id2 = EntityId::new("person_001").unwrap();
        let id3 = EntityId::new("person_002").unwrap();

        let mut set = HashSet::new();
        set.insert(id1.clone());
        set.insert(id2); // Should be deduplicated

        assert_eq!(set.len(), 1);

        set.insert(id3);
        assert_eq!(set.len(), 2);

        assert!(set.contains(&id1));
    }

    #[test]
    fn event_id_works() {
        let id = EventId::new("event_042").unwrap();
        assert_eq!(id.as_str(), "event_042");

        let empty = EventId::new("");
        assert!(empty.is_err());
    }

    #[test]
    fn relationship_id_works() {
        let id = RelationshipId::new("rel_001_002").unwrap();
        assert_eq!(id.as_str(), "rel_001_002");

        let empty = RelationshipId::new("");
        assert!(empty.is_err());
    }

    #[test]
    fn memory_id_works() {
        let id = MemoryId::new("mem_12345").unwrap();
        assert_eq!(id.as_str(), "mem_12345");

        let empty = MemoryId::new("");
        assert!(empty.is_err());
    }

    #[test]
    fn group_id_works() {
        let id = GroupId::new("group_alpha").unwrap();
        assert_eq!(id.as_str(), "group_alpha");

        let empty = GroupId::new("");
        assert!(empty.is_err());
    }

    #[test]
    fn microsystem_id_works() {
        let id = MicrosystemId::new("work_001").unwrap();
        assert_eq!(id.as_str(), "work_001");

        let empty = MicrosystemId::new("");
        assert!(empty.is_err());
    }

    #[test]
    fn subscription_id_works() {
        let id = SubscriptionId::new("sub_001").unwrap();
        assert_eq!(id.as_str(), "sub_001");

        let empty = SubscriptionId::new("");
        assert!(empty.is_err());
    }

    #[test]
    fn try_from_string() {
        let id: EntityId = "person_001".to_string().try_into().unwrap();
        assert_eq!(id.as_str(), "person_001");

        let result: Result<EntityId, _> = "".to_string().try_into();
        assert!(result.is_err());
    }

    #[test]
    fn try_from_str() {
        let id: EntityId = "person_001".try_into().unwrap();
        assert_eq!(id.as_str(), "person_001");

        let result: Result<EntityId, _> = "".try_into();
        assert!(result.is_err());
    }

    #[test]
    fn into_string() {
        let id = EntityId::new("person_001").unwrap();
        let s: String = id.into_string();
        assert_eq!(s, "person_001");
    }

    #[test]
    fn as_ref_str() {
        let id = EntityId::new("person_001").unwrap();
        let s: &str = id.as_ref();
        assert_eq!(s, "person_001");
    }

    #[test]
    fn id_error_display() {
        let err = IdError {
            id_type: "TestId",
            reason: "test reason".to_string(),
        };
        let display = format!("{}", err);
        assert!(display.contains("TestId"));
        assert!(display.contains("test reason"));
    }

    #[test]
    fn clone_preserves_value() {
        let id1 = EntityId::new("person_001").unwrap();
        let id2 = id1.clone();
        assert_eq!(id1, id2);
    }

    #[test]
    fn debug_format() {
        let id = EntityId::new("person_001").unwrap();
        let debug = format!("{:?}", id);
        assert!(debug.contains("person_001"));
    }
}
