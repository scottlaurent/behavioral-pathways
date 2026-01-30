//! Delta structures recording changes applied when a memory was formed.
//!
//! These structures track what changes occurred to relationships and reputation
//! as a result of the event that created the memory.

use crate::types::EntityId;

/// Changes applied to a relationship as a result of an event.
///
/// Each field is optional, allowing partial updates. Only non-None fields
/// indicate actual changes that occurred.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::memory::RelationshipDelta;
/// use behavioral_pathways::types::EntityId;
///
/// let target = EntityId::new("entity_001").unwrap();
/// let delta = RelationshipDelta::new(target)
///     .with_trust_integrity(-0.15);
///
/// assert!((delta.trust_integrity().unwrap() - (-0.15)).abs() < f32::EPSILON);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct RelationshipDelta {
    /// The target entity whose relationship changed.
    target_entity: EntityId,

    /// Change to affinity.
    affinity: Option<f32>,

    /// Change to trust competence component.
    trust_competence: Option<f32>,

    /// Change to trust benevolence component.
    trust_benevolence: Option<f32>,

    /// Change to trust integrity component.
    trust_integrity: Option<f32>,

    /// Change to tension level.
    tension: Option<f32>,
}

impl RelationshipDelta {
    /// Creates a new relationship delta targeting the specified entity.
    ///
    /// All delta values start as None. Use builder methods to set them.
    ///
    /// # Arguments
    ///
    /// * `target_entity` - The entity whose relationship was affected
    #[must_use]
    pub fn new(target_entity: EntityId) -> Self {
        RelationshipDelta {
            target_entity,
            affinity: None,
            trust_competence: None,
            trust_benevolence: None,
            trust_integrity: None,
            tension: None,
        }
    }

    /// Sets the affinity delta.
    #[must_use]
    pub fn with_affinity(mut self, delta: f32) -> Self {
        self.affinity = Some(delta);
        self
    }

    /// Sets the trust competence delta.
    #[must_use]
    pub fn with_trust_competence(mut self, delta: f32) -> Self {
        self.trust_competence = Some(delta);
        self
    }

    /// Sets the trust benevolence delta.
    #[must_use]
    pub fn with_trust_benevolence(mut self, delta: f32) -> Self {
        self.trust_benevolence = Some(delta);
        self
    }

    /// Sets the trust integrity delta.
    #[must_use]
    pub fn with_trust_integrity(mut self, delta: f32) -> Self {
        self.trust_integrity = Some(delta);
        self
    }

    /// Sets the tension delta.
    #[must_use]
    pub fn with_tension(mut self, delta: f32) -> Self {
        self.tension = Some(delta);
        self
    }

    /// Returns the target entity.
    #[must_use]
    pub fn target_entity(&self) -> &EntityId {
        &self.target_entity
    }

    /// Returns the affinity delta, if set.
    #[must_use]
    pub fn affinity(&self) -> Option<f32> {
        self.affinity
    }

    /// Returns the trust competence delta, if set.
    #[must_use]
    pub fn trust_competence(&self) -> Option<f32> {
        self.trust_competence
    }

    /// Returns the trust benevolence delta, if set.
    #[must_use]
    pub fn trust_benevolence(&self) -> Option<f32> {
        self.trust_benevolence
    }

    /// Returns the trust integrity delta, if set.
    #[must_use]
    pub fn trust_integrity(&self) -> Option<f32> {
        self.trust_integrity
    }

    /// Returns the tension delta, if set.
    #[must_use]
    pub fn tension(&self) -> Option<f32> {
        self.tension
    }

    /// Returns true if any delta value is set.
    #[must_use]
    pub fn has_changes(&self) -> bool {
        self.affinity.is_some()
            || self.trust_competence.is_some()
            || self.trust_benevolence.is_some()
            || self.trust_integrity.is_some()
            || self.tension.is_some()
    }
}

/// Changes applied to reputation as a result of an event.
///
/// Each field is optional, allowing partial updates. Only non-None fields
/// indicate actual changes that occurred.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::memory::ReputationDelta;
///
/// let delta = ReputationDelta::new()
///     .with_feared(0.1)
///     .with_hated(0.05);
///
/// assert!(delta.feared().is_some());
/// assert!(delta.hated().is_some());
/// assert!(delta.trusted().is_none());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ReputationDelta {
    /// Change to trusted reputation.
    trusted: Option<f32>,

    /// Change to feared reputation.
    feared: Option<f32>,

    /// Change to hated reputation.
    hated: Option<f32>,
}

impl ReputationDelta {
    /// Creates a new reputation delta with all values unset.
    #[must_use]
    pub fn new() -> Self {
        ReputationDelta::default()
    }

