//! Trust decision computation based on Mayer's trust model.
//!
//! A trust decision represents the willingness to be vulnerable in
//! specific domains, computed from propensity, perceived trustworthiness,
//! and perceived risk.
//!
//! # Key Distinction: Confidence vs Trust
//!
//! Per Mayer's model, these are explicitly different constructs:
//!
//! - **Trust (willingness)**: The intentional choice to accept vulnerability.
//!   This is what the willingness fields represent - they ARE trust per Mayer.
//!
//! - **Trustee Confidence**: Certainty about the trustee's attributes
//!   (competence, benevolence, integrity). Separate from trust itself.
//!
//! - **Decision Certainty**: How confident we are in our willingness assessment.
//!   Based on relationship history and stage.

/// A computed trust decision for a specific trustor-trustee relationship.
///
/// Trust decisions encode the trustor's willingness to be vulnerable
/// in different domains:
/// - Task: Willingness to delegate tasks
/// - Support: Willingness to seek/provide emotional support
/// - Disclosure: Willingness to share vulnerabilities and secrets
///
/// **Important**: The willingness values ARE trust per Mayer's definition.
/// Trust is the willingness to be vulnerable, not a calculation that produces
/// something else. The computation determines these willingness values based
/// on propensity, trustworthiness, and risk.
///
/// # Confidence vs Trust (Mayer's Distinction)
///
/// - `trustee_confidence`: How certain we are about the trustee's attributes
/// - `decision_certainty`: How confident we are in our willingness assessment
/// - Willingness values: The trust itself (accepting vulnerability)
///
/// You can have high trustee_confidence but low willingness (high stakes).
/// You can have low trustee_confidence but still trust (high propensity).
///
/// # Trust Computation Formula
///
/// ```text
/// willingness = propensity_weight * propensity
///             + trustworthiness_weight * perceived_trustworthiness
///             - risk_weight * perceived_risk
/// ```
///
/// Where weights depend on relationship stage (propensity matters more
/// for strangers, trustworthiness matters more for established relationships).
///
/// # Examples
///
/// ```
/// use behavioral_pathways::relationship::TrustDecision;
///
/// let decision = TrustDecision::new(0.6, 0.7, 0.5, 0.8, 0.7);
/// assert!(decision.task_willingness() > 0.5);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TrustDecision {
    /// Willingness to delegate tasks to this person.
    /// Range: 0 (never) to 1 (completely willing)
    /// This IS trust in the task domain per Mayer's definition.
    task_willingness: f32,

    /// Willingness to seek or provide emotional support.
    /// Range: 0 (never) to 1 (completely willing)
    /// This IS trust in the support domain per Mayer's definition.
    support_willingness: f32,

    /// Willingness to share vulnerabilities and secrets.
    /// Range: 0 (never) to 1 (completely willing)
    /// This IS trust in the disclosure domain per Mayer's definition.
    disclosure_willingness: f32,

    /// Certainty in the willingness assessment.
    /// Range: 0 (uncertain) to 1 (highly certain)
    /// Higher for established relationships with consistent history.
    /// Distinct from trustee_confidence (which is certainty about attributes).
    decision_certainty: f32,

    /// Confidence in the trustee's attributes (competence, benevolence, integrity).
    /// Range: 0 (uncertain) to 1 (highly confident)
    /// This is a belief state, distinct from trust (willingness to be vulnerable).
    /// Per Mayer: you can be confident about someone's attributes but still not trust
    /// them if the stakes are too high.
    trustee_confidence: f32,
}

impl TrustDecision {
    /// Creates a new TrustDecision with specified values.
    ///
    /// # Arguments
    ///
    /// * `task_willingness` - Willingness to delegate tasks (0-1)
    /// * `support_willingness` - Willingness for emotional support (0-1)
    /// * `disclosure_willingness` - Willingness to share secrets (0-1)
    /// * `decision_certainty` - Certainty in the willingness assessment (0-1)
    /// * `trustee_confidence` - Confidence in trustee's attributes (0-1)
    ///
    /// Values are clamped to [0, 1].
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::relationship::TrustDecision;
    ///
    /// let decision = TrustDecision::new(0.6, 0.7, 0.5, 0.8, 0.7);
    /// assert!((decision.task_willingness() - 0.6).abs() < f32::EPSILON);
    /// ```
    #[must_use]
    pub fn new(
        task_willingness: f32,
        support_willingness: f32,
        disclosure_willingness: f32,
        decision_certainty: f32,
        trustee_confidence: f32,
    ) -> Self {
        TrustDecision {
            task_willingness: task_willingness.clamp(0.0, 1.0),
            support_willingness: support_willingness.clamp(0.0, 1.0),
            disclosure_willingness: disclosure_willingness.clamp(0.0, 1.0),
            decision_certainty: decision_certainty.clamp(0.0, 1.0),
            trustee_confidence: trustee_confidence.clamp(0.0, 1.0),
        }
    }

