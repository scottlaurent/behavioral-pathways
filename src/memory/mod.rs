//! Memory subsystem for behavioral pathways.
//!
//! This module provides memory storage, retrieval, and management for entities.
//! Memories are organized into four layers with different capacities and time
//! horizons:
//!
//! - **Immediate**: 10 entries, minutes-hours horizon
//! - **Short-term**: 20 entries, days-weeks horizon
//! - **Long-term**: 50 entries, months-years horizon
//! - **Legacy**: Unlimited, milestone-triggered
//!
//! # Key Types
//!
//! - [`MemoryEntry`] - A single memory with emotional context and metadata
//! - [`MemoryLayers`] - Container organizing memories by temporal layer
//! - [`MemoryLayer`] - Enum specifying which layer to operate on
//! - [`EmotionalSnapshot`] - Frozen PAD values at memory formation
//! - [`MemorySource`] - How the entity learned about an event
//! - [`MemoryTag`] - Categorization tags for retrieval
//! - [`DeltasApplied`] - Changes recorded when memory was formed
//!
//! # Retrieval Methods
//!
//! MemoryLayers provides several retrieval methods:
//!
//! - `retrieve_by_salience(threshold)` - Memories above salience threshold
//! - `retrieve_mood_congruent(mood, min_congruence)` - Mood-matching memories
//! - `retrieve_by_tag(tag)` - Memories with specific tag
//! - `retrieve_by_participant(entity_id)` - Memories involving entity
//! - `retrieve_by_context(microsystem_id)` - Memories from context
//!
//! # Examples
//!
//! ```
//! use behavioral_pathways::memory::{
//!     MemoryEntry, MemoryLayers, MemoryLayer, MemoryTag, MemorySource, EmotionalSnapshot,
//! };
//! use behavioral_pathways::types::Duration;
//!
//! let mut layers = MemoryLayers::new();
//!
//! let entry = MemoryEntry::new(Duration::days(10), "An important conversation")
//!     .with_tags(vec![MemoryTag::Personal])
//!     .with_source(MemorySource::Self_)
//!     .with_emotional_snapshot(EmotionalSnapshot::new(0.5, 0.3, 0.0))
//!     .with_salience(0.7);
//!
//! layers.add(MemoryLayer::ShortTerm, entry);
//!
//! // Retrieve by tag
//! let personal = layers.retrieve_by_tag(MemoryTag::Personal);
//! assert_eq!(personal.len(), 1);
//! ```

mod consolidation;
mod deltas;
mod emotional_snapshot;
mod layers;
pub mod maintenance;
mod memory_entry;
mod retrieval;
mod source;
mod tags;

pub use consolidation::{apply_memory_consolidation, compute_priming_deltas, PrimingDeltas};
pub use deltas::{DeltasApplied, RelationshipDelta, ReputationDelta};
pub use emotional_snapshot::EmotionalSnapshot;
pub use layers::{
    MemoryLayer, MemoryLayers, IMMEDIATE_CAPACITY, LONG_TERM_CAPACITY, SHORT_TERM_CAPACITY,
};
pub use memory_entry::MemoryEntry;
pub use retrieval::{
    compute_retrieval_score, sort_by_salience_descending, RetrievalQuery,
    DEFAULT_SALIENCE_HALF_LIFE_DAYS, WEIGHT_BASE_SCORE, WEIGHT_CONTEXT_CONGRUENCE,
    WEIGHT_MOOD_CONGRUENCE, WEIGHT_PARTICIPANT_MATCH, WEIGHT_RECENCY, WEIGHT_SALIENCE,
    WEIGHT_SOURCE_CONFIDENCE, WEIGHT_TAG_RELEVANCE,
};
pub use source::MemorySource;
pub use tags::MemoryTag;
