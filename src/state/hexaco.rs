//! HEXACO personality model implementation.
//!
//! The HEXACO model is a six-factor model of human personality:
//! - Honesty-Humility (H)
//! - Emotionality (E) - similar to Neuroticism in Big Five
//! - eXtraversion (X)
//! - Agreeableness (A)
//! - Conscientiousness (C)
//! - Openness to Experience (O)
//!
//! Each factor ranges from -1.0 to 1.0, where:
//! - Negative values indicate low trait presence
//! - Zero indicates average trait level
//! - Positive values indicate high trait presence

use crate::enums::PersonalityProfile;
use serde::{Deserialize, Serialize};

/// HEXACO personality factors.
///
/// All factors are stored as f32 values in the range -1.0 to 1.0.
/// These represent stable personality traits that rarely change
/// after early adulthood.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::state::Hexaco;
/// use behavioral_pathways::enums::PersonalityProfile;
///
/// // Create from personality profile preset
/// let hexaco = Hexaco::from_profile(PersonalityProfile::Leader);
/// assert!(hexaco.extraversion() > 0.5);
/// assert!(hexaco.neuroticism() < -0.3);
///
/// // Create with custom values
/// let custom = Hexaco::new()
///     .with_openness(0.7)
///     .with_conscientiousness(0.5);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hexaco {
    /// Openness to experience: curiosity, creativity, preference for novelty.
    /// Range: -1.0 (closed) to 1.0 (open)
    openness: f32,

    /// Conscientiousness: organization, dependability, self-discipline.
    /// Range: -1.0 (disorganized) to 1.0 (conscientious)
    conscientiousness: f32,

    /// Extraversion: sociability, assertiveness, positive emotionality.
    /// Range: -1.0 (introverted) to 1.0 (extraverted)
    extraversion: f32,

    /// Agreeableness: cooperation, trust, compliance.
    /// Range: -1.0 (antagonistic) to 1.0 (agreeable)
    agreeableness: f32,

    /// Neuroticism (Emotionality): emotional instability, anxiety, moodiness.
    /// Range: -1.0 (stable) to 1.0 (neurotic)
    neuroticism: f32,

    /// Honesty-Humility: sincerity, fairness, lack of greed.
    /// Range: -1.0 (manipulative) to 1.0 (honest/humble)
    honesty_humility: f32,
}

