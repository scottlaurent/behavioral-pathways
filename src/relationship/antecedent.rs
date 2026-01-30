//! Trust antecedent history for relationship trust updates.
//!
//! Trust antecedents capture events that update perceived competence,
//! benevolence, and integrity with asymmetric weighting.
//!
//! For ability antecedents, competence is domain-specific per Mayer's model:
//! competence in one life domain (e.g., medical) does not imply competence
//! in another (e.g., financial).

use crate::enums::{LifeDomain, TrustDomain};
use crate::types::Timestamp;

/// The trust dimension affected by an antecedent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AntecedentType {
    /// Perceived ability to perform tasks.
    Ability,
    /// Perceived caring and goodwill.
    Benevolence,
    /// Perceived adherence to principles.
    Integrity,
}

impl AntecedentType {
    /// Returns the trust domain associated with this antecedent type.
    #[must_use]
    pub const fn trust_domain(self) -> TrustDomain {
        match self {
            AntecedentType::Ability => TrustDomain::Task,
            AntecedentType::Benevolence => TrustDomain::Support,
            AntecedentType::Integrity => TrustDomain::Disclosure,
        }
    }
}

/// Whether the antecedent is positive or negative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AntecedentDirection {
    /// Positive trust-building signal.
    Positive,
    /// Negative trust-violating signal.
    Negative,
}

/// A single trust antecedent instance.
#[derive(Debug, Clone, PartialEq)]
pub struct TrustAntecedent {
    timestamp: Timestamp,
    antecedent_type: AntecedentType,
    direction: AntecedentDirection,
    magnitude: f32,
    context: String,
    domain: TrustDomain,
    /// For Ability antecedents, the life domain where competence was demonstrated.
    /// Per Mayer's model, competence is domain-specific.
    life_domain: Option<LifeDomain>,
}

impl TrustAntecedent {
    /// Creates a new TrustAntecedent with clamped magnitude.
    ///
    /// For Ability antecedents, use `with_life_domain()` to specify the
    /// domain-specific competence area.
    #[must_use]
    pub fn new(
        timestamp: Timestamp,
        antecedent_type: AntecedentType,
        direction: AntecedentDirection,
        magnitude: f32,
        context: impl Into<String>,
    ) -> Self {
        TrustAntecedent {
            timestamp,
            antecedent_type,
            direction,
            magnitude: magnitude.clamp(0.0, 1.0),
            context: context.into(),
            domain: antecedent_type.trust_domain(),
            life_domain: None,
        }
    }

    /// Sets the life domain for this antecedent.
    ///
    /// Only meaningful for Ability antecedents, where it specifies
    /// which domain the competence demonstration applies to.
    #[must_use]
    pub fn with_life_domain(mut self, domain: LifeDomain) -> Self {
        self.life_domain = Some(domain);
        self
    }

    /// Returns the antecedent timestamp.
    #[must_use]
    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    /// Returns the antecedent type.
    #[must_use]
    pub fn antecedent_type(&self) -> AntecedentType {
        self.antecedent_type
    }

    /// Returns the antecedent direction.
    #[must_use]
    pub fn direction(&self) -> AntecedentDirection {
        self.direction
    }

    /// Returns the antecedent magnitude (0-1).
    #[must_use]
    pub fn magnitude(&self) -> f32 {
        self.magnitude
    }

    /// Returns the antecedent context narrative.
    #[must_use]
    pub fn context(&self) -> &str {
        &self.context
    }

    /// Returns the trust domain impacted by this antecedent.
    #[must_use]
    pub fn domain(&self) -> TrustDomain {
        self.domain
    }

    /// Returns the life domain for Ability antecedents.
    ///
    /// Returns None if not set or for non-Ability antecedents.
    #[must_use]
    pub fn life_domain(&self) -> Option<LifeDomain> {
        self.life_domain
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trust_antecedent_accessors_return_fields() {
        let ts = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let antecedent = TrustAntecedent::new(
            ts,
            AntecedentType::Integrity,
            AntecedentDirection::Negative,
            0.6,
            "betrayal",
        );

        assert_eq!(antecedent.timestamp(), ts);
        assert_eq!(antecedent.antecedent_type(), AntecedentType::Integrity);
        assert_eq!(antecedent.direction(), AntecedentDirection::Negative);
        assert!((antecedent.magnitude() - 0.6).abs() < f32::EPSILON);
        assert_eq!(antecedent.context(), "betrayal");
        assert_eq!(antecedent.domain(), TrustDomain::Disclosure);
        assert!(antecedent.life_domain().is_none());
    }

    #[test]
    fn trust_antecedent_clamps_magnitude() {
        let ts = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let antecedent = TrustAntecedent::new(
            ts,
            AntecedentType::Ability,
            AntecedentDirection::Positive,
            1.5,
            "achievement",
        );

        assert!((antecedent.magnitude() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn with_life_domain_sets_domain_for_ability() {
        let ts = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let antecedent = TrustAntecedent::new(
            ts,
            AntecedentType::Ability,
            AntecedentDirection::Positive,
            0.5,
            "good_advice",
        )
        .with_life_domain(LifeDomain::Health);

        assert_eq!(antecedent.life_domain(), Some(LifeDomain::Health));
        assert_eq!(antecedent.domain(), TrustDomain::Task);
    }

    #[test]
    fn life_domain_works_on_non_ability_antecedents() {
        let ts = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let antecedent = TrustAntecedent::new(
            ts,
            AntecedentType::Benevolence,
            AntecedentDirection::Positive,
            0.5,
            "support",
        )
        .with_life_domain(LifeDomain::Work);

        // Life domain can be set but is semantically only meaningful for Ability
        assert_eq!(antecedent.life_domain(), Some(LifeDomain::Work));
    }
}
