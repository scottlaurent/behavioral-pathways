//! Personality profile presets for quick entity creation.
//!
//! These profiles provide archetypical personality configurations
//! that can be used to quickly create entities with consistent
//! behavioral patterns.

use serde::{Deserialize, Serialize};

/// Preset personality archetypes for quick entity creation.
///
/// Each profile represents a common personality pattern with
/// predefined trait values. These are primarily used for humans
/// and can be adapted for high-social-complexity animals.
///
/// Profiles set initial base values for personality dimensions.
/// These can be further customized after creation.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::enums::PersonalityProfile;
///
/// let profile = PersonalityProfile::Anxious;
/// assert!(profile.neuroticism() > 0.5);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum PersonalityProfile {
    /// Average on all dimensions. A neutral baseline.
    #[default]
    Balanced,

    /// High neuroticism, high attachment anxiety.
    /// Prone to worry and relationship insecurity.
    Anxious,

    /// High avoidance, low extraversion.
    /// Prefers distance in relationships, values independence.
    Avoidant,

    /// High agreeableness, high empathy.
    /// Cooperative and caring toward others.
    Agreeable,

    /// High conscientiousness, high impulse control.
    /// Organized, disciplined, and goal-oriented.
    Conscientious,

    /// High neuroticism, low emotional stability.
    /// Prone to negative emotions and stress.
    Neurotic,

    /// High extraversion, high sociability.
    /// Outgoing, energetic, seeks social interaction.
    Extraverted,

    /// Low extraversion, high avoidance.
    /// Prefers solitude, introspective.
    Introverted,

    /// High extraversion, high conscientiousness, low neuroticism.
    /// Natural leader with confidence and organization.
    Leader,

    /// Low agreeableness, high reactance, low honesty-humility.
    /// Challenges authority, independent-minded.
    Rebel,
}

impl PersonalityProfile {
    /// Returns the extraversion value for this profile (0.0-1.0).
    ///
    /// Higher values indicate more outgoing, energetic personality.
    #[must_use]
    pub const fn extraversion(&self) -> f32 {
        match self {
            PersonalityProfile::Balanced => 0.5,
            PersonalityProfile::Anxious => 0.4,
            PersonalityProfile::Avoidant => 0.3,
            PersonalityProfile::Agreeable => 0.5,
            PersonalityProfile::Conscientious => 0.5,
            PersonalityProfile::Neurotic => 0.4,
            PersonalityProfile::Extraverted => 0.8,
            PersonalityProfile::Introverted => 0.2,
            PersonalityProfile::Leader => 0.8,
            PersonalityProfile::Rebel => 0.5,
        }
    }

    /// Returns the agreeableness value for this profile (0.0-1.0).
    ///
    /// Higher values indicate more cooperative, trusting personality.
    #[must_use]
    pub const fn agreeableness(&self) -> f32 {
        match self {
            PersonalityProfile::Balanced => 0.5,
            PersonalityProfile::Anxious => 0.6,
            PersonalityProfile::Avoidant => 0.4,
            PersonalityProfile::Agreeable => 0.8,
            PersonalityProfile::Conscientious => 0.5,
            PersonalityProfile::Neurotic => 0.4,
            PersonalityProfile::Extraverted => 0.6,
            PersonalityProfile::Introverted => 0.5,
            PersonalityProfile::Leader => 0.5,
            PersonalityProfile::Rebel => 0.2,
        }
    }

    /// Returns the conscientiousness value for this profile (0.0-1.0).
    ///
    /// Higher values indicate more organized, disciplined personality.
    #[must_use]
    pub const fn conscientiousness(&self) -> f32 {
        match self {
            PersonalityProfile::Balanced => 0.5,
            PersonalityProfile::Anxious => 0.5,
            PersonalityProfile::Avoidant => 0.5,
            PersonalityProfile::Agreeable => 0.5,
            PersonalityProfile::Conscientious => 0.8,
            PersonalityProfile::Neurotic => 0.4,
            PersonalityProfile::Extraverted => 0.5,
            PersonalityProfile::Introverted => 0.6,
            PersonalityProfile::Leader => 0.8,
            PersonalityProfile::Rebel => 0.3,
        }
    }

    /// Returns the neuroticism value for this profile (0.0-1.0).
    ///
    /// Higher values indicate more prone to negative emotions and stress.
    #[must_use]
    pub const fn neuroticism(&self) -> f32 {
        match self {
            PersonalityProfile::Balanced => 0.5,
            PersonalityProfile::Anxious => 0.8,
            PersonalityProfile::Avoidant => 0.4,
            PersonalityProfile::Agreeable => 0.4,
            PersonalityProfile::Conscientious => 0.4,
            PersonalityProfile::Neurotic => 0.8,
            PersonalityProfile::Extraverted => 0.3,
            PersonalityProfile::Introverted => 0.5,
            PersonalityProfile::Leader => 0.2,
            PersonalityProfile::Rebel => 0.5,
        }
    }

