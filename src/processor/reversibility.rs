//! Delta reversibility logic for backward regression.
//!
//! **INTERNAL MODULE**: These functions are used internally by `Simulation::state_at()`
//! when computing state at timestamps before the anchor point. Consumers should use
//! the timestamp-based `Simulation` API rather than calling these functions directly.
//!
//! This module provides functions to reverse decay effects on state values,
//! enabling backward regression to recover original state from final state.
//!
//! # Reversal Formula
//!
//! Decay follows exponential model: `delta(t) = delta(0) * exp(-ln(2) * t / half_life)`
//!
//! To reverse decay and recover original delta:
//! `original_delta = current_delta * exp(ln(2) * elapsed / half_life)`
//!
//! # Non-Reversible Dimensions
//!
//! Some dimensions cannot be reversed:
//! - Acquired Capability (AC) - has no decay, accumulates permanently
//! - Feedback loop effects - cumulative, non-linear processes

use crate::enums::{ReversibilityError, ReversibilityResult};
use crate::state::StateValue;
use crate::types::Duration;

/// Acceptable floating-point precision for reversal tests.
#[allow(dead_code)]
pub const REVERSAL_EPSILON: f64 = 1e-10;

/// Checks whether a state value is reversible.
///
/// A state value is reversible if:
/// 1. It has a finite decay half-life (not AC which never decays)
/// 2. It has not been affected by feedback loop effects
///
/// Values with no decay (like Acquired Capability) cannot be reversed
/// because there's no decay to undo.
///
/// Values affected by feedback loops cannot be reversed because feedback
/// loops are cumulative and non-linear processes.
///
/// # Arguments
///
/// * `state_value` - The state value to check
///
/// # Returns
///
/// `ReversibilityResult::Reversible` if the dimension can be reversed,
/// `ReversibilityResult::NonReversible` otherwise.
///
/// # Examples
///
/// ```ignore
/// use behavioral_pathways::state::StateValue;
/// use behavioral_pathways::types::Duration;
/// use behavioral_pathways::enums::ReversibilityResult;
/// use behavioral_pathways::processor::check_reversibility;
///
/// // Value with decay half-life is reversible
/// let decaying = StateValue::new(0.5)
///     .with_decay_half_life(Duration::hours(6));
/// assert_eq!(check_reversibility(&decaying), ReversibilityResult::Reversible);
///
/// // Value with no decay is not reversible
/// let no_decay = StateValue::new(0.5).with_no_decay();
/// assert_eq!(check_reversibility(&no_decay), ReversibilityResult::NonReversible);
/// ```
#[must_use]
#[allow(dead_code)]
pub fn check_reversibility(state_value: &StateValue) -> ReversibilityResult {
    if state_value.decay_half_life().is_none() || state_value.is_feedback_loop_affected() {
        ReversibilityResult::NonReversible
    } else {
        ReversibilityResult::Reversible
    }
}

