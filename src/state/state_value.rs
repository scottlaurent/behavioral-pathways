//! StateValue implementation for psychological state dimensions.
//!
//! StateValue represents a single psychological dimension with:
//! - A stable base value (personality/trait)
//! - A delta value (current deviation from events)
//! - An optional decay half-life (how quickly delta returns to zero)
//! - Optional bounds (min/max for the effective value)

use crate::types::Duration;
use serde::{Deserialize, Serialize};

/// Chronic deltas decay more slowly than acute deltas.
const CHRONIC_HALF_LIFE_MULTIPLIER: u64 = 4;

/// A psychological state value with base, delta, and decay behavior.
///
/// The effective value is `base + delta`, clamped to bounds if set.
/// Delta decays toward zero over time based on the half-life.
///
/// The decay half-life is stored in raw form (species-agnostic).
/// Time scaling is applied at processing time, not at construction.
///
/// A `None` value for `decay_half_life` means the delta never decays.
/// This is used for dimensions like Acquired Capability that are
/// trait-like and accumulate over time without returning to baseline.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::state::StateValue;
/// use behavioral_pathways::types::Duration;
///
/// let mut stress = StateValue::new(0.3)
///     .with_bounds(0.0, 1.0)
///     .with_decay_half_life(Duration::days(3));
///
/// // Apply a stressful event
/// stress.add_delta(0.4);
/// assert!((stress.effective() - 0.7).abs() < 0.01);
///
/// // Decay reduces delta over time
/// stress.apply_decay(Duration::days(3));
/// assert!((stress.effective() - 0.5).abs() < 0.01); // Half of delta decayed
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateValue {
    /// The stable baseline value (personality/trait).
    base: f32,

    /// Current deviation from base (acute events).
    delta: f32,

    /// Persistent deviation from base (chronic patterns).
    chronic_delta: f32,

    /// Half-life for delta decay (raw, not scaled).
    /// None means no decay - delta is permanent.
    decay_half_life: Option<Duration>,

    /// Minimum bound for effective value.
    min_bound: f32,

    /// Maximum bound for effective value.
    max_bound: f32,

    /// Whether this state has been affected by feedback loop effects.
    /// When true, the delta cannot be reversed because feedback loops
    /// are cumulative and non-linear processes.
    feedback_loop_affected: bool,
}

impl StateValue {
    /// Creates a new StateValue with the specified base value.
    ///
    /// Default bounds are 0.0 to 1.0, and default decay half-life is 7 days.
    ///
    /// # Arguments
    ///
    /// * `base` - The stable baseline value
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::state::StateValue;
    ///
    /// let value = StateValue::new(0.5);
    /// assert!((value.base() - 0.5).abs() < f32::EPSILON);
    /// assert!(value.delta().abs() < f32::EPSILON);
    /// ```
    #[must_use]
    pub fn new(base: f32) -> Self {
        StateValue {
            base,
            delta: 0.0,
            chronic_delta: 0.0,
            decay_half_life: Some(Duration::days(7)),
            min_bound: 0.0,
            max_bound: 1.0,
            feedback_loop_affected: false,
        }
    }

    /// Creates a new StateValue that never decays.
    ///
    /// This is used for trait-like dimensions such as Acquired Capability
    /// that accumulate over time and do not return to baseline.
    ///
    /// # Arguments
    ///
    /// * `base` - The stable baseline value
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::state::StateValue;
    /// use behavioral_pathways::types::Duration;
    ///
    /// let mut capability = StateValue::new_no_decay(0.0);
    /// capability.add_delta(0.3);
    ///
    /// // Even after time passes, delta remains
    /// capability.apply_decay(Duration::years(10));
    /// assert!((capability.delta() - 0.3).abs() < f32::EPSILON);
    /// ```
    #[must_use]
    pub fn new_no_decay(base: f32) -> Self {
        StateValue {
            base,
            delta: 0.0,
            chronic_delta: 0.0,
            decay_half_life: None,
            min_bound: 0.0,
            max_bound: 1.0,
            feedback_loop_affected: false,
        }
    }

