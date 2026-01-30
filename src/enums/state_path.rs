//! Typed state access paths for compile-time validated state queries.
//!
//! These enums provide type-safe paths for accessing state dimensions
//! without using magic strings. They enable compile-time validation
//! and IDE autocomplete for state access.
//!
//! # Examples
//!
//! ```
//! use behavioral_pathways::enums::{StatePath, MoodPath, NeedsPath};
//!
//! // Type-safe state path construction
//! let stress_path = StatePath::Needs(NeedsPath::Stress);
//! let valence_path = StatePath::Mood(MoodPath::Valence);
//!
//! // These would be caught at compile time if misspelled
//! ```

/// Top-level state access path.
///
/// This is the root enum for accessing any state dimension. Use this
/// when you need to specify a path to any part of an entity's state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatePath {
    /// Path to a HEXACO personality dimension.
    Hexaco(HexacoPath),

    /// Path to a PAD mood dimension.
    Mood(MoodPath),

    /// Path to a needs dimension.
    Needs(NeedsPath),

    /// Path to a social cognition dimension.
    SocialCognition(SocialCognitionPath),

    /// Path to a mental health dimension.
    MentalHealth(MentalHealthPath),

    /// Path to a disposition dimension.
    Disposition(DispositionPath),

    /// Path to a person characteristics dimension.
    PersonCharacteristics(PersonCharacteristicsPath),
}

/// Path to HEXACO personality dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum HexacoPath {
    /// Openness to experience.
    Openness,

    /// Conscientiousness.
    Conscientiousness,

    /// Extraversion.
    Extraversion,

    /// Agreeableness.
    Agreeableness,

    /// Neuroticism (Emotionality).
    Neuroticism,

    /// Honesty-Humility.
    HonestyHumility,
}

/// Path to PAD mood dimensions.
///
/// Note: Mood contains ONLY PAD dimensions (valence, arousal, dominance).
/// Fatigue and stress are physiological states in [`NeedsPath`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoodPath {
    /// Valence: pleasantness (-1 to +1).
    Valence,

    /// Arousal: activation level (-1 to +1).
    Arousal,

    /// Dominance: sense of control (-1 to +1).
    Dominance,
}

/// Path to needs dimensions.
///
/// These include physiological states (fatigue, stress) and purpose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NeedsPath {
    /// Physical and mental tiredness.
    Fatigue,

    /// Pressure and tension.
    Stress,

    /// Sense of meaning and direction.
    Purpose,
}

/// Path to social cognition dimensions.
///
/// These include beliefs that feed into ITS computations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SocialCognitionPath {
    /// Social isolation - feeling disconnected.
    Loneliness,

    /// Belief that others genuinely care.
    PerceivedReciprocalCaring,

    /// Belief of being a burden to others.
    PerceivedLiability,

    /// Active self-loathing.
    SelfHate,

    /// Sense of competence and efficacy.
    PerceivedCompetence,
}

/// Path to mental health dimensions.
///
/// These include ITS (Interpersonal Theory of Suicide) factors
/// and other mental health indicators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MentalHealthPath {
    /// Depression severity.
    Depression,

    /// Sense of personal value.
    SelfWorth,

    /// General cognitive hopelessness.
    Hopelessness,

    /// Perceived permanence of TB/PB states.
    /// Distinct from general hopelessness.
    InterpersonalHopelessness,

    /// Habituation to pain/fear of death.
    /// Note: This dimension NEVER decays.
    AcquiredCapability,

    /// Computed: Thwarted Belongingness.
    /// TB = (loneliness + (1 - perceived_reciprocal_caring)) / 2
    ThwartedBelongingness,

    /// Computed: Perceived Burdensomeness.
    /// PB = perceived_liability * self_hate
    PerceivedBurdensomeness,

    /// Computed: Suicidal desire.
    /// Requires TB AND PB AND interpersonal_hopelessness above thresholds.
    SuicidalDesire,

    /// Computed: Attempt risk.
    /// Risk = Desire * Acquired Capability
    AttemptRisk,
}

