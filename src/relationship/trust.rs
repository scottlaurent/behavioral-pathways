//! Trust computation based on Mayer's Integrative Model of Trust.
//!
//! # Key Distinction: Confidence vs Trust (Issue 7-8)
//!
//! Per Mayer's model, these are explicitly different:
//!
//! - **Confidence**: Certainty about trustee's attributes (competence, benevolence, integrity).
//!   This is a *belief state* - how sure are we about what this person is like?
//!
//! - **Trust**: Willingness to be vulnerable to another party based on positive expectations.
//!   This is an *intentional state* - choosing to accept uncertainty and exposure.
//!
//! You can have high confidence in trustee attributes but still low trust (if stakes are too high).
//! You can have low confidence but still trust (if propensity is high and risk is low).
//!
//! # Computation Model
//!
//! ```text
//! Trust = f(propensity, trustworthiness, perceived_risk)
//! ```
//!
//! Where:
//! - **Propensity**: The trustor's general willingness to trust (trait)
//! - **Trustworthiness**: Perceived competence, benevolence, integrity of trustee
//! - **Perceived Risk**: Subjective assessment of potential negative consequences
//!
//! The formula weights these factors based on relationship stage - propensity
//! matters more for strangers, trustworthiness matters more for established
//! relationships.
//!
//! # Output: TrustDecision
//!
//! The output of trust computation is a `TrustDecision`, which represents:
//! - **Willingness values**: These ARE trust per Mayer's definition (willingness to be vulnerable)
//! - **Decision certainty**: How confident we are in our willingness assessment
//! - **Trustee confidence**: How certain we are about the trustee's attributes

use crate::relationship::{RelationshipStage, StakesLevel, TrustDecision, TrustworthinessFactors};

/// Configuration for Trust computation weights.
#[derive(Debug, Clone, PartialEq)]
pub struct TrustWeights {
    /// Weight for trustor's propensity (0-1).
    pub propensity_weight: f32,
    /// Weight for perceived trustworthiness (0-1).
    pub trustworthiness_weight: f32,
    /// Weight for perceived risk (0-1).
    pub risk_weight: f32,
}

impl TrustWeights {
    /// Creates weights from a relationship stage.
    ///
    /// Propensity weight diminishes as relationship matures:
    /// - Stranger: 0.6 propensity, 0.4 trustworthiness
    /// - Acquaintance: 0.4 propensity, 0.6 trustworthiness
    /// - Established: 0.2 propensity, 0.8 trustworthiness
    /// - Intimate: 0.1 propensity, 0.9 trustworthiness
    /// - Estranged: 0.3 propensity, 0.7 trustworthiness (reset toward middle)
    #[must_use]
    pub fn from_stage(stage: RelationshipStage) -> Self {
        let (propensity_weight, trustworthiness_weight) = match stage {
            RelationshipStage::Stranger => (0.6, 0.4),
            RelationshipStage::Acquaintance => (0.4, 0.6),
            RelationshipStage::Established => (0.2, 0.8),
            RelationshipStage::Intimate => (0.1, 0.9),
            RelationshipStage::Estranged => (0.3, 0.7),
        };

        TrustWeights {
            propensity_weight,
            trustworthiness_weight,
            risk_weight: 0.5, // Constant factor
        }
    }
}

impl Default for TrustWeights {
    fn default() -> Self {
        TrustWeights::from_stage(RelationshipStage::Stranger)
    }
}

