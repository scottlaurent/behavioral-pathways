//! Behavioral Pathways
//!
//! Domain-agnostic library for modeling individual psychology and social dynamics.
//!
//! This library provides tools for simulating how entities (humans, animals)
//! think, feel, relate, and change over time using established psychological
//! frameworks.
//!
//! # Core Concepts
//!
//! - **Entity**: A living individual with psychological state
//! - **StateValue**: A dimension with base value, delta, and decay behavior
//! - **Species**: Determines lifespan and psychological time scaling
//! - **LifeStage**: Developmental stage affecting plasticity and event impact
//! - **IndividualState**: Aggregate container for all psychological state
//!
//! # Psychological Frameworks
//!
//! - **PAD Model**: Pleasure-Arousal-Dominance for affect representation
//! - **HEXACO**: Six-factor personality model
//! - **ITS**: Joiner's Interpersonal Theory of Suicide for mental health
//! - **PPCT**: Bronfenbrenner's Person-Process-Context-Time model
//!
//! # Example
//!
//! ```
//! use behavioral_pathways::entity::EntityBuilder;
//! use behavioral_pathways::enums::{Species, MoodPath, StatePath};
//!
//! // Create an entity with the builder
//! let entity = EntityBuilder::new()
//!     .species(Species::Human)
//!     .build()
//!     .unwrap();
//!
//! assert_eq!(entity.species(), &Species::Human);
//!
//! // Access state via typed paths
//! let valence = entity.get_effective(StatePath::Mood(MoodPath::Valence));
//! ```
//!
//! # Consumer API Model
//!
//! The consumer API is timestamp-based:
//! - Create a `Simulation` with a reference date
//! - Add entities with an anchor timestamp (their known state at a point in time)
//! - Query state at any timestamp via `state_at(timestamp)`
//! - Each query computes fresh from declared data (no persistence)
//!
//! Internal methods like `advance()`, `regress_by()`, `apply_decay()`, and
//! `reverse_decay()` are implementation details used by `state_at()` and
//! should not be called directly by consumers.

pub mod context;
pub mod entity;
pub mod enums;
pub mod event;
pub mod memory;
pub(crate) mod processor;
pub mod relationship;
pub mod simulation;
pub mod state;
pub mod types;

// Re-export entity types at crate root
pub use entity::{AffectiveState, Entity, EntityBuildError, EntityBuilder, PhysiologicalState};

// Re-export commonly used enums at crate root
pub use enums::{
    AlertSeverity, AlertTrigger, Attribution, AttributionStability, BirthEra, BondType,
    ChronosystemPath,
    ContextPath, Direction, DirectionalPath, DispositionPath, EducationPath, Emotion,
    EventCategory, EventPayload, EventScope, EventTag, EventType, ExosystemPath, FamilyPath,
    HealthcarePath, HexacoPath, HistoricalEventType, HistoricalScope, InteractionTopic, LifeDomain,
    LifeStage, LossType, MacrosystemPath, MentalHealthPath, MicrosystemPath, MoodPath, NeedsPath,
    NeighborhoodPath, PersonCharacteristicsPath, PersonalityProfile, PolicyArea, RealizationType,
    RelPath, RelationshipSchema, ReligiousPath, ReversibilityError, ReversibilityResult,
    SharedPath, SocialCognitionPath, SocialPath, Species, SpiralType, StatePath, SubsystemId,
    SupportType, TraumaType, TrustPath, WeaponType, WorkPath,
};

// Re-export context types at crate root
pub use context::{
    check_proximal_process_gate, ChronosystemContext, CohortEffects, CriticalPeriod,
    CulturalOrientation, EcologicalContext, EducationContext, ExosystemContext, FamilyContext,
    FamilyRole, HealthcareContext, HistoricalPeriod, InstitutionalStructure, InteractionProfile,
    MacrosystemConstraintSet, MacrosystemContext, MesosystemCache, MesosystemLinkage, Microsystem,
    MicrosystemType, NeighborhoodContext, NonNormativeEvent, NormativeTransition, ParentWorkQuality,
    ProximalProcessGateError, ReligiousContext, SocialContext, TurningPoint, TurningPointDomain,
    WorkContext, INTERACTION_COMPLEXITY_THRESHOLD, INTERACTION_FREQUENCY_THRESHOLD,
};

// NOTE: Processor module contains internal implementation details.
// Consumers should use the Simulation API (state_at) instead of calling
// processor functions directly.
// Exception: EmotionIntensities is exported for derived emotion access.
pub use processor::EmotionIntensities;

// Re-export simulation types at crate root
pub use simulation::{
    AnchoredEntity, ComputedState, EntityQueryHandle, RegressionQuality, Simulation,
    SimulationBuildError, SimulationBuilder, TimestampedEvent, TimestampedRelationship,
};

// Re-export commonly used state types at crate root
pub use state::{
    age_plasticity, apply_formative_modifiers, combined_plasticity, cumulative_in_direction,
    effective_base_at, saturation_factor, sensitive_period_modifier, species_plasticity_modifier,
    stability_coefficient, trait_modifier, BaseShiftRecord, DemandCharacteristics, Demographical,
    Disposition, EntityModelConfig, Hexaco, IndividualState, MentalHealth, Mood, Needs,
    PersonCharacteristics, SocialCognition, StateValue, CUMULATIVE_CAP, HOPELESSNESS_THRESHOLD,
    MAX_SINGLE_EVENT_SHIFT, PB_PRESENT_THRESHOLD, SATURATION_CONSTANT, SETTLING_DAYS,
    SEVERE_SHIFT_RETENTION, SEVERE_SHIFT_THRESHOLD, TB_PRESENT_THRESHOLD,
};

// Re-export relationship types at crate root
pub use relationship::{
    AntecedentDirection, AntecedentMapping, AntecedentType, DirectionalDimensions,
    InteractionPattern, PerceivedRisk, Relationship, RelationshipError, RelationshipStage,
    SharedDimensions, StakesLevel, TrustAntecedent, TrustContext, TrustDecision,
    TrustworthinessFactors, Vulnerability, VulnerabilityType, TRUST_ANTECEDENT_TABLE,
};

// Re-export event types at crate root
pub use event::{
    arousal_weight_for_species, compute_arousal_modulated_salience, Event, EventBuildError,
    EventBuilder, EventBus, EventBusError, EventFilter, AROUSAL_CEILING, AROUSAL_THRESHOLD,
    AROUSAL_WEIGHT_ANIMAL, AROUSAL_WEIGHT_HUMAN, AROUSAL_WEIGHT_ROBOTIC,
    EXTREME_AROUSAL_IMPAIRMENT, MAX_CASCADE_DEPTH, NEGATIVITY_BIAS_MULTIPLIER,
};

// Re-export commonly used types at crate root
pub use types::{
    duration_to_timestamp, timestamp_to_duration, Alert, Duration, EntityId, EventId, GroupId,
    MemoryId, MicrosystemId, RelationshipId, RelationshipSlot, SubscriptionId, Timestamp,
    TimestampParseError,
};
