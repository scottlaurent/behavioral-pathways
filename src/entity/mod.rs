//! Entity system for behavioral pathways.
//!
//! This module contains the core `Entity` type and its builder. An Entity
//! represents a living individual with psychological state.
//!
//! # Key Types
//!
//! - [`Entity`] - Core agent container holding individual state
//! - [`EntityBuilder`] - Fluent builder for Entity construction
//! - [`EntityBuildError`] - Error type for build validation failures

#[allow(clippy::module_inception)]
mod entity;
mod affective_state;
mod entity_builder;

pub use affective_state::{AffectiveState, PhysiologicalState};
pub use entity::Entity;
pub use entity_builder::{EntityBuildError, EntityBuilder};
