//! Decay processing for entity state.
//!
//! This module provides decay processors that apply time-based decay to
//! StateValue deltas according to their configured half-lives.
//!
//! The decay formula is: `delta(t) = delta(0) * exp(-ln(2) * t / half_life)`
//!
//! Time scale (from Species) affects how quickly an entity experiences time,
//! effectively shortening decay half-lives proportionally.

use crate::state::IndividualState;
use crate::types::Duration;

/// Trait for applying decay to entity state.
///
/// Decay processors operate on an entity's state over time, reducing
/// delta values toward their base according to half-life configurations.
///
/// # Examples
///
/// ```ignore
/// use behavioral_pathways::processor::{DecayProcessor, StateDecayProcessor};
/// use behavioral_pathways::state::IndividualState;
/// use behavioral_pathways::types::Duration;
///
/// let processor = StateDecayProcessor::new();
/// let mut state = IndividualState::new();
///
/// state.mood_mut().add_valence_delta(0.8);
///
/// // After one half-life (6 hours for valence), delta should be ~half
/// processor.apply_decay(&mut state, Duration::hours(6), 1.0);
/// assert!((state.mood().valence_delta() - 0.4).abs() < 0.01);
/// ```
pub trait DecayProcessor {
    /// Applies decay to an entity's individual state.
    ///
    /// # Arguments
    ///
    /// * `state` - The individual state to modify
    /// * `duration` - The real time that has elapsed
    /// * `time_scale` - The entity's time scaling factor (e.g., 6.7 for dogs)
    fn apply_decay(&self, state: &mut IndividualState, duration: Duration, time_scale: f64);
}

/// No-op decay processor that leaves state unchanged.
///
/// This can be used for testing or for entities that should not experience
/// decay (e.g., robotic stateless entities).
///
/// # Examples
///
/// ```ignore
/// use behavioral_pathways::processor::{DecayProcessor, NoOpDecayProcessor};
/// use behavioral_pathways::state::IndividualState;
/// use behavioral_pathways::types::Duration;
///
/// let processor = NoOpDecayProcessor;
/// let mut state = IndividualState::new();
///
/// state.mood_mut().add_valence_delta(0.5);
/// let delta_before = state.mood().valence_delta();
///
/// processor.apply_decay(&mut state, Duration::weeks(52), 1.0);
///
/// // State is unchanged by no-op processor
/// let delta_after = state.mood().valence_delta();
/// assert!((delta_before - delta_after).abs() < f32::EPSILON);
/// ```
#[derive(Debug, Clone, Copy, Default)]
#[allow(dead_code)]
pub struct NoOpDecayProcessor;

impl NoOpDecayProcessor {
    /// Creates a new no-op decay processor.
    #[must_use]
    #[allow(dead_code)]
    pub const fn new() -> Self {
        NoOpDecayProcessor
    }
}

impl DecayProcessor for NoOpDecayProcessor {
    fn apply_decay(&self, _state: &mut IndividualState, _duration: Duration, _time_scale: f64) {
        // No-op - state unchanged
    }
}

/// Real decay processor that applies exponential decay to state deltas.
///
/// This processor applies the decay formula:
/// `delta(t) = delta(0) * exp(-ln(2) * t / half_life)`
///
/// Where:
/// - `t` is the elapsed time (scaled by entity's time_scale)
/// - `half_life` is the dimension's decay half-life
///
/// Time scale affects how quickly an entity experiences psychological time.
/// A dog with time_scale 6.7 experiences ~7 psychological days per real day,
/// meaning their deltas decay faster in real time.
///
/// # Examples
///
/// ```ignore
/// use behavioral_pathways::processor::{DecayProcessor, StateDecayProcessor};
/// use behavioral_pathways::state::IndividualState;
/// use behavioral_pathways::types::Duration;
///
/// let processor = StateDecayProcessor::new();
/// let mut state = IndividualState::new();
///
/// // Stress has a 12-hour half-life
/// state.needs_mut().add_stress_delta(0.8);
///
/// // After 12 hours, delta should be ~half
/// processor.apply_decay(&mut state, Duration::hours(12), 1.0);
/// assert!((state.needs().stress().delta() - 0.4).abs() < 0.01);
///
/// // After another 12 hours, delta should be ~quarter of original
/// processor.apply_decay(&mut state, Duration::hours(12), 1.0);
/// assert!((state.needs().stress().delta() - 0.2).abs() < 0.01);
/// ```
#[derive(Debug, Clone, Copy, Default)]
#[allow(dead_code)]
pub struct StateDecayProcessor;

