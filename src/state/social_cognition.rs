//! Social cognition state layer.
//!
//! This module captures interpersonal beliefs and perceptions that feed
//! ITS computations, distinct from physiological needs.

use crate::state::StateValue;
use crate::types::Duration;
use serde::{Deserialize, Serialize};

/// Social cognition dimensions.
///
/// These represent beliefs about social belonging, burdensomeness,
/// and self-perception that influence mental health.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SocialCognition {
    /// Social isolation - feeling disconnected from others.
    /// Range: 0 (connected) to 1 (isolated)
    loneliness: StateValue,

    /// Belief that others genuinely care.
    /// Range: 0 (nobody cares) to 1 (strongly cared for)
    perceived_reciprocal_caring: StateValue,

    /// Belief of being a burden or liability to others.
    /// Range: 0 (not a burden) to 1 (extreme burden)
    perceived_liability: StateValue,

    /// Active self-loathing (distinct from low self-worth).
    /// Range: 0 (self-accepting) to 1 (extreme self-hatred)
    self_hate: StateValue,

    /// Sense of competence and efficacy.
    /// Range: 0 (ineffective) to 1 (highly competent)
    perceived_competence: StateValue,
}

impl SocialCognition {
    /// Default decay half-life for loneliness (1 day).
    pub(crate) const LONELINESS_DECAY_HALF_LIFE: Duration = Duration::days(1);

    /// Default decay half-life for perceived reciprocal caring (2 days).
    pub(crate) const PERCEIVED_RECIPROCAL_CARING_DECAY_HALF_LIFE: Duration = Duration::days(2);

    /// Default decay half-life for perceived liability (3 days).
    pub(crate) const PERCEIVED_LIABILITY_DECAY_HALF_LIFE: Duration = Duration::days(3);

    /// Default decay half-life for self-hate (3 days).
    pub(crate) const SELF_HATE_DECAY_HALF_LIFE: Duration = Duration::days(3);

    /// Default decay half-life for perceived competence (7 days).
    pub(crate) const PERCEIVED_COMPETENCE_DECAY_HALF_LIFE: Duration = Duration::days(7);

