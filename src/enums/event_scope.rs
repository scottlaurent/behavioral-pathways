//! Event scope enum for broadcast targeting.
//!
//! Specifies the scope of event distribution when broadcasting
//! events to multiple entities.

use crate::types::{EntityId, GroupId, MicrosystemId};

/// Scope of event distribution for broadcasts.
///
/// When dispatching events, the scope determines which entities
/// receive the event. This is used for environmental/contextual
/// events that affect multiple entities.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::enums::EventScope;
/// use behavioral_pathways::types::{EntityId, GroupId, MicrosystemId};
///
/// // Event affects only one entity
/// let entity = EntityId::new("person_001").unwrap();
/// let individual = EventScope::Individual(entity);
///
/// // Event affects all entities in a group
/// let group = GroupId::new("team_alpha").unwrap();
/// let group_scope = EventScope::Group(group);
///
/// // Event affects all entities
/// let global = EventScope::Global;
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventScope {
    /// Event affects a single entity.
    Individual(EntityId),
    /// Event affects all entities in a group.
    Group(GroupId),
    /// Event affects all entities in a microsystem.
    Microsystem(MicrosystemId),
    /// Event affects all entities.
    Global,
}

impl EventScope {
    /// Creates an individual scope for the given entity.
    #[must_use]
    pub fn individual(entity_id: EntityId) -> Self {
        EventScope::Individual(entity_id)
    }

    /// Creates a group scope for the given group.
    #[must_use]
    pub fn group(group_id: GroupId) -> Self {
        EventScope::Group(group_id)
    }

    /// Creates a microsystem scope for the given microsystem.
    #[must_use]
    pub fn microsystem(microsystem_id: MicrosystemId) -> Self {
        EventScope::Microsystem(microsystem_id)
    }

    /// Creates a global scope.
    #[must_use]
    pub const fn global() -> Self {
        EventScope::Global
    }

    /// Returns true if this is an individual scope.
    #[must_use]
    pub const fn is_individual(&self) -> bool {
        matches!(self, EventScope::Individual(_))
    }

    /// Returns true if this is a group scope.
    #[must_use]
    pub const fn is_group(&self) -> bool {
        matches!(self, EventScope::Group(_))
    }

    /// Returns true if this is a microsystem scope.
    #[must_use]
    pub const fn is_microsystem(&self) -> bool {
        matches!(self, EventScope::Microsystem(_))
    }

    /// Returns true if this is a global scope.
    #[must_use]
    pub const fn is_global(&self) -> bool {
        matches!(self, EventScope::Global)
    }

    /// Returns the entity ID for individual scopes, or None otherwise.
    #[must_use]
    pub fn entity_id(&self) -> Option<&EntityId> {
        match self {
            EventScope::Individual(id) => Some(id),
            _ => None,
        }
    }

    /// Returns the group ID for group scopes, or None otherwise.
    #[must_use]
    pub fn group_id(&self) -> Option<&GroupId> {
        match self {
            EventScope::Group(id) => Some(id),
            _ => None,
        }
    }

    /// Returns the microsystem ID for microsystem scopes, or None otherwise.
    #[must_use]
    pub fn microsystem_id(&self) -> Option<&MicrosystemId> {
        match self {
            EventScope::Microsystem(id) => Some(id),
            _ => None,
        }
    }

