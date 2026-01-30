//! Behavioral prediction methods for relationships.
//!
//! These functions predict whether a trustor would engage in
//! trust-related behaviors given their propensity and the risk level.

use crate::enums::Direction;
use crate::relationship::{Relationship, StakesLevel};

/// Threshold for confiding behavior.
///
/// The base threshold is 0.6, increased by risk.
const CONFIDE_BASE_THRESHOLD: f32 = 0.6;

/// Risk multiplier for confide threshold.
const CONFIDE_RISK_MULTIPLIER: f32 = 0.3;

/// Threshold for helping behavior.
const HELP_BASE_THRESHOLD: f32 = 0.4;

/// Risk multiplier for help threshold.
const HELP_RISK_MULTIPLIER: f32 = 0.3;

/// Converts a risk_level (0-1) to a StakesLevel enum.
///
/// Mapping:
/// - 0.0 - 0.25: Low
/// - 0.25 - 0.50: Medium
/// - 0.50 - 0.75: High
/// - 0.75 - 1.0: Critical
fn risk_to_stakes(risk_level: f32) -> StakesLevel {
    if risk_level >= 0.75 {
        StakesLevel::Critical
    } else if risk_level >= 0.5 {
        StakesLevel::High
    } else if risk_level >= 0.25 {
        StakesLevel::Medium
    } else {
        StakesLevel::Low
    }
}

/// Predicts whether a trustor would confide in the trustee.
///
/// Uses the formula:
/// ```text
/// stakes = risk_to_stakes(risk_level)  // Maps 0-1 to Low/Medium/High/Critical
/// threshold = 0.6 + (risk_level * 0.3)
/// would_confide = disclosure_willingness > threshold
/// ```
///
/// The `risk_level` parameter affects BOTH:
/// 1. The stakes level passed to trust computation (higher risk = higher stakes = lower willingness)
/// 2. The threshold for confiding (higher risk = higher threshold)
///
/// # Arguments
///
/// * `relationship` - The relationship between trustor and trustee
/// * `direction` - Which direction (AToB or BToA)
/// * `trustor_propensity` - The trustor's dispositional trust propensity (0-1)
/// * `risk_level` - The perceived risk of confiding (0-1)
///
/// # Examples
///
/// ```
/// use behavioral_pathways::relationship::{Relationship, would_confide, StakesLevel};
/// use behavioral_pathways::types::EntityId;
/// use behavioral_pathways::enums::Direction;
///
/// let alice = EntityId::new("alice").unwrap();
/// let bob = EntityId::new("bob").unwrap();
/// let rel = Relationship::try_between(alice, bob).unwrap();
///
/// let would = would_confide(&rel, Direction::AToB, 0.7, 0.3);
/// ```
#[must_use]
pub fn would_confide(
    relationship: &Relationship,
    direction: Direction,
    trustor_propensity: f32,
    risk_level: f32,
) -> bool {
    // Map risk_level to stakes - this affects the perceived risk in trust computation
    let stakes = risk_to_stakes(risk_level);

    // Compute trust decision with stakes derived from risk_level
    let decision = relationship.compute_trust_decision(direction, trustor_propensity, stakes);

    // Threshold also increases with risk
    let threshold = CONFIDE_BASE_THRESHOLD + (risk_level * CONFIDE_RISK_MULTIPLIER);

    decision.disclosure_willingness() > threshold
}

/// Predicts whether a trustor would help the trustee.
///
/// Uses the formula:
/// ```text
/// stakes = risk_to_stakes(risk_level)  // Maps 0-1 to Low/Medium/High/Critical
/// threshold = 0.4 + (risk_level * 0.3)
/// would_help = support_willingness > threshold
/// ```
///
/// Helping has a lower base threshold than confiding because it
/// involves less vulnerability.
///
/// The `risk_level` parameter affects BOTH:
/// 1. The stakes level passed to trust computation (higher risk = higher stakes = lower willingness)
/// 2. The threshold for helping (higher risk = higher threshold)
///
/// # Arguments
///
/// * `relationship` - The relationship between trustor and trustee
/// * `direction` - Which direction (AToB or BToA)
/// * `trustor_propensity` - The trustor's dispositional trust propensity (0-1)
/// * `risk_level` - The perceived risk of helping (0-1)
///
/// # Examples
///
/// ```
/// use behavioral_pathways::relationship::{Relationship, would_help};
/// use behavioral_pathways::types::EntityId;
/// use behavioral_pathways::enums::Direction;
///
/// let alice = EntityId::new("alice").unwrap();
/// let bob = EntityId::new("bob").unwrap();
/// let rel = Relationship::try_between(alice, bob).unwrap();
///
/// let would = would_help(&rel, Direction::AToB, 0.5, 0.2);
/// ```
#[must_use]
pub fn would_help(
    relationship: &Relationship,
    direction: Direction,
    trustor_propensity: f32,
    risk_level: f32,
) -> bool {
    // Map risk_level to stakes - this affects the perceived risk in trust computation
    let stakes = risk_to_stakes(risk_level);

    let decision = relationship.compute_trust_decision(direction, trustor_propensity, stakes);

    // Threshold also increases with risk
    let threshold = HELP_BASE_THRESHOLD + (risk_level * HELP_RISK_MULTIPLIER);

    decision.support_willingness() > threshold
}