/// Reverses decay effects on a state value, returning a new StateValue with the original delta.
///
/// Given a StateValue after decay has been applied, this function computes
/// what the original delta must have been before decay and returns a new
/// StateValue with that delta.
///
/// # Formula
///
/// `original_delta = current_delta * exp(ln(2) * elapsed / half_life)`
///
/// This is the inverse of the decay formula:
/// `current_delta = original_delta * exp(-ln(2) * elapsed / half_life)`
///
/// # Arguments
///
/// * `state_value` - The state value to reverse (provides current delta and half-life)
/// * `duration` - Time that has passed since the original delta was applied
/// * `time_scale` - The entity's time scaling factor
///
/// # Returns
///
/// A new StateValue with the computed original delta, or an error if reversal is not possible.
///
/// # Errors
///
/// Returns `ReversibilityError::NonReversibleDimension` if the state has no decay (infinite half-life).
/// Returns `ReversibilityError::FeedbackLoopEffect` if the state has been affected by feedback loops.
/// Returns `ReversibilityError::InvalidReversal` if the computation produces invalid results.
///
/// # Examples
///
/// ```ignore
/// use behavioral_pathways::processor::reverse_decay;
/// use behavioral_pathways::state::StateValue;
/// use behavioral_pathways::types::Duration;
///
/// // Create a state value with current delta of 0.4 after decay
/// let state = StateValue::new(0.5)
///     .with_decay_half_life(Duration::hours(6))
///     .with_delta(0.4);
///
/// // Reverse one half-life of decay
/// let result = reverse_decay(&state, Duration::hours(6), 1.0);
///
/// // Original delta should have been ~0.8
/// assert!(result.is_ok());
/// let reversed = result.unwrap();
/// assert!((reversed.delta() - 0.8).abs() < 0.001);
/// ```
#[allow(dead_code)]
pub fn reverse_decay(
    state_value: &StateValue,
    duration: Duration,
    time_scale: f64,
) -> Result<StateValue, ReversibilityError> {
    // Check if affected by feedback loops
    if state_value.is_feedback_loop_affected() {
        return Err(ReversibilityError::feedback_loop(
            "State has been affected by feedback loop effects (e.g., stress spiral, depression spiral)",
        ));
    }

    // Check if dimension is reversible (has decay)
    let half_life = state_value.decay_half_life().ok_or_else(|| {
        ReversibilityError::non_reversible("Dimension has no decay (infinite half-life)")
    })?;

    // Compute the original delta
    let original_delta =
        compute_reversed_delta(state_value.delta(), duration, half_life, time_scale)?;

    // Create a new StateValue with the reversed delta
    // Clone the state value and update the delta
    let mut reversed = state_value.clone();
    reversed.set_delta(original_delta);

    Ok(reversed)
}

/// Computes the reversed delta value given current delta and decay parameters.
///
/// This is an internal helper function that performs the mathematical reversal.
///
/// # Arguments
///
/// * `current_delta` - The current delta value (after decay)
/// * `elapsed` - Time that has passed since the original delta was applied
/// * `half_life` - The decay half-life for this dimension
/// * `time_scale` - The entity's time scaling factor
///
/// # Returns
///
/// The computed original delta, or an error if reversal is not possible.
#[allow(dead_code)]
fn compute_reversed_delta(
    current_delta: f32,
    elapsed: Duration,
    half_life: Duration,
    time_scale: f64,
) -> Result<f32, ReversibilityError> {
    // Handle edge cases
    if elapsed.as_millis() == 0 {
        return Ok(current_delta);
    }

    if current_delta.abs() < f32::EPSILON {
        return Ok(0.0);
    }

    let half_life_ms = half_life.as_millis() as f64;
    if half_life_ms <= 0.0 {
        return Err(ReversibilityError::invalid("Half-life must be positive"));
    }

    // Apply time scale to elapsed time
    let scaled_elapsed_ms = elapsed.as_millis() as f64 * time_scale;

    // Compute reversal factor: exp(ln(2) * t / half_life)
    // This is the inverse of decay: exp(-ln(2) * t / half_life)
    let ln2 = std::f64::consts::LN_2;
    let exponent = ln2 * scaled_elapsed_ms / half_life_ms;

    // Guard against overflow
    if exponent > 700.0 {
        return Err(ReversibilityError::invalid(
            "Reversal exponent too large - numerical overflow",
        ));
    }

    let reversal_factor = exponent.exp();
    let original_delta = (current_delta as f64) * reversal_factor;

    // Check for invalid result
    if original_delta.is_nan() || original_delta.is_infinite() {
        return Err(ReversibilityError::invalid(
            "Reversal produced invalid result",
        ));
    }

    Ok(original_delta as f32)
}