    /// Creates a TrustDecision with zero willingness in all domains.
    ///
    /// Used for strangers or broken relationships.
    #[must_use]
    pub fn no_trust() -> Self {
        TrustDecision {
            task_willingness: 0.0,
            support_willingness: 0.0,
            disclosure_willingness: 0.0,
            decision_certainty: 0.0,
            trustee_confidence: 0.0,
        }
    }

    /// Creates a TrustDecision with full willingness in all domains.
    ///
    /// Used for highly trusted relationships.
    #[must_use]
    pub fn full_trust() -> Self {
        TrustDecision {
            task_willingness: 1.0,
            support_willingness: 1.0,
            disclosure_willingness: 1.0,
            decision_certainty: 1.0,
            trustee_confidence: 1.0,
        }
    }

    /// Returns the willingness to delegate tasks.
    ///
    /// This IS trust in the task domain per Mayer's definition.
    #[must_use]
    pub fn task_willingness(&self) -> f32 {
        self.task_willingness
    }

    /// Returns the willingness for emotional support.
    ///
    /// This IS trust in the support domain per Mayer's definition.
    #[must_use]
    pub fn support_willingness(&self) -> f32 {
        self.support_willingness
    }

    /// Returns the willingness to share secrets/vulnerabilities.
    ///
    /// This IS trust in the disclosure domain per Mayer's definition.
    #[must_use]
    pub fn disclosure_willingness(&self) -> f32 {
        self.disclosure_willingness
    }

    /// Returns the certainty in the willingness assessment.
    ///
    /// This is distinct from trustee_confidence. Decision certainty
    /// reflects how confident we are in our willingness evaluation,
    /// based on relationship history and stage.
    #[must_use]
    pub fn decision_certainty(&self) -> f32 {
        self.decision_certainty
    }

    /// Returns the confidence in the trustee's attributes.
    ///
    /// Per Mayer's model, this is a belief state (certainty about
    /// competence, benevolence, integrity), distinct from trust itself
    /// (willingness to be vulnerable). You can have high confidence
    /// in someone's attributes but still not trust them if stakes are too high.
    #[must_use]
    pub fn trustee_confidence(&self) -> f32 {
        self.trustee_confidence
    }

    /// Returns the confidence in this decision.
    ///
    /// Deprecated: Use `decision_certainty()` for clarity. This method
    /// remains for backward compatibility.
    #[must_use]
    pub fn confidence(&self) -> f32 {
        self.decision_certainty
    }

    /// Returns the overall willingness (average of all domains).
    ///
    /// **Deprecated**: Per Mayer's model, trust is domain-specific. An entity
    /// can be trusted for tasks but not emotional support. Averaging across
    /// domains loses this critical distinction. Query domain-specific
    /// willingness instead (task_willingness, support_willingness,
    /// disclosure_willingness). If a single score is needed for display,
    /// compute it at the consumer level with context-appropriate weights.
    #[must_use]
    #[deprecated(
        since = "0.2.0",
        note = "Trust is domain-specific per Mayer's model. Use domain-specific methods instead."
    )]
    pub fn overall_willingness(&self) -> f32 {
        (self.task_willingness + self.support_willingness + self.disclosure_willingness) / 3.0
    }

    /// Returns true if task willingness exceeds the threshold.
    ///
    /// # Arguments
    ///
    /// * `threshold` - The minimum willingness required (0-1)
    #[must_use]
    pub fn would_delegate_task(&self, threshold: f32) -> bool {
        self.task_willingness > threshold
    }

    /// Returns true if support willingness exceeds the threshold.
    ///
    /// # Arguments
    ///
    /// * `threshold` - The minimum willingness required (0-1)
    #[must_use]
    pub fn would_seek_support(&self, threshold: f32) -> bool {
        self.support_willingness > threshold
    }

    /// Returns true if disclosure willingness exceeds the threshold.
    ///
    /// # Arguments
    ///
    /// * `threshold` - The minimum willingness required (0-1)
    #[must_use]
    pub fn would_disclose(&self, threshold: f32) -> bool {
        self.disclosure_willingness > threshold
    }

    /// Returns true if all willingness values exceed the threshold.
    ///
    /// Used to check for "fully trusting" relationships.
    #[must_use]
    pub fn fully_willing(&self, threshold: f32) -> bool {
        self.task_willingness > threshold
            && self.support_willingness > threshold
            && self.disclosure_willingness > threshold
    }

    /// Returns true if any willingness value exceeds the threshold.
    #[must_use]
    pub fn any_willing(&self, threshold: f32) -> bool {
        self.task_willingness > threshold
            || self.support_willingness > threshold
            || self.disclosure_willingness > threshold
    }
}

