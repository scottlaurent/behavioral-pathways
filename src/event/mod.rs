//! Event module for behavioral pathways.
//!
//! This module provides event types, construction, dispatch, and salience
//! computation for the simulation. Events are the primary mechanism for
//! state changes.
//!
//! # Key Types
//!
//! - [`Event`] - Core event structure
//! - [`EventBuilder`] - Fluent construction
//! - [`EventBus`] - Dispatch and subscription
//! - [`EventFilter`] - Subscription filtering
//!
//! # Usage
//!
//! ```ignore
//! use behavioral_pathways::event::{Event, EventBuilder, EventBus};
//! use behavioral_pathways::enums::{EventType, EventScope};
//!
//! // Build an event
//! let event = EventBuilder::new(EventType::Violence)
//!     .severity(0.8)
//!     .build()
//!     .unwrap();
//!
//! // Dispatch via bus (scope is required - be explicit)
//! let mut bus = EventBus::new();
//! bus.dispatch(event, EventScope::Global);
//!
//! // Check if bus has pending events
//! assert!(bus.has_pending());
//! ```

#[allow(clippy::module_inception)]
mod event;
mod event_builder;
mod event_bus;
mod salience;

pub use event::Event;
pub use event_builder::{EventBuildError, EventBuilder};
pub use event_bus::{EventBus, EventBusError, EventFilter, ProcessedEvent, MAX_CASCADE_DEPTH};
pub use salience::{
    arousal_weight_for_species, compute_arousal_modulated_salience, AROUSAL_CEILING,
    AROUSAL_THRESHOLD, AROUSAL_WEIGHT_ANIMAL, AROUSAL_WEIGHT_HUMAN, AROUSAL_WEIGHT_ROBOTIC,
    EXTREME_AROUSAL_IMPAIRMENT, NEGATIVITY_BIAS_MULTIPLIER,
};