    /// Sets the bounds for the effective value.
    ///
    /// The effective value will be clamped to these bounds.
    ///
    /// # Arguments
    ///
    /// * `min` - Minimum allowed value
    /// * `max` - Maximum allowed value
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::state::StateValue;
    ///
    /// let value = StateValue::new(0.5).with_bounds(-1.0, 1.0);
    /// ```
    #[must_use]
    pub fn with_bounds(mut self, min: f32, max: f32) -> Self {
        self.min_bound = min;
        self.max_bound = max;
        self
    }

    /// Sets the decay half-life for delta.
    ///
    /// This is the raw half-life, not scaled by species time scale.
    /// Time scaling is applied at processing time.
    ///
    /// # Arguments
    ///
    /// * `half_life` - The duration for delta to decay by half
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::state::StateValue;
    /// use behavioral_pathways::types::Duration;
    ///
    /// let value = StateValue::new(0.5).with_decay_half_life(Duration::days(3));
    /// ```
    #[must_use]
    pub fn with_decay_half_life(mut self, half_life: Duration) -> Self {
        self.decay_half_life = Some(half_life);
        self
    }

    /// Updates the decay half-life for delta.
    ///
    /// This overrides any existing half-life value.
    pub fn set_decay_half_life(&mut self, half_life: Duration) {
        self.decay_half_life = Some(half_life);
    }

    /// Removes the decay half-life, making this value never decay.
    ///
    /// This is equivalent to creating with `new_no_decay()` but can be
    /// used to convert an existing StateValue to non-decaying.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::state::StateValue;
    ///
    /// let value = StateValue::new(0.5).with_no_decay();
    /// assert!(value.decay_half_life().is_none());
    /// ```
    #[must_use]
    pub fn with_no_decay(mut self) -> Self {
        self.decay_half_life = None;
        self
    }

    /// Sets the initial delta value.
    ///
    /// # Arguments
    ///
    /// * `delta` - The initial delta value
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::state::StateValue;
    ///
    /// let value = StateValue::new(0.5).with_delta(0.2);
    /// assert!((value.delta() - 0.2).abs() < f32::EPSILON);
    /// ```
    #[must_use]
    pub fn with_delta(mut self, delta: f32) -> Self {
        self.delta = delta;
        self.chronic_delta = 0.0;
        self
    }

    /// Returns the base value.
    #[must_use]
    pub fn base(&self) -> f32 {
        self.base
    }

    /// Returns the current delta value (acute + chronic).
    #[must_use]
    pub fn delta(&self) -> f32 {
        self.delta + self.chronic_delta
    }

    /// Returns the decay half-life, or None if this value never decays.
    #[must_use]
    pub fn decay_half_life(&self) -> Option<Duration> {
        self.decay_half_life
    }

    /// Returns true if this value decays over time.
    #[must_use]
    pub fn decays(&self) -> bool {
        self.decay_half_life.is_some()
    }

    /// Returns true if this state has been affected by feedback loop effects.
    ///
    /// When a feedback loop (e.g., stress spiral, depression spiral) has
    /// contributed to this state's delta, the state cannot be reversed
    /// because feedback loops are cumulative and non-linear processes.
    #[must_use]
    pub fn is_feedback_loop_affected(&self) -> bool {
        self.feedback_loop_affected
    }

    /// Marks this state as affected by feedback loop effects.
    ///
    /// This should be called when a feedback loop (e.g., stress spiral,
    /// depression spiral) contributes to this state's delta. Once marked,
    /// the state's delta cannot be reversed.
    pub fn mark_feedback_loop_affected(&mut self) {
        self.feedback_loop_affected = true;
    }

    /// Clears the feedback loop affected flag.
    ///
    /// This is typically only used during testing or when resetting state.
    /// Use with caution as it allows reversal of potentially non-reversible changes.
    pub fn clear_feedback_loop_affected(&mut self) {
        self.feedback_loop_affected = false;
    }

    /// Returns the effective value (base + delta), clamped to bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::state::StateValue;
    ///
    /// let value = StateValue::new(0.5).with_delta(0.3);
    /// assert!((value.effective() - 0.8).abs() < f32::EPSILON);
    ///
    /// // Value is clamped to bounds
    /// let clamped = StateValue::new(0.8).with_delta(0.5);
    /// assert!((clamped.effective() - 1.0).abs() < f32::EPSILON);
    /// ```
    #[must_use]
    pub fn effective(&self) -> f32 {
        let total_delta = self.delta + self.chronic_delta;
        (self.base + total_delta).clamp(self.min_bound, self.max_bound)
    }

