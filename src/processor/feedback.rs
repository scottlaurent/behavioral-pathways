//! Feedback loop processing.
//!
//! This module implements the feedback loops that can create self-reinforcing
//! spirals in entity state. These loops are non-reversible because they
//! represent cumulative pathological processes.
//!
//! # Feedback Loops
//!
//! ## Stress Spiral (all entities)
//! High stress leads to fatigue, which reduces impulse control, which can
//! increase depression (Human only when stress > 0.7).
//!
//! ## Depression Spiral (Human only)
//! Depression leads to isolation (loneliness), which feeds back to
//! worsen depression.
//!
//! # Non-Reversibility
//! Feedback loop effects cannot be "undone" by time alone - they represent
//! cumulative pathological processes that require intervention events
//! (not automatic reversal).

use crate::enums::Species;
use crate::state::IndividualState;
use crate::types::Duration;

/// Threshold for stress to trigger the stress spiral.
/// Stress spiral activates when effective stress > 0.6.
#[allow(dead_code)]
pub const STRESS_SPIRAL_THRESHOLD: f32 = 0.6;

/// Threshold for depression to trigger the depression spiral.
/// Depression spiral activates when effective depression > 0.4.
#[allow(dead_code)]
pub const DEPRESSION_SPIRAL_THRESHOLD: f32 = 0.4;

/// Threshold for loneliness to feed back to depression.
/// Loneliness feeds back when effective loneliness > 0.5.
#[allow(dead_code)]
pub const LONELINESS_FEEDBACK_THRESHOLD: f32 = 0.5;

/// Threshold for fatigue to affect impulse control.
/// Fatigue affects impulse control when effective fatigue > 0.5.
#[allow(dead_code)]
pub const FATIGUE_IMPULSE_THRESHOLD: f32 = 0.5;

/// Threshold for chronic stress to affect depression (Human only).
/// Chronic stress affects depression when stress > 0.7.
#[allow(dead_code)]
pub const CHRONIC_STRESS_DEPRESSION_THRESHOLD: f32 = 0.7;

/// Rate at which stress causes fatigue increase.
/// fatigue_delta += stress * STRESS_SPIRAL_RATE
#[allow(dead_code)]
pub const STRESS_SPIRAL_RATE: f32 = 0.02;

/// Rate at which fatigue reduces impulse control.
#[allow(dead_code)]
pub const FATIGUE_IMPULSE_RATE: f32 = 0.01;

/// Rate at which chronic stress increases depression.
#[allow(dead_code)]
pub const CHRONIC_STRESS_RATE: f32 = 0.005;

/// Rate at which depression increases loneliness.
/// loneliness_delta += depression * DEPRESSION_SPIRAL_RATE
#[allow(dead_code)]
pub const DEPRESSION_SPIRAL_RATE: f32 = 0.01;

/// Rate at which loneliness feeds back to depression.
#[allow(dead_code)]
pub const LONELINESS_FEEDBACK_RATE: f32 = 0.005;

/// Result of applying a feedback spiral.
#[derive(Debug, Clone, Copy, Default)]
#[allow(dead_code)]
pub struct SpiralResult {
    /// Whether the spiral was triggered.
    pub triggered: bool,

    /// Change to fatigue delta (stress spiral).
    pub fatigue_change: f32,

    /// Change to impulse control delta (stress spiral).
    pub impulse_control_change: f32,

    /// Change to depression delta (stress spiral or loneliness feedback).
    pub depression_change: f32,

    /// Change to loneliness delta (depression spiral).
    pub loneliness_change: f32,
}

impl SpiralResult {
    /// Returns true if any changes were applied.
    #[must_use]
    #[allow(dead_code)]
    pub fn has_changes(&self) -> bool {
        self.triggered
    }
}

