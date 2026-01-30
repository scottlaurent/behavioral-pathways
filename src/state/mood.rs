//! Mood state using the PAD (Pleasure-Arousal-Dominance) model.
//!
//! The PAD model represents affective state along three dimensions:
//! - **Valence** (Pleasure): The pleasantness of the emotional experience
//! - **Arousal**: The level of activation or energy
//! - **Dominance**: The sense of control or influence
//!
//! Each dimension uses the StateValue pattern with base, delta, and decay.
//! All three PAD dimensions use the full -1.0 to 1.0 range (bipolar).
//!
//! Note: Fatigue and stress are NOT part of Mood. They are physiological
//! states stored in the Needs structure. The API's AffectiveState combines
//! both for convenience, but they are stored separately.

use crate::state::{Hexaco, StateValue};
use crate::types::Duration;
use serde::{Deserialize, Serialize};

/// Mood state containing PAD (Pleasure-Arousal-Dominance) dimensions.
///
/// All dimensions are bipolar, ranging from -1.0 to 1.0:
/// - Valence: -1 (displeasure) to +1 (pleasure)
/// - Arousal: -1 (deactivated/sleepy) to +1 (activated/energetic)
/// - Dominance: -1 (powerless/controlled) to +1 (in-control/dominant)
///
/// # Examples
///
/// ```
/// use behavioral_pathways::state::Mood;
/// use behavioral_pathways::types::Duration;
///
/// let mut mood = Mood::new();
///
/// // Apply a positive event
/// mood.add_valence_delta(0.3);
/// assert!(mood.valence_effective() > 0.2);
///
/// // Mood decays over time
/// mood.apply_decay(Duration::hours(6));
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mood {
    /// Valence: pleasantness of emotional experience.
    /// Range: -1 (displeasure) to +1 (pleasure)
    /// Default decay half-life: 6 hours
    valence: StateValue,

    /// Arousal: level of activation or energy.
    /// Range: -1 (deactivated) to +1 (activated)
    /// Default decay half-life: 6 hours
    arousal: StateValue,

    /// Dominance: sense of control or influence.
    /// Range: -1 (powerless) to +1 (in-control)
    /// Default decay half-life: 12 hours
    dominance: StateValue,
}

impl Mood {
    /// Default decay half-life for valence (6 hours).
    const VALENCE_DECAY_HALF_LIFE: Duration = Duration::hours(6);

    /// Default decay half-life for arousal (6 hours).
    const AROUSAL_DECAY_HALF_LIFE: Duration = Duration::hours(6);

    /// Default decay half-life for dominance (12 hours).
    const DOMINANCE_DECAY_HALF_LIFE: Duration = Duration::hours(12);

