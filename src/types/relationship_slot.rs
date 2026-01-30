//! Relationship slot for entity relationship hooks.
//!
//! This type represents a slot where a relationship can be attached
//! to an entity. Phase 3 provides the basic slot API, and Phase 5
//! expands with full relationship functionality.

use crate::types::RelationshipId;

/// Slot for attaching relationships to an entity.
///
/// Each slot can hold a reference to a single relationship. Slots are
/// initially unattached and can be attached to a relationship ID.
///
/// # Phase 3 API
///
/// - `is_empty()` - Returns true if no relationship attached
/// - `is_attached()` - Returns true if a relationship is attached
/// - `get_attached()` - Returns the attached relationship ID, if any
///
/// # Phase 5 Extensions
///
/// Phase 5 adds internal mutation methods (`attach`, `detach`) for
/// relationship management.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::types::RelationshipSlot;
///
/// let slot = RelationshipSlot::new();
/// assert!(slot.is_empty());
/// assert!(!slot.is_attached());
/// assert!(slot.get_attached().is_none());
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RelationshipSlot {
    /// The attached relationship ID, if any.
    attached: Option<RelationshipId>,
}

impl RelationshipSlot {
    /// Creates a new empty relationship slot.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::RelationshipSlot;
    ///
    /// let slot = RelationshipSlot::new();
    /// assert!(slot.is_empty());
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        RelationshipSlot { attached: None }
    }

    /// Returns true if this slot is empty (no relationship attached).
    ///
    /// This is the inverse of `is_attached()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::RelationshipSlot;
    ///
    /// let slot = RelationshipSlot::new();
    /// assert!(slot.is_empty());
    /// ```
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.attached.is_none()
    }

    /// Returns true if a relationship is attached to this slot.
    ///
    /// This is the inverse of `is_empty()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::RelationshipSlot;
    ///
    /// let slot = RelationshipSlot::new();
    /// assert!(!slot.is_attached());
    /// ```
    #[must_use]
    pub const fn is_attached(&self) -> bool {
        self.attached.is_some()
    }

    /// Returns the attached relationship ID, if any.
    ///
    /// Returns `None` if the slot is empty. Returns an owned copy of the ID
    /// to avoid lifetime complications in external code.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::RelationshipSlot;
    ///
    /// let slot = RelationshipSlot::new();
    /// assert!(slot.get_attached().is_none());
    /// ```
    #[must_use]
    pub fn get_attached(&self) -> Option<RelationshipId> {
        self.attached.clone()
    }

    /// Attaches a relationship to this slot.
    ///
    /// This is a crate-internal method used by the relationship system
    /// in Phase 5. External code should not call this directly.
    ///
    /// If a relationship is already attached, it is replaced.
    #[allow(dead_code)]
    pub(crate) fn attach(&mut self, id: RelationshipId) {
        self.attached = Some(id);
    }

    /// Detaches the relationship from this slot.
    ///
    /// This is a crate-internal method used by the relationship system
    /// in Phase 5. External code should not call this directly.
    ///
    /// Returns the previously attached ID, if any.
    #[allow(dead_code)]
    pub(crate) fn detach(&mut self) -> Option<RelationshipId> {
        self.attached.take()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relationship_slot_is_empty_when_new() {
        let slot = RelationshipSlot::new();
        assert!(slot.is_empty());
        assert!(!slot.is_attached());

        // Verify default also works
        let default_slot = RelationshipSlot::default();
        assert!(default_slot.is_empty());
        assert!(!default_slot.is_attached());
    }

    #[test]
    fn get_attached_returns_none_when_empty() {
        let slot = RelationshipSlot::new();
        assert!(slot.get_attached().is_none());
    }

    #[test]
    fn attach_makes_slot_attached() {
        let mut slot = RelationshipSlot::new();
        let id = RelationshipId::new("rel_001").unwrap();

        slot.attach(id.clone());

        assert!(slot.is_attached());
        assert!(!slot.is_empty());
        assert_eq!(slot.get_attached(), Some(id));
    }

    #[test]
    fn detach_makes_slot_empty() {
        let mut slot = RelationshipSlot::new();
        let id = RelationshipId::new("rel_002").unwrap();

        slot.attach(id.clone());
        assert!(slot.is_attached());

        let detached = slot.detach();

        assert!(slot.is_empty());
        assert!(!slot.is_attached());
        assert!(slot.get_attached().is_none());
        assert_eq!(detached, Some(id));
    }

    #[test]
    fn detach_empty_slot_returns_none() {
        let mut slot = RelationshipSlot::new();
        let detached = slot.detach();

        assert!(detached.is_none());
        assert!(slot.is_empty());
    }

    #[test]
    fn attach_replaces_existing() {
        let mut slot = RelationshipSlot::new();
        let id1 = RelationshipId::new("rel_001").unwrap();
        let id2 = RelationshipId::new("rel_002").unwrap();

        slot.attach(id1);
        slot.attach(id2.clone());

        assert_eq!(slot.get_attached(), Some(id2));
    }

    #[test]
    fn relationship_slot_equality() {
        let slot1 = RelationshipSlot::new();
        let slot2 = RelationshipSlot::new();
        assert_eq!(slot1, slot2);

        let mut slot3 = RelationshipSlot::new();
        slot3.attach(RelationshipId::new("rel_001").unwrap());

        assert_ne!(slot1, slot3);
    }

    #[test]
    fn relationship_slot_clone() {
        let mut original = RelationshipSlot::new();
        original.attach(RelationshipId::new("rel_abc").unwrap());

        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(original.get_attached(), cloned.get_attached());
    }

    #[test]
    fn relationship_slot_debug() {
        let slot = RelationshipSlot::new();
        let debug = format!("{:?}", slot);
        assert!(debug.contains("RelationshipSlot"));
    }

    #[test]
    fn is_attached_and_is_empty_are_inverses() {
        let slot = RelationshipSlot::new();
        assert_eq!(slot.is_attached(), !slot.is_empty());

        let mut attached_slot = RelationshipSlot::new();
        attached_slot.attach(RelationshipId::new("rel_xyz").unwrap());
        assert_eq!(attached_slot.is_attached(), !attached_slot.is_empty());
    }
}
