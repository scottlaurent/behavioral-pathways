//! Enum definitions for behavioral pathways.
//!
//! This module contains enumerations for species, life stages,
//! personality profiles, relationship types, state access paths,
//! context access paths, emotions, alerts, reversibility, and events.

mod alert_severity;
mod alert_trigger;
mod attribution;
mod birth_era;
mod bond_type;
mod context_path;
mod developmental_category;
mod emotion;
mod event_payload;
mod event_scope;
mod event_type;
mod life_stage;
mod personality_profile;
mod rel_path;
mod relationship_schema;
mod reversibility;
mod species;
mod state_path;
mod subsystem_id;
mod trust_domain;

pub use alert_severity::AlertSeverity;
pub use alert_trigger::{AlertTrigger, ItsAlert, SpiralType};
pub use attribution::{Attribution, AttributionStability};
pub use birth_era::BirthEra;
pub use bond_type::BondType;
pub use context_path::{
    ChronosystemPath, ContextPath, EducationPath, ExosystemPath, FamilyPath, HealthcarePath,
    MacrosystemPath, MicrosystemPath, NeighborhoodPath, ReligiousPath, SocialPath, WorkPath,
};
pub(crate) use developmental_category::DevelopmentalCategory;
pub use emotion::Emotion;
pub use event_payload::{
    EventPayload, HistoricalEventType, HistoricalScope, InteractionTopic, LifeDomain, LossType,
    PolicyArea, RealizationType, SupportType, TraumaType, WeaponType,
};
pub use event_scope::EventScope;
pub use event_type::{EventCategory, EventTag, EventType};
pub use life_stage::LifeStage;
pub use personality_profile::PersonalityProfile;
pub use rel_path::{Direction, DirectionalPath, RelPath, SharedPath, TrustPath};
pub use relationship_schema::RelationshipSchema;
pub use reversibility::{ReversibilityError, ReversibilityResult};
pub use species::Species;
pub use state_path::{
    DispositionPath, HexacoPath, MentalHealthPath, MoodPath, NeedsPath, PersonCharacteristicsPath,
    SocialCognitionPath, StatePath,
};
pub use subsystem_id::SubsystemId;
pub use trust_domain::TrustDomain;