    /// Creates a new Mood with neutral (0.0) base values.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::state::Mood;
    ///
    /// let mood = Mood::new();
    /// assert!((mood.valence_effective() - 0.0).abs() < f32::EPSILON);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Mood {
            valence: StateValue::new(0.0)
                .with_bounds(-1.0, 1.0)
                .with_decay_half_life(Self::VALENCE_DECAY_HALF_LIFE),
            arousal: StateValue::new(0.0)
                .with_bounds(-1.0, 1.0)
                .with_decay_half_life(Self::AROUSAL_DECAY_HALF_LIFE),
            dominance: StateValue::new(0.0)
                .with_bounds(-1.0, 1.0)
                .with_decay_half_life(Self::DOMINANCE_DECAY_HALF_LIFE),
        }
    }

    /// Creates a Mood with baseline affect derived from HEXACO personality traits.
    ///
    /// This implements the well-established relationship between personality
    /// and affective temperament:
    /// - **Valence**: Influenced by Extraversion (+) and Neuroticism (-)
    /// - **Arousal**: Influenced by Neuroticism (+, reactivity) and Openness (+)
    /// - **Dominance**: Influenced by Extraversion (+, assertiveness)
    ///
    /// The derived values are attenuated (scaled by 0.3) to produce modest
    /// individual differences rather than extreme baselines.
    ///
    /// # Arguments
    ///
    /// * `hexaco` - The HEXACO personality profile to derive baselines from
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::state::{Hexaco, Mood};
    ///
    /// let extraverted = Hexaco::new().with_extraversion(0.8);
    /// let mood = Mood::from_personality(&extraverted);
    ///
    /// // Extraverts have higher baseline valence and dominance
    /// assert!(mood.valence_base() > 0.0);
    /// assert!(mood.dominance_base() > 0.0);
    /// ```
    #[must_use]
    pub fn from_personality(hexaco: &Hexaco) -> Self {
        // Scale factor for personality -> affect mapping
        // Keeps baselines modest (around -0.3 to +0.3 range)
        const ATTENUATION: f32 = 0.3;

        // Extraversion (+) and Neuroticism (-) influence valence
        // High extraversion = positive affect, high neuroticism = negative affect
        let valence_base =
            ATTENUATION * (hexaco.extraversion() * 0.6 - hexaco.neuroticism() * 0.6);

        // Neuroticism (+) and Openness (+) influence arousal
        // Neurotic individuals have higher reactivity, open individuals more engaged
        let arousal_base =
            ATTENUATION * (hexaco.neuroticism() * 0.4 + hexaco.openness() * 0.3);

        // Extraversion (+) influences dominance (assertiveness component)
        let dominance_base = ATTENUATION * (hexaco.extraversion() * 0.5);

        Mood {
            valence: StateValue::new(valence_base)
                .with_bounds(-1.0, 1.0)
                .with_decay_half_life(Self::VALENCE_DECAY_HALF_LIFE),
            arousal: StateValue::new(arousal_base)
                .with_bounds(-1.0, 1.0)
                .with_decay_half_life(Self::AROUSAL_DECAY_HALF_LIFE),
            dominance: StateValue::new(dominance_base)
                .with_bounds(-1.0, 1.0)
                .with_decay_half_life(Self::DOMINANCE_DECAY_HALF_LIFE),
        }
    }

    // Builder methods

    /// Sets the base valence.
    #[must_use]
    pub fn with_valence_base(mut self, value: f32) -> Self {
        self.valence.set_base(value);
        self
    }

    /// Sets the base arousal.
    #[must_use]
    pub fn with_arousal_base(mut self, value: f32) -> Self {
        self.arousal.set_base(value);
        self
    }

    /// Sets the base dominance.
    #[must_use]
    pub fn with_dominance_base(mut self, value: f32) -> Self {
        self.dominance.set_base(value);
        self
    }

    // Accessors for effective values (base + delta)

    /// Returns the effective valence (base + delta), clamped to -1.0 to 1.0.
    #[must_use]
    pub fn valence_effective(&self) -> f32 {
        self.valence.effective()
    }

    /// Returns the effective arousal (base + delta), clamped to -1.0 to 1.0.
    #[must_use]
    pub fn arousal_effective(&self) -> f32 {
        self.arousal.effective()
    }

    /// Returns the effective dominance (base + delta), clamped to -1.0 to 1.0.
    #[must_use]
    pub fn dominance_effective(&self) -> f32 {
        self.dominance.effective()
    }

    // Accessors for base values

    /// Returns the base valence.
    #[must_use]
    pub fn valence_base(&self) -> f32 {
        self.valence.base()
    }

    /// Returns the base arousal.
    #[must_use]
    pub fn arousal_base(&self) -> f32 {
        self.arousal.base()
    }

    /// Returns the base dominance.
    #[must_use]
    pub fn dominance_base(&self) -> f32 {
        self.dominance.base()
    }

    // Accessors for delta values

    /// Returns the valence delta.
    #[must_use]
    pub fn valence_delta(&self) -> f32 {
        self.valence.delta()
    }

    /// Returns the arousal delta.
    #[must_use]
    pub fn arousal_delta(&self) -> f32 {
        self.arousal.delta()
    }

    /// Returns the dominance delta.
    #[must_use]
    pub fn dominance_delta(&self) -> f32 {
        self.dominance.delta()
    }

    // Direct access to StateValue references

    /// Returns a reference to the valence StateValue.
    #[must_use]
    pub fn valence(&self) -> &StateValue {
        &self.valence
    }

    /// Returns a reference to the arousal StateValue.
    #[must_use]
    pub fn arousal(&self) -> &StateValue {
        &self.arousal
    }

    /// Returns a reference to the dominance StateValue.
    #[must_use]
    pub fn dominance(&self) -> &StateValue {
        &self.dominance
    }

    /// Returns a mutable reference to the valence StateValue.
    pub fn valence_mut(&mut self) -> &mut StateValue {
        &mut self.valence
    }

    /// Returns a mutable reference to the arousal StateValue.
    pub fn arousal_mut(&mut self) -> &mut StateValue {
        &mut self.arousal
    }

    /// Returns a mutable reference to the dominance StateValue.
    pub fn dominance_mut(&mut self) -> &mut StateValue {
        &mut self.dominance
    }

    // Delta modifiers

    /// Adds to the valence delta.
    pub fn add_valence_delta(&mut self, amount: f32) {
        self.valence.add_delta(amount);
    }

    /// Adds to the arousal delta.
    pub fn add_arousal_delta(&mut self, amount: f32) {
        self.arousal.add_delta(amount);
    }

    /// Adds to the dominance delta.
    pub fn add_dominance_delta(&mut self, amount: f32) {
        self.dominance.add_delta(amount);
    }

    // Decay

    /// Applies decay to all mood dimensions over the specified duration.
    ///
    /// Each dimension decays according to its own half-life.
    pub fn apply_decay(&mut self, elapsed: Duration) {
        self.valence.apply_decay(elapsed);
        self.arousal.apply_decay(elapsed);
        self.dominance.apply_decay(elapsed);
    }

    /// Resets all deltas to zero.
    pub fn reset_deltas(&mut self) {
        self.valence.reset_delta();
        self.arousal.reset_delta();
        self.dominance.reset_delta();
    }
}