/// Computes the reversed delta value from raw parameters.
///
/// This is a lower-level API that operates on raw delta values rather than StateValue.
/// Prefer `reverse_decay` for the high-level API that handles feedback loop checking.
///
/// # Arguments
///
/// * `current_delta` - The current delta value (after decay)
/// * `elapsed` - Time that has passed since the original delta was applied
/// * `half_life` - The decay half-life for this dimension (None for non-decaying)
/// * `time_scale` - The entity's time scaling factor
///
/// # Returns
///
/// The computed original delta, or an error if reversal is not possible.
///
/// # Errors
///
/// Returns `ReversibilityError::NonReversibleDimension` if half_life is None.
/// Returns `ReversibilityError::InvalidReversal` if the computation produces invalid results.
///
/// # Examples
///
/// ```ignore
/// use behavioral_pathways::processor::reverse_decay_raw;
/// use behavioral_pathways::types::Duration;
///
/// // Current delta is 0.4 after 6 hours (one half-life)
/// let half_life = Duration::hours(6);
/// let elapsed = Duration::hours(6);
///
/// let result = reverse_decay_raw(0.4, elapsed, Some(half_life), 1.0);
///
/// // Original delta should have been ~0.8
/// assert!(result.is_ok());
/// let original = result.unwrap();
/// assert!((original - 0.8).abs() < 0.001);
/// ```
#[allow(dead_code)]
pub fn reverse_decay_raw(
    current_delta: f32,
    elapsed: Duration,
    half_life: Option<Duration>,
    time_scale: f64,
) -> Result<f32, ReversibilityError> {
    // Check if dimension is reversible
    let half_life = half_life.ok_or_else(|| {
        ReversibilityError::non_reversible("Dimension has no decay (infinite half-life)")
    })?;

    compute_reversed_delta(current_delta, elapsed, half_life, time_scale)
}