/// Trust computation encapsulating the Mayer model.
///
/// Trust represents the willingness to be vulnerable to another party
/// based on positive expectations about their behavior. This struct
/// provides the computation logic for deriving trust decisions from
/// the component factors.
///
/// # Mayer Model Formula
///
/// ```text
/// willingness = context_multiplier * (
///                 propensity_weight * propensity
///               + trustworthiness_weight * perceived_trustworthiness
///             )
///             - risk_weight * perceived_risk
/// ```
///
/// # Examples
///
/// ```
/// use behavioral_pathways::relationship::{
///     Trust, TrustworthinessFactors, RelationshipStage, StakesLevel
/// };
///
/// let trustworthiness = TrustworthinessFactors::new();
/// let trust = Trust::new(
///     0.6,                            // trustor propensity
///     &trustworthiness,
///     0.2,                            // base risk
///     RelationshipStage::Acquaintance,
///     0.3,                            // history
///     1.0,                            // context multiplier
/// );
///
/// let decision = trust.compute_decision(StakesLevel::Medium);
/// assert!(decision.task_willingness() > 0.0);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Trust {
    /// The trustor's dispositional trust propensity (0-1).
    propensity: f32,

    /// Perceived competence of the trustee (0-1).
    perceived_competence: f32,

    /// Perceived benevolence of the trustee (0-1).
    perceived_benevolence: f32,

    /// Perceived integrity of the trustee (0-1).
    perceived_integrity: f32,

    /// Base perceived risk (0-1), before stakes adjustment.
    base_risk: f32,

    /// Weights for computation.
    weights: TrustWeights,

    /// Stage risk modifier (multiplier on base risk).
    stage_risk_modifier: f32,

    /// Relationship history (0-1), for confidence calculation.
    history: f32,

    /// Context multiplier (institutional/cultural moderation).
    context_multiplier: f32,

    /// Relationship stage, for confidence calculation.
    stage: RelationshipStage,
}

impl Trust {
    /// Creates a new Trust computation from component factors.
    ///
    /// # Arguments
    ///
    /// * `propensity` - Trustor's dispositional trust propensity (0-1)
    /// * `trustworthiness` - Perceived trustworthiness factors
    /// * `base_risk` - Base perceived risk before stakes (0-1)
    /// * `stage` - Relationship stage (affects weights)
    /// * `history` - Relationship history (0-1), for confidence
    /// * `context_multiplier` - Context moderation multiplier
    #[must_use]
    pub fn new(
        propensity: f32,
        trustworthiness: &TrustworthinessFactors,
        base_risk: f32,
        stage: RelationshipStage,
        history: f32,
        context_multiplier: f32,
    ) -> Self {
        Trust {
            propensity: propensity.clamp(0.0, 1.0),
            perceived_competence: trustworthiness.competence_effective(),
            perceived_benevolence: trustworthiness.benevolence_effective(),
            perceived_integrity: trustworthiness.integrity_effective(),
            base_risk: base_risk.clamp(0.0, 1.0),
            weights: TrustWeights::from_stage(stage),
            stage_risk_modifier: stage.risk_modifier(),
            history: history.clamp(0.0, 1.0),
            context_multiplier: context_multiplier.clamp(0.0, 2.0),
            stage,
        }
    }

    /// Returns the trustor's propensity.
    #[must_use]
    pub fn propensity(&self) -> f32 {
        self.propensity
    }

    /// Returns the perceived competence.
    #[must_use]
    pub fn perceived_competence(&self) -> f32 {
        self.perceived_competence
    }

    /// Returns the perceived benevolence.
    #[must_use]
    pub fn perceived_benevolence(&self) -> f32 {
        self.perceived_benevolence
    }

    /// Returns the perceived integrity.
    #[must_use]
    pub fn perceived_integrity(&self) -> f32 {
        self.perceived_integrity
    }

    /// Returns the base risk.
    #[must_use]
    pub fn base_risk(&self) -> f32 {
        self.base_risk
    }

    /// Returns the context multiplier.
    #[must_use]
    pub fn context_multiplier(&self) -> f32 {
        self.context_multiplier
    }

    /// Returns the computation weights.
    #[must_use]
    pub fn weights(&self) -> &TrustWeights {
        &self.weights
    }

    /// Computes the perceived risk adjusted for stakes level.
    ///
    /// Formula: `base_risk + stakes_contribution + stage_modifier`
    ///
    /// This uses additive composition, matching PerceivedRisk behavior.
    #[must_use]
    pub fn compute_risk(&self, stakes: StakesLevel) -> f32 {
        let stakes_contribution = stakes.risk_contribution();

        // Additive composition: base + stakes + stage modifier
        (self.base_risk + stakes_contribution + self.stage_risk_modifier).clamp(0.0, 1.0)
    }

    /// Computes willingness for a domain using the Mayer formula.
    ///
    /// Formula:
    /// ```text
    /// willingness = context_multiplier * (
    ///                 propensity_weight * propensity
    ///               + trustworthiness_weight * domain_trustworthiness
    ///             )
    ///             - risk_weight * perceived_risk
    /// ```
    fn compute_willingness(&self, domain_trustworthiness: f32, perceived_risk: f32) -> f32 {
        let base = self.weights.propensity_weight * self.propensity
            + self.weights.trustworthiness_weight * domain_trustworthiness;
        let willingness =
            (base * self.context_multiplier) - self.weights.risk_weight * perceived_risk;

        willingness.clamp(0.0, 1.0)
    }