/// Applies the stress spiral feedback loop.
///
/// When stress > 0.6:
/// - fatigue_delta += stress * 0.02
/// - if fatigue > 0.5: impulse_control_delta -= 0.01
/// - if Human and stress > 0.7: depression_delta += 0.005
///
/// Rates are per-tick. When processing time, multiply by duration in days.
///
/// # Arguments
///
/// * `state` - The individual state to modify
/// * `species` - The entity's species (affects depression contribution)
/// * `duration` - The time elapsed (rates are scaled by days)
///
/// # Returns
///
/// Result describing what changes were made.
///
/// # Examples
///
/// ```ignore
/// use behavioral_pathways::processor::apply_stress_spiral;
/// use behavioral_pathways::state::IndividualState;
/// use behavioral_pathways::enums::Species;
/// use behavioral_pathways::types::Duration;
///
/// let mut state = IndividualState::new();
/// state.needs_mut().stress_mut().set_base(0.8);
/// state.needs_mut().fatigue_mut().set_base(0.6);
///
/// let result = apply_stress_spiral(&mut state, &Species::Human, Duration::days(1));
///
/// assert!(result.triggered);
/// assert!(result.fatigue_change > 0.0);
/// assert!(result.impulse_control_change < 0.0);
/// ```
#[allow(dead_code)]
pub fn apply_stress_spiral(
    state: &mut IndividualState,
    species: &Species,
    duration: Duration,
) -> SpiralResult {
    let mut result = SpiralResult::default();

    let stress = state.needs().stress_effective();

    if stress <= STRESS_SPIRAL_THRESHOLD {
        return result;
    }

    result.triggered = true;
    let days = duration.as_days() as f32;

    // Stress causes fatigue
    let fatigue_increase = stress * STRESS_SPIRAL_RATE * days;
    state.needs_mut().add_fatigue_delta(fatigue_increase);
    state
        .needs_mut()
        .fatigue_mut()
        .mark_feedback_loop_affected();
    result.fatigue_change = fatigue_increase;

    // High fatigue reduces impulse control
    let fatigue = state.needs().fatigue_effective();
    if fatigue > FATIGUE_IMPULSE_THRESHOLD {
        let impulse_decrease = FATIGUE_IMPULSE_RATE * days;
        state
            .disposition_mut()
            .add_impulse_control_delta(-impulse_decrease);
        state
            .disposition_mut()
            .impulse_control_mut()
            .mark_feedback_loop_affected();
        result.impulse_control_change = -impulse_decrease;
    }

    // Chronic high stress contributes to depression (Human only)
    if *species == Species::Human && stress > CHRONIC_STRESS_DEPRESSION_THRESHOLD {
        let depression_increase = CHRONIC_STRESS_RATE * days;
        state
            .mental_health_mut()
            .add_depression_delta(depression_increase);
        state
            .mental_health_mut()
            .depression_mut()
            .mark_feedback_loop_affected();
        result.depression_change = depression_increase;
    }

    result
}

