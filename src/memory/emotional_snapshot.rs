//! Emotional snapshot capturing PAD values at memory formation.
//!
//! An emotional snapshot is a frozen copy of the entity's mood at the time
//! a memory was formed. Unlike mood, snapshots do not decay over time.

use crate::state::Mood;

/// A frozen snapshot of PAD (Pleasure-Arousal-Dominance) values.
///
/// Captures how the entity felt at the moment a memory was formed.
/// Unlike `Mood`, this is immutable and does not decay over time.
///
/// All dimensions are in the range -1.0 to 1.0:
/// - Valence: -1 (displeasure) to +1 (pleasure)
/// - Arousal: -1 (deactivated) to +1 (activated)
/// - Dominance: -1 (powerless) to +1 (in-control)
///
/// # Examples
///
/// ```
/// use behavioral_pathways::memory::EmotionalSnapshot;
///
/// let snapshot = EmotionalSnapshot::new(0.5, 0.3, -0.2);
/// assert!((snapshot.valence() - 0.5).abs() < f32::EPSILON);
/// assert!((snapshot.arousal() - 0.3).abs() < f32::EPSILON);
/// assert!((snapshot.dominance() - (-0.2)).abs() < f32::EPSILON);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EmotionalSnapshot {
    /// Valence: pleasantness of emotional experience.
    /// Range: -1 (displeasure) to +1 (pleasure)
    valence: f32,

    /// Arousal: level of activation or energy.
    /// Range: -1 (deactivated) to +1 (activated)
    arousal: f32,

    /// Dominance: sense of control or influence.
    /// Range: -1 (powerless) to +1 (in-control)
    dominance: f32,
}

impl EmotionalSnapshot {
    /// Creates a new emotional snapshot with the given PAD values.
    ///
    /// Values are clamped to the valid range of -1.0 to 1.0.
    ///
    /// # Arguments
    ///
    /// * `valence` - Pleasure/displeasure dimension
    /// * `arousal` - Activation level dimension
    /// * `dominance` - Control/powerlessness dimension
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::memory::EmotionalSnapshot;
    ///
    /// let snapshot = EmotionalSnapshot::new(0.5, 0.3, -0.2);
    /// ```
    #[must_use]
    pub fn new(valence: f32, arousal: f32, dominance: f32) -> Self {
        EmotionalSnapshot {
            valence: valence.clamp(-1.0, 1.0),
            arousal: arousal.clamp(-1.0, 1.0),
            dominance: dominance.clamp(-1.0, 1.0),
        }
    }

    /// Creates an emotional snapshot from a Mood, capturing its current effective values.
    ///
    /// # Arguments
    ///
    /// * `mood` - The mood to capture
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::state::Mood;
    /// use behavioral_pathways::memory::EmotionalSnapshot;
    ///
    /// let mood = Mood::new().with_valence_base(0.3);
    /// let snapshot = EmotionalSnapshot::from_mood(&mood);
    /// assert!((snapshot.valence() - 0.3).abs() < f32::EPSILON);
    /// ```
    #[must_use]
    pub fn from_mood(mood: &Mood) -> Self {
        EmotionalSnapshot {
            valence: mood.valence_effective(),
            arousal: mood.arousal_effective(),
            dominance: mood.dominance_effective(),
        }
    }

    /// Creates a neutral emotional snapshot with all dimensions at 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::memory::EmotionalSnapshot;
    ///
    /// let neutral = EmotionalSnapshot::neutral();
    /// assert!(neutral.valence().abs() < f32::EPSILON);
    /// assert!(neutral.arousal().abs() < f32::EPSILON);
    /// assert!(neutral.dominance().abs() < f32::EPSILON);
    /// ```
    #[must_use]
    pub fn neutral() -> Self {
        EmotionalSnapshot {
            valence: 0.0,
            arousal: 0.0,
            dominance: 0.0,
        }
    }

    /// Returns the valence (pleasure/displeasure) value.
    #[must_use]
    pub fn valence(&self) -> f32 {
        self.valence
    }

    /// Returns the arousal (activation level) value.
    #[must_use]
    pub fn arousal(&self) -> f32 {
        self.arousal
    }

    /// Returns the dominance (control) value.
    #[must_use]
    pub fn dominance(&self) -> f32 {
        self.dominance
    }