    /// Returns the raw (unclamped) effective value.
    ///
    /// This is useful for checking if the value would exceed bounds.
    #[must_use]
    pub fn effective_raw(&self) -> f32 {
        self.base + self.delta + self.chronic_delta
    }

    /// Sets the base value.
    ///
    /// This is typically only done at entity creation.
    pub fn set_base(&mut self, base: f32) {
        self.base = base;
    }

    /// Adds to the delta value.
    ///
    /// Delta accumulates from acute events.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount to add to delta (can be negative)
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::state::StateValue;
    ///
    /// let mut value = StateValue::new(0.5);
    /// value.add_delta(0.2);
    /// value.add_delta(0.1);
    /// assert!((value.delta() - 0.3).abs() < f32::EPSILON);
    /// ```
    pub fn add_delta(&mut self, amount: f32) {
        self.delta += amount;
    }

    /// Adds to the chronic delta value.
    ///
    /// Chronic deltas decay more slowly than acute deltas.
    pub fn add_chronic_delta(&mut self, amount: f32) {
        self.chronic_delta += amount;
    }

    /// Sets the delta value directly.
    ///
    /// Use `add_delta` for accumulating changes from events.
    pub fn set_delta(&mut self, delta: f32) {
        self.delta = delta;
        self.chronic_delta = 0.0;
    }

    /// Applies decay to the delta value over the specified duration.
    ///
    /// Uses exponential decay based on the half-life.
    /// After one half-life, delta is reduced to half its value.
    ///
    /// If this StateValue has no decay (half-life is None), this method
    /// does nothing - the delta remains unchanged.
    ///
    /// This method takes raw duration. Time scaling should be applied
    /// by the caller if needed.
    ///
    /// # Arguments
    ///
    /// * `elapsed` - The time duration over which to decay
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::state::StateValue;
    /// use behavioral_pathways::types::Duration;
    ///
    /// let mut value = StateValue::new(0.5)
    ///     .with_delta(0.4)
    ///     .with_decay_half_life(Duration::days(3));
    ///
    /// // After one half-life, delta should be halved
    /// value.apply_decay(Duration::days(3));
    /// assert!((value.delta() - 0.2).abs() < 0.01);
    /// ```
    pub fn apply_decay(&mut self, elapsed: Duration) {
        // If no decay half-life, delta never decays
        let half_life = match self.decay_half_life {
            Some(hl) => hl,
            None => return,
        };

        if half_life.is_zero() || elapsed.is_zero() {
            return;
        }

        // Calculate decay factor using exponential decay
        // factor = 0.5^(elapsed / half_life)
        let elapsed_seconds = elapsed.as_seconds() as f64;
        let half_life_seconds = half_life.as_seconds() as f64;

        let decay_factor = 0.5_f64.powf(elapsed_seconds / half_life_seconds);
        self.delta *= decay_factor as f32;

        let chronic_half_life = half_life * CHRONIC_HALF_LIFE_MULTIPLIER;
        let chronic_half_life_seconds = chronic_half_life.as_seconds() as f64;
        let chronic_decay_factor = 0.5_f64.powf(elapsed_seconds / chronic_half_life_seconds);
        self.chronic_delta *= chronic_decay_factor as f32;
    }

    /// Resets delta to zero.
    pub fn reset_delta(&mut self) {
        self.delta = 0.0;
        self.chronic_delta = 0.0;
    }
}