/// Applies the depression spiral feedback loop (Human only).
///
/// When depression > 0.4:
/// - loneliness_delta += depression * 0.01
/// - if loneliness > 0.5: depression_delta += loneliness * 0.005
///
/// This creates a self-reinforcing cycle where depression leads to isolation,
/// which worsens depression.
///
/// # Arguments
///
/// * `state` - The individual state to modify
/// * `species` - The entity's species (only affects Human)
/// * `duration` - The time elapsed (rates are scaled by days)
///
/// # Returns
///
/// Result describing what changes were made.
///
/// # Examples
///
/// ```ignore
/// use behavioral_pathways::processor::apply_depression_spiral;
/// use behavioral_pathways::state::IndividualState;
/// use behavioral_pathways::enums::Species;
/// use behavioral_pathways::types::Duration;
///
/// let mut state = IndividualState::new();
/// state.mental_health_mut().depression_mut().set_base(0.6);
/// state.social_cognition_mut().loneliness_mut().set_base(0.6);
///
/// let result = apply_depression_spiral(&mut state, &Species::Human, Duration::days(1));
///
/// assert!(result.triggered);
/// assert!(result.loneliness_change > 0.0);
/// assert!(result.depression_change > 0.0);
/// ```
#[allow(dead_code)]
pub fn apply_depression_spiral(
    state: &mut IndividualState,
    species: &Species,
    duration: Duration,
) -> SpiralResult {
    let mut result = SpiralResult::default();

    // Depression spiral only applies to Human
    if *species != Species::Human {
        return result;
    }

    let depression = state.mental_health().depression_effective();

    if depression <= DEPRESSION_SPIRAL_THRESHOLD {
        return result;
    }

    result.triggered = true;
    let days = duration.as_days() as f32;

    // Depression increases loneliness passively
    let loneliness_increase = depression * DEPRESSION_SPIRAL_RATE * days;
    state
        .social_cognition_mut()
        .add_loneliness_delta(loneliness_increase);
    state
        .social_cognition_mut()
        .loneliness_mut()
        .mark_feedback_loop_affected();
    result.loneliness_change = loneliness_increase;

    // Loneliness feeds back to depression
    let loneliness = state.social_cognition().loneliness_effective();
    if loneliness > LONELINESS_FEEDBACK_THRESHOLD {
        let depression_increase = loneliness * LONELINESS_FEEDBACK_RATE * days;
        state
            .mental_health_mut()
            .add_depression_delta(depression_increase);
        state
            .mental_health_mut()
            .depression_mut()
            .mark_feedback_loop_affected();
        result.depression_change = depression_increase;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Tests from phase-4.md ---

    #[test]
    fn stress_spiral_triggers_above_threshold() {
        // Test name from phase-4.md
        let mut state = IndividualState::new();

        // Below threshold - should not trigger
        state.needs_mut().stress_mut().set_base(0.5);
        let result = apply_stress_spiral(&mut state, &Species::Human, Duration::days(1));
        assert!(!result.triggered);

        // At threshold - should not trigger (must be > 0.6)
        state.needs_mut().stress_mut().set_base(0.6);
        let result2 = apply_stress_spiral(&mut state, &Species::Human, Duration::days(1));
        assert!(!result2.triggered);

        // Above threshold - should trigger
        state.needs_mut().stress_mut().set_base(0.7);
        let result3 = apply_stress_spiral(&mut state, &Species::Human, Duration::days(1));
        assert!(result3.triggered);
    }

    #[test]
    fn stress_spiral_causes_fatigue() {
        // Test name from phase-4.md
        let mut state = IndividualState::new();
        state.needs_mut().stress_mut().set_base(0.8);

        let fatigue_before = state.needs().fatigue_effective();
        let result = apply_stress_spiral(&mut state, &Species::Human, Duration::days(1));

        assert!(result.triggered);
        assert!(result.fatigue_change > 0.0);

        // Expected: fatigue += 0.8 * 0.02 * 1 = 0.016
        assert!((result.fatigue_change - 0.016).abs() < 0.001);

        let fatigue_after = state.needs().fatigue_effective();
        assert!(fatigue_after > fatigue_before);
    }

    #[test]
    fn stress_spiral_reduces_impulse_control() {
        // Test name from phase-4.md
        let mut state = IndividualState::new();
        state.needs_mut().stress_mut().set_base(0.8);
        state.needs_mut().fatigue_mut().set_base(0.6); // Above fatigue threshold

        let impulse_before = state.disposition().impulse_control_effective();
        let result = apply_stress_spiral(&mut state, &Species::Human, Duration::days(1));

        assert!(result.triggered);
        assert!(result.impulse_control_change < 0.0);

        // Expected: impulse_control -= 0.01 * 1 = -0.01
        assert!((result.impulse_control_change - (-0.01)).abs() < 0.001);

        let impulse_after = state.disposition().impulse_control_effective();
        assert!(impulse_after < impulse_before);
    }

    #[test]
    fn chronic_stress_contributes_to_depression() {
        // Test name from phase-4.md
        let mut state = IndividualState::new();
        state.needs_mut().stress_mut().set_base(0.8); // Above 0.7 threshold

        let depression_before = state.mental_health().depression_effective();
        let result = apply_stress_spiral(&mut state, &Species::Human, Duration::days(1));

        assert!(result.triggered);
        assert!(result.depression_change > 0.0);

        // Expected: depression += 0.005 * 1 = 0.005
        assert!((result.depression_change - 0.005).abs() < 0.001);

        let depression_after = state.mental_health().depression_effective();
        assert!(depression_after > depression_before);
    }

    #[test]
    fn depression_spiral_triggers_above_threshold() {
        // Test name from phase-4.md
        let mut state = IndividualState::new();

        // Below threshold - should not trigger
        state.mental_health_mut().depression_mut().set_base(0.3);
        let result = apply_depression_spiral(&mut state, &Species::Human, Duration::days(1));
        assert!(!result.triggered);

        // At threshold - should not trigger (must be > 0.4)
        state.mental_health_mut().depression_mut().set_base(0.4);
        let result2 = apply_depression_spiral(&mut state, &Species::Human, Duration::days(1));
        assert!(!result2.triggered);

        // Above threshold - should trigger
        state.mental_health_mut().depression_mut().set_base(0.5);
        let result3 = apply_depression_spiral(&mut state, &Species::Human, Duration::days(1));
        assert!(result3.triggered);
    }

    #[test]
    fn depression_spiral_increases_loneliness() {
        // Test name from phase-4.md
        let mut state = IndividualState::new();
        state.mental_health_mut().depression_mut().set_base(0.6);

        let loneliness_before = state.social_cognition().loneliness_effective();
        let result = apply_depression_spiral(&mut state, &Species::Human, Duration::days(1));

        assert!(result.triggered);
        assert!(result.loneliness_change > 0.0);

        // Expected: loneliness += 0.6 * 0.01 * 1 = 0.006
        assert!((result.loneliness_change - 0.006).abs() < 0.001);

        let loneliness_after = state.social_cognition().loneliness_effective();
        assert!(loneliness_after > loneliness_before);
    }

    #[test]
    fn loneliness_feeds_back_to_depression() {
        // Test name from phase-4.md
        let mut state = IndividualState::new();
        state.mental_health_mut().depression_mut().set_base(0.6);
        state
            .social_cognition_mut()
            .loneliness_mut()
            .set_base(0.6); // Above 0.5 threshold

        let depression_before = state.mental_health().depression_effective();
        let result = apply_depression_spiral(&mut state, &Species::Human, Duration::days(1));

        assert!(result.triggered);
        assert!(result.depression_change > 0.0);

        // Expected: depression += 0.6 * 0.005 * 1 = 0.003
        assert!((result.depression_change - 0.003).abs() < 0.001);

        let depression_after = state.mental_health().depression_effective();
        assert!(depression_after > depression_before);
    }

    #[test]
    fn feedback_loops_are_non_reversible() {
        // Test name from phase-4.md
        // This test documents that feedback loop effects are non-reversible
        // The actual enforcement is in the reversibility module

        // Apply feedback effects
        let mut state = IndividualState::new();
        state.needs_mut().stress_mut().set_base(0.8);
        state.needs_mut().fatigue_mut().set_base(0.6);

        apply_stress_spiral(&mut state, &Species::Human, Duration::days(1));

        let fatigue_after_spiral = state.needs().fatigue_effective();

        // Feedback loop effects cannot be reversed by decay
        // They require intervention events
        // This is a conceptual test - the actual reversal prevention
        // is implemented in the reversibility module
        assert!(fatigue_after_spiral > 0.0);
    }

    #[test]
    fn feedback_loops_respect_bounds() {
        // Test name from phase-4.md
        let mut state = IndividualState::new();

        // Set already-high values
        state.needs_mut().stress_mut().set_base(0.95);
        state.needs_mut().fatigue_mut().set_base(0.95);

        // Apply spiral repeatedly
        for _ in 0..100 {
            apply_stress_spiral(&mut state, &Species::Human, Duration::days(1));
        }

        // Values should still be within bounds
        assert!(state.needs().fatigue_effective() <= 1.0);
        assert!(state.disposition().impulse_control_effective() >= 0.0);
        assert!(state.mental_health().depression_effective() <= 1.0);
    }

    // --- Additional tests ---

    #[test]
    fn depression_spiral_human_only() {
        let mut state = IndividualState::new();
        state.mental_health_mut().depression_mut().set_base(0.6);

        // Dog should not trigger depression spiral
        let dog_result = apply_depression_spiral(&mut state, &Species::Dog, Duration::days(1));
        assert!(!dog_result.triggered);

        // Human should trigger
        let human_result = apply_depression_spiral(&mut state, &Species::Human, Duration::days(1));
        assert!(human_result.triggered);
    }

    #[test]
    fn chronic_stress_depression_human_only() {
        let mut state = IndividualState::new();
        state.needs_mut().stress_mut().set_base(0.8);

        // Dog stress spiral should not affect depression
        let dog_result = apply_stress_spiral(&mut state, &Species::Dog, Duration::days(1));
        assert!(dog_result.triggered);
        assert!(dog_result.depression_change.abs() < f32::EPSILON);

        // Human stress spiral with high stress should affect depression
        let mut human_state = IndividualState::new();
        human_state.needs_mut().stress_mut().set_base(0.8);
        let human_result =
            apply_stress_spiral(&mut human_state, &Species::Human, Duration::days(1));
        assert!(human_result.triggered);
        assert!(human_result.depression_change > 0.0);
    }

    #[test]
    fn stress_spiral_no_impulse_control_below_fatigue_threshold() {
        let mut state = IndividualState::new();
        state.needs_mut().stress_mut().set_base(0.8);
        state.needs_mut().fatigue_mut().set_base(0.4); // Below fatigue threshold

        let result = apply_stress_spiral(&mut state, &Species::Human, Duration::days(1));

        assert!(result.triggered);
        assert!(result.fatigue_change > 0.0);
        assert!(result.impulse_control_change.abs() < f32::EPSILON);
    }

    #[test]
    fn depression_spiral_no_feedback_below_loneliness_threshold() {
        let mut state = IndividualState::new();
        state.mental_health_mut().depression_mut().set_base(0.6);
        state
            .social_cognition_mut()
            .loneliness_mut()
            .set_base(0.4); // Below loneliness threshold

        let result = apply_depression_spiral(&mut state, &Species::Human, Duration::days(1));

        assert!(result.triggered);
        assert!(result.loneliness_change > 0.0);
        assert!(result.depression_change.abs() < f32::EPSILON);
    }

    #[test]
    fn spiral_result_default() {
        let result = SpiralResult::default();

        assert!(!result.triggered);
        assert!(result.fatigue_change.abs() < f32::EPSILON);
        assert!(result.impulse_control_change.abs() < f32::EPSILON);
        assert!(result.depression_change.abs() < f32::EPSILON);
        assert!(result.loneliness_change.abs() < f32::EPSILON);
    }

    #[test]
    fn spiral_result_has_changes() {
        let mut result = SpiralResult::default();
        assert!(!result.has_changes());

        result.triggered = true;
        assert!(result.has_changes());
    }

    #[test]
    fn duration_scaling() {
        let mut state1 = IndividualState::new();
        state1.needs_mut().stress_mut().set_base(0.8);

        let mut state2 = IndividualState::new();
        state2.needs_mut().stress_mut().set_base(0.8);

        let result1 = apply_stress_spiral(&mut state1, &Species::Human, Duration::days(1));
        let result2 = apply_stress_spiral(&mut state2, &Species::Human, Duration::days(2));

        // Double duration should produce double effect
        assert!((result2.fatigue_change - result1.fatigue_change * 2.0).abs() < 0.001);
    }

    #[test]
    fn zero_duration_no_effect() {
        let mut state = IndividualState::new();
        state.needs_mut().stress_mut().set_base(0.8);

        let result = apply_stress_spiral(&mut state, &Species::Human, Duration::from_millis(0));

        assert!(result.triggered);
        assert!(result.fatigue_change.abs() < f32::EPSILON);
    }

    // --- Feedback loop flag tests ---

    #[test]
    fn stress_spiral_marks_fatigue_feedback_affected() {
        let mut state = IndividualState::new();
        state.needs_mut().stress_mut().set_base(0.8);

        // Initially not affected
        assert!(!state.needs().fatigue().is_feedback_loop_affected());

        apply_stress_spiral(&mut state, &Species::Human, Duration::days(1));

        // After spiral, fatigue should be marked as feedback loop affected
        assert!(state.needs().fatigue().is_feedback_loop_affected());
    }

    #[test]
    fn stress_spiral_marks_impulse_control_feedback_affected() {
        let mut state = IndividualState::new();
        state.needs_mut().stress_mut().set_base(0.8);
        state.needs_mut().fatigue_mut().set_base(0.6); // Above threshold

        // Initially not affected
        assert!(!state
            .disposition()
            .impulse_control()
            .is_feedback_loop_affected());

        apply_stress_spiral(&mut state, &Species::Human, Duration::days(1));

        // After spiral with high fatigue, impulse control should be marked
        assert!(state
            .disposition()
            .impulse_control()
            .is_feedback_loop_affected());
    }

    #[test]
    fn stress_spiral_marks_depression_feedback_affected_human() {
        let mut state = IndividualState::new();
        state.needs_mut().stress_mut().set_base(0.8); // Above chronic threshold

        // Initially not affected
        assert!(!state
            .mental_health()
            .depression()
            .is_feedback_loop_affected());

        apply_stress_spiral(&mut state, &Species::Human, Duration::days(1));

        // After spiral with chronic stress (Human), depression should be marked
        assert!(state
            .mental_health()
            .depression()
            .is_feedback_loop_affected());
    }

    #[test]
    fn stress_spiral_does_not_mark_depression_for_non_human() {
        let mut state = IndividualState::new();
        state.needs_mut().stress_mut().set_base(0.8);

        apply_stress_spiral(&mut state, &Species::Dog, Duration::days(1));

        // Dog stress spiral should not mark depression
        assert!(!state
            .mental_health()
            .depression()
            .is_feedback_loop_affected());
    }

    #[test]
    fn depression_spiral_marks_loneliness_feedback_affected() {
        let mut state = IndividualState::new();
        state.mental_health_mut().depression_mut().set_base(0.6);

        // Initially not affected
        assert!(
            !state
                .social_cognition()
                .loneliness()
                .is_feedback_loop_affected()
        );

        apply_depression_spiral(&mut state, &Species::Human, Duration::days(1));

        // After spiral, loneliness should be marked as feedback loop affected
        assert!(
            state
                .social_cognition()
                .loneliness()
                .is_feedback_loop_affected()
        );
    }

    #[test]
    fn depression_spiral_marks_depression_feedback_affected() {
        let mut state = IndividualState::new();
        state.mental_health_mut().depression_mut().set_base(0.6);
        state
            .social_cognition_mut()
            .loneliness_mut()
            .set_base(0.6); // Above threshold

        // Initially not affected (unless set by external means)
        assert!(!state
            .mental_health()
            .depression()
            .is_feedback_loop_affected());

        apply_depression_spiral(&mut state, &Species::Human, Duration::days(1));

        // After spiral with high loneliness, depression should be marked
        assert!(state
            .mental_health()
            .depression()
            .is_feedback_loop_affected());
    }

    #[test]
    fn depression_spiral_no_marking_for_non_human() {
        let mut state = IndividualState::new();
        state.mental_health_mut().depression_mut().set_base(0.6);
        state
            .social_cognition_mut()
            .loneliness_mut()
            .set_base(0.6);

        apply_depression_spiral(&mut state, &Species::Dog, Duration::days(1));

        // Dog does not have depression spiral, so nothing should be marked
        assert!(
            !state
                .social_cognition()
                .loneliness()
                .is_feedback_loop_affected()
        );
        assert!(!state
            .mental_health()
            .depression()
            .is_feedback_loop_affected());
    }

    #[test]
    fn feedback_affected_prevents_reversal() {
        use crate::processor::reverse_decay;

        let mut state = IndividualState::new();
        state.needs_mut().stress_mut().set_base(0.8);

        // Apply stress spiral to mark fatigue as feedback affected
        apply_stress_spiral(&mut state, &Species::Human, Duration::days(1));

        // Fatigue should now be marked
        assert!(state.needs().fatigue().is_feedback_loop_affected());

        // Attempt to reverse decay on fatigue should fail
        let result = reverse_decay(state.needs().fatigue(), Duration::hours(6), 1.0);
        assert!(result.is_err());
        assert!(result.unwrap_err().is_feedback_loop_effect());
    }
}