    /// Creates a new SocialCognition with default base values.
    #[must_use]
    pub fn new() -> Self {
        SocialCognition {
            loneliness: StateValue::new(0.2)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Self::LONELINESS_DECAY_HALF_LIFE),
            perceived_reciprocal_caring: StateValue::new(0.6)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Self::PERCEIVED_RECIPROCAL_CARING_DECAY_HALF_LIFE),
            perceived_liability: StateValue::new(0.0)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Self::PERCEIVED_LIABILITY_DECAY_HALF_LIFE),
            self_hate: StateValue::new(0.1)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Self::SELF_HATE_DECAY_HALF_LIFE),
            perceived_competence: StateValue::new(0.5)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Self::PERCEIVED_COMPETENCE_DECAY_HALF_LIFE),
        }
    }

    // Builder methods for base values

    /// Sets the base loneliness.
    #[must_use]
    pub fn with_loneliness_base(mut self, value: f32) -> Self {
        self.loneliness.set_base(value);
        self
    }

    /// Sets the base perceived reciprocal caring.
    #[must_use]
    pub fn with_perceived_reciprocal_caring_base(mut self, value: f32) -> Self {
        self.perceived_reciprocal_caring.set_base(value);
        self
    }

    /// Sets the base perceived liability.
    #[must_use]
    pub fn with_perceived_liability_base(mut self, value: f32) -> Self {
        self.perceived_liability.set_base(value);
        self
    }

    /// Sets the base self-hate.
    #[must_use]
    pub fn with_self_hate_base(mut self, value: f32) -> Self {
        self.self_hate.set_base(value);
        self
    }

    /// Sets the base perceived competence.
    #[must_use]
    pub fn with_perceived_competence_base(mut self, value: f32) -> Self {
        self.perceived_competence.set_base(value);
        self
    }

    // Effective value accessors

    /// Returns the effective loneliness (base + delta).
    #[must_use]
    pub fn loneliness_effective(&self) -> f32 {
        self.loneliness.effective()
    }

    /// Returns the effective perceived reciprocal caring.
    #[must_use]
    pub fn perceived_reciprocal_caring_effective(&self) -> f32 {
        self.perceived_reciprocal_caring.effective()
    }

    /// Returns the effective perceived liability.
    #[must_use]
    pub fn perceived_liability_effective(&self) -> f32 {
        self.perceived_liability.effective()
    }

    /// Returns the effective self-hate.
    #[must_use]
    pub fn self_hate_effective(&self) -> f32 {
        self.self_hate.effective()
    }

    /// Returns the effective perceived competence.
    #[must_use]
    pub fn perceived_competence_effective(&self) -> f32 {
        self.perceived_competence.effective()
    }

    // Base accessors

    /// Returns the base loneliness.
    #[must_use]
    pub fn loneliness_base(&self) -> f32 {
        self.loneliness.base()
    }

    /// Returns the base perceived reciprocal caring.
    #[must_use]
    pub fn perceived_reciprocal_caring_base(&self) -> f32 {
        self.perceived_reciprocal_caring.base()
    }

    /// Returns the base perceived liability.
    #[must_use]
    pub fn perceived_liability_base(&self) -> f32 {
        self.perceived_liability.base()
    }

    /// Returns the base self-hate.
    #[must_use]
    pub fn self_hate_base(&self) -> f32 {
        self.self_hate.base()
    }

    /// Returns the base perceived competence.
    #[must_use]
    pub fn perceived_competence_base(&self) -> f32 {
        self.perceived_competence.base()
    }

    // StateValue references

    /// Returns a reference to the loneliness StateValue.
    #[must_use]
    pub fn loneliness(&self) -> &StateValue {
        &self.loneliness
    }

    /// Returns a reference to the perceived reciprocal caring StateValue.
    #[must_use]
    pub fn perceived_reciprocal_caring(&self) -> &StateValue {
        &self.perceived_reciprocal_caring
    }

    /// Returns a reference to the perceived liability StateValue.
    #[must_use]
    pub fn perceived_liability(&self) -> &StateValue {
        &self.perceived_liability
    }

    /// Returns a reference to the self-hate StateValue.
    #[must_use]
    pub fn self_hate(&self) -> &StateValue {
        &self.self_hate
    }

    /// Returns a reference to the perceived competence StateValue.
    #[must_use]
    pub fn perceived_competence(&self) -> &StateValue {
        &self.perceived_competence
    }

    /// Returns a mutable reference to the loneliness StateValue.
    pub fn loneliness_mut(&mut self) -> &mut StateValue {
        &mut self.loneliness
    }

    /// Returns a mutable reference to the perceived reciprocal caring StateValue.
    pub fn perceived_reciprocal_caring_mut(&mut self) -> &mut StateValue {
        &mut self.perceived_reciprocal_caring
    }

    /// Returns a mutable reference to the perceived liability StateValue.
    pub fn perceived_liability_mut(&mut self) -> &mut StateValue {
        &mut self.perceived_liability
    }

    /// Returns a mutable reference to the self-hate StateValue.
    pub fn self_hate_mut(&mut self) -> &mut StateValue {
        &mut self.self_hate
    }

    /// Returns a mutable reference to the perceived competence StateValue.
    pub fn perceived_competence_mut(&mut self) -> &mut StateValue {
        &mut self.perceived_competence
    }

    // Delta modifiers

    /// Adds to the loneliness delta.
    pub fn add_loneliness_delta(&mut self, amount: f32) {
        self.loneliness.add_delta(amount);
    }

    /// Adds to the perceived reciprocal caring delta.
    pub fn add_perceived_reciprocal_caring_delta(&mut self, amount: f32) {
        self.perceived_reciprocal_caring.add_delta(amount);
    }

    /// Adds to the perceived liability delta.
    pub fn add_perceived_liability_delta(&mut self, amount: f32) {
        self.perceived_liability.add_delta(amount);
    }

    /// Adds to the self-hate delta.
    pub fn add_self_hate_delta(&mut self, amount: f32) {
        self.self_hate.add_delta(amount);
    }

    /// Adds to the perceived competence delta.
    pub fn add_perceived_competence_delta(&mut self, amount: f32) {
        self.perceived_competence.add_delta(amount);
    }

    // Decay

    /// Applies decay to all social cognition dimensions.
    pub fn apply_decay(&mut self, elapsed: Duration) {
        self.loneliness.apply_decay(elapsed);
        self.perceived_reciprocal_caring.apply_decay(elapsed);
        self.perceived_liability.apply_decay(elapsed);
        self.self_hate.apply_decay(elapsed);
        self.perceived_competence.apply_decay(elapsed);
    }

    /// Resets all deltas to zero.
    pub fn reset_deltas(&mut self) {
        self.loneliness.reset_delta();
        self.perceived_reciprocal_caring.reset_delta();
        self.perceived_liability.reset_delta();
        self.self_hate.reset_delta();
        self.perceived_competence.reset_delta();
    }
}

