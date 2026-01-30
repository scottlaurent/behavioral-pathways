//! Core Simulation container for timestamp-based state queries.
//!
//! The Simulation struct is the primary container for the consumer API.
//! It holds entities, events, and relationships with their timestamps,
//! enabling state queries at any point in time.

use crate::entity::Entity;
use crate::enums::RelationshipSchema;
use crate::event::Event;
use crate::processor::process_event_to_relationships;
use crate::relationship::Relationship;
use crate::simulation::state_query::EntityQueryHandle;
use crate::types::{EntityId, RelationshipId, Timestamp};
use std::collections::HashMap;

/// An entity with its anchor timestamp.
///
/// The anchor timestamp represents when this entity's state was observed.
/// All state queries compute relative to this anchor point.
#[derive(Debug, Clone)]
pub struct AnchoredEntity {
    /// The entity instance.
    entity: Entity,
    /// When this state snapshot was captured.
    anchor_timestamp: Timestamp,
}

impl AnchoredEntity {
    /// Creates a new anchored entity.
    #[must_use]
    pub fn new(entity: Entity, anchor_timestamp: Timestamp) -> Self {
        AnchoredEntity {
            entity,
            anchor_timestamp,
        }
    }

    /// Returns a reference to the entity.
    #[must_use]
    pub fn entity(&self) -> &Entity {
        &self.entity
    }

    /// Returns a mutable reference to the entity.
    pub fn entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }

    /// Returns the anchor timestamp.
    #[must_use]
    pub fn anchor_timestamp(&self) -> Timestamp {
        self.anchor_timestamp
    }
}

/// An event with its absolute timestamp.
#[derive(Debug, Clone)]
pub struct TimestampedEvent {
    /// The event instance.
    event: Event,
    /// When this event occurred.
    timestamp: Timestamp,
}

impl TimestampedEvent {
    /// Creates a new timestamped event.
    #[must_use]
    pub fn new(event: Event, timestamp: Timestamp) -> Self {
        TimestampedEvent { event, timestamp }
    }

    /// Returns a reference to the event.
    #[must_use]
    pub fn event(&self) -> &Event {
        &self.event
    }

    /// Returns the timestamp when this event occurred.
    #[must_use]
    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }
}

/// A relationship with its formation timestamp.
#[derive(Debug, Clone)]
pub struct TimestampedRelationship {
    /// The relationship instance.
    relationship: Relationship,
    /// Entity A's ID.
    entity_a: EntityId,
    /// Entity B's ID.
    entity_b: EntityId,
    /// When the relationship was formed.
    formed_timestamp: Timestamp,
}

impl TimestampedRelationship {
    /// Creates a new timestamped relationship.
    #[must_use]
    pub fn new(
        relationship: Relationship,
        entity_a: EntityId,
        entity_b: EntityId,
        formed_timestamp: Timestamp,
    ) -> Self {
        TimestampedRelationship {
            relationship,
            entity_a,
            entity_b,
            formed_timestamp,
        }
    }

    /// Returns a reference to the relationship.
    #[must_use]
    pub fn relationship(&self) -> &Relationship {
        &self.relationship
    }

    /// Returns a mutable reference to the relationship.
    pub fn relationship_mut(&mut self) -> &mut Relationship {
        &mut self.relationship
    }

    /// Returns Entity A's ID.
    #[must_use]
    pub fn entity_a(&self) -> &EntityId {
        &self.entity_a
    }

    /// Returns Entity B's ID.
    #[must_use]
    pub fn entity_b(&self) -> &EntityId {
        &self.entity_b
    }

    /// Returns the formation timestamp.
    #[must_use]
    pub fn formed_timestamp(&self) -> Timestamp {
        self.formed_timestamp
    }

    /// Returns true if the relationship involves the given entity.
    #[must_use]
    pub fn involves(&self, entity_id: &EntityId) -> bool {
        &self.entity_a == entity_id || &self.entity_b == entity_id
    }
}