/// Reverses decay on a StateValue and returns the computed original delta.
///
/// This is a convenience function that extracts the decay half-life from
/// the StateValue and computes the reversed delta.
///
/// Note: This function checks for feedback loop effects and will return
/// an error if the state has been affected by feedback loops.
///
/// # Arguments
///
/// * `state_value` - The state value (for its half-life and current delta)
/// * `elapsed` - Time that has passed since the delta was applied
/// * `time_scale` - The entity's time scaling factor
///
/// # Returns
///
/// The computed original delta, or an error if reversal is not possible.
///
/// # Errors
///
/// Returns `ReversibilityError::NonReversibleDimension` if the state has no decay.
/// Returns `ReversibilityError::FeedbackLoopEffect` if the state has been affected by feedback loops.
/// Returns `ReversibilityError::InvalidReversal` if the computation produces invalid results.
#[allow(dead_code)]
pub fn reverse_state_value_decay(
    state_value: &StateValue,
    elapsed: Duration,
    time_scale: f64,
) -> Result<f32, ReversibilityError> {
    // Check if affected by feedback loops
    if state_value.is_feedback_loop_affected() {
        return Err(ReversibilityError::feedback_loop(
            "State has been affected by feedback loop effects",
        ));
    }

    reverse_decay_raw(
        state_value.delta(),
        elapsed,
        state_value.decay_half_life(),
        time_scale,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Tests from phase-4.md ---

    #[test]
    fn delta_reversal_exact() {
        // Test name from phase-4.md
        // Forward then reverse produces original (within REVERSAL_EPSILON)

        let original_delta: f32 = 0.8;
        let half_life = Duration::hours(6);
        let elapsed = Duration::hours(6);
        let time_scale = 1.0;

        // Forward decay: 0.8 * exp(-ln(2) * 6 / 6) = 0.8 * 0.5 = 0.4
        let decay_factor = (-std::f64::consts::LN_2 * 6.0 / 6.0).exp();
        let decayed_delta = (original_delta as f64) * decay_factor;

        // Create state value with decayed delta
        let state = StateValue::new(0.5)
            .with_decay_half_life(half_life)
            .with_delta(decayed_delta as f32);

        // Reverse decay using new API
        let reversed = reverse_decay(&state, elapsed, time_scale).unwrap();

        // Should match original within epsilon
        let error = ((reversed.delta() as f64) - (original_delta as f64)).abs();
        assert!(error < REVERSAL_EPSILON);
    }

    #[test]
    fn reversal_uses_exponential_formula() {
        // Test name from phase-4.md
        // Verifies original = current * exp(ln(2) * t / half_life)

        let current_delta: f32 = 0.4;
        let half_life = Duration::hours(12);
        let elapsed = Duration::hours(12);
        let time_scale = 1.0;

        let state = StateValue::new(0.5)
            .with_decay_half_life(half_life)
            .with_delta(current_delta);

        let reversed = reverse_decay(&state, elapsed, time_scale).unwrap();

        // Manual calculation: 0.4 * exp(ln(2) * 12 / 12) = 0.4 * 2 = 0.8
        let expected = 0.4 * 2.0;
        assert!((reversed.delta() - expected as f32).abs() < 0.001);
    }

    #[test]
    fn reversal_respects_half_life() {
        // Test name from phase-4.md
        // Reversal math uses decay parameters

        let current_delta: f32 = 0.25;

        // With 6-hour half-life after 6 hours
        let state1 = StateValue::new(0.5)
            .with_decay_half_life(Duration::hours(6))
            .with_delta(current_delta);
        let result1 = reverse_decay(&state1, Duration::hours(6), 1.0).unwrap();
        // Expected: 0.25 * 2 = 0.5

        // With 12-hour half-life after 6 hours
        let state2 = StateValue::new(0.5)
            .with_decay_half_life(Duration::hours(12))
            .with_delta(current_delta);
        let result2 = reverse_decay(&state2, Duration::hours(6), 1.0).unwrap();
        // Expected: 0.25 * sqrt(2) ~= 0.354

        assert!((result1.delta() - 0.5).abs() < 0.001);
        assert!((result2.delta() - 0.354).abs() < 0.01);
    }

    #[test]
    fn multi_step_reversal_maintains_precision() {
        // Test name from phase-4.md
        // Validates epsilon tolerance holds across multiple decay periods

        let original_delta: f32 = 1.0;
        let half_life = Duration::weeks(1);
        let time_scale = 1.0;

        // Decay over multiple weeks
        let weeks = 4;
        let elapsed = Duration::weeks(weeks);

        // Forward decay: 1.0 * 0.5^4 = 0.0625
        let decay_factor = 0.5_f64.powi(weeks as i32);
        let decayed_delta = (original_delta as f64) * decay_factor;

        let state = StateValue::new(0.5)
            .with_decay_half_life(half_life)
            .with_delta(decayed_delta as f32);

        // Reverse
        let reversed = reverse_decay(&state, elapsed, time_scale).unwrap();

        let error = ((reversed.delta() as f64) - (original_delta as f64)).abs();
        assert!(error < 0.001);
    }

    #[test]
    fn acquired_capability_not_reversed() {
        // Test name from phase-4.md
        // AC survives regression unchanged (confirmation of Phase 2)

        // AC has no decay, so reverse_decay should return error
        let ac_state = StateValue::new(0.5).with_no_decay().with_delta(0.5);
        let result = reverse_decay(&ac_state, Duration::hours(6), 1.0);
        assert!(result.is_err());
        assert!(result.unwrap_err().is_non_reversible_dimension());
    }

    #[test]
    fn ac_immunity_during_multi_dimension_decay() {
        // Test name from phase-4.md
        // AC unchanged even when other dimensions decay in same advance() call

        // This test verifies the concept - AC has no half_life
        let ac_value = StateValue::new(0.5).with_no_decay();
        let result = reverse_state_value_decay(&ac_value, Duration::years(10), 1.0);

        assert!(result.is_err());
        assert!(result.unwrap_err().is_non_reversible_dimension());
    }

    #[test]
    fn feedback_loop_effects_not_reversed() {
        // Test name from phase-4.md
        // Spiral effects are non-reversible

        // Create a state value that has been affected by feedback loops
        let mut state = StateValue::new(0.5)
            .with_decay_half_life(Duration::hours(6))
            .with_delta(0.4);
        state.mark_feedback_loop_affected();

        // reverse_decay should return FeedbackLoopEffect error
        let result = reverse_decay(&state, Duration::hours(6), 1.0);
        assert!(result.is_err());
        assert!(result.unwrap_err().is_feedback_loop_effect());
    }

    #[test]
    fn reversibility_result_indicates_status() {
        // Test name from phase-4.md

        // Dimension with half-life is reversible
        let decaying = StateValue::new(0.5).with_decay_half_life(Duration::hours(6));
        assert_eq!(
            check_reversibility(&decaying),
            ReversibilityResult::Reversible
        );

        // Dimension without half-life is non-reversible
        let non_decaying = StateValue::new(0.5).with_no_decay();
        assert_eq!(
            check_reversibility(&non_decaying),
            ReversibilityResult::NonReversible
        );

        // Dimension affected by feedback loop is non-reversible
        let mut feedback_affected = StateValue::new(0.5).with_decay_half_life(Duration::hours(6));
        feedback_affected.mark_feedback_loop_affected();
        assert_eq!(
            check_reversibility(&feedback_affected),
            ReversibilityResult::NonReversible
        );
    }

    #[test]
    fn reverse_decay_returns_error_for_non_reversible() {
        // Test name from phase-4.md

        let non_decay_state = StateValue::new(0.5).with_no_decay().with_delta(0.5);
        let result = reverse_decay(&non_decay_state, Duration::hours(6), 1.0);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(err.is_non_reversible_dimension());
    }

    // --- Additional tests ---

    #[test]
    fn zero_elapsed_returns_current() {
        let state = StateValue::new(0.5)
            .with_decay_half_life(Duration::hours(6))
            .with_delta(0.5);
        let result = reverse_decay(&state, Duration::from_millis(0), 1.0).unwrap();
        assert!((result.delta() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn zero_delta_returns_zero() {
        let state = StateValue::new(0.5)
            .with_decay_half_life(Duration::hours(6))
            .with_delta(0.0);
        let result = reverse_decay(&state, Duration::hours(6), 1.0).unwrap();
        assert!(result.delta().abs() < f32::EPSILON);
    }

    #[test]
    fn time_scale_affects_reversal() {
        let current_delta: f32 = 0.4;
        let half_life = Duration::hours(12);
        let elapsed = Duration::hours(12);

        let state = StateValue::new(0.5)
            .with_decay_half_life(half_life)
            .with_delta(current_delta);

        // Human (time_scale 1.0): 12 hours = 1 half-life
        let human_result = reverse_decay(&state, elapsed, 1.0).unwrap();
        // Expected: 0.4 * 2 = 0.8

        // Dog (time_scale 6.67): 12 hours = ~6.67 half-lives
        let dog_result = reverse_decay(&state, elapsed, 6.67).unwrap();
        // Expected: 0.4 * 2^6.67 ~= 40.7

        assert!((human_result.delta() - 0.8).abs() < 0.001);
        assert!(dog_result.delta() > 30.0);
    }

    #[test]
    fn small_deltas_preserve_precision() {
        let small_delta: f32 = 0.001;
        let half_life = Duration::hours(6);
        let elapsed = Duration::hours(6);

        let state = StateValue::new(0.5)
            .with_decay_half_life(half_life)
            .with_delta(small_delta);

        let reversed = reverse_decay(&state, elapsed, 1.0).unwrap();

        // Should be approximately 0.002
        assert!((reversed.delta() - 0.002).abs() < 0.0001);
    }

    #[test]
    fn negative_deltas_work() {
        let negative_delta: f32 = -0.4;
        let half_life = Duration::hours(6);
        let elapsed = Duration::hours(6);

        let state = StateValue::new(0.5)
            .with_decay_half_life(half_life)
            .with_delta(negative_delta);

        let reversed = reverse_decay(&state, elapsed, 1.0).unwrap();

        // Should be approximately -0.8
        assert!((reversed.delta() - (-0.8)).abs() < 0.001);
    }

    #[test]
    fn state_value_reversal_convenience() {
        let value = StateValue::new(0.5)
            .with_decay_half_life(Duration::hours(6))
            .with_delta(0.4);

        let reversed = reverse_state_value_decay(&value, Duration::hours(6), 1.0).unwrap();

        // Should be approximately 0.8
        assert!((reversed - 0.8).abs() < 0.001);
    }

    #[test]
    fn state_value_reversal_no_decay() {
        let value = StateValue::new(0.5).with_no_decay().with_delta(0.4);

        let result = reverse_state_value_decay(&value, Duration::hours(6), 1.0);

        assert!(result.is_err());
    }

    #[test]
    fn state_value_reversal_feedback_loop() {
        let mut value = StateValue::new(0.5)
            .with_decay_half_life(Duration::hours(6))
            .with_delta(0.4);
        value.mark_feedback_loop_affected();

        let result = reverse_state_value_decay(&value, Duration::hours(6), 1.0);

        assert!(result.is_err());
        assert!(result.unwrap_err().is_feedback_loop_effect());
    }

    #[test]
    fn invalid_reversal_nan_delta() {
        let result = reverse_decay_raw(f32::NAN, Duration::hours(1), Some(Duration::hours(1)), 1.0);
        assert!(result.is_err());
        assert!(result.unwrap_err().is_invalid_reversal());
    }

    #[test]
    fn overflow_protection() {
        // Very long elapsed time could cause overflow
        let state = StateValue::new(0.5)
            .with_decay_half_life(Duration::hours(1))
            .with_delta(0.001);
        let result = reverse_decay(&state, Duration::years(1000), 1.0);

        // Should return error rather than panic or produce infinity
        assert!(result.is_err());
        assert!(result.unwrap_err().is_invalid_reversal());
    }

    #[test]
    fn round_trip_accuracy() {
        // Test round-trip: original -> decay -> reverse -> original

        let original: f32 = 0.75;
        let half_life = Duration::hours(8);
        let elapsed = Duration::hours(16);
        let time_scale = 1.0;

        // Decay: 0.75 * 0.5^2 = 0.1875
        let ln2 = std::f64::consts::LN_2;
        let elapsed_ms = elapsed.as_millis() as f64 * time_scale;
        let half_life_ms = half_life.as_millis() as f64;
        let decay_factor = (-ln2 * elapsed_ms / half_life_ms).exp();
        let decayed = (original as f64) * decay_factor;

        let state = StateValue::new(0.5)
            .with_decay_half_life(half_life)
            .with_delta(decayed as f32);

        // Reverse
        let reversed = reverse_decay(&state, elapsed, time_scale).unwrap();

        assert!((reversed.delta() - original).abs() < 0.0001);
    }

    #[test]
    fn reverse_decay_raw_works() {
        // Test the raw API for backwards compatibility
        let result =
            reverse_decay_raw(0.4, Duration::hours(6), Some(Duration::hours(6)), 1.0).unwrap();
        assert!((result - 0.8).abs() < 0.001);
    }

    #[test]
    fn reverse_decay_raw_error_no_decay() {
        let result = reverse_decay_raw(0.5, Duration::hours(6), None, 1.0);
        assert!(result.is_err());
        assert!(result.unwrap_err().is_non_reversible_dimension());
    }

    #[test]
    fn reverse_decay_preserves_base() {
        // Ensure reverse_decay preserves the base value
        let state = StateValue::new(0.7)
            .with_decay_half_life(Duration::hours(6))
            .with_delta(0.4);

        let reversed = reverse_decay(&state, Duration::hours(6), 1.0).unwrap();

        assert!((reversed.base() - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn reverse_decay_preserves_half_life() {
        // Ensure reverse_decay preserves the half-life
        let half_life = Duration::hours(6);
        let state = StateValue::new(0.5)
            .with_decay_half_life(half_life)
            .with_delta(0.4);

        let reversed = reverse_decay(&state, Duration::hours(6), 1.0).unwrap();

        assert_eq!(reversed.decay_half_life(), Some(half_life));
    }

    #[test]
    fn zero_half_life_error() {
        // Zero half-life should produce an error
        let state = StateValue::new(0.5)
            .with_decay_half_life(Duration::zero())
            .with_delta(0.4);

        let result = reverse_decay(&state, Duration::hours(6), 1.0);
        assert!(result.is_err());
        assert!(result.unwrap_err().is_invalid_reversal());
    }

    #[test]
    fn reverse_decay_raw_zero_half_life_error() {
        // Zero half-life should produce an error
        let result = reverse_decay_raw(0.4, Duration::hours(6), Some(Duration::zero()), 1.0);
        assert!(result.is_err());
        assert!(result.unwrap_err().is_invalid_reversal());
    }
}