    /// Computes mood congruence with another snapshot or effective mood values.
    ///
    /// Uses weighted formula from Phase 6 spec:
    /// - `valence_match = 1.0 - abs(memory_valence - mood_valence)`
    /// - `arousal_match = 1.0 - abs(memory_arousal - mood_arousal)`
    /// - `dominance_match = 1.0 - abs(memory_dominance - mood_dominance)`
    /// - `congruence = valence_match * 0.60 + arousal_match * 0.25 + dominance_match * 0.15`
    ///
    /// Match values are clamped to [0.0, 1.0] range. When PAD values differ by more
    /// than 1.0 on any dimension, match for that dimension is 0.0.
    ///
    /// Returns a value from 0.0 (completely incongruent) to 1.0 (perfect match).
    ///
    /// # Arguments
    ///
    /// * `valence` - Current valence to compare
    /// * `arousal` - Current arousal to compare
    /// * `dominance` - Current dominance to compare
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::memory::EmotionalSnapshot;
    ///
    /// let snapshot = EmotionalSnapshot::new(0.5, 0.3, -0.2);
    ///
    /// // Same values should give perfect congruence
    /// let congruence = snapshot.compute_congruence(0.5, 0.3, -0.2);
    /// assert!((congruence - 1.0).abs() < f32::EPSILON);
    ///
    /// // Opposite values should give low congruence
    /// let opposite = snapshot.compute_congruence(-0.5, -0.3, 0.2);
    /// assert!(opposite < 0.5);
    /// ```
    #[must_use]
    pub fn compute_congruence(&self, valence: f32, arousal: f32, dominance: f32) -> f32 {
        // Weights from spec: valence 0.60, arousal 0.25, dominance 0.15
        const VALENCE_WEIGHT: f32 = 0.60;
        const AROUSAL_WEIGHT: f32 = 0.25;
        const DOMINANCE_WEIGHT: f32 = 0.15;

        // Match formula from Phase 6 spec:
        // match = 1.0 - abs(memory_value - mood_value)
        // Clamped to [0.0, 1.0] to handle differences > 1.0
        let valence_match = (1.0 - (self.valence - valence).abs()).clamp(0.0, 1.0);
        let arousal_match = (1.0 - (self.arousal - arousal).abs()).clamp(0.0, 1.0);
        let dominance_match = (1.0 - (self.dominance - dominance).abs()).clamp(0.0, 1.0);

        valence_match * VALENCE_WEIGHT
            + arousal_match * AROUSAL_WEIGHT
            + dominance_match * DOMINANCE_WEIGHT
    }

    /// Computes mood congruence with a Mood.
    ///
    /// Convenience method that extracts effective values from the mood.
    ///
    /// # Arguments
    ///
    /// * `mood` - The mood to compare against
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::state::Mood;
    /// use behavioral_pathways::memory::EmotionalSnapshot;
    ///
    /// let snapshot = EmotionalSnapshot::new(0.5, 0.3, -0.2);
    /// let mood = Mood::new()
    ///     .with_valence_base(0.5)
    ///     .with_arousal_base(0.3)
    ///     .with_dominance_base(-0.2);
    ///
    /// let congruence = snapshot.compute_congruence_with_mood(&mood);
    /// assert!((congruence - 1.0).abs() < f32::EPSILON);
    /// ```
    #[must_use]
    pub fn compute_congruence_with_mood(&self, mood: &Mood) -> f32 {
        self.compute_congruence(
            mood.valence_effective(),
            mood.arousal_effective(),
            mood.dominance_effective(),
        )
    }
}

