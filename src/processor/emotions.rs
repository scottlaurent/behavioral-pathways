//! Derived emotion computation from PAD dimensions.
//!
//! This module computes graded emotion intensities from the PAD (Pleasure-Arousal-Dominance)
//! model using fuzzy membership per Mehrabian-Russell. Each octant emotion receives a
//! membership value between 0.0 and 1.0 rather than selecting a single discrete emotion.
//! Disgust is gated by a recent moral violation flag.
//!
//! # PAD Octant Memberships
//!
//! | Valence | Arousal | Dominance | Emotion |
//! |---------|---------|-----------|---------|
//! | + | + | + | Exuberant |
//! | + | + | - | Dependent |
//! | + | - | + | Relaxed |
//! | + | - | - | Docile |
//! | - | + | + | Hostile |
//! | - | + | - | Anxious |
//! | - | - | + | Bored |
//! | - | - | - | Depressed |
//!
//! # API
//!
//! - `derive_emotion(valence, arousal, dominance, moral_violation_flag)` - Compute graded emotions from raw PAD values
//! - `get_derived_emotion(state)` - Compute graded emotions from IndividualState's effective PAD values

use crate::enums::Emotion;
use crate::state::IndividualState;

/// Graded emotion intensities derived from PAD dimensions.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EmotionIntensities {
    pub exuberant: f32,
    pub dependent: f32,
    pub relaxed: f32,
    pub docile: f32,
    pub hostile: f32,
    pub disgust: f32,
    pub anxious: f32,
    pub bored: f32,
    pub depressed: f32,
}

impl EmotionIntensities {
    /// Returns the intensity for the requested emotion.
    #[allow(dead_code)]
    #[must_use]
    pub fn intensity(&self, emotion: Emotion) -> f32 {
        match emotion {
            Emotion::Exuberant => self.exuberant,
            Emotion::Dependent => self.dependent,
            Emotion::Relaxed => self.relaxed,
            Emotion::Docile => self.docile,
            Emotion::Hostile => self.hostile,
            Emotion::Disgust => self.disgust,
            Emotion::Anxious => self.anxious,
            Emotion::Bored => self.bored,
            Emotion::Depressed => self.depressed,
            Emotion::Neutral => 0.0,
        }
    }
}

#[allow(dead_code)]
fn clamp01(value: f32) -> f32 {
    if value < 0.0 {
        0.0
    } else if value > 1.0 {
        1.0
    } else {
        value
    }
}

#[allow(dead_code)]
fn normalize(value: f32) -> f32 {
    clamp01((value + 1.0) / 2.0)
}

#[allow(dead_code)]
fn high(value_norm: f32) -> f32 {
    clamp01((value_norm - 0.5) / 0.5)
}

#[allow(dead_code)]
fn low(value_norm: f32) -> f32 {
    clamp01((0.5 - value_norm) / 0.5)
}

#[allow(dead_code)]
fn min3(a: f32, b: f32, c: f32) -> f32 {
    a.min(b).min(c)
}

/// Derives graded emotions from PAD (Pleasure-Arousal-Dominance) values.
///
/// Uses the effective (base + delta) values from the mood dimensions.
///
/// # Arguments
///
/// * `valence` - Pleasure dimension (-1.0 to 1.0)
/// * `arousal` - Arousal dimension (-1.0 to 1.0)
/// * `dominance` - Dominance dimension (-1.0 to 1.0)
/// * `moral_violation_flag` - Recent moral violation flag (0.0 to 1.0)
///
/// # Returns
///
/// The graded emotion intensities for each PAD octant.
///
/// # Examples
///
/// ```ignore
/// use behavioral_pathways::processor::derive_emotion;
///
/// let emotions = derive_emotion(1.0, 1.0, 1.0, 0.0);
/// assert_eq!(emotions.exuberant, 1.0);
/// assert_eq!(emotions.anxious, 0.0);
/// ```
#[allow(dead_code)]
#[must_use]
pub fn derive_emotion(
    valence: f32,
    arousal: f32,
    dominance: f32,
    moral_violation_flag: f32,
) -> EmotionIntensities {
    let v_norm = normalize(valence);
    let a_norm = normalize(arousal);
    let d_norm = normalize(dominance);
    let flag = clamp01(moral_violation_flag);

    let v_high = high(v_norm);
    let v_low = low(v_norm);
    let a_high = high(a_norm);
    let a_low = low(a_norm);
    let d_high = high(d_norm);
    let d_low = low(d_norm);

    EmotionIntensities {
        exuberant: min3(v_high, a_high, d_high),
        dependent: min3(v_high, a_high, d_low),
        relaxed: min3(v_high, a_low, d_high),
        docile: min3(v_high, a_low, d_low),
        hostile: min3(v_low, a_high, d_high),
        disgust: min3(v_low, a_high, d_high) * flag,
        anxious: min3(v_low, a_high, d_low),
        bored: min3(v_low, a_low, d_high),
        depressed: min3(v_low, a_low, d_low),
    }
}

