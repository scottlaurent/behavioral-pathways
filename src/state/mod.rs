//! State management for behavioral pathways.
//!
//! This module contains types for managing psychological state values,
//! personality, mood, needs, mental health, and other individual state.
//!
//! # Key Types
//!
//! - [`StateValue`] - Core pattern for state dimensions with base, delta, decay
//! - [`Hexaco`] - Six-factor personality model
//! - [`Mood`] - PAD (Pleasure-Arousal-Dominance) affect dimensions
//! - [`Needs`] - Physiological and psychological needs
//! - [`SocialCognition`] - Interpersonal beliefs and perceptions
//! - [`MentalHealth`] - ITS (Interpersonal Theory of Suicide) factors
//! - [`Disposition`] - Behavioral tendencies
//! - [`PersonCharacteristics`] - PPCT person characteristics
//! - [`Demographical`] - Demographical metadata
//! - [`DemandCharacteristics`] - Observable social signals
//! - [`EntityModelConfig`] - Subsystem activation flags
//! - [`IndividualState`] - Aggregate container for all state

mod disposition;
mod demand_characteristics;
mod demographical;
mod entity_model_config;
mod formative;
mod hexaco;
mod individual_state;
mod mental_health;
mod mood;
mod needs;
mod social_cognition;
mod person_characteristics;
mod state_value;
mod state_interpreter;

pub use demand_characteristics::DemandCharacteristics;
pub use demographical::Demographical;
pub use disposition::Disposition;
pub use entity_model_config::EntityModelConfig;
pub use hexaco::Hexaco;
pub use individual_state::IndividualState;
pub use mental_health::{
    MentalHealth, HOPELESSNESS_THRESHOLD, PB_PRESENT_THRESHOLD, TB_PRESENT_THRESHOLD,
};
pub use mood::Mood;
pub use needs::Needs;
pub use social_cognition::SocialCognition;
pub use person_characteristics::PersonCharacteristics;
pub use state_value::StateValue;
pub use state_interpreter::StateInterpreter;
pub use formative::{
    age_plasticity, apply_formative_modifiers, combined_plasticity, cumulative_in_direction,
    effective_base_at, saturation_factor, sensitive_period_modifier, species_plasticity_modifier,
    stability_coefficient, trait_modifier, BaseShiftRecord, CUMULATIVE_CAP, MAX_SINGLE_EVENT_SHIFT,
    SATURATION_CONSTANT, SETTLING_DAYS, SEVERE_SHIFT_RETENTION, SEVERE_SHIFT_THRESHOLD,
};
