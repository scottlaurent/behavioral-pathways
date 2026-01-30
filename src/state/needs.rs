//! Psychological and physiological needs state.
//!
//! This module contains needs that drive behavior and represent
//! physiological or motivational states. Fatigue and stress are
//! distinct from PAD mood dimensions.

use crate::state::StateValue;
use crate::types::Duration;
use serde::{Deserialize, Serialize};

/// Psychological and physiological needs.
///
/// These needs influence behavior and represent states that
/// motivate action when unmet.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::state::Needs;
/// use behavioral_pathways::types::Duration;
///
/// let mut needs = Needs::new();
///
/// // Apply stressor event
/// needs.add_stress_delta(0.3);
/// assert!(needs.stress_effective() > 0.2);
///
/// // Needs decay over time
/// needs.apply_decay(Duration::days(1));
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Needs {
    /// Physical and mental tiredness.
    /// Range: 0 (fresh) to 1 (exhausted)
    /// Default decay half-life: 8 hours
    fatigue: StateValue,

    /// Pressure and tension.
    /// Range: 0 (relaxed) to 1 (highly stressed)
    /// Default decay half-life: 12 hours
    stress: StateValue,

    /// Sense of meaning and direction in life.
    /// Range: 0 (purposeless) to 1 (strong sense of purpose)
    /// Default decay half-life: 3 days
    purpose: StateValue,
}

impl Needs {
    /// Default decay half-life for fatigue (8 hours).
    const FATIGUE_DECAY_HALF_LIFE: Duration = Duration::hours(8);

    /// Default decay half-life for stress (12 hours).
    const STRESS_DECAY_HALF_LIFE: Duration = Duration::hours(12);

    /// Default decay half-life for purpose (3 days).
    const PURPOSE_DECAY_HALF_LIFE: Duration = Duration::days(3);

    /// Creates a new Needs with default base values.
    ///
    /// Default bases are neutral (0.2-0.3 range) representing a
    /// healthy baseline without significant unmet needs.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::state::Needs;
    ///
    /// let needs = Needs::new();
    /// assert!(needs.fatigue_effective() < 0.5);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Needs {
            fatigue: StateValue::new(0.2)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Self::FATIGUE_DECAY_HALF_LIFE),
            stress: StateValue::new(0.2)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Self::STRESS_DECAY_HALF_LIFE),
            purpose: StateValue::new(0.7)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Self::PURPOSE_DECAY_HALF_LIFE),
        }
    }

    // Builder methods for base values

    /// Sets the base fatigue.
    #[must_use]
    pub fn with_fatigue_base(mut self, value: f32) -> Self {
        self.fatigue.set_base(value);
        self
    }

    /// Sets the base stress.
    #[must_use]
    pub fn with_stress_base(mut self, value: f32) -> Self {
        self.stress.set_base(value);
        self
    }

    /// Sets the base purpose.
    #[must_use]
    pub fn with_purpose_base(mut self, value: f32) -> Self {
        self.purpose.set_base(value);
        self
    }

    // Effective value accessors (base + delta)

    /// Returns the effective fatigue (base + delta).
    #[must_use]
    pub fn fatigue_effective(&self) -> f32 {
        self.fatigue.effective()
    }

    /// Returns the effective stress (base + delta).
    #[must_use]
    pub fn stress_effective(&self) -> f32 {
        self.stress.effective()
    }

    /// Returns the effective purpose (base + delta).
    #[must_use]
    pub fn purpose_effective(&self) -> f32 {
        self.purpose.effective()
    }

    // Base value accessors

    /// Returns the base fatigue.
    #[must_use]
    pub fn fatigue_base(&self) -> f32 {
        self.fatigue.base()
    }

    /// Returns the base stress.
    #[must_use]
    pub fn stress_base(&self) -> f32 {
        self.stress.base()
    }

    /// Returns the base purpose.
    #[must_use]
    pub fn purpose_base(&self) -> f32 {
        self.purpose.base()
    }

    // StateValue references

    /// Returns a reference to the fatigue StateValue.
    #[must_use]
    pub fn fatigue(&self) -> &StateValue {
        &self.fatigue
    }

    /// Returns a reference to the stress StateValue.
    #[must_use]
    pub fn stress(&self) -> &StateValue {
        &self.stress
    }

    /// Returns a reference to the purpose StateValue.
    #[must_use]
    pub fn purpose(&self) -> &StateValue {
        &self.purpose
    }

    /// Returns a mutable reference to the fatigue StateValue.
    pub fn fatigue_mut(&mut self) -> &mut StateValue {
        &mut self.fatigue
    }

    /// Returns a mutable reference to the stress StateValue.
    pub fn stress_mut(&mut self) -> &mut StateValue {
        &mut self.stress
    }

    /// Returns a mutable reference to the purpose StateValue.
    pub fn purpose_mut(&mut self) -> &mut StateValue {
        &mut self.purpose
    }

    // Delta modifiers

    /// Adds to the fatigue delta.
    pub fn add_fatigue_delta(&mut self, amount: f32) {
        self.fatigue.add_delta(amount);
    }

    /// Adds to the stress delta.
    pub fn add_stress_delta(&mut self, amount: f32) {
        self.stress.add_delta(amount);
    }

    /// Adds to the purpose delta.
    pub fn add_purpose_delta(&mut self, amount: f32) {
        self.purpose.add_delta(amount);
    }

    /// Applies decay to all needs based on elapsed time.
    pub fn apply_decay(&mut self, elapsed: Duration) {
        self.fatigue.apply_decay(elapsed);
        self.stress.apply_decay(elapsed);
        self.purpose.apply_decay(elapsed);
    }

    /// Resets all deltas to zero.
    pub fn reset_deltas(&mut self) {
        self.fatigue.reset_delta();
        self.stress.reset_delta();
        self.purpose.reset_delta();
    }
}