/// Derives graded emotions from an IndividualState's effective PAD values.
///
/// This is the primary API for getting the current derived emotions from an entity's
/// state. It uses the effective (base + delta) values from the mood dimensions,
/// which reflects the entity's current emotional state including any temporary
/// changes from events.
///
/// # Arguments
///
/// * `state` - The individual state containing mood (PAD) dimensions
///
/// # Returns
///
/// The graded emotion intensities for each PAD octant.
#[allow(dead_code)]
#[must_use]
pub fn get_derived_emotion(state: &IndividualState) -> EmotionIntensities {
    let mood = state.mood();
    derive_emotion(
        mood.valence_effective(),
        mood.arousal_effective(),
        mood.dominance_effective(),
        state.recent_moral_violation_flag(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_approx(actual: f32, expected: f32) {
        let delta = (actual - expected).abs();
        assert!(delta < 1e-6);
    }

    #[test]
    fn exuberant_octant_peaks_at_full_positive() {
        let emotions = derive_emotion(1.0, 1.0, 1.0, 0.0);
        assert_approx(emotions.exuberant, 1.0);
        assert_approx(emotions.dependent, 0.0);
        assert_approx(emotions.relaxed, 0.0);
        assert_approx(emotions.docile, 0.0);
        assert_approx(emotions.hostile, 0.0);
        assert_approx(emotions.disgust, 0.0);
        assert_approx(emotions.anxious, 0.0);
        assert_approx(emotions.bored, 0.0);
        assert_approx(emotions.depressed, 0.0);
    }

    #[test]
    fn depressed_octant_peaks_at_full_negative() {
        let emotions = derive_emotion(-1.0, -1.0, -1.0, 0.0);
        assert_approx(emotions.depressed, 1.0);
        assert_approx(emotions.exuberant, 0.0);
        assert_approx(emotions.dependent, 0.0);
        assert_approx(emotions.relaxed, 0.0);
        assert_approx(emotions.docile, 0.0);
        assert_approx(emotions.hostile, 0.0);
        assert_approx(emotions.disgust, 0.0);
        assert_approx(emotions.anxious, 0.0);
        assert_approx(emotions.bored, 0.0);
    }

    #[test]
    fn relaxed_membership_uses_min_of_dimensions() {
        let emotions = derive_emotion(0.5, -0.5, 0.5, 0.0);
        assert_approx(emotions.relaxed, 0.5);
        assert_approx(emotions.exuberant, 0.0);
        assert_approx(emotions.dependent, 0.0);
        assert_approx(emotions.docile, 0.0);
    }

    #[test]
    fn dependent_membership_reflects_low_dominance() {
        let emotions = derive_emotion(0.4, 0.8, -0.2, 0.0);
        assert_approx(emotions.dependent, 0.2);
        assert_approx(emotions.exuberant, 0.0);
    }

    #[test]
    fn intensity_lookup_matches_struct_fields() {
        let emotions = derive_emotion(0.5, -0.5, 0.5, 0.0);
        assert_approx(emotions.intensity(Emotion::Relaxed), 0.5);
        assert_approx(emotions.intensity(Emotion::Neutral), 0.0);
    }

    #[test]
    fn disgust_membership_gated_by_flag() {
        let emotions = derive_emotion(-0.8, 0.9, 0.7, 1.0);
        assert_approx(emotions.disgust, 0.7);

        let zero_flag = derive_emotion(-0.8, 0.9, 0.7, 0.0);
        assert_approx(zero_flag.disgust, 0.0);
    }

    #[test]
    fn get_derived_emotion_uses_effective_values() {
        let mut state = IndividualState::new();
        state.mood_mut().add_valence_delta(0.6);
        state.mood_mut().add_arousal_delta(-0.2);
        state.mood_mut().add_dominance_delta(0.8);

        let emotions = get_derived_emotion(&state);
        assert_approx(emotions.relaxed, 0.2);
    }

    #[test]
    fn get_derived_emotion_applies_moral_violation_flag() {
        let mut state = IndividualState::new();
        state.set_recent_moral_violation_flag(1.0);
        state.mood_mut().add_valence_delta(-0.8);
        state.mood_mut().add_arousal_delta(0.9);
        state.mood_mut().add_dominance_delta(0.7);

        let emotions = get_derived_emotion(&state);
        assert_approx(emotions.disgust, 0.7);
    }

    #[test]
    fn clamp01_below_zero_returns_zero() {
        assert_approx(clamp01(-1.5), 0.0);
        assert_approx(clamp01(-0.1), 0.0);
    }

    #[test]
    fn clamp01_above_one_returns_one() {
        assert_approx(clamp01(1.5), 1.0);
        assert_approx(clamp01(2.0), 1.0);
    }

    #[test]
    fn clamp01_in_range_returns_same() {
        assert_approx(clamp01(0.0), 0.0);
        assert_approx(clamp01(0.5), 0.5);
        assert_approx(clamp01(1.0), 1.0);
    }

    #[test]
    fn normalize_maps_negative_one_to_zero() {
        assert_approx(normalize(-1.0), 0.0);
    }

    #[test]
    fn normalize_maps_zero_to_half() {
        assert_approx(normalize(0.0), 0.5);
    }

    #[test]
    fn normalize_maps_positive_one_to_one() {
        assert_approx(normalize(1.0), 1.0);
    }

    #[test]
    fn high_below_half_returns_zero() {
        assert_approx(high(0.0), 0.0);
        assert_approx(high(0.25), 0.0);
    }

    #[test]
    fn high_at_half_returns_zero() {
        assert_approx(high(0.5), 0.0);
    }

    #[test]
    fn high_above_half_scales_proportionally() {
        assert_approx(high(0.75), 0.5);
        assert_approx(high(1.0), 1.0);
    }

    #[test]
    fn low_below_half_scales_proportionally() {
        assert_approx(low(0.0), 1.0);
        assert_approx(low(0.25), 0.5);
    }

    #[test]
    fn low_at_half_returns_zero() {
        assert_approx(low(0.5), 0.0);
    }

    #[test]
    fn low_above_half_returns_zero() {
        assert_approx(low(0.75), 0.0);
        assert_approx(low(1.0), 0.0);
    }

    #[test]
    fn min3_returns_minimum_of_three() {
        assert_approx(min3(0.8, 0.5, 0.9), 0.5);
        assert_approx(min3(1.0, 1.0, 1.0), 1.0);
        assert_approx(min3(0.0, 0.5, 0.3), 0.0);
    }

    #[test]
    fn intensity_returns_all_emotions() {
        let emotions = derive_emotion(0.5, 0.5, 0.5, 0.0);
        assert!(emotions.intensity(Emotion::Exuberant) > 0.0);
        assert_approx(emotions.intensity(Emotion::Dependent), emotions.dependent);
        assert_approx(emotions.intensity(Emotion::Relaxed), emotions.relaxed);
        assert_approx(emotions.intensity(Emotion::Docile), emotions.docile);
        assert_approx(emotions.intensity(Emotion::Hostile), emotions.hostile);
        assert_approx(emotions.intensity(Emotion::Disgust), emotions.disgust);
        assert_approx(emotions.intensity(Emotion::Anxious), emotions.anxious);
        assert_approx(emotions.intensity(Emotion::Bored), emotions.bored);
        assert_approx(emotions.intensity(Emotion::Depressed), emotions.depressed);
    }

    #[test]
    fn dependent_octant_peaks_at_high_valence_arousal_low_dominance() {
        let emotions = derive_emotion(0.8, 0.8, -0.8, 0.0);
        assert!(emotions.dependent > 0.5);
        assert_approx(emotions.exuberant, 0.0);
    }

    #[test]
    fn relaxed_octant_peaks_at_high_valence_low_arousal_high_dominance() {
        let emotions = derive_emotion(0.8, -0.8, 0.8, 0.0);
        assert!(emotions.relaxed > 0.5);
    }

    #[test]
    fn docile_octant_peaks_at_high_valence_low_arousal_low_dominance() {
        let emotions = derive_emotion(0.8, -0.8, -0.8, 0.0);
        assert!(emotions.docile > 0.5);
    }

    #[test]
    fn hostile_octant_peaks_at_low_valence_high_arousal_high_dominance() {
        let emotions = derive_emotion(-0.8, 0.8, 0.8, 0.0);
        assert!(emotions.hostile > 0.5);
    }

    #[test]
    fn anxious_octant_peaks_at_low_valence_high_arousal_low_dominance() {
        let emotions = derive_emotion(-0.8, 0.8, -0.8, 0.0);
        assert!(emotions.anxious > 0.5);
    }

    #[test]
    fn bored_octant_peaks_at_low_valence_low_arousal_high_dominance() {
        let emotions = derive_emotion(-0.8, -0.8, 0.8, 0.0);
        assert!(emotions.bored > 0.5);
    }

    #[test]
    fn disgust_requires_both_low_valence_high_arousal_and_flag() {
        // Without flag, disgust is zero even with optimal PAD
        let no_flag = derive_emotion(-0.8, 0.8, 0.8, 0.0);
        assert_approx(no_flag.disgust, 0.0);

        // With flag, disgust matches hostile membership
        let with_flag = derive_emotion(-0.8, 0.8, 0.8, 1.0);
        assert!(with_flag.disgust > 0.5);
    }

    #[test]
    fn disgust_partial_flag_scales_membership() {
        let emotions = derive_emotion(-0.8, 0.8, 0.8, 0.5);
        assert_approx(emotions.disgust, emotions.hostile * 0.5);
    }

    #[test]
    fn disgust_clamped_flag_above_one() {
        let emotions_1_0 = derive_emotion(-0.8, 0.8, 0.8, 1.0);
        let emotions_2_0 = derive_emotion(-0.8, 0.8, 0.8, 2.0);
        assert_approx(emotions_1_0.disgust, emotions_2_0.disgust);
    }

    #[test]
    fn all_octants_sum_less_than_or_equal_to_nine() {
        let emotions = derive_emotion(0.5, 0.5, 0.5, 0.5);
        let sum = emotions.exuberant
            + emotions.dependent
            + emotions.relaxed
            + emotions.docile
            + emotions.hostile
            + emotions.disgust
            + emotions.anxious
            + emotions.bored
            + emotions.depressed;
        assert!(sum <= 9.0 + 1e-6);
    }

    #[test]
    fn edge_case_zero_pad_produces_valid_emotions() {
        let emotions = derive_emotion(0.0, 0.0, 0.0, 0.0);
        assert!(emotions.exuberant >= 0.0 && emotions.exuberant <= 1.0);
        assert!(emotions.depressed >= 0.0 && emotions.depressed <= 1.0);
    }

    #[test]
    fn mid_range_values_produce_moderate_intensities() {
        let emotions = derive_emotion(0.3, -0.4, 0.2, 0.0);
        let max_intensity = emotions
            .exuberant
            .max(emotions.dependent)
            .max(emotions.relaxed)
            .max(emotions.docile)
            .max(emotions.hostile)
            .max(emotions.disgust)
            .max(emotions.anxious)
            .max(emotions.bored)
            .max(emotions.depressed);
        assert!(max_intensity < 1.0);
    }

    #[test]
    fn get_derived_emotion_with_zero_state() {
        let state = IndividualState::new();
        let emotions = get_derived_emotion(&state);
        // All values should be in valid range [0, 1]
        assert!(emotions.exuberant >= 0.0 && emotions.exuberant <= 1.0);
        assert!(emotions.depressed >= 0.0 && emotions.depressed <= 1.0);
    }

    #[test]
    fn get_derived_emotion_with_extreme_positive_pad() {
        let mut state = IndividualState::new();
        state.mood_mut().add_valence_delta(2.0);
        state.mood_mut().add_arousal_delta(2.0);
        state.mood_mut().add_dominance_delta(2.0);

        let emotions = get_derived_emotion(&state);
        assert_approx(emotions.exuberant, 1.0);
    }

    #[test]
    fn get_derived_emotion_with_extreme_negative_pad() {
        let mut state = IndividualState::new();
        state.mood_mut().add_valence_delta(-2.0);
        state.mood_mut().add_arousal_delta(-2.0);
        state.mood_mut().add_dominance_delta(-2.0);

        let emotions = get_derived_emotion(&state);
        assert_approx(emotions.depressed, 1.0);
    }

    #[test]
    fn get_derived_emotion_with_partial_flag() {
        let mut state = IndividualState::new();
        state.set_recent_moral_violation_flag(0.6);
        state.mood_mut().add_valence_delta(-0.8);
        state.mood_mut().add_arousal_delta(0.9);
        state.mood_mut().add_dominance_delta(0.7);

        let emotions = get_derived_emotion(&state);
        assert!(emotions.disgust > 0.0);
        assert!(emotions.disgust < 0.7);
    }

    #[test]
    fn derive_emotion_with_negative_flag_treated_as_zero() {
        let emotions = derive_emotion(-0.8, 0.8, 0.8, -0.5);
        assert_approx(emotions.disgust, 0.0);
    }

    #[test]
    fn adjacent_octants_influence_each_other() {
        // Pure exuberant (+++): full positive
        let exuberant = derive_emotion(1.0, 1.0, 1.0, 0.0);
        assert_approx(exuberant.exuberant, 1.0);
        assert_approx(exuberant.dependent, 0.0);
        assert_approx(exuberant.relaxed, 0.0);
        assert_approx(exuberant.docile, 0.0);

        // Transition toward dependent (++-):
        let transition = derive_emotion(0.8, 0.8, -0.2, 0.0);
        assert!(transition.exuberant < exuberant.exuberant);
        assert!(transition.dependent > 0.0);
    }

    #[test]
    fn intensity_neutral_emotion_returns_zero() {
        let emotions = derive_emotion(0.5, 0.5, 0.5, 0.0);
        assert_approx(emotions.intensity(Emotion::Neutral), 0.0);
    }

    #[test]
    fn clamp01_boundary_just_below_zero() {
        assert_approx(clamp01(-0.0001), 0.0);
    }

    #[test]
    fn clamp01_boundary_just_above_one() {
        assert_approx(clamp01(1.0001), 1.0);
    }

    #[test]
    fn high_boundary_at_exact_half() {
        assert_approx(high(0.5), 0.0);
    }

    #[test]
    fn low_boundary_at_exact_half() {
        assert_approx(low(0.5), 0.0);
    }

    #[test]
    fn normalize_with_out_of_range_values() {
        // normalize clamps internally
        assert_approx(normalize(-2.0), 0.0);
        assert_approx(normalize(2.0), 1.0);
    }

    #[test]
    fn derive_emotion_all_neutral_dimensions() {
        let emotions = derive_emotion(0.0, 0.0, 0.0, 0.5);
        // With zero PAD, all dimensions normalize to 0.5, so high(0.5)=0, low(0.5)=0
        // Therefore all octant emotions should be 0.0 (min of zeros and zeroes)
        let sum = emotions.exuberant
            + emotions.dependent
            + emotions.relaxed
            + emotions.docile
            + emotions.hostile
            + emotions.anxious
            + emotions.bored
            + emotions.depressed
            + emotions.disgust;
        assert_approx(sum, 0.0);
    }

    #[test]
    fn clamp01_multiple_branches_via_direct_calls() {
        // Test each branch path explicitly
        let below = clamp01(-5.0);
        assert_approx(below, 0.0);

        let above = clamp01(10.0);
        assert_approx(above, 1.0);

        let middle = clamp01(0.7);
        assert_approx(middle, 0.7);

        // Test exact boundary values
        let exactly_zero = clamp01(0.0);
        assert_approx(exactly_zero, 0.0);

        let exactly_one = clamp01(1.0);
        assert_approx(exactly_one, 1.0);
    }

    #[test]
    fn high_function_boundary_conditions() {
        // Test transitions around the boundary
        assert!(high(0.49) <= high(0.50));
        assert!(high(0.50) <= high(0.51));
    }

    #[test]
    fn low_function_boundary_conditions() {
        // Test transitions around the boundary
        assert!(low(0.49) >= low(0.50));
        assert!(low(0.50) >= low(0.51));
    }

    #[test]
    fn clamp01_exercises_all_branches() {
        // Explicitly test all three branches of clamp01
        let below = clamp01(-1.0);  // < 0.0 branch
        assert_approx(below, 0.0);

        let above = clamp01(2.0);   // > 1.0 branch
        assert_approx(above, 1.0);

        let middle = clamp01(0.5);  // else branch
        assert_approx(middle, 0.5);
    }

    #[test]
    fn high_exercises_clamp01_branches() {
        // high internally calls clamp01, testing those branches
        let result_negative = high(-1.0);  // Will exercise clamp01's < 0.0 branch
        assert_approx(result_negative, 0.0);

        let result_positive = high(2.0);   // Will exercise clamp01's > 1.0 branch
        assert_approx(result_positive, 1.0);
    }

    #[test]
    fn low_exercises_clamp01_branches() {
        // low internally calls clamp01, testing those branches
        let result_negative = low(-1.0);   // Will exercise clamp01's < 0.0 branch
        assert_approx(result_negative, 1.0);

        let result_positive = low(2.0);    // Will exercise clamp01's > 1.0 branch
        assert_approx(result_positive, 0.0);
    }

    #[test]
    fn comprehensive_pad_space_exploration() {
        // Test multiple points in the PAD space to ensure comprehensive coverage
        let test_cases = vec![
            (0.2, 0.3, 0.4),
            (-0.3, -0.2, -0.1),
            (0.0, 0.5, -0.5),
            (-0.5, 0.5, 0.0),
            (0.1, -0.1, 0.1),
            (-0.1, 0.1, -0.1),
        ];

        for (v, a, d) in test_cases {
            let emotions = derive_emotion(v, a, d, 0.5);
            // All emotions should be in valid [0, 1] range
            assert!(emotions.exuberant >= 0.0 && emotions.exuberant <= 1.0);
            assert!(emotions.dependent >= 0.0 && emotions.dependent <= 1.0);
            assert!(emotions.relaxed >= 0.0 && emotions.relaxed <= 1.0);
            assert!(emotions.docile >= 0.0 && emotions.docile <= 1.0);
            assert!(emotions.hostile >= 0.0 && emotions.hostile <= 1.0);
            assert!(emotions.disgust >= 0.0 && emotions.disgust <= 1.0);
            assert!(emotions.anxious >= 0.0 && emotions.anxious <= 1.0);
            assert!(emotions.bored >= 0.0 && emotions.bored <= 1.0);
            assert!(emotions.depressed >= 0.0 && emotions.depressed <= 1.0);
        }
    }

    #[test]
    fn clamp01_exactly_one() {
        assert_approx(clamp01(1.0), 1.0);
    }

    #[test]
    fn disgust_zero_when_moral_flag_zero() {
        // This specifically tests the uncovered region: disgust multiplication by flag
        // When flag=0.0, the min3(...) * flag = 0.0 even if min3 is nonzero
        let emotions = derive_emotion(-0.9, 0.95, 0.8, 0.0);
        assert_approx(emotions.disgust, 0.0);
    }

    #[test]
    fn disgust_partial_when_flag_partial() {
        // Test that flag properly scales disgust (uncovered region multiplication)
        let emotions_full_flag = derive_emotion(-0.8, 0.9, 0.7, 1.0);
        let emotions_half_flag = derive_emotion(-0.8, 0.9, 0.7, 0.5);

        // Half flag should give roughly half the disgust
        assert!(emotions_half_flag.disgust < emotions_full_flag.disgust);
        assert!(emotions_half_flag.disgust > 0.0);
    }
}