    /// Returns the openness value for this profile (0.0-1.0).
    ///
    /// Higher values indicate more open to new experiences and ideas.
    #[must_use]
    pub const fn openness(&self) -> f32 {
        match self {
            PersonalityProfile::Balanced => 0.5,
            PersonalityProfile::Anxious => 0.4,
            PersonalityProfile::Avoidant => 0.5,
            PersonalityProfile::Agreeable => 0.5,
            PersonalityProfile::Conscientious => 0.4,
            PersonalityProfile::Neurotic => 0.5,
            PersonalityProfile::Extraverted => 0.6,
            PersonalityProfile::Introverted => 0.6,
            PersonalityProfile::Leader => 0.6,
            PersonalityProfile::Rebel => 0.7,
        }
    }

    /// Returns the honesty-humility value for this profile (0.0-1.0).
    ///
    /// Higher values indicate more sincere, fair, modest personality.
    /// This is the sixth factor in the HEXACO model.
    #[must_use]
    pub const fn honesty_humility(&self) -> f32 {
        match self {
            PersonalityProfile::Balanced => 0.5,
            PersonalityProfile::Anxious => 0.5,
            PersonalityProfile::Avoidant => 0.5,
            PersonalityProfile::Agreeable => 0.6,
            PersonalityProfile::Conscientious => 0.6,
            PersonalityProfile::Neurotic => 0.5,
            PersonalityProfile::Extraverted => 0.5,
            PersonalityProfile::Introverted => 0.6,
            PersonalityProfile::Leader => 0.5,
            PersonalityProfile::Rebel => 0.3,
        }
    }

    /// Returns a human-readable name for this profile.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            PersonalityProfile::Balanced => "Balanced",
            PersonalityProfile::Anxious => "Anxious",
            PersonalityProfile::Avoidant => "Avoidant",
            PersonalityProfile::Agreeable => "Agreeable",
            PersonalityProfile::Conscientious => "Conscientious",
            PersonalityProfile::Neurotic => "Neurotic",
            PersonalityProfile::Extraverted => "Extraverted",
            PersonalityProfile::Introverted => "Introverted",
            PersonalityProfile::Leader => "Leader",
            PersonalityProfile::Rebel => "Rebel",
        }
    }

    /// Returns a brief description of this profile.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        match self {
            PersonalityProfile::Balanced => "Average on all dimensions",
            PersonalityProfile::Anxious => "High worry and relationship insecurity",
            PersonalityProfile::Avoidant => "Prefers distance in relationships",
            PersonalityProfile::Agreeable => "Cooperative and caring toward others",
            PersonalityProfile::Conscientious => "Organized, disciplined, goal-oriented",
            PersonalityProfile::Neurotic => "Prone to negative emotions and stress",
            PersonalityProfile::Extraverted => "Outgoing, energetic, social",
            PersonalityProfile::Introverted => "Prefers solitude, introspective",
            PersonalityProfile::Leader => "Confident, organized, low anxiety",
            PersonalityProfile::Rebel => "Challenges authority, independent-minded",
        }
    }

    /// Returns all personality profiles.
    #[must_use]
    pub const fn all() -> [PersonalityProfile; 10] {
        [
            PersonalityProfile::Balanced,
            PersonalityProfile::Anxious,
            PersonalityProfile::Avoidant,
            PersonalityProfile::Agreeable,
            PersonalityProfile::Conscientious,
            PersonalityProfile::Neurotic,
            PersonalityProfile::Extraverted,
            PersonalityProfile::Introverted,
            PersonalityProfile::Leader,
            PersonalityProfile::Rebel,
        ]
    }
}