/// Path to disposition dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DispositionPath {
    /// Self-regulation capacity.
    ImpulseControl,

    /// Concern for others' wellbeing.
    Empathy,

    /// Baseline hostility level.
    Aggression,

    /// Accumulated sense of injustice.
    Grievance,

    /// Resistance to restrictions.
    Reactance,

    /// General willingness to trust others.
    TrustPropensity,
}

/// Path to person characteristics (PPCT model).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PersonCharacteristicsPath {
    // Resource characteristics
    /// Reasoning and problem-solving ability.
    CognitiveAbility,

    /// Learned coping and regulation skills.
    EmotionalRegulationAssets,

    /// Count and quality of supportive relationships.
    SocialCapital,

    /// Access to material resources.
    MaterialSecurity,

    /// Variety of life domains encountered.
    ExperienceDiversity,

    // Force characteristics
    /// Drive to initiate action.
    BaselineMotivation,

    /// Tendency to persist despite difficulty.
    PersistenceTendency,

    /// Tendency to seek information and novelty.
    CuriosityTendency,

    // Composite accessors
    /// Overall resource characteristic level.
    Resource,

    /// Overall force characteristic level.
    Force,
}

// Implement name methods for each path enum

impl HexacoPath {
    /// Returns all HEXACO path variants.
    #[must_use]
    pub const fn all() -> [HexacoPath; 6] {
        [
            HexacoPath::Openness,
            HexacoPath::Conscientiousness,
            HexacoPath::Extraversion,
            HexacoPath::Agreeableness,
            HexacoPath::Neuroticism,
            HexacoPath::HonestyHumility,
        ]
    }

    /// Returns a human-readable name for this path.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            HexacoPath::Openness => "Openness",
            HexacoPath::Conscientiousness => "Conscientiousness",
            HexacoPath::Extraversion => "Extraversion",
            HexacoPath::Agreeableness => "Agreeableness",
            HexacoPath::Neuroticism => "Neuroticism",
            HexacoPath::HonestyHumility => "Honesty-Humility",
        }
    }
}

impl MoodPath {
    /// Returns all Mood path variants.
    #[must_use]
    pub const fn all() -> [MoodPath; 3] {
        [MoodPath::Valence, MoodPath::Arousal, MoodPath::Dominance]
    }

    /// Returns a human-readable name for this path.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            MoodPath::Valence => "Valence",
            MoodPath::Arousal => "Arousal",
            MoodPath::Dominance => "Dominance",
        }
    }
}

impl NeedsPath {
    /// Returns all Needs path variants.
    #[must_use]
    pub const fn all() -> [NeedsPath; 3] {
        [NeedsPath::Fatigue, NeedsPath::Stress, NeedsPath::Purpose]
    }

    /// Returns a human-readable name for this path.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            NeedsPath::Fatigue => "Fatigue",
            NeedsPath::Stress => "Stress",
            NeedsPath::Purpose => "Purpose",
        }
    }
}

impl SocialCognitionPath {
    /// Returns all SocialCognition path variants.
    #[must_use]
    pub const fn all() -> [SocialCognitionPath; 5] {
        [
            SocialCognitionPath::Loneliness,
            SocialCognitionPath::PerceivedReciprocalCaring,
            SocialCognitionPath::PerceivedLiability,
            SocialCognitionPath::SelfHate,
            SocialCognitionPath::PerceivedCompetence,
        ]
    }

    /// Returns a human-readable name for this path.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            SocialCognitionPath::Loneliness => "Loneliness",
            SocialCognitionPath::PerceivedReciprocalCaring => "Perceived Reciprocal Caring",
            SocialCognitionPath::PerceivedLiability => "Perceived Liability",
            SocialCognitionPath::SelfHate => "Self Hate",
            SocialCognitionPath::PerceivedCompetence => "Perceived Competence",
        }
    }
}

impl MentalHealthPath {
    /// Returns all MentalHealth path variants.
    #[must_use]
    pub const fn all() -> [MentalHealthPath; 9] {
        [
            MentalHealthPath::Depression,
            MentalHealthPath::SelfWorth,
            MentalHealthPath::Hopelessness,
            MentalHealthPath::InterpersonalHopelessness,
            MentalHealthPath::AcquiredCapability,
            MentalHealthPath::ThwartedBelongingness,
            MentalHealthPath::PerceivedBurdensomeness,
            MentalHealthPath::SuicidalDesire,
            MentalHealthPath::AttemptRisk,
        ]
    }