/// Quality indicator for backward regression.
///
/// Some state computations cannot be exactly reversed:
/// - Events that triggered feedback loops (spirals)
/// - Events that modified non-reversible dimensions
///
/// This enum indicates whether the regression was mathematically exact
/// or an approximation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RegressionQuality {
    /// Regression is mathematically exact.
    #[default]
    Exact,
    /// Regression is an approximation (e.g., through spiral events).
    Approximate,
}

impl RegressionQuality {
    /// Returns true if the regression was exact.
    #[must_use]
    pub fn is_exact(&self) -> bool {
        matches!(self, RegressionQuality::Exact)
    }

    /// Returns true if the regression was approximate.
    #[must_use]
    pub fn is_approximate(&self) -> bool {
        matches!(self, RegressionQuality::Approximate)
    }
}

/// The main simulation container.
///
/// Holds entities, events, and relationships with their timestamps.
/// Provides the `state_at()` API for querying entity state at any timestamp.
#[derive(Debug, Clone)]
pub struct Simulation {
    /// The simulation's reference date.
    reference_date: Timestamp,
    /// Entities indexed by their ID.
    entities: HashMap<EntityId, AnchoredEntity>,
    /// Events in the simulation.
    events: Vec<TimestampedEvent>,
    /// Relationships indexed by their ID.
    relationships: HashMap<RelationshipId, TimestampedRelationship>,
    /// Counter for generating relationship IDs.
    relationship_counter: u64,
}

impl Simulation {
    /// Creates a new simulation with the given reference date.
    ///
    /// The reference date serves as the simulation's temporal anchor point.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::simulation::Simulation;
    /// use behavioral_pathways::types::Timestamp;
    ///
    /// let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    /// let sim = Simulation::new(reference);
    ///
    /// assert_eq!(sim.entity_count(), 0);
    /// ```
    #[must_use]
    pub fn new(reference_date: Timestamp) -> Self {
        Simulation {
            reference_date,
            entities: HashMap::new(),
            events: Vec::new(),
            relationships: HashMap::new(),
            relationship_counter: 0,
        }
    }

    /// Returns the simulation's reference date.
    #[must_use]
    pub fn reference_date(&self) -> Timestamp {
        self.reference_date
    }

    // --- Entity Management ---

    /// Adds an entity to the simulation with its anchor timestamp.
    ///
    /// The anchor timestamp represents when the entity's state was observed.
    /// All state queries compute relative to this anchor point.
    ///
    /// # Returns
    ///
    /// The entity's ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::simulation::Simulation;
    /// use behavioral_pathways::entity::EntityBuilder;
    /// use behavioral_pathways::types::Timestamp;
    /// use behavioral_pathways::enums::Species;
    ///
    /// let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    /// let mut sim = Simulation::new(reference);
    ///
    /// let entity = EntityBuilder::new()
    ///     .id("person_001")
    ///     .species(Species::Human)
    ///     .build()
    ///     .unwrap();
    ///
    /// let anchor = Timestamp::from_ymd_hms(2024, 1, 1, 9, 0, 0);
    /// let id = sim.add_entity(entity, anchor);
    ///
    /// assert_eq!(sim.entity_count(), 1);
    /// ```
    pub fn add_entity(&mut self, entity: Entity, anchor_timestamp: Timestamp) -> EntityId {
        let id = entity.id().clone();
        let anchored = AnchoredEntity::new(entity, anchor_timestamp);
        self.entities.insert(id.clone(), anchored);
        id
    }