impl Default for StateValue {
    fn default() -> Self {
        StateValue::new(0.5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_returns_base_plus_delta() {
        let value = StateValue::new(0.5).with_delta(0.3);
        let effective = value.effective();
        assert!((effective - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn current_clamps_to_max() {
        let value = StateValue::new(0.8).with_delta(0.5);
        let effective = value.effective();
        assert!((effective - 1.0).abs() < f32::EPSILON);

        // Check raw value isn't clamped
        assert!((value.effective_raw() - 1.3).abs() < f32::EPSILON);
    }

    #[test]
    fn current_clamps_to_min() {
        let value = StateValue::new(0.2).with_delta(-0.5);
        let effective = value.effective();
        assert!(effective.abs() < f32::EPSILON);
    }

    #[test]
    fn add_delta_accumulates() {
        let mut value = StateValue::new(0.5);

        value.add_delta(0.1);
        assert!((value.delta() - 0.1).abs() < f32::EPSILON);

        value.add_delta(0.2);
        assert!((value.delta() - 0.3).abs() < f32::EPSILON);

        value.add_delta(-0.1);
        assert!((value.delta() - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn add_chronic_delta_accumulates() {
        let mut value = StateValue::new(0.5);
        value.add_chronic_delta(0.3);
        assert!((value.delta() - 0.3).abs() < f32::EPSILON);

        value.add_chronic_delta(0.2);
        assert!((value.delta() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn chronic_delta_decays_slower_than_acute() {
        let mut value = StateValue::new(0.5)
            .with_decay_half_life(Duration::days(1))
            .with_delta(0.4);
        value.add_chronic_delta(0.4);

        value.apply_decay(Duration::days(1));

        let acute_expected = 0.4 * 0.5;
        let chronic_decay = 0.5_f64.powf(1.0 / 4.0) as f32;
        let chronic_expected = 0.4 * chronic_decay;
        let expected_total = acute_expected + chronic_expected;

        assert!((value.delta() - expected_total).abs() < 0.01);
    }

    #[test]
    fn state_value_decay_is_species_independent() {
        // This test verifies that StateValue stores raw half-life
        // and does NOT apply time scaling internally.

        let value1 = StateValue::new(0.5)
            .with_delta(0.4)
            .with_decay_half_life(Duration::days(3));

        let value2 = StateValue::new(0.5)
            .with_delta(0.4)
            .with_decay_half_life(Duration::days(3));

        // Both should have the same decay half-life, regardless of
        // what species they might belong to. Time scaling is applied
        // at processing time, not at StateValue level.
        assert_eq!(value1.decay_half_life(), value2.decay_half_life());

        // The half-life stored is raw days, not scaled
        assert_eq!(value1.decay_half_life().unwrap().as_days(), 3);
    }

    #[test]
    fn decay_halves_delta_after_half_life() {
        let mut value = StateValue::new(0.5)
            .with_delta(0.4)
            .with_decay_half_life(Duration::days(3));

        value.apply_decay(Duration::days(3));

        // Delta should be halved (within tolerance for floating point)
        assert!((value.delta() - 0.2).abs() < 0.01);
    }

    #[test]
    fn decay_quarter_after_two_half_lives() {
        let mut value = StateValue::new(0.5)
            .with_delta(0.4)
            .with_decay_half_life(Duration::days(3));

        value.apply_decay(Duration::days(6));

        // Delta should be quartered (0.5^2 = 0.25)
        assert!((value.delta() - 0.1).abs() < 0.01);
    }

    #[test]
    fn decay_with_zero_elapsed() {
        let mut value = StateValue::new(0.5).with_delta(0.4);
        let original_delta = value.delta();

        value.apply_decay(Duration::zero());

        assert!((value.delta() - original_delta).abs() < f32::EPSILON);
    }

    #[test]
    fn decay_with_zero_half_life() {
        let mut value = StateValue::new(0.5)
            .with_delta(0.4)
            .with_decay_half_life(Duration::zero());

        let original_delta = value.delta();
        value.apply_decay(Duration::days(1));

        assert!((value.delta() - original_delta).abs() < f32::EPSILON);
    }

    #[test]
    fn reset_delta() {
        let mut value = StateValue::new(0.5).with_delta(0.4);
        value.reset_delta();
        assert!(value.delta().abs() < f32::EPSILON);
    }

    #[test]
    fn set_base() {
        let mut value = StateValue::new(0.5);
        value.set_base(0.7);
        assert!((value.base() - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn custom_bounds() {
        let value = StateValue::new(0.0).with_bounds(-1.0, 1.0).with_delta(0.5);

        assert!((value.effective() - 0.5).abs() < f32::EPSILON);

        let negative = StateValue::new(0.0).with_bounds(-1.0, 1.0).with_delta(-0.5);

        assert!((negative.effective() - (-0.5)).abs() < f32::EPSILON);
    }

    #[test]
    fn default_state_value() {
        let value = StateValue::default();
        assert!((value.base() - 0.5).abs() < f32::EPSILON);
        assert!(value.delta().abs() < f32::EPSILON);
    }

    #[test]
    fn clone_and_equality() {
        let value1 = StateValue::new(0.5).with_delta(0.2);
        let value2 = value1.clone();
        assert_eq!(value1, value2);
    }

    #[test]
    fn set_delta_directly() {
        let mut value = StateValue::new(0.5);
        value.set_delta(0.3);
        assert!((value.delta() - 0.3).abs() < f32::EPSILON);

        value.set_delta(-0.2);
        assert!((value.delta() - (-0.2)).abs() < f32::EPSILON);
    }

    #[test]
    fn effective_raw_not_clamped() {
        let value = StateValue::new(0.9).with_delta(0.5);
        // Effective should be clamped to 1.0
        assert!((value.effective() - 1.0).abs() < f32::EPSILON);
        // Raw should not be clamped
        assert!((value.effective_raw() - 1.4).abs() < f32::EPSILON);
    }

    #[test]
    fn decay_half_life_accessor() {
        let value = StateValue::new(0.5).with_decay_half_life(Duration::days(5));
        assert_eq!(value.decay_half_life().unwrap().as_days(), 5);
    }

    #[test]
    fn no_decay_value_never_decays() {
        let mut value = StateValue::new_no_decay(0.0).with_delta(0.5);

        // Apply decay over a very long time
        value.apply_decay(Duration::years(100));

        // Delta should remain unchanged
        assert!((value.delta() - 0.5).abs() < f32::EPSILON);
        assert!(!value.decays());
        assert!(value.decay_half_life().is_none());
    }

    #[test]
    fn with_no_decay_removes_half_life() {
        let value = StateValue::new(0.5)
            .with_decay_half_life(Duration::days(7))
            .with_no_decay();

        assert!(value.decay_half_life().is_none());
        assert!(!value.decays());
    }

    #[test]
    fn decays_returns_true_for_decaying_value() {
        let value = StateValue::new(0.5);
        assert!(value.decays());
        assert!(value.decay_half_life().is_some());
    }

    #[test]
    fn negative_delta() {
        let value = StateValue::new(0.5).with_delta(-0.2);
        assert!((value.effective() - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn debug_format() {
        let value = StateValue::new(0.5);
        let debug = format!("{:?}", value);
        assert!(debug.contains("StateValue"));
    }

    #[test]
    fn base_accessor() {
        let value = StateValue::new(0.7);
        assert!((value.base() - 0.7).abs() < f32::EPSILON);
    }

    // --- Feedback loop affected flag tests ---

    #[test]
    fn new_value_not_feedback_affected() {
        let value = StateValue::new(0.5);
        assert!(!value.is_feedback_loop_affected());
    }

    #[test]
    fn mark_feedback_loop_affected() {
        let mut value = StateValue::new(0.5);
        value.mark_feedback_loop_affected();
        assert!(value.is_feedback_loop_affected());
    }

    #[test]
    fn clear_feedback_loop_affected() {
        let mut value = StateValue::new(0.5);
        value.mark_feedback_loop_affected();
        assert!(value.is_feedback_loop_affected());

        value.clear_feedback_loop_affected();
        assert!(!value.is_feedback_loop_affected());
    }

    #[test]
    fn feedback_flag_preserved_in_clone() {
        let mut value = StateValue::new(0.5);
        value.mark_feedback_loop_affected();

        let cloned = value.clone();
        assert!(cloned.is_feedback_loop_affected());
    }

    #[test]
    fn feedback_flag_in_equality() {
        let mut value1 = StateValue::new(0.5);
        let mut value2 = StateValue::new(0.5);

        assert_eq!(value1, value2);

        value1.mark_feedback_loop_affected();
        assert_ne!(value1, value2);

        value2.mark_feedback_loop_affected();
        assert_eq!(value1, value2);
    }

    #[test]
    fn no_decay_value_not_feedback_affected() {
        let value = StateValue::new_no_decay(0.5);
        assert!(!value.is_feedback_loop_affected());
    }

    #[test]
    fn set_decay_half_life() {
        let mut value = StateValue::new_no_decay(0.5);
        assert!(value.decay_half_life().is_none());

        value.set_decay_half_life(Duration::days(5));
        assert_eq!(value.decay_half_life().unwrap().as_days(), 5);
        assert!(value.decays());
    }
}