    /// Returns a human-readable name for this scope type.
    #[must_use]
    pub const fn type_name(&self) -> &'static str {
        match self {
            EventScope::Individual(_) => "Individual",
            EventScope::Group(_) => "Group",
            EventScope::Microsystem(_) => "Microsystem",
            EventScope::Global => "Global",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_scope_individual_creation() {
        let entity = EntityId::new("person_001").unwrap();
        let scope = EventScope::individual(entity.clone());
        assert!(scope.is_individual());
        assert_eq!(scope.entity_id(), Some(&entity));
    }

    #[test]
    fn event_scope_group_creation() {
        let group = GroupId::new("team_alpha").unwrap();
        let scope = EventScope::group(group.clone());
        assert!(scope.is_group());
        assert_eq!(scope.group_id(), Some(&group));
    }

    #[test]
    fn event_scope_microsystem_creation() {
        let microsystem = MicrosystemId::new("work_001").unwrap();
        let scope = EventScope::microsystem(microsystem.clone());
        assert!(scope.is_microsystem());
        assert_eq!(scope.microsystem_id(), Some(&microsystem));
    }

    #[test]
    fn event_scope_global_creation() {
        let scope = EventScope::global();
        assert!(scope.is_global());
    }

    #[test]
    fn event_scope_type_names() {
        let entity = EntityId::new("person").unwrap();
        assert_eq!(EventScope::Individual(entity).type_name(), "Individual");

        let group = GroupId::new("group").unwrap();
        assert_eq!(EventScope::Group(group).type_name(), "Group");

        let micro = MicrosystemId::new("micro").unwrap();
        assert_eq!(EventScope::Microsystem(micro).type_name(), "Microsystem");

        assert_eq!(EventScope::Global.type_name(), "Global");
    }

    #[test]
    fn event_scope_entity_id_none_for_non_individual() {
        let group = GroupId::new("group").unwrap();
        assert!(EventScope::Group(group).entity_id().is_none());

        let micro = MicrosystemId::new("micro").unwrap();
        assert!(EventScope::Microsystem(micro).entity_id().is_none());

        assert!(EventScope::Global.entity_id().is_none());
    }

    #[test]
    fn event_scope_group_id_none_for_non_group() {
        let entity = EntityId::new("person").unwrap();
        assert!(EventScope::Individual(entity).group_id().is_none());

        let micro = MicrosystemId::new("micro").unwrap();
        assert!(EventScope::Microsystem(micro).group_id().is_none());

        assert!(EventScope::Global.group_id().is_none());
    }

    #[test]
    fn event_scope_microsystem_id_none_for_non_microsystem() {
        let entity = EntityId::new("person").unwrap();
        assert!(EventScope::Individual(entity).microsystem_id().is_none());

        let group = GroupId::new("group").unwrap();
        assert!(EventScope::Group(group).microsystem_id().is_none());

        assert!(EventScope::Global.microsystem_id().is_none());
    }

    #[test]
    fn event_scope_is_predicates() {
        let entity = EntityId::new("person").unwrap();
        let individual = EventScope::Individual(entity);
        assert!(individual.is_individual());
        assert!(!individual.is_group());
        assert!(!individual.is_microsystem());
        assert!(!individual.is_global());

        let group = GroupId::new("group").unwrap();
        let group_scope = EventScope::Group(group);
        assert!(!group_scope.is_individual());
        assert!(group_scope.is_group());
        assert!(!group_scope.is_microsystem());
        assert!(!group_scope.is_global());

        let micro = MicrosystemId::new("micro").unwrap();
        let micro_scope = EventScope::Microsystem(micro);
        assert!(!micro_scope.is_individual());
        assert!(!micro_scope.is_group());
        assert!(micro_scope.is_microsystem());
        assert!(!micro_scope.is_global());

        let global = EventScope::Global;
        assert!(!global.is_individual());
        assert!(!global.is_group());
        assert!(!global.is_microsystem());
        assert!(global.is_global());
    }

    #[test]
    fn event_scope_clone() {
        let entity = EntityId::new("person").unwrap();
        let scope = EventScope::Individual(entity);
        let cloned = scope.clone();
        assert_eq!(scope, cloned);
    }

    #[test]
    fn event_scope_debug_format() {
        let entity = EntityId::new("person").unwrap();
        let scope = EventScope::Individual(entity);
        let debug = format!("{:?}", scope);
        assert!(debug.contains("Individual"));
        assert!(debug.contains("person"));
    }

    #[test]
    fn event_scope_equality() {
        let entity1 = EntityId::new("person").unwrap();
        let entity2 = EntityId::new("person").unwrap();
        let entity3 = EntityId::new("other").unwrap();

        let scope1 = EventScope::Individual(entity1);
        let scope2 = EventScope::Individual(entity2);
        let scope3 = EventScope::Individual(entity3);

        assert_eq!(scope1, scope2);
        assert_ne!(scope1, scope3);
    }
}
