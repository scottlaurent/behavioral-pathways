//! Relationship modeling for behavioral pathways.
//!
//! This module contains types and functions for modeling relationships
//! between entities, including:
//!
//! - Trust decomposition (competence, benevolence, integrity)
//! - Perceived risk assessment
//! - Relationship stages (Stranger -> Intimate)
//! - Shared and directional dimensions
//! - Behavioral predictions (would confide, would help)
//!
//! # Trust Model
//!
//! Based on Mayer's model, trust is decomposed into:
//!
//! - **Propensity**: The trustor's general willingness to trust (from Entity)
//! - **Trustworthiness**: Perceived competence, benevolence, and integrity of trustee
//! - **Perceived Risk**: Subjective assessment of potential negative consequences
//!
//! Trust decision formula:
//! ```text
//! willingness = propensity_weight * propensity
//!             + trustworthiness_weight * perceived_trustworthiness
//!             - risk_weight * perceived_risk
//! ```
//!
//! Weights depend on relationship stage - propensity matters more for strangers,
//! trustworthiness matters more for established relationships.
//!
//! # Example
//!
//! ```
//! use behavioral_pathways::relationship::{Relationship, RelationshipStage, StakesLevel};
//! use behavioral_pathways::types::EntityId;
//! use behavioral_pathways::enums::{BondType, Direction};
//!
//! // Create a relationship
//! let alice = EntityId::new("alice").unwrap();
//! let bob = EntityId::new("bob").unwrap();
//!
//! let mut rel = Relationship::try_between(alice, bob).unwrap()
//!     .with_bond(BondType::Colleague)
//!     .with_stage(RelationshipStage::Acquaintance);
//!
//! // Compute trust decision
//! let trust_propensity = 0.6; // Alice's general trust propensity
//! let decision = rel.compute_trust_decision(
//!     Direction::AToB,
//!     trust_propensity,
//!     StakesLevel::Medium
//! );
//!
//! // Check if Alice would delegate a task to Bob
//! if decision.would_delegate_task(0.5) {
//!     println!("Alice would delegate medium-difficulty tasks to Bob");
//! }
//! ```

mod antecedent;
mod antecedent_mapping;
mod interaction_pattern;
mod directional_dimensions;
mod perceived_risk;
mod predictions;
#[allow(clippy::module_inception)]
mod relationship;
mod shared_dimensions;
mod stage;
mod trust;
mod trust_context;
mod trust_decision;
mod trustworthiness;

pub use directional_dimensions::DirectionalDimensions;
pub use trust_context::TrustContext;
pub use perceived_risk::{PerceivedRisk, StakesLevel, Vulnerability, VulnerabilityType};
pub use predictions::{would_confide, would_help};
pub use interaction_pattern::InteractionPattern;
pub use antecedent::{AntecedentDirection, AntecedentType, TrustAntecedent};
pub use antecedent_mapping::{get_antecedent_for_event, AntecedentMapping, TRUST_ANTECEDENT_TABLE};
pub use relationship::{Relationship, RelationshipError, StageTransitionError};
pub use shared_dimensions::SharedDimensions;
pub use stage::RelationshipStage;
pub use trust::Trust;
pub use trust_decision::TrustDecision;
pub use trustworthiness::TrustworthinessFactors;