impl Default for SocialCognition {
    fn default() -> Self {
        SocialCognition::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_default_values() {
        let social = SocialCognition::new();
        assert!((social.loneliness_effective() - 0.2).abs() < f32::EPSILON);
        assert!(
            (social.perceived_reciprocal_caring_effective() - 0.6).abs() < f32::EPSILON
        );
        assert!((social.perceived_liability_effective() - 0.0).abs() < f32::EPSILON);
        assert!((social.self_hate_effective() - 0.1).abs() < f32::EPSILON);
        assert!((social.perceived_competence_effective() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn builder_methods_set_bases() {
        let social = SocialCognition::new()
            .with_loneliness_base(0.4)
            .with_perceived_reciprocal_caring_base(0.3)
            .with_perceived_liability_base(0.2)
            .with_self_hate_base(0.5)
            .with_perceived_competence_base(0.7);

        assert!((social.loneliness_base() - 0.4).abs() < f32::EPSILON);
        assert!((social.perceived_reciprocal_caring_base() - 0.3).abs() < f32::EPSILON);
        assert!((social.perceived_liability_base() - 0.2).abs() < f32::EPSILON);
        assert!((social.self_hate_base() - 0.5).abs() < f32::EPSILON);
        assert!((social.perceived_competence_base() - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn delta_methods_update_values() {
        let mut social = SocialCognition::new();
        social.add_loneliness_delta(0.2);
        social.add_perceived_reciprocal_caring_delta(-0.1);
        social.add_perceived_liability_delta(0.3);
        social.add_self_hate_delta(0.4);
        social.add_perceived_competence_delta(0.5);

        assert!((social.loneliness().delta() - 0.2).abs() < f32::EPSILON);
        assert!(
            (social.perceived_reciprocal_caring().delta() - (-0.1)).abs() < f32::EPSILON
        );
        assert!((social.perceived_liability().delta() - 0.3).abs() < f32::EPSILON);
        assert!((social.self_hate().delta() - 0.4).abs() < f32::EPSILON);
        assert!((social.perceived_competence().delta() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn mutable_references_work() {
        let mut social = SocialCognition::new();
        social.loneliness_mut().add_delta(0.1);
        social.perceived_reciprocal_caring_mut().add_delta(0.2);
        social.perceived_liability_mut().add_delta(0.3);
        social.self_hate_mut().add_delta(0.4);
        social.perceived_competence_mut().add_delta(0.5);

        assert!((social.loneliness().delta() - 0.1).abs() < f32::EPSILON);
        assert!((social.perceived_reciprocal_caring().delta() - 0.2).abs() < f32::EPSILON);
        assert!((social.perceived_liability().delta() - 0.3).abs() < f32::EPSILON);
        assert!((social.self_hate().delta() - 0.4).abs() < f32::EPSILON);
        assert!((social.perceived_competence().delta() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn apply_decay_and_reset() {
        let mut social = SocialCognition::new();
        social.add_loneliness_delta(0.4);
        social.add_perceived_reciprocal_caring_delta(0.4);
        social.add_perceived_liability_delta(0.4);
        social.add_self_hate_delta(0.4);
        social.add_perceived_competence_delta(0.4);

        social.apply_decay(Duration::days(1));
        assert!(social.loneliness().delta() < 0.4);

        social.reset_deltas();
        assert!(social.loneliness().delta().abs() < f32::EPSILON);
        assert!(social.perceived_reciprocal_caring().delta().abs() < f32::EPSILON);
        assert!(social.perceived_liability().delta().abs() < f32::EPSILON);
        assert!(social.self_hate().delta().abs() < f32::EPSILON);
        assert!(social.perceived_competence().delta().abs() < f32::EPSILON);
    }

    #[test]
    fn default_equals_new() {
        let d = SocialCognition::default();
        let n = SocialCognition::new();
        assert_eq!(d, n);
    }

    #[test]
    fn clone_and_equality() {
        let s1 = SocialCognition::new().with_perceived_competence_base(0.8);
        let s2 = s1.clone();
        assert_eq!(s1, s2);
    }

    #[test]
    fn debug_format() {
        let social = SocialCognition::new();
        let debug = format!("{:?}", social);
        assert!(debug.contains("SocialCognition"));
    }
}
