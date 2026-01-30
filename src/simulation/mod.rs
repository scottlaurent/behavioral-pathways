//! Simulation container for timestamp-based state queries.
//!
//! This module provides the `Simulation` container, which is the primary
//! consumer API for the behavioral-pathways library. Simulations hold
//! entities, events, and relationships with their associated timestamps,
//! enabling state queries at any point in time.
//!
//! # Usage Model
//!
//! ```ignore
//! use behavioral_pathways::simulation::{Simulation, SimulationBuilder};
//! use behavioral_pathways::entity::EntityBuilder;
//! use behavioral_pathways::types::Timestamp;
//! use behavioral_pathways::enums::Species;
//!
//! // Create a simulation with a reference date
//! let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
//!
//! // Build an entity with a birth date
//! let entity = EntityBuilder::new()
//!     .id("person_001")
//!     .species(Species::Human)
//!     .birth_date(Timestamp::from_ymd_hms(1990, 6, 15, 0, 0, 0))
//!     .build()
//!     .unwrap();
//!
//! // Add entity with its anchor timestamp (when state was observed)
//! let anchor = Timestamp::from_ymd_hms(2024, 1, 1, 9, 0, 0);
//!
//! let mut sim = Simulation::new(reference);
//! sim.add_entity(entity, anchor);
//!
//! // Query state at any timestamp
//! let query_time = Timestamp::from_ymd_hms(2024, 6, 15, 12, 0, 0);
//! let handle = sim.entity(&EntityId::new("person_001").unwrap());
//! if let Some(h) = handle {
//!     let state = h.state_at(query_time);
//! }
//! ```
//!
//! # Key Concepts
//!
//! - **Reference Date**: The simulation's reference point for time calculations
//! - **Anchor Timestamp**: When an entity's state was observed
//! - **Birth Date**: When an entity was born (for age calculations)
//! - **state_at()**: The core API for computing state at any timestamp

#[allow(clippy::module_inception)]
mod simulation;
mod simulation_builder;
mod state_query;

pub use simulation::{
    AnchoredEntity, RegressionQuality, Simulation, TimestampedEvent, TimestampedRelationship,
};
pub use simulation_builder::{SimulationBuildError, SimulationBuilder};
pub use state_query::{ComputedState, EntityQueryHandle};