    /// Sets the trusted delta.
    #[must_use]
    pub fn with_trusted(mut self, delta: f32) -> Self {
        self.trusted = Some(delta);
        self
    }

    /// Sets the feared delta.
    #[must_use]
    pub fn with_feared(mut self, delta: f32) -> Self {
        self.feared = Some(delta);
        self
    }

    /// Sets the hated delta.
    #[must_use]
    pub fn with_hated(mut self, delta: f32) -> Self {
        self.hated = Some(delta);
        self
    }

    /// Returns the trusted delta, if set.
    #[must_use]
    pub fn trusted(&self) -> Option<f32> {
        self.trusted
    }

    /// Returns the feared delta, if set.
    #[must_use]
    pub fn feared(&self) -> Option<f32> {
        self.feared
    }

    /// Returns the hated delta, if set.
    #[must_use]
    pub fn hated(&self) -> Option<f32> {
        self.hated
    }

    /// Returns true if any delta value is set.
    #[must_use]
    pub fn has_changes(&self) -> bool {
        self.trusted.is_some() || self.feared.is_some() || self.hated.is_some()
    }
}

/// Container for all deltas applied when a memory was formed.
///
/// This structure records what changes occurred as a result of the event
/// that created the memory.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::memory::{DeltasApplied, RelationshipDelta, ReputationDelta};
/// use behavioral_pathways::types::EntityId;
///
/// let target = EntityId::new("entity_001").unwrap();
/// let rel_delta = RelationshipDelta::new(target).with_trust_integrity(-0.15);
/// let deltas = DeltasApplied::new().with_relationship_delta(rel_delta);
///
/// assert!(deltas.relationship_delta().is_some());
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DeltasApplied {
    /// Changes to a relationship.
    relationship_delta: Option<RelationshipDelta>,

    /// Changes to reputation.
    reputation_delta: Option<ReputationDelta>,
}

impl DeltasApplied {
    /// Creates a new empty DeltasApplied.
    #[must_use]
    pub fn new() -> Self {
        DeltasApplied::default()
    }

    /// Sets the relationship delta.
    #[must_use]
    pub fn with_relationship_delta(mut self, delta: RelationshipDelta) -> Self {
        self.relationship_delta = Some(delta);
        self
    }

    /// Sets the reputation delta.
    #[must_use]
    pub fn with_reputation_delta(mut self, delta: ReputationDelta) -> Self {
        self.reputation_delta = Some(delta);
        self
    }

    /// Returns the relationship delta, if set.
    #[must_use]
    pub fn relationship_delta(&self) -> Option<&RelationshipDelta> {
        self.relationship_delta.as_ref()
    }

    /// Returns the reputation delta, if set.
    #[must_use]
    pub fn reputation_delta(&self) -> Option<&ReputationDelta> {
        self.reputation_delta.as_ref()
    }