impl StateDecayProcessor {
    /// Creates a new state decay processor.
    #[must_use]
    #[allow(dead_code)]
    pub const fn new() -> Self {
        StateDecayProcessor
    }
}

impl DecayProcessor for StateDecayProcessor {
    fn apply_decay(&self, state: &mut IndividualState, duration: Duration, time_scale: f64) {
        // Apply time scaling - psychological time passes faster for shorter-lived species
        // For a dog with time_scale 6.7, 1 real day feels like ~7 psychological days
        let scaled_duration =
            Duration::from_millis((duration.as_millis() as f64 * time_scale) as u64);

        // Apply decay to all state components
        // Note: IndividualState.apply_decay handles which components decay
        // and respects dimensions with no decay (like acquired_capability)
        state.apply_decay(scaled_duration);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- NoOpDecayProcessor Tests ---

    #[test]
    fn no_op_processor_leaves_state_unchanged() {
        let processor = NoOpDecayProcessor::new();
        let mut state = IndividualState::new();

        state.mood_mut().add_valence_delta(0.5);
        let delta_before = state.mood().valence_delta();

        processor.apply_decay(&mut state, Duration::years(100), 1.0);

        let delta_after = state.mood().valence_delta();
        assert!((delta_before - delta_after).abs() < f32::EPSILON);
    }

    #[test]
    fn no_op_processor_ignores_time_scale() {
        let processor = NoOpDecayProcessor::new();
        let mut state = IndividualState::new();

        state.needs_mut().add_stress_delta(0.7);
        let delta_before = state.needs().stress().delta();

        // Even with extreme time scale, no change
        processor.apply_decay(&mut state, Duration::days(1), 100.0);

        let delta_after = state.needs().stress().delta();
        assert!((delta_before - delta_after).abs() < f32::EPSILON);
    }

    #[test]
    fn no_op_processor_default() {
        let processor = NoOpDecayProcessor::default();
        let mut state = IndividualState::new();

        state.disposition_mut().add_grievance_delta(0.3);
        processor.apply_decay(&mut state, Duration::weeks(4), 1.0);

        assert!((state.disposition().grievance().delta() - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn no_op_processor_clone() {
        let p1 = NoOpDecayProcessor::new();
        let p2 = p1.clone();

        let mut state = IndividualState::new();
        state.mood_mut().add_arousal_delta(0.4);

        p1.apply_decay(&mut state, Duration::days(1), 1.0);
        p2.apply_decay(&mut state, Duration::days(1), 1.0);

        assert!((state.mood().arousal_delta() - 0.4).abs() < f32::EPSILON);
    }

    #[test]
    fn no_op_processor_copy() {
        let p1 = NoOpDecayProcessor::new();
        let p2 = p1; // Copy

        let mut state = IndividualState::new();
        state.mood_mut().add_dominance_delta(-0.2);

        p1.apply_decay(&mut state, Duration::hours(12), 2.0);
        p2.apply_decay(&mut state, Duration::hours(12), 2.0);

        assert!((state.mood().dominance_delta() - (-0.2)).abs() < f32::EPSILON);
    }

    #[test]
    fn no_op_processor_debug() {
        let processor = NoOpDecayProcessor::new();
        let debug = format!("{:?}", processor);
        assert!(debug.contains("NoOpDecayProcessor"));
    }

    #[test]
    fn trait_object_usage() {
        let processor: &dyn DecayProcessor = &NoOpDecayProcessor::new();
        let mut state = IndividualState::new();

        state.mental_health_mut().add_depression_delta(0.2);

        processor.apply_decay(&mut state, Duration::weeks(2), 1.5);

        assert!((state.mental_health().depression().delta() - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn boxed_trait_object() {
        let processor: Box<dyn DecayProcessor> = Box::new(NoOpDecayProcessor::new());
        let mut state = IndividualState::new();

        state
            .person_characteristics_mut()
            .social_capital_mut()
            .add_delta(0.15);

        processor.apply_decay(&mut state, Duration::months(3), 1.0);

        assert!(
            (state.person_characteristics().social_capital().delta() - 0.15).abs() < f32::EPSILON
        );
    }

    // --- StateDecayProcessor Tests (Phase 4) ---

    #[test]
    fn stress_decays_to_half() {
        // Test name from phase-4.md: stress_decays_to_half
        let processor = StateDecayProcessor::new();
        let mut state = IndividualState::new();

        state.needs_mut().add_stress_delta(0.8);

        // Stress has 12-hour half-life
        processor.apply_decay(&mut state, Duration::hours(12), 1.0);

        // After one half-life, delta should be approximately half
        let delta = state.needs().stress().delta();
        assert!((delta - 0.4).abs() < 0.01);
    }

    #[test]
    fn decay_respects_time_scale() {
        // Test name from phase-4.md: decay_respects_time_scale
        let processor = StateDecayProcessor::new();

        // Test human (time_scale = 1.0)
        let mut human_state = IndividualState::new();
        human_state.needs_mut().add_stress_delta(0.8);

        // Test dog (time_scale = 6.67)
        let mut dog_state = IndividualState::new();
        dog_state.needs_mut().add_stress_delta(0.8);

        // Apply same real-time duration to both
        processor.apply_decay(&mut human_state, Duration::hours(12), 1.0);
        processor.apply_decay(&mut dog_state, Duration::hours(12), 6.67);

        let human_delta = human_state.needs().stress().delta();
        let dog_delta = dog_state.needs().stress().delta();

        // Human: 12 hours = 1 half-life -> delta ~0.4
        assert!((human_delta - 0.4).abs() < 0.01);

        // Dog: 12 hours * 6.67 = ~80 hours = ~6.67 half-lives -> delta very small
        // 0.8 * 0.5^6.67 ~= 0.008
        assert!(dog_delta < 0.02);
    }

    #[test]
    fn decay_processor_replaces_stub() {
        // Test name from phase-4.md: decay_processor_replaces_stub
        let real_processor = StateDecayProcessor::new();
        let noop_processor = NoOpDecayProcessor::new();

        let mut real_state = IndividualState::new();
        real_state.mood_mut().add_valence_delta(0.8);

        let mut noop_state = IndividualState::new();
        noop_state.mood_mut().add_valence_delta(0.8);

        // Apply same decay to both
        real_processor.apply_decay(&mut real_state, Duration::hours(6), 1.0);
        noop_processor.apply_decay(&mut noop_state, Duration::hours(6), 1.0);

        // Real processor changes state
        assert!((real_state.mood().valence_delta() - 0.4).abs() < 0.01);

        // No-op processor leaves state unchanged
        assert!((noop_state.mood().valence_delta() - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn no_decay_for_none_half_life() {
        // Test name from phase-4.md: no_decay_for_none_half_life
        let processor = StateDecayProcessor::new();
        let mut state = IndividualState::new();

        // Acquired capability has no decay (infinite half-life)
        state.mental_health_mut().add_acquired_capability_delta(0.5);

        // Apply significant time
        processor.apply_decay(&mut state, Duration::years(10), 1.0);

        // AC should remain unchanged
        let delta = state.mental_health().acquired_capability().delta();
        assert!((delta - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn state_decay_processor_default() {
        let processor = StateDecayProcessor::default();
        let mut state = IndividualState::new();

        state.mood_mut().add_valence_delta(0.8);
        processor.apply_decay(&mut state, Duration::hours(6), 1.0);

        // Should decay like new()
        assert!((state.mood().valence_delta() - 0.4).abs() < 0.01);
    }

    #[test]
    fn state_decay_processor_clone() {
        let p1 = StateDecayProcessor::new();
        let p2 = p1.clone();

        let mut state = IndividualState::new();
        state.mood_mut().add_valence_delta(0.8);

        p1.apply_decay(&mut state, Duration::hours(3), 1.0);
        // First decay: 3 hours is half of 6-hour half-life
        // 0.8 * exp(-ln(2) * 3 / 6) = 0.8 * sqrt(0.5) ~= 0.566

        let mid_delta = state.mood().valence_delta();
        assert!(mid_delta > 0.5 && mid_delta < 0.7);

        p2.apply_decay(&mut state, Duration::hours(3), 1.0);
        // Total 6 hours: should be ~0.4

        assert!((state.mood().valence_delta() - 0.4).abs() < 0.05);
    }

    #[test]
    fn state_decay_processor_copy() {
        let p1 = StateDecayProcessor::new();
        let p2 = p1; // Copy

        let mut state = IndividualState::new();
        state.mood_mut().add_valence_delta(0.8);

        p1.apply_decay(&mut state, Duration::hours(3), 1.0);
        p2.apply_decay(&mut state, Duration::hours(3), 1.0);

        // Total 6 hours -> ~0.4
        assert!((state.mood().valence_delta() - 0.4).abs() < 0.05);
    }

    #[test]
    fn state_decay_processor_debug() {
        let processor = StateDecayProcessor::new();
        let debug = format!("{:?}", processor);
        assert!(debug.contains("StateDecayProcessor"));
    }

    #[test]
    fn state_decay_processor_trait_object() {
        let processor: &dyn DecayProcessor = &StateDecayProcessor::new();
        let mut state = IndividualState::new();

        state.mood_mut().add_valence_delta(0.8);
        processor.apply_decay(&mut state, Duration::hours(6), 1.0);

        assert!((state.mood().valence_delta() - 0.4).abs() < 0.01);
    }

    #[test]
    fn state_decay_processor_boxed() {
        let processor: Box<dyn DecayProcessor> = Box::new(StateDecayProcessor::new());
        let mut state = IndividualState::new();

        state.mood_mut().add_valence_delta(0.8);
        processor.apply_decay(&mut state, Duration::hours(6), 1.0);

        assert!((state.mood().valence_delta() - 0.4).abs() < 0.01);
    }

    #[test]
    fn valence_half_life_six_hours() {
        let processor = StateDecayProcessor::new();
        let mut state = IndividualState::new();

        state.mood_mut().add_valence_delta(1.0);

        // After 6 hours (1 half-life)
        processor.apply_decay(&mut state, Duration::hours(6), 1.0);
        assert!((state.mood().valence_delta() - 0.5).abs() < 0.01);

        // After another 6 hours (2 half-lives total)
        processor.apply_decay(&mut state, Duration::hours(6), 1.0);
        assert!((state.mood().valence_delta() - 0.25).abs() < 0.01);
    }

    #[test]
    fn arousal_half_life_six_hours() {
        let processor = StateDecayProcessor::new();
        let mut state = IndividualState::new();

        state.mood_mut().add_arousal_delta(1.0);

        processor.apply_decay(&mut state, Duration::hours(6), 1.0);
        assert!((state.mood().arousal_delta() - 0.5).abs() < 0.01);
    }

    #[test]
    fn dominance_half_life_twelve_hours() {
        let processor = StateDecayProcessor::new();
        let mut state = IndividualState::new();

        state.mood_mut().add_dominance_delta(1.0);

        processor.apply_decay(&mut state, Duration::hours(12), 1.0);
        assert!((state.mood().dominance_delta() - 0.5).abs() < 0.01);
    }

    #[test]
    fn depression_half_life_one_week() {
        let processor = StateDecayProcessor::new();
        let mut state = IndividualState::new();

        state.mental_health_mut().add_depression_delta(1.0);

        processor.apply_decay(&mut state, Duration::weeks(1), 1.0);
        assert!((state.mental_health().depression().delta() - 0.5).abs() < 0.01);
    }

    #[test]
    fn grievance_half_life_one_week() {
        let processor = StateDecayProcessor::new();
        let mut state = IndividualState::new();

        state.disposition_mut().add_grievance_delta(1.0);

        processor.apply_decay(&mut state, Duration::weeks(1), 1.0);
        assert!((state.disposition().grievance().delta() - 0.5).abs() < 0.01);
    }

    #[test]
    fn impulse_control_half_life_one_month() {
        let processor = StateDecayProcessor::new();
        let mut state = IndividualState::new();

        state.disposition_mut().add_impulse_control_delta(1.0);

        processor.apply_decay(&mut state, Duration::months(1), 1.0);
        assert!((state.disposition().impulse_control().delta() - 0.5).abs() < 0.01);
    }

    #[test]
    fn zero_duration_no_decay() {
        let processor = StateDecayProcessor::new();
        let mut state = IndividualState::new();

        state.mood_mut().add_valence_delta(0.8);

        processor.apply_decay(&mut state, Duration::from_millis(0), 1.0);

        assert!((state.mood().valence_delta() - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn multiple_dimensions_decay_independently() {
        let processor = StateDecayProcessor::new();
        let mut state = IndividualState::new();

        state.mood_mut().add_valence_delta(0.8);
        state.needs_mut().add_stress_delta(0.8);
        state.mental_health_mut().add_depression_delta(0.8);

        // Apply 12 hours
        processor.apply_decay(&mut state, Duration::hours(12), 1.0);

        // Valence: 2 half-lives (6h each) -> ~0.2
        assert!((state.mood().valence_delta() - 0.2).abs() < 0.05);

        // Stress: 1 half-life (12h) -> ~0.4
        assert!((state.needs().stress().delta() - 0.4).abs() < 0.05);

        // Depression: 12h / 168h = ~0.07 half-lives -> ~0.76
        let depression_delta = state.mental_health().depression().delta();
        assert!(depression_delta > 0.7 && depression_delta < 0.85);
    }
}