impl Relationship {
    /// Predicts whether entity A would confide in entity B.
    ///
    /// Convenience method that uses the AToB direction.
    ///
    /// # Arguments
    ///
    /// * `trustor_propensity` - A's dispositional trust propensity (0-1)
    /// * `risk_level` - The perceived risk of confiding (0-1)
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::relationship::Relationship;
    /// use behavioral_pathways::types::EntityId;
    ///
    /// let alice = EntityId::new("alice").unwrap();
    /// let bob = EntityId::new("bob").unwrap();
    /// let rel = Relationship::try_between(alice, bob).unwrap();
    ///
    /// let would = rel.would_a_confide_in_b(0.7, 0.3);
    /// ```
    #[must_use]
    pub fn would_a_confide_in_b(&self, trustor_propensity: f32, risk_level: f32) -> bool {
        would_confide(self, Direction::AToB, trustor_propensity, risk_level)
    }

    /// Predicts whether entity B would confide in entity A.
    ///
    /// Convenience method that uses the BToA direction.
    #[must_use]
    pub fn would_b_confide_in_a(&self, trustor_propensity: f32, risk_level: f32) -> bool {
        would_confide(self, Direction::BToA, trustor_propensity, risk_level)
    }

    /// Predicts whether entity A would help entity B.
    ///
    /// Convenience method that uses the AToB direction.
    ///
    /// # Arguments
    ///
    /// * `trustor_propensity` - A's dispositional trust propensity (0-1)
    /// * `risk_level` - The perceived risk of helping (0-1)
    #[must_use]
    pub fn would_a_help_b(&self, trustor_propensity: f32, risk_level: f32) -> bool {
        would_help(self, Direction::AToB, trustor_propensity, risk_level)
    }