impl Default for Needs {
    fn default() -> Self {
        Needs::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn needs_contains_fatigue_stress() {
        // Verify that fatigue and stress are in Needs (not in Mood)
        let needs = Needs::new();
        let _ = needs.fatigue_effective();
        let _ = needs.stress_effective();
    }

    #[test]
    fn purpose_is_need_dimension() {
        let needs = Needs::new();
        let _ = needs.purpose_effective();
    }

    #[test]
    fn new_creates_healthy_defaults() {
        let needs = Needs::new();

        // Low negative states
        assert!(needs.fatigue_effective() < 0.5);
        assert!(needs.stress_effective() < 0.5);

        // High positive states
        assert!(needs.purpose_effective() > 0.5);
    }

    #[test]
    fn builder_methods_set_base_values() {
        let needs = Needs::new()
            .with_fatigue_base(0.5)
            .with_stress_base(0.6)
            .with_purpose_base(0.2);

        assert!((needs.fatigue_base() - 0.5).abs() < f32::EPSILON);
        assert!((needs.stress_base() - 0.6).abs() < f32::EPSILON);
        assert!((needs.purpose_base() - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn add_delta_modifies_effective() {
        let mut needs = Needs::new();
        let original = needs.stress_effective();

        needs.add_stress_delta(0.3);

        assert!((needs.stress_effective() - (original + 0.3)).abs() < f32::EPSILON);
    }

    #[test]
    fn fatigue_decays_over_time() {
        let mut needs = Needs::new();
        needs.add_fatigue_delta(0.8);

        // After 8 hours (one half-life), delta should be halved
        needs.apply_decay(Duration::hours(8));
        assert!((needs.fatigue().delta() - 0.4).abs() < 0.01);
    }

    #[test]
    fn stress_decays_over_time() {
        let mut needs = Needs::new();
        needs.add_stress_delta(0.6);

        // After 12 hours (one half-life), delta should be halved
        needs.apply_decay(Duration::hours(12));
        assert!((needs.stress().delta() - 0.3).abs() < 0.01);
    }

    #[test]
    fn purpose_decays_over_time() {
        let mut needs = Needs::new();
        needs.add_purpose_delta(-0.4);

        // After 3 days (one half-life), delta should be halved
        needs.apply_decay(Duration::days(3));
        assert!((needs.purpose().delta() - (-0.2)).abs() < 0.01);
    }

    #[test]
    fn reset_deltas_clears_all() {
        let mut needs = Needs::new();
        needs.add_fatigue_delta(0.5);
        needs.add_stress_delta(0.3);
        needs.add_purpose_delta(0.2);

        needs.reset_deltas();

        assert!(needs.fatigue().delta().abs() < f32::EPSILON);
        assert!(needs.stress().delta().abs() < f32::EPSILON);
        assert!(needs.purpose().delta().abs() < f32::EPSILON);
    }

    #[test]
    fn mutable_references_work() {
        let mut needs = Needs::new();
        needs.fatigue_mut().add_delta(0.2);
        assert!((needs.fatigue().delta() - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn default_is_new() {
        let needs = Needs::default();
        assert!(needs.stress_effective() < 0.5);
    }

    #[test]
    fn clone_and_equality() {
        let needs1 = Needs::new().with_stress_base(0.5);
        let needs2 = needs1.clone();
        assert_eq!(needs1, needs2);
    }

    #[test]
    fn debug_format() {
        let needs = Needs::new();
        let debug = format!("{:?}", needs);
        assert!(debug.contains("Needs"));
    }

    #[test]
    fn all_delta_modifiers() {
        let mut n = Needs::new();

        n.add_fatigue_delta(0.1);
        assert!((n.fatigue().delta() - 0.1).abs() < f32::EPSILON);

        n.add_stress_delta(-0.1);
        assert!((n.stress().delta() - (-0.1)).abs() < f32::EPSILON);

        n.add_purpose_delta(0.2);
        assert!((n.purpose().delta() - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn all_mutable_refs() {
        let mut n = Needs::new();

        n.fatigue_mut().add_delta(0.05);
        n.stress_mut().add_delta(0.06);
        n.purpose_mut().add_delta(-0.1);

        assert!((n.fatigue().delta() - 0.05).abs() < f32::EPSILON);
        assert!((n.stress().delta() - 0.06).abs() < f32::EPSILON);
        assert!((n.purpose().delta() - (-0.1)).abs() < f32::EPSILON);
    }

    #[test]
    fn all_base_accessors() {
        let n = Needs::new();

        // Verify all base accessors work
        let _ = n.fatigue_base();
        let _ = n.stress_base();
        let _ = n.purpose_base();
    }
}