impl Default for Mood {
    fn default() -> Self {
        Mood::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mood_contains_only_pad_dimensions() {
        // This test verifies that Mood contains exactly valence, arousal, dominance
        // and NOT fatigue or stress (those belong in Needs)
        let mood = Mood::new();

        // These should exist
        let _ = mood.valence_effective();
        let _ = mood.arousal_effective();
        let _ = mood.dominance_effective();

        // Fatigue and stress are NOT in Mood - they are in Needs
        // This is verified by the absence of such methods
    }

    #[test]
    fn new_creates_neutral_mood() {
        let mood = Mood::new();
        assert!((mood.valence_effective() - 0.0).abs() < f32::EPSILON);
        assert!((mood.arousal_effective() - 0.0).abs() < f32::EPSILON);
        assert!((mood.dominance_effective() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn valence_delta_decays_toward_zero() {
        let mut mood = Mood::new();
        mood.add_valence_delta(0.8);

        // After one half-life (6 hours), delta should be halved
        mood.apply_decay(Duration::hours(6));
        assert!((mood.valence_delta() - 0.4).abs() < 0.01);
    }

    #[test]
    fn arousal_delta_decays_toward_zero() {
        let mut mood = Mood::new();
        mood.add_arousal_delta(0.6);

        // After one half-life (6 hours), delta should be halved
        mood.apply_decay(Duration::hours(6));
        assert!((mood.arousal_delta() - 0.3).abs() < 0.01);
    }

    #[test]
    fn dominance_delta_decays_toward_zero() {
        let mut mood = Mood::new();
        mood.add_dominance_delta(0.4);

        // After one half-life (12 hours), delta should be halved
        mood.apply_decay(Duration::hours(12));
        assert!((mood.dominance_delta() - 0.2).abs() < 0.01);
    }

    #[test]
    fn builder_methods_set_base_values() {
        let mood = Mood::new()
            .with_valence_base(0.3)
            .with_arousal_base(-0.2)
            .with_dominance_base(0.5);

        assert!((mood.valence_base() - 0.3).abs() < f32::EPSILON);
        assert!((mood.arousal_base() - (-0.2)).abs() < f32::EPSILON);
        assert!((mood.dominance_base() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn add_delta_modifies_effective_value() {
        let mut mood = Mood::new().with_valence_base(0.2);
        mood.add_valence_delta(0.3);

        assert!((mood.valence_effective() - 0.5).abs() < f32::EPSILON);
        assert!((mood.valence_delta() - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn reset_deltas_clears_all() {
        let mut mood = Mood::new();
        mood.add_valence_delta(0.5);
        mood.add_arousal_delta(0.3);
        mood.add_dominance_delta(-0.2);

        mood.reset_deltas();

        assert!(mood.valence_delta().abs() < f32::EPSILON);
        assert!(mood.arousal_delta().abs() < f32::EPSILON);
        assert!(mood.dominance_delta().abs() < f32::EPSILON);
    }

    #[test]
    fn effective_values_are_clamped() {
        let mut mood = Mood::new().with_valence_base(0.8);
        mood.add_valence_delta(0.5);

        // Effective should be clamped to 1.0
        assert!((mood.valence_effective() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn negative_values_are_valid() {
        let mut mood = Mood::new().with_valence_base(-0.3);
        mood.add_valence_delta(-0.2);

        assert!((mood.valence_effective() - (-0.5)).abs() < f32::EPSILON);
    }

    #[test]
    fn state_value_references_accessible() {
        let mut mood = Mood::new();

        // Can access immutable references
        let _ = mood.valence();
        let _ = mood.arousal();
        let _ = mood.dominance();

        // Can access mutable references
        mood.valence_mut().add_delta(0.1);
        assert!((mood.valence_delta() - 0.1).abs() < f32::EPSILON);
    }

    #[test]
    fn default_is_neutral() {
        let mood = Mood::default();
        assert!((mood.valence_effective() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn clone_and_equality() {
        let mood1 = Mood::new().with_valence_base(0.5);
        let mood2 = mood1.clone();
        assert_eq!(mood1, mood2);
    }

    #[test]
    fn debug_format() {
        let mood = Mood::new();
        let debug = format!("{:?}", mood);
        assert!(debug.contains("Mood"));
    }

    #[test]
    fn decay_affects_all_dimensions() {
        let mut mood = Mood::new();
        mood.add_valence_delta(1.0);
        mood.add_arousal_delta(1.0);
        mood.add_dominance_delta(1.0);

        mood.apply_decay(Duration::hours(24));

        // All deltas should be significantly reduced after 24 hours
        assert!(mood.valence_delta() < 0.2);
        assert!(mood.arousal_delta() < 0.2);
        assert!(mood.dominance_delta() < 0.3);
    }

    #[test]
    fn all_base_accessors() {
        let mood = Mood::new()
            .with_valence_base(0.3)
            .with_arousal_base(-0.2)
            .with_dominance_base(0.4);

        assert!((mood.valence_base() - 0.3).abs() < f32::EPSILON);
        assert!((mood.arousal_base() - (-0.2)).abs() < f32::EPSILON);
        assert!((mood.dominance_base() - 0.4).abs() < f32::EPSILON);
    }

    #[test]
    fn all_mutable_refs() {
        let mut mood = Mood::new();

        mood.arousal_mut().add_delta(0.3);
        mood.dominance_mut().add_delta(0.4);

        assert!((mood.arousal_delta() - 0.3).abs() < f32::EPSILON);
        assert!((mood.dominance_delta() - 0.4).abs() < f32::EPSILON);
    }

    #[test]
    fn from_personality_with_high_extraversion() {
        let hexaco = Hexaco::new().with_extraversion(0.8);
        let mood = Mood::from_personality(&hexaco);

        // High extraversion -> positive valence and dominance
        assert!(mood.valence_base() > 0.0);
        assert!(mood.dominance_base() > 0.0);
    }

    #[test]
    fn from_personality_with_high_neuroticism() {
        let hexaco = Hexaco::new().with_neuroticism(0.8);
        let mood = Mood::from_personality(&hexaco);

        // High neuroticism -> negative valence and higher arousal
        assert!(mood.valence_base() < 0.0);
        assert!(mood.arousal_base() > 0.0);
    }

    #[test]
    fn from_personality_balanced_produces_neutral() {
        // Balanced profile has all traits at 0.0 in the -1 to 1 range
        let hexaco = Hexaco::new();
        let mood = Mood::from_personality(&hexaco);

        // With all traits neutral, baseline affect should be near zero
        assert!(mood.valence_base().abs() < 0.01);
        assert!(mood.dominance_base().abs() < 0.01);
    }

    #[test]
    fn from_personality_creates_modest_baselines() {
        // Even extreme personality should produce modest baseline affect
        let extreme = Hexaco::new()
            .with_extraversion(1.0)
            .with_neuroticism(1.0)
            .with_openness(1.0);
        let mood = Mood::from_personality(&extreme);

        // Baselines should be attenuated, not extreme
        assert!(mood.valence_base().abs() < 0.5);
        assert!(mood.arousal_base().abs() < 0.5);
        assert!(mood.dominance_base().abs() < 0.5);
    }
}