impl Default for EmotionalSnapshot {
    fn default() -> Self {
        EmotionalSnapshot::neutral()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emotional_snapshot_captures_pad() {
        let snapshot = EmotionalSnapshot::new(0.5, 0.3, -0.2);
        assert!((snapshot.valence() - 0.5).abs() < f32::EPSILON);
        assert!((snapshot.arousal() - 0.3).abs() < f32::EPSILON);
        assert!((snapshot.dominance() - (-0.2)).abs() < f32::EPSILON);
    }

    #[test]
    fn emotional_snapshot_bounds_enforced() {
        // Values beyond range should be clamped
        let snapshot = EmotionalSnapshot::new(1.5, -1.5, 2.0);
        assert!((snapshot.valence() - 1.0).abs() < f32::EPSILON);
        assert!((snapshot.arousal() - (-1.0)).abs() < f32::EPSILON);
        assert!((snapshot.dominance() - 1.0).abs() < f32::EPSILON);

        // Verify clamping at lower bound
        let snapshot2 = EmotionalSnapshot::new(-2.0, -2.0, -2.0);
        assert!((snapshot2.valence() - (-1.0)).abs() < f32::EPSILON);
        assert!((snapshot2.arousal() - (-1.0)).abs() < f32::EPSILON);
        assert!((snapshot2.dominance() - (-1.0)).abs() < f32::EPSILON);
    }

    #[test]
    fn from_mood_captures_effective_values() {
        let mut mood = Mood::new().with_valence_base(0.3);
        mood.add_valence_delta(0.2);
        mood.add_arousal_delta(0.4);
        mood.add_dominance_delta(-0.1);

        let snapshot = EmotionalSnapshot::from_mood(&mood);

        // Should capture effective values (base + delta)
        assert!((snapshot.valence() - 0.5).abs() < f32::EPSILON);
        assert!((snapshot.arousal() - 0.4).abs() < f32::EPSILON);
        assert!((snapshot.dominance() - (-0.1)).abs() < f32::EPSILON);
    }

    #[test]
    fn neutral_is_all_zeros() {
        let snapshot = EmotionalSnapshot::neutral();
        assert!(snapshot.valence().abs() < f32::EPSILON);
        assert!(snapshot.arousal().abs() < f32::EPSILON);
        assert!(snapshot.dominance().abs() < f32::EPSILON);
    }

    #[test]
    fn default_is_neutral() {
        let snapshot = EmotionalSnapshot::default();
        assert!(snapshot.valence().abs() < f32::EPSILON);
        assert!(snapshot.arousal().abs() < f32::EPSILON);
        assert!(snapshot.dominance().abs() < f32::EPSILON);
    }

    #[test]
    fn compute_congruence_perfect_match() {
        let snapshot = EmotionalSnapshot::new(0.5, 0.3, -0.2);
        let congruence = snapshot.compute_congruence(0.5, 0.3, -0.2);
        assert!((congruence - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn compute_congruence_uses_pad_weights_60_25_15() {
        // Create snapshot at one extreme
        let snapshot = EmotionalSnapshot::new(1.0, 1.0, 1.0);

        // Test with only valence difference (max diff = 2.0)
        // valence_match = max(0, 1.0 - 2.0) = 0.0
        // arousal_match = 1.0, dominance_match = 1.0
        // congruence = 0.0 * 0.60 + 1.0 * 0.25 + 1.0 * 0.15 = 0.40
        let congruence_valence_diff = snapshot.compute_congruence(-1.0, 1.0, 1.0);
        assert!((congruence_valence_diff - 0.40).abs() < 0.01);

        // Test with only arousal difference (max diff = 2.0)
        // valence_match = 1.0, arousal_match = 0.0, dominance_match = 1.0
        // congruence = 1.0 * 0.60 + 0.0 * 0.25 + 1.0 * 0.15 = 0.75
        let congruence_arousal_diff = snapshot.compute_congruence(1.0, -1.0, 1.0);
        assert!((congruence_arousal_diff - 0.75).abs() < 0.01);

        // Test with only dominance difference (max diff = 2.0)
        // valence_match = 1.0, arousal_match = 1.0, dominance_match = 0.0
        // congruence = 1.0 * 0.60 + 1.0 * 0.25 + 0.0 * 0.15 = 0.85
        let congruence_dominance_diff = snapshot.compute_congruence(1.0, 1.0, -1.0);
        assert!((congruence_dominance_diff - 0.85).abs() < 0.01);
    }

    #[test]
    fn compute_congruence_max_difference() {
        // Maximum difference on all dimensions
        let snapshot = EmotionalSnapshot::new(1.0, 1.0, 1.0);
        let congruence = snapshot.compute_congruence(-1.0, -1.0, -1.0);

        // All matches are 0.0
        // congruence = 0.0 * 0.60 + 0.0 * 0.25 + 0.0 * 0.15 = 0.0
        assert!(congruence.abs() < f32::EPSILON);
    }

    #[test]
    fn compute_congruence_with_mood() {
        let snapshot = EmotionalSnapshot::new(0.5, 0.3, -0.2);
        let mood = Mood::new()
            .with_valence_base(0.5)
            .with_arousal_base(0.3)
            .with_dominance_base(-0.2);

        let congruence = snapshot.compute_congruence_with_mood(&mood);
        assert!((congruence - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn compute_congruence_partial_match() {
        let snapshot = EmotionalSnapshot::new(0.5, 0.5, 0.5);

        // Difference of 0.5 on all dimensions
        // Each dimension match = 1.0 - 0.5 = 0.5
        // congruence = 0.5 * 0.60 + 0.5 * 0.25 + 0.5 * 0.15 = 0.5
        let congruence = snapshot.compute_congruence(0.0, 0.0, 0.0);
        assert!((congruence - 0.5).abs() < 0.01);
    }

    #[test]
    fn clone_and_copy() {
        let snapshot = EmotionalSnapshot::new(0.5, 0.3, -0.2);
        let cloned = snapshot.clone();
        let copied = snapshot;

        assert_eq!(snapshot, cloned);
        assert_eq!(snapshot, copied);
    }

    #[test]
    fn equality() {
        let snapshot1 = EmotionalSnapshot::new(0.5, 0.3, -0.2);
        let snapshot2 = EmotionalSnapshot::new(0.5, 0.3, -0.2);
        let snapshot3 = EmotionalSnapshot::new(0.5, 0.3, 0.0);

        assert_eq!(snapshot1, snapshot2);
        assert_ne!(snapshot1, snapshot3);
    }

    #[test]
    fn debug_format() {
        let snapshot = EmotionalSnapshot::new(0.5, 0.3, -0.2);
        let debug = format!("{:?}", snapshot);
        assert!(debug.contains("EmotionalSnapshot"));
        assert!(debug.contains("valence"));
        assert!(debug.contains("arousal"));
        assert!(debug.contains("dominance"));
    }
}