    /// Returns true if any delta is set.
    #[must_use]
    pub fn has_changes(&self) -> bool {
        self.relationship_delta.is_some() || self.reputation_delta.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // RelationshipDelta tests

    #[test]
    fn relationship_delta_new_has_no_changes() {
        let target = EntityId::new("entity_001").unwrap();
        let delta = RelationshipDelta::new(target);

        assert!(!delta.has_changes());
        assert!(delta.affinity().is_none());
        assert!(delta.trust_competence().is_none());
        assert!(delta.trust_benevolence().is_none());
        assert!(delta.trust_integrity().is_none());
        assert!(delta.tension().is_none());
    }

    #[test]
    fn relationship_delta_builder_sets_values() {
        let target = EntityId::new("entity_001").unwrap();
        let delta = RelationshipDelta::new(target.clone())
            .with_affinity(0.1)
            .with_trust_competence(0.2)
            .with_trust_benevolence(0.3)
            .with_trust_integrity(-0.15)
            .with_tension(0.05);

        assert_eq!(delta.target_entity(), &target);
        assert!((delta.affinity().unwrap() - 0.1).abs() < f32::EPSILON);
        assert!((delta.trust_competence().unwrap() - 0.2).abs() < f32::EPSILON);
        assert!((delta.trust_benevolence().unwrap() - 0.3).abs() < f32::EPSILON);
        assert!((delta.trust_integrity().unwrap() - (-0.15)).abs() < f32::EPSILON);
        assert!((delta.tension().unwrap() - 0.05).abs() < f32::EPSILON);
    }

    #[test]
    fn relationship_delta_has_changes_when_any_set() {
        let target = EntityId::new("entity_001").unwrap();

        let delta_affinity = RelationshipDelta::new(target.clone()).with_affinity(0.1);
        assert!(delta_affinity.has_changes());

        let delta_trust_comp = RelationshipDelta::new(target.clone()).with_trust_competence(0.1);
        assert!(delta_trust_comp.has_changes());

        let delta_trust_ben = RelationshipDelta::new(target.clone()).with_trust_benevolence(0.1);
        assert!(delta_trust_ben.has_changes());

        let delta_trust_int = RelationshipDelta::new(target.clone()).with_trust_integrity(0.1);
        assert!(delta_trust_int.has_changes());

        let delta_tension = RelationshipDelta::new(target).with_tension(0.1);
        assert!(delta_tension.has_changes());
    }

    #[test]
    fn relationship_delta_clone() {
        let target = EntityId::new("entity_001").unwrap();
        let delta = RelationshipDelta::new(target).with_trust_integrity(-0.15);
        let cloned = delta.clone();
        assert_eq!(delta, cloned);
    }

    #[test]
    fn relationship_delta_debug() {
        let target = EntityId::new("entity_001").unwrap();
        let delta = RelationshipDelta::new(target);
        let debug = format!("{:?}", delta);
        assert!(debug.contains("RelationshipDelta"));
    }

    // ReputationDelta tests

    #[test]
    fn reputation_delta_new_has_no_changes() {
        let delta = ReputationDelta::new();

        assert!(!delta.has_changes());
        assert!(delta.trusted().is_none());
        assert!(delta.feared().is_none());
        assert!(delta.hated().is_none());
    }

    #[test]
    fn reputation_delta_builder_sets_values() {
        let delta = ReputationDelta::new()
            .with_trusted(0.1)
            .with_feared(0.2)
            .with_hated(0.3);

        assert!((delta.trusted().unwrap() - 0.1).abs() < f32::EPSILON);
        assert!((delta.feared().unwrap() - 0.2).abs() < f32::EPSILON);
        assert!((delta.hated().unwrap() - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn reputation_delta_has_changes_when_any_set() {
        let delta_trusted = ReputationDelta::new().with_trusted(0.1);
        assert!(delta_trusted.has_changes());

        let delta_feared = ReputationDelta::new().with_feared(0.1);
        assert!(delta_feared.has_changes());

        let delta_hated = ReputationDelta::new().with_hated(0.1);
        assert!(delta_hated.has_changes());
    }

    #[test]
    fn reputation_delta_default() {
        let delta = ReputationDelta::default();
        assert!(!delta.has_changes());
    }

    #[test]
    fn reputation_delta_clone_and_copy() {
        let delta = ReputationDelta::new().with_feared(0.1);
        let cloned = delta.clone();
        let copied = delta;
        assert_eq!(delta, cloned);
        assert_eq!(delta, copied);
    }

    #[test]
    fn reputation_delta_debug() {
        let delta = ReputationDelta::new();
        let debug = format!("{:?}", delta);
        assert!(debug.contains("ReputationDelta"));
    }

    // DeltasApplied tests

    #[test]
    fn deltas_applied_new_is_empty() {
        let deltas = DeltasApplied::new();

        assert!(!deltas.has_changes());
        assert!(deltas.relationship_delta().is_none());
        assert!(deltas.reputation_delta().is_none());
    }

    #[test]
    fn deltas_applied_builder_sets_values() {
        let target = EntityId::new("entity_001").unwrap();
        let rel_delta = RelationshipDelta::new(target).with_trust_integrity(-0.15);
        let rep_delta = ReputationDelta::new().with_feared(0.1);

        let deltas = DeltasApplied::new()
            .with_relationship_delta(rel_delta)
            .with_reputation_delta(rep_delta);

        assert!(deltas.relationship_delta().is_some());
        assert!(deltas.reputation_delta().is_some());
    }

    #[test]
    fn deltas_applied_has_changes_when_relationship_set() {
        let target = EntityId::new("entity_001").unwrap();
        let rel_delta = RelationshipDelta::new(target);
        let deltas = DeltasApplied::new().with_relationship_delta(rel_delta);

        assert!(deltas.has_changes());
    }

    #[test]
    fn deltas_applied_has_changes_when_reputation_set() {
        let rep_delta = ReputationDelta::new();
        let deltas = DeltasApplied::new().with_reputation_delta(rep_delta);

        assert!(deltas.has_changes());
    }

    #[test]
    fn deltas_applied_default() {
        let deltas = DeltasApplied::default();
        assert!(!deltas.has_changes());
    }

    #[test]
    fn deltas_applied_clone() {
        let target = EntityId::new("entity_001").unwrap();
        let rel_delta = RelationshipDelta::new(target);
        let deltas = DeltasApplied::new().with_relationship_delta(rel_delta);
        let cloned = deltas.clone();
        assert_eq!(deltas, cloned);
    }

    #[test]
    fn deltas_applied_debug() {
        let deltas = DeltasApplied::new();
        let debug = format!("{:?}", deltas);
        assert!(debug.contains("DeltasApplied"));
    }
}