impl Default for TrustDecision {
    fn default() -> Self {
        // Moderate starting point
        TrustDecision::new(0.3, 0.3, 0.2, 0.3, 0.3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_with_values() {
        let decision = TrustDecision::new(0.6, 0.7, 0.5, 0.8, 0.7);
        assert!((decision.task_willingness() - 0.6).abs() < f32::EPSILON);
        assert!((decision.support_willingness() - 0.7).abs() < f32::EPSILON);
        assert!((decision.disclosure_willingness() - 0.5).abs() < f32::EPSILON);
        assert!((decision.decision_certainty() - 0.8).abs() < f32::EPSILON);
        assert!((decision.trustee_confidence() - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn new_clamps_values() {
        let decision = TrustDecision::new(1.5, -0.5, 2.0, -1.0, 2.0);
        assert!((decision.task_willingness() - 1.0).abs() < f32::EPSILON);
        assert!(decision.support_willingness().abs() < f32::EPSILON);
        assert!((decision.disclosure_willingness() - 1.0).abs() < f32::EPSILON);
        assert!(decision.decision_certainty().abs() < f32::EPSILON);
        assert!((decision.trustee_confidence() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn no_trust_is_zero() {
        let decision = TrustDecision::no_trust();
        assert!(decision.task_willingness().abs() < f32::EPSILON);
        assert!(decision.support_willingness().abs() < f32::EPSILON);
        assert!(decision.disclosure_willingness().abs() < f32::EPSILON);
        assert!(decision.decision_certainty().abs() < f32::EPSILON);
        assert!(decision.trustee_confidence().abs() < f32::EPSILON);
    }

    #[test]
    fn full_trust_is_one() {
        let decision = TrustDecision::full_trust();
        assert!((decision.task_willingness() - 1.0).abs() < f32::EPSILON);
        assert!((decision.support_willingness() - 1.0).abs() < f32::EPSILON);
        assert!((decision.disclosure_willingness() - 1.0).abs() < f32::EPSILON);
        assert!((decision.decision_certainty() - 1.0).abs() < f32::EPSILON);
        assert!((decision.trustee_confidence() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    #[allow(deprecated)]
    fn overall_willingness_is_average() {
        let decision = TrustDecision::new(0.3, 0.6, 0.9, 0.5, 0.5);
        let expected = (0.3 + 0.6 + 0.9) / 3.0;
        assert!((decision.overall_willingness() - expected).abs() < f32::EPSILON);
    }

    #[test]
    fn would_delegate_task() {
        let decision = TrustDecision::new(0.5, 0.3, 0.2, 0.5, 0.5);
        assert!(decision.would_delegate_task(0.4));
        assert!(!decision.would_delegate_task(0.6));
    }

    #[test]
    fn would_seek_support() {
        let decision = TrustDecision::new(0.3, 0.7, 0.2, 0.5, 0.5);
        assert!(decision.would_seek_support(0.6));
        assert!(!decision.would_seek_support(0.8));
    }

    #[test]
    fn would_disclose() {
        let decision = TrustDecision::new(0.3, 0.3, 0.8, 0.5, 0.5);
        assert!(decision.would_disclose(0.7));
        assert!(!decision.would_disclose(0.9));
    }

    #[test]
    fn fully_willing() {
        let high = TrustDecision::new(0.8, 0.8, 0.8, 0.9, 0.9);
        let mixed = TrustDecision::new(0.8, 0.3, 0.8, 0.5, 0.5);

        assert!(high.fully_willing(0.7));
        assert!(!mixed.fully_willing(0.7));
    }

    #[test]
    fn any_willing() {
        let low = TrustDecision::new(0.2, 0.2, 0.2, 0.3, 0.3);
        let one_high = TrustDecision::new(0.2, 0.9, 0.2, 0.5, 0.5);

        assert!(!low.any_willing(0.5));
        assert!(one_high.any_willing(0.5));
    }

    #[test]
    fn default_is_moderate() {
        let decision = TrustDecision::default();
        assert!(decision.task_willingness() > 0.0);
        assert!(decision.task_willingness() < 0.5);
    }

    #[test]
    fn clone_and_equality() {
        let d1 = TrustDecision::new(0.5, 0.6, 0.7, 0.8, 0.7);
        let d2 = d1.clone();
        assert_eq!(d1, d2);
    }

    #[test]
    fn debug_format() {
        let decision = TrustDecision::new(0.5, 0.6, 0.7, 0.8, 0.7);
        let debug = format!("{:?}", decision);
        assert!(debug.contains("TrustDecision"));
    }

    #[test]
    fn confidence_backward_compat() {
        let decision = TrustDecision::new(0.5, 0.6, 0.7, 0.8, 0.7);
        // confidence() returns decision_certainty for backward compatibility
        assert!((decision.confidence() - decision.decision_certainty()).abs() < f32::EPSILON);
    }

    #[test]
    fn trustee_confidence_distinct_from_decision_certainty() {
        // High confidence in trustee attributes, lower certainty in decision
        let decision = TrustDecision::new(0.5, 0.6, 0.7, 0.4, 0.9);
        assert!((decision.trustee_confidence() - 0.9).abs() < f32::EPSILON);
        assert!((decision.decision_certainty() - 0.4).abs() < f32::EPSILON);
    }
}