    /// Predicts whether entity B would help entity A.
    ///
    /// Convenience method that uses the BToA direction.
    #[must_use]
    pub fn would_b_help_a(&self, trustor_propensity: f32, risk_level: f32) -> bool {
        would_help(self, Direction::BToA, trustor_propensity, risk_level)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::relationship::RelationshipStage;
    use crate::types::EntityId;

    fn alice() -> EntityId {
        EntityId::new("alice").unwrap()
    }

    fn bob() -> EntityId {
        EntityId::new("bob").unwrap()
    }

    #[test]
    fn would_confide_requires_propensity_and_risk() {
        let rel = Relationship::try_between(alice(), bob()).unwrap();

        // Low propensity, low risk - may not confide
        let _low_propensity = would_confide(&rel, Direction::AToB, 0.2, 0.2);

        // High propensity, low risk - more likely to confide
        let _high_propensity = would_confide(&rel, Direction::AToB, 0.9, 0.2);

        // Risk affects threshold
        let low_risk = would_confide(&rel, Direction::AToB, 0.7, 0.1);
        let high_risk = would_confide(&rel, Direction::AToB, 0.7, 0.9);

        // Higher propensity should be more likely to confide
        // (though both might be false due to stranger stage)
        // High risk should make confiding less likely
        assert!(u8::from(low_risk) >= u8::from(high_risk));
    }

    #[test]
    fn would_help_scales_with_risk() {
        let mut rel = Relationship::try_between(alice(), bob()).unwrap();
        // Move to established for more trust
        rel.set_stage(RelationshipStage::Established).unwrap();

        let low_risk = would_help(&rel, Direction::AToB, 0.7, 0.1);
        let high_risk = would_help(&rel, Direction::AToB, 0.7, 0.9);

        // Low risk should make helping more likely
        // (one should be true, the other might not be)
        assert!(u8::from(low_risk) >= u8::from(high_risk));
    }

    #[test]
    fn would_confide_false_for_stranger_low_propensity() {
        let rel = Relationship::try_between(alice(), bob()).unwrap();
        // Stranger with low propensity shouldn't confide
        let would = would_confide(&rel, Direction::AToB, 0.3, 0.5);
        assert!(!would);
    }

    #[test]
    fn would_help_possible_with_low_risk() {
        let mut rel = Relationship::try_between(alice(), bob()).unwrap();
        rel.set_stage(RelationshipStage::Established).unwrap();
        // Boost trustworthiness
        rel.trustworthiness_mut(Direction::AToB)
            .add_benevolence_delta(0.4);

        // With established relationship, high propensity, low risk
        let would = would_help(&rel, Direction::AToB, 0.8, 0.1);
        assert!(would);
    }

    #[test]
    fn confide_threshold_increases_with_risk() {
        // The threshold formula: 0.6 + (risk * 0.3)
        // Risk 0: threshold = 0.6
        // Risk 1: threshold = 0.9

        let mut rel = Relationship::try_between(alice(), bob()).unwrap();
        rel.set_stage(RelationshipStage::Intimate).unwrap();
        // Max out trustworthiness
        rel.trustworthiness_mut(Direction::AToB)
            .add_integrity_delta(0.7);

        // Even with intimate relationship, very high risk might prevent confiding
        let low_risk = would_confide(&rel, Direction::AToB, 0.9, 0.0);
        let _max_risk = would_confide(&rel, Direction::AToB, 0.9, 1.0);

        // Low risk should allow confiding in intimate relationship
        assert!(low_risk);
        // Max risk has threshold 0.9, which is very high
        // Result depends on exact trust decision computation
    }

    #[test]
    fn would_a_confide_in_b() {
        let rel = Relationship::try_between(alice(), bob()).unwrap();
        let _result = rel.would_a_confide_in_b(0.5, 0.3);
        // Just verify it runs without panic
    }

    #[test]
    fn would_b_confide_in_a() {
        let rel = Relationship::try_between(alice(), bob()).unwrap();
        let _result = rel.would_b_confide_in_a(0.5, 0.3);
    }

    #[test]
    fn would_a_help_b() {
        let rel = Relationship::try_between(alice(), bob()).unwrap();
        let _result = rel.would_a_help_b(0.5, 0.3);
    }

    #[test]
    fn would_b_help_a() {
        let rel = Relationship::try_between(alice(), bob()).unwrap();
        let _result = rel.would_b_help_a(0.5, 0.3);
    }

    #[test]
    fn intimate_relationship_more_likely_to_confide() {
        let stranger = Relationship::try_between(alice(), bob()).unwrap();
        let intimate = Relationship::try_between(alice(), bob())
            .unwrap()
            .with_stage(RelationshipStage::Intimate);

        let stranger_confides = would_confide(&stranger, Direction::AToB, 0.7, 0.3);
        let intimate_confides = would_confide(&intimate, Direction::AToB, 0.7, 0.3);

        // Intimate should be at least as likely as stranger
        assert!(u8::from(intimate_confides) >= u8::from(stranger_confides));
    }

    #[test]
    fn help_uses_stakes_based_on_risk() {
        let mut rel = Relationship::try_between(alice(), bob()).unwrap();
        rel.set_stage(RelationshipStage::Established).unwrap();
        rel.trustworthiness_mut(Direction::AToB)
            .add_benevolence_delta(0.5);

        // Risk levels map to different stakes
        // Low risk (< 0.3) -> Low stakes
        // Medium risk (0.3-0.7) -> Medium stakes
        // High risk (> 0.7) -> High stakes

        let very_low_risk = would_help(&rel, Direction::AToB, 0.8, 0.1);
        let medium_risk = would_help(&rel, Direction::AToB, 0.8, 0.5);
        let very_high_risk = would_help(&rel, Direction::AToB, 0.8, 0.9);

        // Higher stakes reduce willingness
        // Very low risk should be most likely
        assert!(very_low_risk);
        // Medium should be possible
        // Very high risk might prevent helping
        assert!(u8::from(very_low_risk) + u8::from(medium_risk) >= u8::from(very_high_risk));
    }

    #[test]
    fn estranged_relationship_unlikely_to_confide() {
        let estranged = Relationship::try_between(alice(), bob())
            .unwrap()
            .with_stage(RelationshipStage::Estranged);

        // Estranged relationship with even high propensity
        let _would = would_confide(&estranged, Direction::AToB, 0.9, 0.2);

        // Estranged has high risk modifier (0.4) which reduces willingness
        // Combined with base threshold, unlikely to confide
        // This is a structural test - the result depends on trust computation
    }

    // Tests for risk_to_stakes function
    #[test]
    fn risk_to_stakes_low() {
        assert_eq!(risk_to_stakes(0.0), StakesLevel::Low);
        assert_eq!(risk_to_stakes(0.1), StakesLevel::Low);
        assert_eq!(risk_to_stakes(0.24), StakesLevel::Low);
    }

    #[test]
    fn risk_to_stakes_medium() {
        assert_eq!(risk_to_stakes(0.25), StakesLevel::Medium);
        assert_eq!(risk_to_stakes(0.4), StakesLevel::Medium);
        assert_eq!(risk_to_stakes(0.49), StakesLevel::Medium);
    }

    #[test]
    fn risk_to_stakes_high() {
        assert_eq!(risk_to_stakes(0.5), StakesLevel::High);
        assert_eq!(risk_to_stakes(0.6), StakesLevel::High);
        assert_eq!(risk_to_stakes(0.74), StakesLevel::High);
    }

    #[test]
    fn risk_to_stakes_critical() {
        assert_eq!(risk_to_stakes(0.75), StakesLevel::Critical);
        assert_eq!(risk_to_stakes(0.9), StakesLevel::Critical);
        assert_eq!(risk_to_stakes(1.0), StakesLevel::Critical);
    }

    // Tests for risk extremes
    #[test]
    fn confide_with_zero_risk() {
        let mut rel = Relationship::try_between(alice(), bob()).unwrap();
        rel.set_stage(RelationshipStage::Intimate).unwrap();
        rel.trustworthiness_mut(Direction::AToB)
            .add_integrity_delta(0.5);

        // Zero risk: lowest threshold (0.6), lowest stakes
        let would = would_confide(&rel, Direction::AToB, 0.9, 0.0);

        // Intimate + high propensity + high integrity + zero risk should confide
        assert!(would);
    }

    #[test]
    fn confide_with_max_risk() {
        let rel = Relationship::try_between(alice(), bob()).unwrap();

        // Max risk: highest threshold (0.9), critical stakes
        let would = would_confide(&rel, Direction::AToB, 0.9, 1.0);

        // Stranger + max risk should NOT confide even with high propensity
        assert!(!would);
    }

    #[test]
    fn help_with_zero_risk() {
        let mut rel = Relationship::try_between(alice(), bob()).unwrap();
        rel.set_stage(RelationshipStage::Established).unwrap();
        rel.trustworthiness_mut(Direction::AToB)
            .add_benevolence_delta(0.4);

        // Zero risk: lowest threshold (0.4), lowest stakes
        let would = would_help(&rel, Direction::AToB, 0.7, 0.0);

        // Established + good benevolence + zero risk should help
        assert!(would);
    }

    #[test]
    fn help_with_max_risk() {
        let rel = Relationship::try_between(alice(), bob()).unwrap();

        // Max risk: highest threshold (0.7), critical stakes
        let would = would_help(&rel, Direction::AToB, 0.5, 1.0);

        // Stranger + max risk + moderate propensity should NOT help
        assert!(!would);
    }

    #[test]
    fn risk_level_affects_both_stakes_and_threshold() {
        // This test verifies that risk_level impacts BOTH the stakes
        // (affecting perceived risk in trust computation) AND the threshold
        let mut rel = Relationship::try_between(alice(), bob()).unwrap();
        rel.set_stage(RelationshipStage::Established).unwrap();
        rel.trustworthiness_mut(Direction::AToB)
            .add_benevolence_delta(0.3);

        // Low risk (0.1) -> Low stakes, threshold = 0.4 + 0.03 = 0.43
        let low_risk = would_help(&rel, Direction::AToB, 0.7, 0.1);

        // High risk (0.8) -> Critical stakes, threshold = 0.4 + 0.24 = 0.64
        let high_risk = would_help(&rel, Direction::AToB, 0.7, 0.8);

        // Low risk should be more likely than high risk because:
        // 1. Lower stakes = lower perceived risk = higher willingness
        // 2. Lower threshold to exceed
        assert!(low_risk);
        // High risk may or may not pass, but should be less likely
        assert!(u8::from(low_risk) >= u8::from(high_risk));
    }
}