impl Hexaco {
    /// Creates a new Hexaco with all factors at the neutral point (0.0).
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::state::Hexaco;
    ///
    /// let hexaco = Hexaco::new();
    /// assert!((hexaco.openness() - 0.0).abs() < f32::EPSILON);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Hexaco {
            openness: 0.0,
            conscientiousness: 0.0,
            extraversion: 0.0,
            agreeableness: 0.0,
            neuroticism: 0.0,
            honesty_humility: 0.0,
        }
    }

    /// Creates a Hexaco from a PersonalityProfile preset.
    ///
    /// The profile's 0.0-1.0 values are converted to -1.0 to 1.0 range.
    ///
    /// # Arguments
    ///
    /// * `profile` - The personality profile preset to use
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::state::Hexaco;
    /// use behavioral_pathways::enums::PersonalityProfile;
    ///
    /// let hexaco = Hexaco::from_profile(PersonalityProfile::Anxious);
    /// assert!(hexaco.neuroticism() > 0.5);
    /// ```
    #[must_use]
    pub fn from_profile(profile: PersonalityProfile) -> Self {
        // Convert 0-1 range to -1 to 1 range: value * 2 - 1
        Hexaco {
            openness: profile.openness() * 2.0 - 1.0,
            conscientiousness: profile.conscientiousness() * 2.0 - 1.0,
            extraversion: profile.extraversion() * 2.0 - 1.0,
            agreeableness: profile.agreeableness() * 2.0 - 1.0,
            neuroticism: profile.neuroticism() * 2.0 - 1.0,
            honesty_humility: profile.honesty_humility() * 2.0 - 1.0,
        }
    }

    /// Creates a Hexaco with all factors set to the specified value.
    ///
    /// The value is clamped to the valid range -1.0 to 1.0.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set for all factors
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::state::Hexaco;
    ///
    /// let hexaco = Hexaco::uniform(0.5);
    /// assert!((hexaco.openness() - 0.5).abs() < f32::EPSILON);
    /// assert!((hexaco.extraversion() - 0.5).abs() < f32::EPSILON);
    /// ```
    #[must_use]
    pub fn uniform(value: f32) -> Self {
        let clamped = value.clamp(-1.0, 1.0);
        Hexaco {
            openness: clamped,
            conscientiousness: clamped,
            extraversion: clamped,
            agreeableness: clamped,
            neuroticism: clamped,
            honesty_humility: clamped,
        }
    }

    // Builder methods

    /// Sets the openness factor.
    #[must_use]
    pub fn with_openness(mut self, value: f32) -> Self {
        self.openness = value.clamp(-1.0, 1.0);
        self
    }

    /// Sets the conscientiousness factor.
    #[must_use]
    pub fn with_conscientiousness(mut self, value: f32) -> Self {
        self.conscientiousness = value.clamp(-1.0, 1.0);
        self
    }

    /// Sets the extraversion factor.
    #[must_use]
    pub fn with_extraversion(mut self, value: f32) -> Self {
        self.extraversion = value.clamp(-1.0, 1.0);
        self
    }

    /// Sets the agreeableness factor.
    #[must_use]
    pub fn with_agreeableness(mut self, value: f32) -> Self {
        self.agreeableness = value.clamp(-1.0, 1.0);
        self
    }

    /// Sets the neuroticism factor.
    #[must_use]
    pub fn with_neuroticism(mut self, value: f32) -> Self {
        self.neuroticism = value.clamp(-1.0, 1.0);
        self
    }

    /// Sets the honesty-humility factor.
    #[must_use]
    pub fn with_honesty_humility(mut self, value: f32) -> Self {
        self.honesty_humility = value.clamp(-1.0, 1.0);
        self
    }

    // Accessors

    /// Returns the openness factor.
    #[must_use]
    pub fn openness(&self) -> f32 {
        self.openness
    }

    /// Returns the conscientiousness factor.
    #[must_use]
    pub fn conscientiousness(&self) -> f32 {
        self.conscientiousness
    }

    /// Returns the extraversion factor.
    #[must_use]
    pub fn extraversion(&self) -> f32 {
        self.extraversion
    }

    /// Returns the agreeableness factor.
    #[must_use]
    pub fn agreeableness(&self) -> f32 {
        self.agreeableness
    }

    /// Returns the neuroticism factor.
    ///
    /// Note: In HEXACO, this is called "Emotionality". Use `emotionality()`
    /// for HEXACO-standard naming.
    #[must_use]
    pub fn neuroticism(&self) -> f32 {
        self.neuroticism
    }

    /// Returns the emotionality factor (HEXACO standard name for neuroticism).
    ///
    /// Emotionality measures emotional reactivity, sensitivity, and anxiety.
    /// Higher values indicate stronger emotional responses to events.
    #[must_use]
    pub fn emotionality(&self) -> f32 {
        self.neuroticism
    }

    /// Returns the honesty-humility factor.
    #[must_use]
    pub fn honesty_humility(&self) -> f32 {
        self.honesty_humility
    }

    // Mutators

    /// Sets the openness factor directly.
    pub fn set_openness(&mut self, value: f32) {
        self.openness = value.clamp(-1.0, 1.0);
    }

    /// Sets the conscientiousness factor directly.
    pub fn set_conscientiousness(&mut self, value: f32) {
        self.conscientiousness = value.clamp(-1.0, 1.0);
    }

    /// Sets the extraversion factor directly.
    pub fn set_extraversion(&mut self, value: f32) {
        self.extraversion = value.clamp(-1.0, 1.0);
    }

    /// Sets the agreeableness factor directly.
    pub fn set_agreeableness(&mut self, value: f32) {
        self.agreeableness = value.clamp(-1.0, 1.0);
    }

    /// Sets the neuroticism factor directly.
    pub fn set_neuroticism(&mut self, value: f32) {
        self.neuroticism = value.clamp(-1.0, 1.0);
    }

    /// Sets the honesty-humility factor directly.
    pub fn set_honesty_humility(&mut self, value: f32) {
        self.honesty_humility = value.clamp(-1.0, 1.0);
    }
}