    /// Returns a human-readable name for this path.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            MentalHealthPath::Depression => "Depression",
            MentalHealthPath::SelfWorth => "Self Worth",
            MentalHealthPath::Hopelessness => "Hopelessness",
            MentalHealthPath::InterpersonalHopelessness => "Interpersonal Hopelessness",
            MentalHealthPath::AcquiredCapability => "Acquired Capability",
            MentalHealthPath::ThwartedBelongingness => "Thwarted Belongingness",
            MentalHealthPath::PerceivedBurdensomeness => "Perceived Burdensomeness",
            MentalHealthPath::SuicidalDesire => "Suicidal Desire",
            MentalHealthPath::AttemptRisk => "Attempt Risk",
        }
    }

    /// Returns true if this path represents a computed (not stored) value.
    #[must_use]
    pub const fn is_computed(&self) -> bool {
        matches!(
            self,
            MentalHealthPath::ThwartedBelongingness
                | MentalHealthPath::PerceivedBurdensomeness
                | MentalHealthPath::SuicidalDesire
                | MentalHealthPath::AttemptRisk
        )
    }
}

impl DispositionPath {
    /// Returns all Disposition path variants.
    #[must_use]
    pub const fn all() -> [DispositionPath; 6] {
        [
            DispositionPath::ImpulseControl,
            DispositionPath::Empathy,
            DispositionPath::Aggression,
            DispositionPath::Grievance,
            DispositionPath::Reactance,
            DispositionPath::TrustPropensity,
        ]
    }

    /// Returns a human-readable name for this path.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            DispositionPath::ImpulseControl => "Impulse Control",
            DispositionPath::Empathy => "Empathy",
            DispositionPath::Aggression => "Aggression",
            DispositionPath::Grievance => "Grievance",
            DispositionPath::Reactance => "Reactance",
            DispositionPath::TrustPropensity => "Trust Propensity",
        }
    }
}

impl PersonCharacteristicsPath {
    /// Returns all PersonCharacteristics path variants.
    #[must_use]
    pub const fn all() -> [PersonCharacteristicsPath; 10] {
        [
            PersonCharacteristicsPath::CognitiveAbility,
            PersonCharacteristicsPath::EmotionalRegulationAssets,
            PersonCharacteristicsPath::SocialCapital,
            PersonCharacteristicsPath::MaterialSecurity,
            PersonCharacteristicsPath::ExperienceDiversity,
            PersonCharacteristicsPath::BaselineMotivation,
            PersonCharacteristicsPath::PersistenceTendency,
            PersonCharacteristicsPath::CuriosityTendency,
            PersonCharacteristicsPath::Resource,
            PersonCharacteristicsPath::Force,
        ]
    }

    /// Returns a human-readable name for this path.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            PersonCharacteristicsPath::CognitiveAbility => "Cognitive Ability",
            PersonCharacteristicsPath::EmotionalRegulationAssets => "Emotional Regulation Assets",
            PersonCharacteristicsPath::SocialCapital => "Social Capital",
            PersonCharacteristicsPath::MaterialSecurity => "Material Security",
            PersonCharacteristicsPath::ExperienceDiversity => "Experience Diversity",
            PersonCharacteristicsPath::BaselineMotivation => "Baseline Motivation",
            PersonCharacteristicsPath::PersistenceTendency => "Persistence Tendency",
            PersonCharacteristicsPath::CuriosityTendency => "Curiosity Tendency",
            PersonCharacteristicsPath::Resource => "Resource",
            PersonCharacteristicsPath::Force => "Force",
        }
    }

    /// Returns true if this path represents a composite (computed) value.
    #[must_use]
    pub const fn is_composite(&self) -> bool {
        matches!(
            self,
            PersonCharacteristicsPath::Resource | PersonCharacteristicsPath::Force
        )
    }
}