impl std::fmt::Display for PersonalityProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balanced_is_neutral() {
        let profile = PersonalityProfile::Balanced;
        assert!((profile.extraversion() - 0.5).abs() < f32::EPSILON);
        assert!((profile.agreeableness() - 0.5).abs() < f32::EPSILON);
        assert!((profile.conscientiousness() - 0.5).abs() < f32::EPSILON);
        assert!((profile.neuroticism() - 0.5).abs() < f32::EPSILON);
        assert!((profile.openness() - 0.5).abs() < f32::EPSILON);
        assert!((profile.honesty_humility() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn anxious_has_high_neuroticism() {
        let profile = PersonalityProfile::Anxious;
        assert!(profile.neuroticism() > 0.7);
    }

    #[test]
    fn leader_has_low_neuroticism_high_extraversion() {
        let profile = PersonalityProfile::Leader;
        assert!(profile.neuroticism() < 0.3);
        assert!(profile.extraversion() > 0.7);
        assert!(profile.conscientiousness() > 0.7);
    }

    #[test]
    fn rebel_has_low_agreeableness_and_honesty() {
        let profile = PersonalityProfile::Rebel;
        assert!(profile.agreeableness() < 0.3);
        assert!(profile.honesty_humility() < 0.4);
    }

    #[test]
    fn introverted_has_low_extraversion() {
        let profile = PersonalityProfile::Introverted;
        assert!(profile.extraversion() < 0.3);
    }

    #[test]
    fn all_values_in_valid_range() {
        for profile in PersonalityProfile::all() {
            assert!(profile.extraversion() >= 0.0 && profile.extraversion() <= 1.0);
            assert!(profile.agreeableness() >= 0.0 && profile.agreeableness() <= 1.0);
            assert!(profile.conscientiousness() >= 0.0 && profile.conscientiousness() <= 1.0);
            assert!(profile.neuroticism() >= 0.0 && profile.neuroticism() <= 1.0);
            assert!(profile.openness() >= 0.0 && profile.openness() <= 1.0);
            assert!(profile.honesty_humility() >= 0.0 && profile.honesty_humility() <= 1.0);
        }
    }

    #[test]
    fn all_profiles() {
        let profiles = PersonalityProfile::all();
        assert_eq!(profiles.len(), 10);
    }

    #[test]
    fn display_format() {
        assert_eq!(format!("{}", PersonalityProfile::Balanced), "Balanced");
        assert_eq!(format!("{}", PersonalityProfile::Leader), "Leader");
    }

    #[test]
    fn default_is_balanced() {
        assert_eq!(PersonalityProfile::default(), PersonalityProfile::Balanced);
    }

    #[test]
    fn name_and_description() {
        let profile = PersonalityProfile::Anxious;
        assert_eq!(profile.name(), "Anxious");
        assert!(!profile.description().is_empty());
    }

    #[test]
    fn equality_and_hash() {
        use std::collections::HashSet;

        assert_eq!(PersonalityProfile::Leader, PersonalityProfile::Leader);
        assert_ne!(PersonalityProfile::Leader, PersonalityProfile::Rebel);

        let mut set = HashSet::new();
        set.insert(PersonalityProfile::Leader);
        set.insert(PersonalityProfile::Leader);
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn copy_and_clone() {
        let original = PersonalityProfile::Anxious;
        let copied = original;
        let cloned = original.clone();

        assert_eq!(original, copied);
        assert_eq!(original, cloned);
    }

    #[test]
    fn all_profiles_have_names() {
        for profile in PersonalityProfile::all() {
            assert!(!profile.name().is_empty());
        }
    }

    #[test]
    fn all_profiles_have_descriptions() {
        for profile in PersonalityProfile::all() {
            assert!(!profile.description().is_empty());
        }
    }

    #[test]
    fn avoidant_profile() {
        let profile = PersonalityProfile::Avoidant;
        assert!(profile.extraversion() < 0.4);
        assert_eq!(profile.name(), "Avoidant");
    }

    #[test]
    fn agreeable_profile() {
        let profile = PersonalityProfile::Agreeable;
        assert!(profile.agreeableness() > 0.7);
        assert_eq!(profile.name(), "Agreeable");
    }

    #[test]
    fn conscientious_profile() {
        let profile = PersonalityProfile::Conscientious;
        assert!(profile.conscientiousness() > 0.7);
        assert_eq!(profile.name(), "Conscientious");
    }

    #[test]
    fn neurotic_profile() {
        let profile = PersonalityProfile::Neurotic;
        assert!(profile.neuroticism() > 0.7);
        assert_eq!(profile.name(), "Neurotic");
    }

    #[test]
    fn extraverted_profile() {
        let profile = PersonalityProfile::Extraverted;
        assert!(profile.extraversion() > 0.7);
        assert_eq!(profile.name(), "Extraverted");
    }

    #[test]
    fn all_profiles_trait_values() {
        // Exercise all trait methods for all profiles
        for profile in PersonalityProfile::all() {
            let _ = profile.extraversion();
            let _ = profile.agreeableness();
            let _ = profile.conscientiousness();
            let _ = profile.neuroticism();
            let _ = profile.openness();
            let _ = profile.honesty_humility();
        }
    }

    #[test]
    fn debug_format() {
        let profile = PersonalityProfile::Leader;
        let debug = format!("{:?}", profile);
        assert!(debug.contains("Leader"));
    }
}
