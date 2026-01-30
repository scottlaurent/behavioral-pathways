//! Builder pattern for creating Simulation instances.
//!
//! Provides a fluent API for constructing simulations with
//! entities, events, and relationships.

use crate::entity::Entity;
use crate::enums::RelationshipSchema;
use crate::event::Event;
use crate::simulation::Simulation;
use crate::types::{EntityId, EventId, RelationshipId, Timestamp};
use std::fmt;

/// Error type for simulation build failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulationBuildError {
    /// An entity was added with a duplicate ID.
    DuplicateEntityId(EntityId),
    /// An event references an entity that doesn't exist in the simulation.
    /// Contains the event ID and the unknown entity ID.
    EventReferencesUnknownEntity(EventId, EntityId),
    /// A relationship references an entity that doesn't exist in the simulation.
    /// Contains the relationship ID and the unknown entity ID.
    RelationshipReferencesUnknownEntity(RelationshipId, EntityId),
    /// A relationship between an entity and itself was attempted.
    SelfRelationship(EntityId),
}

impl fmt::Display for SimulationBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimulationBuildError::DuplicateEntityId(id) => {
                write!(f, "Duplicate entity ID: {}", id.as_str())
            }
            SimulationBuildError::EventReferencesUnknownEntity(event_id, entity_id) => {
                write!(
                    f,
                    "Event '{}' references unknown entity: {}",
                    event_id.as_str(),
                    entity_id.as_str()
                )
            }
            SimulationBuildError::RelationshipReferencesUnknownEntity(rel_id, entity_id) => {
                write!(
                    f,
                    "Relationship '{}' references unknown entity: {}",
                    rel_id.as_str(),
                    entity_id.as_str()
                )
            }
            SimulationBuildError::SelfRelationship(id) => {
                write!(
                    f,
                    "Cannot create relationship between entity '{}' and itself",
                    id.as_str()
                )
            }
        }
    }
}

impl std::error::Error for SimulationBuildError {}

/// Pending entity to be added to the simulation.
struct PendingEntity {
    entity: Entity,
    anchor_timestamp: Timestamp,
}

/// Pending event to be added to the simulation.
struct PendingEvent {
    event: Event,
    timestamp: Timestamp,
}

/// Pending relationship to be added to the simulation.
struct PendingRelationship {
    id: RelationshipId,
    entity_a: EntityId,
    entity_b: EntityId,
    schema: RelationshipSchema,
    formed_timestamp: Timestamp,
}

/// Counter for generating unique relationship IDs during building.
static PENDING_REL_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn generate_pending_relationship_id() -> RelationshipId {
    let count = PENDING_REL_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    // Safe to unwrap - format string is never empty
    RelationshipId::new(format!("rel_{}", count)).unwrap()
}

/// Builder for creating Simulation instances.
///
/// Provides a fluent API for constructing simulations with
/// entities, events, and relationships.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::simulation::SimulationBuilder;
/// use behavioral_pathways::entity::EntityBuilder;
/// use behavioral_pathways::types::Timestamp;
/// use behavioral_pathways::enums::Species;
///
/// let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
///
/// let entity = EntityBuilder::new()
///     .id("person_001")
///     .species(Species::Human)
///     .build()
///     .unwrap();
///
/// let sim = SimulationBuilder::new(reference)
///     .add_entity(entity, reference)
///     .build()
///     .unwrap();
///
/// assert_eq!(sim.entity_count(), 1);
/// ```
pub struct SimulationBuilder {
    reference_date: Timestamp,
    entities: Vec<PendingEntity>,
    events: Vec<PendingEvent>,
    relationships: Vec<PendingRelationship>,
}

impl SimulationBuilder {
    /// Creates a new simulation builder with a reference date.
    ///
    /// The reference date is the simulation's baseline time point.
    ///
    /// # Arguments
    ///
    /// * `reference_date` - The simulation's reference date
    #[must_use]
    pub fn new(reference_date: Timestamp) -> Self {
        SimulationBuilder {
            reference_date,
            entities: Vec::new(),
            events: Vec::new(),
            relationships: Vec::new(),
        }
    }

    /// Adds an entity with its anchor timestamp.
    ///
    /// The anchor timestamp represents when the entity's state was observed.
    #[must_use]
    pub fn add_entity(mut self, entity: Entity, anchor_timestamp: Timestamp) -> Self {
        self.entities.push(PendingEntity {
            entity,
            anchor_timestamp,
        });
        self
    }