    /// Computes certainty in the trust decision (decision_certainty).
    ///
    /// Higher for established relationships with consistent history.
    /// This is distinct from trustee_confidence (certainty about attributes).
    fn compute_decision_certainty(&self) -> f32 {
        let stage_certainty = match self.stage {
            RelationshipStage::Stranger => 0.1,
            RelationshipStage::Acquaintance => 0.3,
            RelationshipStage::Established => 0.6,
            RelationshipStage::Intimate => 0.9,
            RelationshipStage::Estranged => 0.5, // Know them, but trust is broken
        };

        (self.history * 0.3 + stage_certainty * 0.7).clamp(0.0, 1.0)
    }

    /// Computes confidence in the trustee's attributes (trustee_confidence).
    ///
    /// Per Mayer's model, this is a belief state representing certainty about
    /// the trustee's competence, benevolence, and integrity. This is distinct
    /// from trust itself (willingness to be vulnerable).
    ///
    /// Factors:
    /// - Relationship stage (more interactions = more data)
    /// - History consistency (stable observations = more confidence)
    /// - Time known (longer relationships have more confidence)
    fn compute_trustee_confidence(&self) -> f32 {
        let stage_data = match self.stage {
            RelationshipStage::Stranger => 0.1,    // Very little data
            RelationshipStage::Acquaintance => 0.4, // Some observations
            RelationshipStage::Established => 0.7,  // Substantial data
            RelationshipStage::Intimate => 0.9,     // Deep knowledge
            RelationshipStage::Estranged => 0.8,    // Had data, still know them
        };

        // Combine stage data with history for trustee confidence
        (self.history * 0.4 + stage_data * 0.6).clamp(0.0, 1.0)
    }