impl std::fmt::Display for StatePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatePath::Hexaco(p) => write!(f, "Hexaco::{}", p.name()),
            StatePath::Mood(p) => write!(f, "Mood::{}", p.name()),
            StatePath::Needs(p) => write!(f, "Needs::{}", p.name()),
            StatePath::SocialCognition(p) => write!(f, "SocialCognition::{}", p.name()),
            StatePath::MentalHealth(p) => write!(f, "MentalHealth::{}", p.name()),
            StatePath::Disposition(p) => write!(f, "Disposition::{}", p.name()),
            StatePath::PersonCharacteristics(p) => write!(f, "PersonCharacteristics::{}", p.name()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_path_mood_variants_exist() {
        // Verify MoodPath variants compile
        let _ = MoodPath::Valence;
        let _ = MoodPath::Arousal;
        let _ = MoodPath::Dominance;

        // Verify they can be wrapped in StatePath
        let _ = StatePath::Mood(MoodPath::Valence);
    }

    #[test]
    fn state_path_needs_variants_exist() {
        // Verify NeedsPath variants compile
        let _ = NeedsPath::Stress;
        let _ = NeedsPath::Fatigue;
        let _ = NeedsPath::Purpose;

        // Verify they can be wrapped in StatePath
        let _ = StatePath::Needs(NeedsPath::Stress);
    }

    #[test]
    fn state_path_social_cognition_variants_exist() {
        let _ = SocialCognitionPath::Loneliness;
        let _ = SocialCognitionPath::PerceivedReciprocalCaring;
        let _ = SocialCognitionPath::PerceivedLiability;
        let _ = SocialCognitionPath::SelfHate;
        let _ = SocialCognitionPath::PerceivedCompetence;

        let _ = StatePath::SocialCognition(SocialCognitionPath::Loneliness);
    }

    #[test]
    fn state_path_mental_health_variants_exist() {
        // Verify MentalHealthPath variants compile
        let _ = MentalHealthPath::Depression;
        let _ = MentalHealthPath::SelfWorth;
        let _ = MentalHealthPath::Hopelessness;
        let _ = MentalHealthPath::InterpersonalHopelessness;
        let _ = MentalHealthPath::AcquiredCapability;
        let _ = MentalHealthPath::ThwartedBelongingness;
        let _ = MentalHealthPath::PerceivedBurdensomeness;
        let _ = MentalHealthPath::SuicidalDesire;
        let _ = MentalHealthPath::AttemptRisk;

        // Verify they can be wrapped in StatePath
        let _ = StatePath::MentalHealth(MentalHealthPath::Depression);
    }

    #[test]
    fn hexaco_path_all() {
        let all = HexacoPath::all();
        assert_eq!(all.len(), 6);
    }

    #[test]
    fn mood_path_all() {
        let all = MoodPath::all();
        assert_eq!(all.len(), 3);
    }

    #[test]
    fn needs_path_all() {
        let all = NeedsPath::all();
        assert_eq!(all.len(), 3);
    }

    #[test]
    fn social_cognition_path_all() {
        let all = SocialCognitionPath::all();
        assert_eq!(all.len(), 5);
    }

    #[test]
    fn mental_health_path_all() {
        let all = MentalHealthPath::all();
        assert_eq!(all.len(), 9);
    }

    #[test]
    fn disposition_path_all() {
        let all = DispositionPath::all();
        assert_eq!(all.len(), 6);
    }

    #[test]
    fn person_characteristics_path_all() {
        let all = PersonCharacteristicsPath::all();
        assert_eq!(all.len(), 10);
    }

    #[test]
    fn path_names_not_empty() {
        for p in HexacoPath::all() {
            assert!(!p.name().is_empty());
        }
        for p in MoodPath::all() {
            assert!(!p.name().is_empty());
        }
        for p in NeedsPath::all() {
            assert!(!p.name().is_empty());
        }
        for p in SocialCognitionPath::all() {
            assert!(!p.name().is_empty());
        }
        for p in MentalHealthPath::all() {
            assert!(!p.name().is_empty());
        }
        for p in DispositionPath::all() {
            assert!(!p.name().is_empty());
        }
        for p in PersonCharacteristicsPath::all() {
            assert!(!p.name().is_empty());
        }
    }

    #[test]
    fn mental_health_computed_paths() {
        assert!(!MentalHealthPath::Depression.is_computed());
        assert!(MentalHealthPath::ThwartedBelongingness.is_computed());
        assert!(MentalHealthPath::PerceivedBurdensomeness.is_computed());
        assert!(MentalHealthPath::SuicidalDesire.is_computed());
        assert!(MentalHealthPath::AttemptRisk.is_computed());
    }

    #[test]
    fn person_characteristics_composite_paths() {
        assert!(!PersonCharacteristicsPath::CognitiveAbility.is_composite());
        assert!(PersonCharacteristicsPath::Resource.is_composite());
        assert!(PersonCharacteristicsPath::Force.is_composite());
    }

    #[test]
    fn state_path_display() {
        let path = StatePath::Mood(MoodPath::Valence);
        let display = format!("{}", path);
        assert!(display.contains("Mood"));
        assert!(display.contains("Valence"));
    }

    #[test]
    fn paths_are_copy() {
        let p1 = MoodPath::Valence;
        let p2 = p1; // Copy
        assert_eq!(p1, p2);
    }

    #[test]
    fn paths_are_hashable() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(StatePath::Mood(MoodPath::Valence));
        set.insert(StatePath::Mood(MoodPath::Valence));
        assert_eq!(set.len(), 1);

        set.insert(StatePath::Mood(MoodPath::Arousal));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn debug_format() {
        let path = StatePath::Needs(NeedsPath::Stress);
        let debug = format!("{:?}", path);
        assert!(debug.contains("Needs"));
        assert!(debug.contains("Stress"));
    }

    #[test]
    fn equality() {
        assert_eq!(MoodPath::Valence, MoodPath::Valence);
        assert_ne!(MoodPath::Valence, MoodPath::Arousal);

        assert_eq!(
            StatePath::Mood(MoodPath::Valence),
            StatePath::Mood(MoodPath::Valence)
        );
        assert_ne!(
            StatePath::Mood(MoodPath::Valence),
            StatePath::Mood(MoodPath::Arousal)
        );
    }

    #[test]
    fn all_state_path_display_variants() {
        // Test Display for all StatePath variants
        let paths = [
            StatePath::Hexaco(HexacoPath::Openness),
            StatePath::Mood(MoodPath::Arousal),
            StatePath::Needs(NeedsPath::Fatigue),
            StatePath::SocialCognition(SocialCognitionPath::Loneliness),
            StatePath::MentalHealth(MentalHealthPath::Depression),
            StatePath::Disposition(DispositionPath::Empathy),
            StatePath::PersonCharacteristics(PersonCharacteristicsPath::CognitiveAbility),
        ];

        for path in paths {
            let display = format!("{}", path);
            assert!(!display.is_empty());
        }
    }

    #[test]
    fn all_hexaco_names() {
        for p in HexacoPath::all() {
            match p {
                HexacoPath::Openness => assert_eq!(p.name(), "Openness"),
                HexacoPath::Conscientiousness => assert_eq!(p.name(), "Conscientiousness"),
                HexacoPath::Extraversion => assert_eq!(p.name(), "Extraversion"),
                HexacoPath::Agreeableness => assert_eq!(p.name(), "Agreeableness"),
                HexacoPath::Neuroticism => assert_eq!(p.name(), "Neuroticism"),
                HexacoPath::HonestyHumility => assert_eq!(p.name(), "Honesty-Humility"),
            }
        }
    }

    #[test]
    fn all_mood_names() {
        for p in MoodPath::all() {
            match p {
                MoodPath::Valence => assert_eq!(p.name(), "Valence"),
                MoodPath::Arousal => assert_eq!(p.name(), "Arousal"),
                MoodPath::Dominance => assert_eq!(p.name(), "Dominance"),
            }
        }
    }

    #[test]
    fn all_needs_names() {
        for p in NeedsPath::all() {
            match p {
                NeedsPath::Fatigue => assert_eq!(p.name(), "Fatigue"),
                NeedsPath::Stress => assert_eq!(p.name(), "Stress"),
                NeedsPath::Purpose => assert_eq!(p.name(), "Purpose"),
            }
        }
    }

    #[test]
    fn all_social_cognition_names() {
        for p in SocialCognitionPath::all() {
            match p {
                SocialCognitionPath::Loneliness => assert_eq!(p.name(), "Loneliness"),
                SocialCognitionPath::PerceivedReciprocalCaring => {
                    assert_eq!(p.name(), "Perceived Reciprocal Caring")
                }
                SocialCognitionPath::PerceivedLiability => assert_eq!(p.name(), "Perceived Liability"),
                SocialCognitionPath::SelfHate => assert_eq!(p.name(), "Self Hate"),
                SocialCognitionPath::PerceivedCompetence => assert_eq!(p.name(), "Perceived Competence"),
            }
        }
    }

    #[test]
    fn all_mental_health_names() {
        for p in MentalHealthPath::all() {
            match p {
                MentalHealthPath::Depression => assert_eq!(p.name(), "Depression"),
                MentalHealthPath::SelfWorth => assert_eq!(p.name(), "Self Worth"),
                MentalHealthPath::Hopelessness => assert_eq!(p.name(), "Hopelessness"),
                MentalHealthPath::InterpersonalHopelessness => {
                    assert_eq!(p.name(), "Interpersonal Hopelessness")
                }
                MentalHealthPath::AcquiredCapability => assert_eq!(p.name(), "Acquired Capability"),
                MentalHealthPath::ThwartedBelongingness => {
                    assert_eq!(p.name(), "Thwarted Belongingness")
                }
                MentalHealthPath::PerceivedBurdensomeness => {
                    assert_eq!(p.name(), "Perceived Burdensomeness")
                }
                MentalHealthPath::SuicidalDesire => assert_eq!(p.name(), "Suicidal Desire"),
                MentalHealthPath::AttemptRisk => assert_eq!(p.name(), "Attempt Risk"),
            }
        }
    }

    #[test]
    fn all_disposition_names() {
        for p in DispositionPath::all() {
            match p {
                DispositionPath::ImpulseControl => assert_eq!(p.name(), "Impulse Control"),
                DispositionPath::Empathy => assert_eq!(p.name(), "Empathy"),
                DispositionPath::Aggression => assert_eq!(p.name(), "Aggression"),
                DispositionPath::Grievance => assert_eq!(p.name(), "Grievance"),
                DispositionPath::Reactance => assert_eq!(p.name(), "Reactance"),
                DispositionPath::TrustPropensity => assert_eq!(p.name(), "Trust Propensity"),
            }
        }
    }

    #[test]
    fn all_person_characteristics_names() {
        for p in PersonCharacteristicsPath::all() {
            match p {
                PersonCharacteristicsPath::CognitiveAbility => {
                    assert_eq!(p.name(), "Cognitive Ability")
                }
                PersonCharacteristicsPath::EmotionalRegulationAssets => {
                    assert_eq!(p.name(), "Emotional Regulation Assets")
                }
                PersonCharacteristicsPath::SocialCapital => assert_eq!(p.name(), "Social Capital"),
                PersonCharacteristicsPath::MaterialSecurity => {
                    assert_eq!(p.name(), "Material Security")
                }
                PersonCharacteristicsPath::ExperienceDiversity => {
                    assert_eq!(p.name(), "Experience Diversity")
                }
                PersonCharacteristicsPath::BaselineMotivation => {
                    assert_eq!(p.name(), "Baseline Motivation")
                }
                PersonCharacteristicsPath::PersistenceTendency => {
                    assert_eq!(p.name(), "Persistence Tendency")
                }
                PersonCharacteristicsPath::CuriosityTendency => {
                    assert_eq!(p.name(), "Curiosity Tendency")
                }
                PersonCharacteristicsPath::Resource => assert_eq!(p.name(), "Resource"),
                PersonCharacteristicsPath::Force => assert_eq!(p.name(), "Force"),
            }
        }
    }
}