impl Default for Hexaco {
    fn default() -> Self {
        Hexaco::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_neutral_factors() {
        let hexaco = Hexaco::new();
        assert!((hexaco.openness() - 0.0).abs() < f32::EPSILON);
        assert!((hexaco.conscientiousness() - 0.0).abs() < f32::EPSILON);
        assert!((hexaco.extraversion() - 0.0).abs() < f32::EPSILON);
        assert!((hexaco.agreeableness() - 0.0).abs() < f32::EPSILON);
        assert!((hexaco.neuroticism() - 0.0).abs() < f32::EPSILON);
        assert!((hexaco.honesty_humility() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn from_profile_balanced_is_neutral() {
        let hexaco = Hexaco::from_profile(PersonalityProfile::Balanced);
        // Balanced profile has 0.5 for all factors, which maps to 0.0 in -1 to 1 range
        assert!((hexaco.openness() - 0.0).abs() < f32::EPSILON);
        assert!((hexaco.conscientiousness() - 0.0).abs() < f32::EPSILON);
        assert!((hexaco.extraversion() - 0.0).abs() < f32::EPSILON);
        assert!((hexaco.agreeableness() - 0.0).abs() < f32::EPSILON);
        assert!((hexaco.neuroticism() - 0.0).abs() < f32::EPSILON);
        assert!((hexaco.honesty_humility() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn from_profile_anxious_has_high_neuroticism() {
        let hexaco = Hexaco::from_profile(PersonalityProfile::Anxious);
        // Anxious profile has 0.8 neuroticism, which maps to 0.6 in -1 to 1 range
        assert!(hexaco.neuroticism() > 0.5);
    }

    #[test]
    fn from_profile_leader_characteristics() {
        let hexaco = Hexaco::from_profile(PersonalityProfile::Leader);
        assert!(hexaco.extraversion() > 0.5);
        assert!(hexaco.conscientiousness() > 0.5);
        assert!(hexaco.neuroticism() < -0.3);
    }

    #[test]
    fn uniform_sets_all_factors() {
        let hexaco = Hexaco::uniform(0.5);
        assert!((hexaco.openness() - 0.5).abs() < f32::EPSILON);
        assert!((hexaco.conscientiousness() - 0.5).abs() < f32::EPSILON);
        assert!((hexaco.extraversion() - 0.5).abs() < f32::EPSILON);
        assert!((hexaco.agreeableness() - 0.5).abs() < f32::EPSILON);
        assert!((hexaco.neuroticism() - 0.5).abs() < f32::EPSILON);
        assert!((hexaco.honesty_humility() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn uniform_clamps_values() {
        let high = Hexaco::uniform(2.0);
        assert!((high.openness() - 1.0).abs() < f32::EPSILON);

        let low = Hexaco::uniform(-2.0);
        assert!((low.openness() - (-1.0)).abs() < f32::EPSILON);
    }

    #[test]
    fn builder_methods_work() {
        let hexaco = Hexaco::new()
            .with_openness(0.7)
            .with_conscientiousness(0.5)
            .with_extraversion(-0.3)
            .with_agreeableness(0.2)
            .with_neuroticism(-0.5)
            .with_honesty_humility(0.8);

        assert!((hexaco.openness() - 0.7).abs() < f32::EPSILON);
        assert!((hexaco.conscientiousness() - 0.5).abs() < f32::EPSILON);
        assert!((hexaco.extraversion() - (-0.3)).abs() < f32::EPSILON);
        assert!((hexaco.agreeableness() - 0.2).abs() < f32::EPSILON);
        assert!((hexaco.neuroticism() - (-0.5)).abs() < f32::EPSILON);
        assert!((hexaco.honesty_humility() - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn builder_methods_clamp_values() {
        let hexaco = Hexaco::new().with_openness(2.0).with_neuroticism(-2.0);

        assert!((hexaco.openness() - 1.0).abs() < f32::EPSILON);
        assert!((hexaco.neuroticism() - (-1.0)).abs() < f32::EPSILON);
    }

    #[test]
    fn setters_work() {
        let mut hexaco = Hexaco::new();

        hexaco.set_openness(0.7);
        assert!((hexaco.openness() - 0.7).abs() < f32::EPSILON);

        hexaco.set_conscientiousness(0.5);
        assert!((hexaco.conscientiousness() - 0.5).abs() < f32::EPSILON);

        hexaco.set_extraversion(-0.3);
        assert!((hexaco.extraversion() - (-0.3)).abs() < f32::EPSILON);

        hexaco.set_agreeableness(0.2);
        assert!((hexaco.agreeableness() - 0.2).abs() < f32::EPSILON);

        hexaco.set_neuroticism(-0.5);
        assert!((hexaco.neuroticism() - (-0.5)).abs() < f32::EPSILON);

        hexaco.set_honesty_humility(0.8);
        assert!((hexaco.honesty_humility() - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn setters_clamp_values() {
        let mut hexaco = Hexaco::new();

        hexaco.set_openness(5.0);
        assert!((hexaco.openness() - 1.0).abs() < f32::EPSILON);

        hexaco.set_neuroticism(-5.0);
        assert!((hexaco.neuroticism() - (-1.0)).abs() < f32::EPSILON);
    }

    #[test]
    fn default_is_neutral() {
        let hexaco = Hexaco::default();
        assert!((hexaco.openness() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn clone_and_equality() {
        let hexaco1 = Hexaco::new().with_openness(0.5);
        let hexaco2 = hexaco1.clone();
        assert_eq!(hexaco1, hexaco2);
    }

    #[test]
    fn debug_format() {
        let hexaco = Hexaco::new();
        let debug = format!("{:?}", hexaco);
        assert!(debug.contains("Hexaco"));
    }

    #[test]
    fn from_profile_rebel_characteristics() {
        let hexaco = Hexaco::from_profile(PersonalityProfile::Rebel);
        // Rebel has low agreeableness (0.2) and low honesty_humility (0.3)
        assert!(hexaco.agreeableness() < -0.5);
        assert!(hexaco.honesty_humility() < -0.3);
    }

    #[test]
    fn from_profile_introverted_characteristics() {
        let hexaco = Hexaco::from_profile(PersonalityProfile::Introverted);
        // Introverted has low extraversion (0.2)
        assert!(hexaco.extraversion() < -0.5);
    }

    #[test]
    fn emotionality_is_alias_for_neuroticism() {
        let hexaco = Hexaco::new().with_neuroticism(0.7);
        assert!((hexaco.emotionality() - 0.7).abs() < f32::EPSILON);
        assert!((hexaco.emotionality() - hexaco.neuroticism()).abs() < f32::EPSILON);
    }
}