    /// Computes a TrustDecision for the given stakes level.
    ///
    /// Uses the Mayer model to compute willingness in each domain:
    /// - Task willingness uses competence
    /// - Support willingness uses benevolence
    /// - Disclosure willingness uses integrity
    ///
    /// The willingness values ARE trust per Mayer's definition (willingness
    /// to be vulnerable). The computation determines these based on
    /// propensity, trustworthiness factors, and perceived risk.
    ///
    /// # Arguments
    ///
    /// * `stakes` - The stakes level affecting perceived risk
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::relationship::{
    ///     Trust, TrustworthinessFactors, RelationshipStage, StakesLevel
    /// };
    ///
    /// let trustworthiness = TrustworthinessFactors::new();
    /// let trust = Trust::new(0.6, &trustworthiness, 0.2, RelationshipStage::Established, 0.5, 1.0);
    ///
    /// let decision = trust.compute_decision(StakesLevel::Medium);
    /// assert!(decision.task_willingness() > 0.0);
    /// ```
    #[must_use]
    pub fn compute_decision(&self, stakes: StakesLevel) -> TrustDecision {
        let perceived_risk = self.compute_risk(stakes);

        let task_willingness = self.compute_willingness(self.perceived_competence, perceived_risk);
        let support_willingness =
            self.compute_willingness(self.perceived_benevolence, perceived_risk);
        let disclosure_willingness =
            self.compute_willingness(self.perceived_integrity, perceived_risk);
        let decision_certainty = self.compute_decision_certainty();
        let trustee_confidence = self.compute_trustee_confidence();

        TrustDecision::new(
            task_willingness,
            support_willingness,
            disclosure_willingness,
            decision_certainty,
            trustee_confidence,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_trustworthiness() -> TrustworthinessFactors {
        TrustworthinessFactors::new()
    }

    #[test]
    fn trust_weights_from_stranger() {
        let weights = TrustWeights::from_stage(RelationshipStage::Stranger);
        assert!((weights.propensity_weight - 0.6).abs() < f32::EPSILON);
        assert!((weights.trustworthiness_weight - 0.4).abs() < f32::EPSILON);
        assert!((weights.risk_weight - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn trust_weights_from_acquaintance() {
        let weights = TrustWeights::from_stage(RelationshipStage::Acquaintance);
        assert!((weights.propensity_weight - 0.4).abs() < f32::EPSILON);
        assert!((weights.trustworthiness_weight - 0.6).abs() < f32::EPSILON);
    }

    #[test]
    fn trust_weights_from_established() {
        let weights = TrustWeights::from_stage(RelationshipStage::Established);
        assert!((weights.propensity_weight - 0.2).abs() < f32::EPSILON);
        assert!((weights.trustworthiness_weight - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn trust_weights_from_intimate() {
        let weights = TrustWeights::from_stage(RelationshipStage::Intimate);
        assert!((weights.propensity_weight - 0.1).abs() < f32::EPSILON);
        assert!((weights.trustworthiness_weight - 0.9).abs() < f32::EPSILON);
    }

    #[test]
    fn trust_weights_from_estranged() {
        let weights = TrustWeights::from_stage(RelationshipStage::Estranged);
        assert!((weights.propensity_weight - 0.3).abs() < f32::EPSILON);
        assert!((weights.trustworthiness_weight - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn trust_weights_default() {
        let weights = TrustWeights::default();
        assert!((weights.propensity_weight - 0.6).abs() < f32::EPSILON);
    }

    #[test]
    fn trust_creation() {
        let tw = default_trustworthiness();
        let trust = Trust::new(0.6, &tw, 0.2, RelationshipStage::Acquaintance, 0.5, 1.0);

        assert!((trust.propensity() - 0.6).abs() < f32::EPSILON);
        assert!((trust.base_risk() - 0.2).abs() < f32::EPSILON);
        assert!((trust.context_multiplier() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn trust_clamps_values() {
        let tw = default_trustworthiness();
        let trust = Trust::new(1.5, &tw, -0.5, RelationshipStage::Stranger, 2.0, 3.0);

        assert!((trust.propensity() - 1.0).abs() < f32::EPSILON);
        assert!(trust.base_risk().abs() < f32::EPSILON);
        assert!((trust.context_multiplier() - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn trust_accessors() {
        let tw = default_trustworthiness();
        let trust = Trust::new(0.5, &tw, 0.3, RelationshipStage::Established, 0.4, 1.0);

        assert!(trust.perceived_competence() > 0.0);
        assert!(trust.perceived_benevolence() > 0.0);
        assert!(trust.perceived_integrity() > 0.0);
        assert!(trust.weights().propensity_weight > 0.0);
    }

    #[test]
    fn compute_risk_scales_with_stakes() {
        let tw = default_trustworthiness();
        // Use Established stage (risk_modifier = 0.0) and low base_risk
        // to avoid clamping and see the stakes difference clearly
        let trust = Trust::new(0.5, &tw, 0.1, RelationshipStage::Established, 0.0, 1.0);

        let low = trust.compute_risk(StakesLevel::Low);
        let medium = trust.compute_risk(StakesLevel::Medium);
        let high = trust.compute_risk(StakesLevel::High);
        let critical = trust.compute_risk(StakesLevel::Critical);

        // Low: 0.1 + 0.0 + 0.0 = 0.1
        // Medium: 0.1 + 0.2 + 0.0 = 0.3
        // High: 0.1 + 0.4 + 0.0 = 0.5
        // Critical: 0.1 + 0.6 + 0.0 = 0.7
        assert!(low < medium);
        assert!(medium < high);
        assert!(high < critical);
    }

    #[test]
    fn context_multiplier_scales_willingness() {
        let tw = default_trustworthiness();
        let baseline = Trust::new(0.6, &tw, 0.1, RelationshipStage::Established, 0.5, 1.0);
        let constrained = Trust::new(0.6, &tw, 0.1, RelationshipStage::Established, 0.5, 0.5);

        let baseline_decision = baseline.compute_decision(StakesLevel::Low);
        let constrained_decision = constrained.compute_decision(StakesLevel::Low);

        assert!(constrained_decision.task_willingness() < baseline_decision.task_willingness());
    }

    #[test]
    fn compute_risk_clamps_to_one() {
        let tw = default_trustworthiness();
        // High base risk with critical stakes
        let trust = Trust::new(0.5, &tw, 0.9, RelationshipStage::Estranged, 0.0, 1.0);

        let risk = trust.compute_risk(StakesLevel::Critical);
        assert!(risk <= 1.0);
    }

    #[test]
    fn compute_decision_returns_valid_values() {
        let tw = default_trustworthiness();
        let trust = Trust::new(0.6, &tw, 0.2, RelationshipStage::Established, 0.5, 1.0);

        let decision = trust.compute_decision(StakesLevel::Medium);

        assert!(decision.task_willingness() >= 0.0);
        assert!(decision.task_willingness() <= 1.0);
        assert!(decision.support_willingness() >= 0.0);
        assert!(decision.support_willingness() <= 1.0);
        assert!(decision.disclosure_willingness() >= 0.0);
        assert!(decision.disclosure_willingness() <= 1.0);
        assert!(decision.confidence() >= 0.0);
        assert!(decision.confidence() <= 1.0);
    }

    #[test]
    fn higher_propensity_increases_willingness() {
        let tw = default_trustworthiness();
        let low_prop = Trust::new(0.2, &tw, 0.2, RelationshipStage::Stranger, 0.0, 1.0);
        let high_prop = Trust::new(0.9, &tw, 0.2, RelationshipStage::Stranger, 0.0, 1.0);

        let low_decision = low_prop.compute_decision(StakesLevel::Low);
        let high_decision = high_prop.compute_decision(StakesLevel::Low);

        // For strangers, propensity weight is high (0.6), so difference should be noticeable
        assert!(high_decision.task_willingness() > low_decision.task_willingness());
    }

    #[test]
    fn higher_risk_decreases_willingness() {
        let tw = default_trustworthiness();
        let low_risk = Trust::new(0.5, &tw, 0.1, RelationshipStage::Established, 0.5, 1.0);
        let high_risk = Trust::new(0.5, &tw, 0.8, RelationshipStage::Established, 0.5, 1.0);

        let low_decision = low_risk.compute_decision(StakesLevel::Medium);
        let high_decision = high_risk.compute_decision(StakesLevel::Medium);

        assert!(low_decision.task_willingness() > high_decision.task_willingness());
    }

    #[test]
    fn intimate_stage_has_higher_confidence() {
        let tw = default_trustworthiness();
        let stranger = Trust::new(0.5, &tw, 0.2, RelationshipStage::Stranger, 0.0, 1.0);
        let intimate = Trust::new(0.5, &tw, 0.2, RelationshipStage::Intimate, 0.5, 1.0);

        let stranger_decision = stranger.compute_decision(StakesLevel::Low);
        let intimate_decision = intimate.compute_decision(StakesLevel::Low);

        assert!(intimate_decision.confidence() > stranger_decision.confidence());
    }

    #[test]
    fn confidence_matches_acquaintance_and_estranged_stages() {
        let tw = default_trustworthiness();
        let acquaintance = Trust::new(0.5, &tw, 0.2, RelationshipStage::Acquaintance, 0.0, 1.0);
        let estranged = Trust::new(0.5, &tw, 0.2, RelationshipStage::Estranged, 0.0, 1.0);

        let acquaintance_confidence = acquaintance.compute_decision(StakesLevel::Low).confidence();
        let estranged_confidence = estranged.compute_decision(StakesLevel::Low).confidence();

        assert!((acquaintance_confidence - 0.21).abs() < 1e-6);
        assert!((estranged_confidence - 0.35).abs() < 1e-6);
    }

    #[test]
    fn trust_weights_clone_and_equality() {
        let w1 = TrustWeights::from_stage(RelationshipStage::Established);
        let w2 = w1.clone();
        assert_eq!(w1, w2);
    }

    #[test]
    fn trust_clone_and_equality() {
        let tw = default_trustworthiness();
        let t1 = Trust::new(0.5, &tw, 0.3, RelationshipStage::Acquaintance, 0.4, 1.0);
        let t2 = t1.clone();
        assert_eq!(t1, t2);
    }

    #[test]
    fn trust_debug_format() {
        let tw = default_trustworthiness();
        let trust = Trust::new(0.5, &tw, 0.3, RelationshipStage::Acquaintance, 0.4, 1.0);
        let debug = format!("{:?}", trust);
        assert!(debug.contains("Trust"));
    }

    #[test]
    fn trust_weights_debug_format() {
        let weights = TrustWeights::from_stage(RelationshipStage::Stranger);
        let debug = format!("{:?}", weights);
        assert!(debug.contains("TrustWeights"));
    }
}