    /// Returns a query handle for the given entity ID.
    ///
    /// The handle provides the `state_at()` method for querying state.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::simulation::Simulation;
    /// use behavioral_pathways::entity::EntityBuilder;
    /// use behavioral_pathways::types::{Timestamp, EntityId};
    /// use behavioral_pathways::enums::Species;
    ///
    /// let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    /// let mut sim = Simulation::new(reference);
    ///
    /// let entity = EntityBuilder::new()
    ///     .id("person_001")
    ///     .species(Species::Human)
    ///     .build()
    ///     .unwrap();
    ///
    /// sim.add_entity(entity, reference);
    ///
    /// let handle = sim.entity(&EntityId::new("person_001").unwrap());
    /// assert!(handle.is_some());
    ///
    /// let unknown = sim.entity(&EntityId::new("unknown").unwrap());
    /// assert!(unknown.is_none());
    /// ```
    #[must_use]
    pub fn entity(&self, id: &EntityId) -> Option<EntityQueryHandle<'_>> {
        if self.entities.contains_key(id) {
            Some(EntityQueryHandle::new(self, id.clone()))
        } else {
            None
        }
    }

    /// Returns an iterator over all anchored entities.
    pub fn entities(&self) -> impl Iterator<Item = &AnchoredEntity> {
        self.entities.values()
    }

    /// Returns the number of entities in the simulation.
    #[must_use]
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    /// Returns the anchored entity for the given ID.
    #[must_use]
    pub fn get_anchored_entity(&self, id: &EntityId) -> Option<&AnchoredEntity> {
        self.entities.get(id)
    }

    /// Returns a mutable reference to the anchored entity.
    pub fn get_anchored_entity_mut(&mut self, id: &EntityId) -> Option<&mut AnchoredEntity> {
        self.entities.get_mut(id)
    }

    // --- Event Management ---

    /// Adds an event to the simulation with its timestamp.
    ///
    /// Events are applied during state computation when their timestamp
    /// falls within the query range.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::simulation::Simulation;
    /// use behavioral_pathways::event::EventBuilder;
    /// use behavioral_pathways::types::{Timestamp, EntityId};
    /// use behavioral_pathways::enums::EventType;
    ///
    /// let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    /// let mut sim = Simulation::new(reference);
    ///
    /// let target = EntityId::new("person_001").unwrap();
    /// let event = EventBuilder::new(EventType::SocialExclusion)
    ///     .target(target.clone())
    ///     .severity(0.7)
    ///     .build()
    ///     .unwrap();
    ///
    /// let event_time = Timestamp::from_ymd_hms(2024, 1, 15, 14, 0, 0);
    /// sim.add_event(event, event_time);
    /// ```
    pub fn add_event(&mut self, event: Event, timestamp: Timestamp) {
        self.events.push(TimestampedEvent::new(event, timestamp));

        let last_event = self
            .events
            .last()
            .expect("event just pushed should be present");
        for relationship in self.relationships.values_mut() {
            if last_event.timestamp() < relationship.formed_timestamp() {
                continue;
            }
            let rel_slice = std::slice::from_mut(relationship.relationship_mut());
            process_event_to_relationships(last_event.event(), last_event.timestamp(), rel_slice);
        }
    }

    /// Returns all events that target the given entity.
    ///
    /// Events are returned in no particular order. Use `events_between`
    /// for time-range queries.
    #[must_use]
    pub fn events_for(&self, entity_id: &EntityId) -> Vec<&TimestampedEvent> {
        self.events
            .iter()
            .filter(|te| te.event.target() == Some(entity_id))
            .collect()
    }

    /// Returns all events between the start and end timestamps (inclusive).
    ///
    /// Events are returned in no particular order.
    #[must_use]
    pub fn events_between(&self, start: Timestamp, end: Timestamp) -> Vec<&TimestampedEvent> {
        self.events
            .iter()
            .filter(|te| te.timestamp >= start && te.timestamp <= end)
            .collect()
    }

    /// Returns all events in the simulation.
    pub fn all_events(&self) -> impl Iterator<Item = &TimestampedEvent> {
        self.events.iter()
    }

    // --- Relationship Management ---

    fn resolve_schema_constraints(
        &self,
        entity_a: &EntityId,
        entity_b: &EntityId,
        requested: RelationshipSchema,
    ) -> RelationshipSchema {
        let (Some(entity_a), Some(entity_b)) =
            (self.entities.get(entity_a), self.entities.get(entity_b))
        else {
            return requested;
        };

        let constraints_a = entity_a.entity().context().macrosystem().constraint_set();
        let constraints_b = entity_b.entity().context().macrosystem().constraint_set();

        if constraints_a.allows_schema(requested) && constraints_b.allows_schema(requested) {
            return requested;
        }

        constraints_a
            .allowed_relationship_schemas
            .iter()
            .copied()
            .find(|schema| constraints_b.allows_schema(*schema))
            .unwrap_or(RelationshipSchema::default())
    }

    /// Adds a relationship between two entities.
    ///
    /// # Returns
    ///
    /// The relationship's ID.
    ///
    /// # Panics
    ///
    /// Panics if a relationship between an entity and itself is attempted.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::simulation::Simulation;
    /// use behavioral_pathways::types::{Timestamp, EntityId};
    /// use behavioral_pathways::enums::RelationshipSchema;
    ///
    /// let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    /// let mut sim = Simulation::new(reference);
    ///
    /// let alice = EntityId::new("alice").unwrap();
    /// let bob = EntityId::new("bob").unwrap();
    ///
    /// let formed = Timestamp::from_ymd_hms(2024, 1, 5, 0, 0, 0);
    /// let rel_id = sim.add_relationship(alice, bob, RelationshipSchema::Peer, formed);
    /// ```
    pub fn add_relationship(
        &mut self,
        entity_a: EntityId,
        entity_b: EntityId,
        schema: RelationshipSchema,
        formed_timestamp: Timestamp,
    ) -> RelationshipId {
        assert!(
            entity_a != entity_b,
            "Cannot create relationship between an entity and itself"
        );

        let schema = self.resolve_schema_constraints(&entity_a, &entity_b, schema);

        // Create the relationship
        let relationship = Relationship::try_between(entity_a.clone(), entity_b.clone())
            .expect("Failed to create relationship")
            .with_schema(schema);

        // Generate ID
        self.relationship_counter += 1;
        let rel_id =
            RelationshipId::new(format!("rel_{:016x}", self.relationship_counter)).unwrap();

        let timestamped =
            TimestampedRelationship::new(relationship, entity_a, entity_b, formed_timestamp);

        self.relationships.insert(rel_id.clone(), timestamped);
        rel_id
    }

    /// Returns all relationships involving the given entity.
    #[must_use]
    pub fn relationships_for(&self, entity_id: &EntityId) -> Vec<&TimestampedRelationship> {
        self.relationships
            .values()
            .filter(|tr| tr.involves(entity_id))
            .collect()
    }

    /// Returns the relationship with the given ID.
    #[must_use]
    pub fn get_relationship(&self, id: &RelationshipId) -> Option<&TimestampedRelationship> {
        self.relationships.get(id)
    }

    /// Returns the number of relationships.
    #[must_use]
    pub fn relationship_count(&self) -> usize {
        self.relationships.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::EntityBuilder;
    use crate::enums::{Direction, EventType, Species};
    use crate::event::EventBuilder;
    use crate::types::Duration;

    fn create_simulation() -> Simulation {
        let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        Simulation::new(reference)
    }

    fn create_human(id: &str) -> Entity {
        EntityBuilder::new()
            .id(id)
            .species(Species::Human)
            .build()
            .unwrap()
    }

    #[test]
    fn simulation_new_with_reference_date() {
        let reference = Timestamp::from_ymd_hms(2024, 6, 15, 12, 0, 0);
        let sim = Simulation::new(reference);

        assert_eq!(sim.reference_date(), reference);
        assert_eq!(sim.entity_count(), 0);
    }

    #[test]
    fn simulation_add_entity_with_anchor() {
        let mut sim = create_simulation();
        let entity = create_human("person_001");
        let anchor = Timestamp::from_ymd_hms(2024, 1, 1, 9, 0, 0);

        let id = sim.add_entity(entity, anchor);

        assert_eq!(id.as_str(), "person_001");
        assert_eq!(sim.entity_count(), 1);
    }

    #[test]
    fn simulation_entity_returns_handle() {
        let mut sim = create_simulation();
        let entity = create_human("person_001");
        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        let handle = sim.entity(&EntityId::new("person_001").unwrap());
        assert!(handle.is_some());
    }

    #[test]
    fn simulation_entity_unknown_returns_none() {
        let sim = create_simulation();

        let handle = sim.entity(&EntityId::new("unknown").unwrap());
        assert!(handle.is_none());
    }

    #[test]
    fn simulation_add_event_with_timestamp() {
        let mut sim = create_simulation();
        let target = EntityId::new("person_001").unwrap();

        let event = EventBuilder::new(EventType::SocialExclusion)
            .target(target.clone())
            .severity(0.7)
            .build()
            .unwrap();

        let event_time = Timestamp::from_ymd_hms(2024, 1, 15, 14, 0, 0);
        sim.add_event(event, event_time);

        let events = sim.events_for(&target);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].timestamp(), event_time);
    }

    #[test]
    fn simulation_add_event_updates_relationships() {
        let mut sim = create_simulation();
        let alice = EntityId::new("alice").unwrap();
        let bob = EntityId::new("bob").unwrap();

        let formed = sim.reference_date();
        let rel_id =
            sim.add_relationship(alice.clone(), bob.clone(), RelationshipSchema::Peer, formed);

        let event = EventBuilder::new(EventType::Support)
            .source(alice)
            .target(bob.clone())
            .severity(1.0)
            .build()
            .unwrap();

        let event_time = formed + Duration::days(1);
        sim.add_event(event, event_time);

        let relationship = sim.get_relationship(&rel_id).unwrap();
        let history = relationship.relationship().antecedent_history(Direction::BToA);
        assert!(!history.is_empty());
    }

    #[test]
    fn simulation_add_event_before_relationship_formed_is_ignored() {
        let mut sim = create_simulation();
        let alice = EntityId::new("alice").unwrap();
        let bob = EntityId::new("bob").unwrap();

        let formed = sim.reference_date() + Duration::days(10);
        let rel_id =
            sim.add_relationship(alice.clone(), bob.clone(), RelationshipSchema::Peer, formed);

        let event = EventBuilder::new(EventType::Support)
            .source(alice)
            .target(bob.clone())
            .severity(1.0)
            .build()
            .unwrap();

        let event_time = sim.reference_date() + Duration::days(1);
        sim.add_event(event, event_time);

        let relationship = sim.get_relationship(&rel_id).unwrap();
        let history = relationship.relationship().antecedent_history(Direction::BToA);
        assert!(history.is_empty());
    }

    #[test]
    fn simulation_add_relationship_respects_macrosystem_constraints() {
        let mut sim = create_simulation();
        let anchor = sim.reference_date();

        let mut alice = create_human("alice");
        alice
            .context_mut()
            .macrosystem_mut()
            .cultural_orientation
            .power_distance = 0.9;
        let mut bob = create_human("bob");
        bob.context_mut()
            .macrosystem_mut()
            .cultural_orientation
            .power_distance = 0.9;

        let alice_id = sim.add_entity(alice, anchor);
        let bob_id = sim.add_entity(bob, anchor);

        let rel_id = sim.add_relationship(
            alice_id.clone(),
            bob_id.clone(),
            RelationshipSchema::Romantic,
            anchor,
        );
        let relationship = sim.get_relationship(&rel_id).unwrap();
        let schema = relationship.relationship().schema();

        let constraints_a = sim
            .entities
            .get(&alice_id)
            .unwrap()
            .entity()
            .context()
            .macrosystem()
            .constraint_set();
        let constraints_b = sim
            .entities
            .get(&bob_id)
            .unwrap()
            .entity()
            .context()
            .macrosystem()
            .constraint_set();

        assert!(constraints_a.allows_schema(schema));
        assert!(constraints_b.allows_schema(schema));
    }

    #[test]
    fn simulation_events_for_entity_filters_correctly() {
        let mut sim = create_simulation();
        let person1 = EntityId::new("person_001").unwrap();
        let person2 = EntityId::new("person_002").unwrap();

        // Event for person1
        let event1 = EventBuilder::new(EventType::SocialExclusion)
            .target(person1.clone())
            .build()
            .unwrap();
        sim.add_event(event1, Timestamp::from_ymd_hms(2024, 1, 10, 0, 0, 0));

        // Event for person2
        let event2 = EventBuilder::new(EventType::SocialInclusion)
            .target(person2.clone())
            .build()
            .unwrap();
        sim.add_event(event2, Timestamp::from_ymd_hms(2024, 1, 11, 0, 0, 0));

        // Event for person1
        let event3 = EventBuilder::new(EventType::Achievement)
            .target(person1.clone())
            .build()
            .unwrap();
        sim.add_event(event3, Timestamp::from_ymd_hms(2024, 1, 12, 0, 0, 0));

        let person1_events = sim.events_for(&person1);
        assert_eq!(person1_events.len(), 2);

        let person2_events = sim.events_for(&person2);
        assert_eq!(person2_events.len(), 1);
    }

    #[test]
    fn simulation_events_between_filters_range() {
        let mut sim = create_simulation();
        let target = EntityId::new("person_001").unwrap();

        // Add events at different times
        for day in [5, 10, 15, 20, 25] {
            let event = EventBuilder::new(EventType::Interaction)
                .target(target.clone())
                .build()
                .unwrap();
            sim.add_event(event, Timestamp::from_ymd_hms(2024, 1, day, 12, 0, 0));
        }

        let start = Timestamp::from_ymd_hms(2024, 1, 10, 0, 0, 0);
        let end = Timestamp::from_ymd_hms(2024, 1, 20, 23, 59, 59);

        let events = sim.events_between(start, end);
        assert_eq!(events.len(), 3); // days 10, 15, 20
    }

    #[test]
    fn simulation_add_relationship_with_timestamp() {
        let mut sim = create_simulation();
        let alice = EntityId::new("alice").unwrap();
        let bob = EntityId::new("bob").unwrap();

        let formed = Timestamp::from_ymd_hms(2024, 1, 5, 0, 0, 0);
        let rel_id =
            sim.add_relationship(alice.clone(), bob.clone(), RelationshipSchema::Peer, formed);

        assert!(rel_id.as_str().starts_with("rel_"));
        assert_eq!(sim.relationship_count(), 1);

        let rels = sim.relationships_for(&alice);
        assert_eq!(rels.len(), 1);
        assert_eq!(rels[0].formed_timestamp(), formed);
        // Also test the relationship() accessor
        let _ = rels[0].relationship();
    }

    #[test]
    #[should_panic(expected = "Cannot create relationship between an entity and itself")]
    fn simulation_add_relationship_self_panics() {
        let mut sim = create_simulation();
        let alice = EntityId::new("alice").unwrap();

        sim.add_relationship(
            alice.clone(),
            alice,
            RelationshipSchema::Peer,
            sim.reference_date(),
        );
    }

    #[test]
    fn anchored_entity_accessors() {
        let entity = create_human("test");
        let anchor = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);

        let anchored = AnchoredEntity::new(entity, anchor);

        assert_eq!(anchored.entity().id().as_str(), "test");
        assert_eq!(anchored.anchor_timestamp(), anchor);
    }

    #[test]
    fn anchored_entity_mutable() {
        let entity = create_human("test");
        let anchor = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);

        let mut anchored = AnchoredEntity::new(entity, anchor);
        let _ = anchored.entity_mut();
    }

    #[test]
    fn timestamped_event_accessors() {
        let event = EventBuilder::new(EventType::Interaction)
            .severity(0.5)
            .build()
            .unwrap();
        let ts = Timestamp::from_ymd_hms(2024, 1, 15, 14, 30, 0);

        let te = TimestampedEvent::new(event.clone(), ts);

        assert_eq!(te.event().severity(), 0.5);
        assert_eq!(te.timestamp(), ts);
    }

    #[test]
    fn timestamped_relationship_accessors() {
        let alice = EntityId::new("alice").unwrap();
        let bob = EntityId::new("bob").unwrap();
        let relationship = Relationship::try_between(alice.clone(), bob.clone()).unwrap();
        let formed = Timestamp::from_ymd_hms(2024, 1, 5, 0, 0, 0);

        let tr = TimestampedRelationship::new(relationship, alice.clone(), bob.clone(), formed);

        assert_eq!(tr.entity_a(), &alice);
        assert_eq!(tr.entity_b(), &bob);
        assert_eq!(tr.formed_timestamp(), formed);
        assert!(tr.involves(&alice));
        assert!(tr.involves(&bob));

        let carol = EntityId::new("carol").unwrap();
        assert!(!tr.involves(&carol));
    }

    #[test]
    fn timestamped_relationship_mutable_accessor() {
        use crate::enums::{Direction, LifeDomain};

        let alice = EntityId::new("alice").unwrap();
        let bob = EntityId::new("bob").unwrap();
        let relationship = Relationship::try_between(alice.clone(), bob.clone()).unwrap();
        let formed = Timestamp::from_ymd_hms(2024, 1, 5, 0, 0, 0);

        let mut tr = TimestampedRelationship::new(relationship, alice, bob, formed);
        tr.relationship_mut()
            .trustworthiness_mut(Direction::AToB)
            .add_competence_delta(0.2);

        // add_competence_delta affects all domains, so check any domain
        assert!(tr
            .relationship()
            .trustworthiness(Direction::AToB)
            .competence(LifeDomain::Work)
            .unwrap()
            .delta()
            > 0.0);
    }

    #[test]
    fn regression_quality_exact() {
        let quality = RegressionQuality::Exact;
        assert!(quality.is_exact());
        assert!(!quality.is_approximate());
    }

    #[test]
    fn regression_quality_approximate() {
        let quality = RegressionQuality::Approximate;
        assert!(!quality.is_exact());
        assert!(quality.is_approximate());
    }

    #[test]
    fn regression_quality_default() {
        let quality = RegressionQuality::default();
        assert!(quality.is_exact());
    }

    #[test]
    fn simulation_entities_iterator() {
        let mut sim = create_simulation();
        sim.add_entity(create_human("alice"), sim.reference_date());
        sim.add_entity(create_human("bob"), sim.reference_date());
        sim.add_entity(create_human("carol"), sim.reference_date());

        let count = sim.entities().count();
        assert_eq!(count, 3);
    }

    #[test]
    fn simulation_get_anchored_entity() {
        let mut sim = create_simulation();
        let anchor = Timestamp::from_ymd_hms(2024, 1, 15, 0, 0, 0);
        sim.add_entity(create_human("alice"), anchor);

        let alice_id = EntityId::new("alice").unwrap();
        let anchored = sim.get_anchored_entity(&alice_id);
        assert!(anchored.is_some());
        assert_eq!(anchored.unwrap().anchor_timestamp(), anchor);

        let unknown_id = EntityId::new("unknown").unwrap();
        assert!(sim.get_anchored_entity(&unknown_id).is_none());
    }

    #[test]
    fn simulation_get_anchored_entity_mut() {
        let mut sim = create_simulation();
        sim.add_entity(create_human("alice"), sim.reference_date());

        let alice_id = EntityId::new("alice").unwrap();
        let anchored = sim.get_anchored_entity_mut(&alice_id);
        assert!(anchored.is_some());
    }

    #[test]
    fn simulation_all_events() {
        let mut sim = create_simulation();
        let target = EntityId::new("person").unwrap();

        for _ in 0..5 {
            let event = EventBuilder::new(EventType::Interaction)
                .target(target.clone())
                .build()
                .unwrap();
            sim.add_event(event, sim.reference_date());
        }

        let count = sim.all_events().count();
        assert_eq!(count, 5);
    }

    #[test]
    fn simulation_get_relationship() {
        let mut sim = create_simulation();
        let alice = EntityId::new("alice").unwrap();
        let bob = EntityId::new("bob").unwrap();

        let rel_id =
            sim.add_relationship(alice, bob, RelationshipSchema::Peer, sim.reference_date());

        let rel = sim.get_relationship(&rel_id);
        assert!(rel.is_some());

        let unknown = RelationshipId::new("unknown").unwrap();
        assert!(sim.get_relationship(&unknown).is_none());
    }

    #[test]
    fn simulation_debug() {
        let sim = create_simulation();
        let debug = format!("{:?}", sim);
        assert!(debug.contains("Simulation"));
    }

    #[test]
    fn simulation_clone() {
        let mut sim = create_simulation();
        sim.add_entity(create_human("alice"), sim.reference_date());

        let cloned = sim.clone();
        assert_eq!(cloned.entity_count(), 1);
    }

    #[test]
    fn anchored_entity_debug() {
        let entity = create_human("test");
        let anchor = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let anchored = AnchoredEntity::new(entity, anchor);

        let debug = format!("{:?}", anchored);
        assert!(debug.contains("AnchoredEntity"));
    }

    #[test]
    fn anchored_entity_clone() {
        let entity = create_human("test");
        let anchor = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let anchored = AnchoredEntity::new(entity, anchor);
        let cloned = anchored.clone();

        assert_eq!(anchored.anchor_timestamp(), cloned.anchor_timestamp());
    }

    #[test]
    fn timestamped_event_debug() {
        let event = EventBuilder::new(EventType::Interaction).build().unwrap();
        let ts = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let te = TimestampedEvent::new(event, ts);

        let debug = format!("{:?}", te);
        assert!(debug.contains("TimestampedEvent"));
    }

    #[test]
    fn timestamped_event_clone() {
        let event = EventBuilder::new(EventType::Interaction).build().unwrap();
        let ts = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let te = TimestampedEvent::new(event, ts);
        let cloned = te.clone();

        assert_eq!(te.timestamp(), cloned.timestamp());
    }

    #[test]
    fn timestamped_relationship_debug() {
        let alice = EntityId::new("alice").unwrap();
        let bob = EntityId::new("bob").unwrap();
        let relationship = Relationship::try_between(alice.clone(), bob.clone()).unwrap();
        let formed = Timestamp::from_ymd_hms(2024, 1, 5, 0, 0, 0);

        let tr = TimestampedRelationship::new(relationship, alice, bob, formed);
        let debug = format!("{:?}", tr);
        assert!(debug.contains("TimestampedRelationship"));
    }

    #[test]
    fn timestamped_relationship_clone() {
        let alice = EntityId::new("alice").unwrap();
        let bob = EntityId::new("bob").unwrap();
        let relationship = Relationship::try_between(alice.clone(), bob.clone()).unwrap();
        let formed = Timestamp::from_ymd_hms(2024, 1, 5, 0, 0, 0);

        let tr = TimestampedRelationship::new(relationship, alice, bob, formed);
        let cloned = tr.clone();

        assert_eq!(tr.formed_timestamp(), cloned.formed_timestamp());
    }

    #[test]
    fn regression_quality_equality() {
        assert_eq!(RegressionQuality::Exact, RegressionQuality::Exact);
        assert_ne!(RegressionQuality::Exact, RegressionQuality::Approximate);
    }

    #[test]
    fn regression_quality_debug() {
        let quality = RegressionQuality::Exact;
        let debug = format!("{:?}", quality);
        assert!(debug.contains("Exact"));
    }

    #[test]
    fn regression_quality_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(RegressionQuality::Exact);
        set.insert(RegressionQuality::Exact);
        assert_eq!(set.len(), 1);

        set.insert(RegressionQuality::Approximate);
        assert_eq!(set.len(), 2);
    }
}