    /// Adds an event with its timestamp.
    #[must_use]
    pub fn add_event(mut self, event: Event, timestamp: Timestamp) -> Self {
        self.events.push(PendingEvent { event, timestamp });
        self
    }

    /// Adds a relationship between two entities.
    #[must_use]
    pub fn add_relationship(
        mut self,
        entity_a: EntityId,
        entity_b: EntityId,
        schema: RelationshipSchema,
        formed_timestamp: Timestamp,
    ) -> Self {
        self.relationships.push(PendingRelationship {
            id: generate_pending_relationship_id(),
            entity_a,
            entity_b,
            schema,
            formed_timestamp,
        });
        self
    }

    /// Builds the simulation.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - A duplicate entity ID was added
    /// - An event references an entity that doesn't exist
    /// - A relationship references an entity that doesn't exist
    /// - A relationship between an entity and itself was attempted
    pub fn build(self) -> Result<Simulation, SimulationBuildError> {
        let mut simulation = Simulation::new(self.reference_date);

        // Track entity IDs for duplicate detection and reference validation
        let mut seen_ids = std::collections::HashSet::new();

        // Add entities
        for pending in &self.entities {
            let id = pending.entity.id().clone();
            if !seen_ids.insert(id.clone()) {
                return Err(SimulationBuildError::DuplicateEntityId(id));
            }
        }

        // Validate event source and target references
        for pending in &self.events {
            // Validate source if present
            if let Some(source) = pending.event.source() {
                if !seen_ids.contains(source) {
                    return Err(SimulationBuildError::EventReferencesUnknownEntity(
                        pending.event.id().clone(),
                        source.clone(),
                    ));
                }
            }
            // Validate target if present
            if let Some(target) = pending.event.target() {
                if !seen_ids.contains(target) {
                    return Err(SimulationBuildError::EventReferencesUnknownEntity(
                        pending.event.id().clone(),
                        target.clone(),
                    ));
                }
            }
        }

        // Validate relationship references
        for pending in &self.relationships {
            // Check for self-relationship
            if pending.entity_a == pending.entity_b {
                return Err(SimulationBuildError::SelfRelationship(
                    pending.entity_a.clone(),
                ));
            }

            // Check entity_a exists
            if !seen_ids.contains(&pending.entity_a) {
                return Err(SimulationBuildError::RelationshipReferencesUnknownEntity(
                    pending.id.clone(),
                    pending.entity_a.clone(),
                ));
            }

            // Check entity_b exists
            if !seen_ids.contains(&pending.entity_b) {
                return Err(SimulationBuildError::RelationshipReferencesUnknownEntity(
                    pending.id.clone(),
                    pending.entity_b.clone(),
                ));
            }
        }

        // All validations passed - now add everything to the simulation
        for pending in self.entities {
            simulation.add_entity(pending.entity, pending.anchor_timestamp);
        }

        for pending in self.events {
            simulation.add_event(pending.event, pending.timestamp);
        }

        for pending in self.relationships {
            simulation.add_relationship(
                pending.entity_a,
                pending.entity_b,
                pending.schema,
                pending.formed_timestamp,
            );
        }

        Ok(simulation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::EntityBuilder;
    use crate::enums::{EventType, Species};
    use crate::event::EventBuilder;

    fn create_human(id: &str) -> Entity {
        EntityBuilder::new()
            .id(id)
            .species(Species::Human)
            .build()
            .unwrap()
    }

    fn reference_date() -> Timestamp {
        Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0)
    }

    #[test]
    fn builder_creates_empty_simulation() {
        let sim = SimulationBuilder::new(reference_date()).build().unwrap();

        assert_eq!(sim.entity_count(), 0);
        assert_eq!(sim.reference_date(), reference_date());
    }

    #[test]
    fn builder_add_entity() {
        let entity = create_human("person_001");

        let sim = SimulationBuilder::new(reference_date())
            .add_entity(entity, reference_date())
            .build()
            .unwrap();

        assert_eq!(sim.entity_count(), 1);
    }

    #[test]
    fn builder_add_multiple_entities() {
        let sim = SimulationBuilder::new(reference_date())
            .add_entity(create_human("alice"), reference_date())
            .add_entity(create_human("bob"), reference_date())
            .add_entity(create_human("carol"), reference_date())
            .build()
            .unwrap();

        assert_eq!(sim.entity_count(), 3);
    }

    #[test]
    fn builder_duplicate_entity_id_fails() {
        let result = SimulationBuilder::new(reference_date())
            .add_entity(create_human("alice"), reference_date())
            .add_entity(create_human("alice"), reference_date())
            .build();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, SimulationBuildError::DuplicateEntityId(ref id) if id.as_str() == "alice")
        );
    }

    #[test]
    fn builder_add_event() {
        let target = EntityId::new("person_001").unwrap();
        let event = EventBuilder::new(EventType::SocialExclusion)
            .target(target.clone())
            .severity(0.7)
            .build()
            .unwrap();

        let sim = SimulationBuilder::new(reference_date())
            .add_entity(create_human("person_001"), reference_date())
            .add_event(event, Timestamp::from_ymd_hms(2024, 1, 15, 0, 0, 0))
            .build()
            .unwrap();

        let events = sim.events_for(&target);
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn builder_event_with_valid_source_succeeds() {
        let source = EntityId::new("person_001").unwrap();
        let target = EntityId::new("person_002").unwrap();
        let event = EventBuilder::new(EventType::SocialExclusion)
            .source(source.clone())
            .target(target.clone())
            .severity(0.7)
            .build()
            .unwrap();

        let sim = SimulationBuilder::new(reference_date())
            .add_entity(create_human("person_001"), reference_date())
            .add_entity(create_human("person_002"), reference_date())
            .add_event(event, Timestamp::from_ymd_hms(2024, 1, 15, 0, 0, 0))
            .build()
            .unwrap();

        // Both entities exist, event should be added
        let events = sim.events_for(&target);
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn builder_event_unknown_target_fails() {
        let target = EntityId::new("unknown").unwrap();
        let event = EventBuilder::new(EventType::SocialExclusion)
            .target(target.clone())
            .severity(0.7)
            .build()
            .unwrap();

        let result = SimulationBuilder::new(reference_date())
            .add_event(event, Timestamp::from_ymd_hms(2024, 1, 15, 0, 0, 0))
            .build();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            SimulationBuildError::EventReferencesUnknownEntity(_, ref entity_id)
                if entity_id.as_str() == "unknown"
        ));
    }

    #[test]
    fn builder_event_unknown_source_fails() {
        let source = EntityId::new("unknown_source").unwrap();
        let target = EntityId::new("person_001").unwrap();
        let event = EventBuilder::new(EventType::SocialExclusion)
            .source(source.clone())
            .target(target.clone())
            .severity(0.7)
            .build()
            .unwrap();

        let result = SimulationBuilder::new(reference_date())
            .add_entity(create_human("person_001"), reference_date())
            .add_event(event, Timestamp::from_ymd_hms(2024, 1, 15, 0, 0, 0))
            .build();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            SimulationBuildError::EventReferencesUnknownEntity(_, ref entity_id)
                if entity_id.as_str() == "unknown_source"
        ));
    }

    #[test]
    fn builder_add_relationship() {
        let alice_id = EntityId::new("alice").unwrap();
        let bob_id = EntityId::new("bob").unwrap();

        let sim = SimulationBuilder::new(reference_date())
            .add_entity(create_human("alice"), reference_date())
            .add_entity(create_human("bob"), reference_date())
            .add_relationship(
                alice_id.clone(),
                bob_id,
                RelationshipSchema::Peer,
                reference_date(),
            )
            .build()
            .unwrap();

        assert_eq!(sim.relationship_count(), 1);
        assert_eq!(sim.relationships_for(&alice_id).len(), 1);
    }

    #[test]
    fn builder_relationship_unknown_entity_a_fails() {
        let alice_id = EntityId::new("alice").unwrap();
        let bob_id = EntityId::new("bob").unwrap();

        let result = SimulationBuilder::new(reference_date())
            .add_entity(create_human("bob"), reference_date())
            .add_relationship(
                alice_id.clone(),
                bob_id,
                RelationshipSchema::Peer,
                reference_date(),
            )
            .build();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            SimulationBuildError::RelationshipReferencesUnknownEntity(_, ref entity_id)
                if entity_id.as_str() == "alice"
        ));
    }

    #[test]
    fn builder_relationship_unknown_entity_b_fails() {
        let alice_id = EntityId::new("alice").unwrap();
        let bob_id = EntityId::new("bob").unwrap();

        let result = SimulationBuilder::new(reference_date())
            .add_entity(create_human("alice"), reference_date())
            .add_relationship(
                alice_id,
                bob_id.clone(),
                RelationshipSchema::Peer,
                reference_date(),
            )
            .build();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            SimulationBuildError::RelationshipReferencesUnknownEntity(_, ref entity_id)
                if entity_id.as_str() == "bob"
        ));
    }

    #[test]
    fn builder_self_relationship_fails() {
        let alice_id = EntityId::new("alice").unwrap();

        let result = SimulationBuilder::new(reference_date())
            .add_entity(create_human("alice"), reference_date())
            .add_relationship(
                alice_id.clone(),
                alice_id,
                RelationshipSchema::Peer,
                reference_date(),
            )
            .build();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            SimulationBuildError::SelfRelationship(ref id)
                if id.as_str() == "alice"
        ));
    }

    #[test]
    fn builder_fluent_chain() {
        let alice = create_human("alice");
        let bob = create_human("bob");
        let alice_id = EntityId::new("alice").unwrap();
        let bob_id = EntityId::new("bob").unwrap();

        let event = EventBuilder::new(EventType::Interaction)
            .target(alice_id.clone())
            .build()
            .unwrap();

        let sim = SimulationBuilder::new(reference_date())
            .add_entity(alice, reference_date())
            .add_entity(bob, reference_date())
            .add_event(event, Timestamp::from_ymd_hms(2024, 1, 10, 0, 0, 0))
            .add_relationship(
                alice_id.clone(),
                bob_id,
                RelationshipSchema::Peer,
                Timestamp::from_ymd_hms(2024, 1, 5, 0, 0, 0),
            )
            .build()
            .unwrap();

        assert_eq!(sim.entity_count(), 2);
        assert_eq!(sim.relationship_count(), 1);
        assert_eq!(sim.events_for(&alice_id).len(), 1);
    }

    #[test]
    fn simulation_build_error_display() {
        let alice_id = EntityId::new("alice").unwrap();
        let bob_id = EntityId::new("bob").unwrap();
        let carol_id = EntityId::new("carol").unwrap();
        let event_id = EventId::new("test_event").unwrap();
        let rel_id = RelationshipId::new("test_rel").unwrap();

        let err1 = SimulationBuildError::DuplicateEntityId(alice_id);
        assert!(format!("{}", err1).contains("alice"));
        assert!(format!("{}", err1).contains("Duplicate"));

        let err2 = SimulationBuildError::EventReferencesUnknownEntity(event_id, bob_id.clone());
        assert!(format!("{}", err2).contains("bob"));
        assert!(format!("{}", err2).contains("unknown"));
        assert!(format!("{}", err2).contains("test_event"));

        let err3 = SimulationBuildError::RelationshipReferencesUnknownEntity(rel_id, bob_id);
        assert!(format!("{}", err3).contains("bob"));
        assert!(format!("{}", err3).contains("unknown"));
        assert!(format!("{}", err3).contains("test_rel"));

        let err4 = SimulationBuildError::SelfRelationship(carol_id);
        assert!(format!("{}", err4).contains("carol"));
        assert!(format!("{}", err4).contains("itself"));
    }

    #[test]
    fn simulation_build_error_debug() {
        let alice_id = EntityId::new("alice").unwrap();
        let err = SimulationBuildError::DuplicateEntityId(alice_id);
        let debug = format!("{:?}", err);
        assert!(debug.contains("DuplicateEntityId"));
    }

    #[test]
    fn simulation_build_error_clone() {
        let alice_id = EntityId::new("alice").unwrap();
        let err1 = SimulationBuildError::DuplicateEntityId(alice_id);
        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }

    #[test]
    fn event_without_target_allowed() {
        // Events without targets (broadcast events) are allowed
        let event = EventBuilder::new(EventType::PolicyChange)
            .severity(0.5)
            .build()
            .unwrap();

        let sim = SimulationBuilder::new(reference_date())
            .add_event(event, Timestamp::from_ymd_hms(2024, 1, 15, 0, 0, 0))
            .build()
            .unwrap();

        assert_eq!(sim.entity_count(), 0);
    }

    #[test]
    fn simulation_build_error_is_std_error() {
        let alice_id = EntityId::new("alice").unwrap();
        let err: Box<dyn std::error::Error> =
            Box::new(SimulationBuildError::DuplicateEntityId(alice_id));
        // Verify it can be used as a std::error::Error
        assert!(err.source().is_none());
    }

    #[test]
    fn simulation_build_error_eq() {
        let alice1 = EntityId::new("alice").unwrap();
        let alice2 = EntityId::new("alice").unwrap();
        let bob = EntityId::new("bob").unwrap();

        let err1 = SimulationBuildError::DuplicateEntityId(alice1);
        let err2 = SimulationBuildError::DuplicateEntityId(alice2);
        let err3 = SimulationBuildError::DuplicateEntityId(bob);

        assert_eq!(err1, err2);
        assert_ne!(err1, err3);
    }
}
