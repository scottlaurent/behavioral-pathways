//! Emotion category enum derived from PAD dimensions.
//!
//! This module defines the emotion categories associated with PAD
//! (Pleasure-Arousal-Dominance) octants using the Mehrabian-Russell model.

/// Emotion category derived from PAD octants.
///
/// Each emotion corresponds to a specific combination of positive/negative
/// values on the Valence (V), Arousal (A), and Dominance (D) dimensions.
///
/// These categories are used for graded membership rather than a single
/// discrete selection.
///
/// # PAD Octant Mapping
///
/// | V | A | D | Emotion |
/// |---|---|---|---------|
/// | + | + | + | Exuberant |
/// | + | + | - | Dependent |
/// | + | - | + | Relaxed |
/// | + | - | - | Docile |
/// | - | + | + | Hostile |
/// | - | + | - | Anxious |
/// | - | - | + | Bored |
/// | - | - | - | Depressed |
///
/// # Examples
///
/// ```
/// use behavioral_pathways::enums::Emotion;
///
/// let emotion = Emotion::Anxious;
/// assert_eq!(emotion.name(), "Anxious");
/// assert!(!emotion.is_positive());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Emotion {
    /// V+ A+ D+ - Joyful, energetic, in-control.
    Exuberant,

    /// V+ A+ D- - Happy and excited but feeling dependent on others.
    Dependent,

    /// V+ A- D+ - Content, calm, and in-control.
    Relaxed,

    /// V+ A- D- - Pleasant, calm, submissive.
    Docile,

    /// V- A+ D+ - Angry, energetic, in-control.
    Hostile,

    /// V- A+ D+ (moral violation gated) - Revulsed, energized, in-control.
    Disgust,

    /// V- A+ D- - Fearful, tense, feeling out of control.
    Anxious,

    /// V- A- D+ - Unpleasant, low energy, but still feeling in control.
    Bored,

    /// V- A- D- - Unpleasant, low energy, feeling helpless.
    Depressed,

    /// Neutral placeholder for situations with no dominant emotion.
    Neutral,
}

impl Emotion {
    /// Returns all emotion variants.
    #[must_use]
    pub const fn all() -> [Emotion; 10] {
        [
            Emotion::Exuberant,
            Emotion::Dependent,
            Emotion::Relaxed,
            Emotion::Docile,
            Emotion::Hostile,
            Emotion::Disgust,
            Emotion::Anxious,
            Emotion::Bored,
            Emotion::Depressed,
            Emotion::Neutral,
        ]
    }

    /// Returns a human-readable name for this emotion.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Emotion::Exuberant => "Exuberant",
            Emotion::Dependent => "Dependent",
            Emotion::Relaxed => "Relaxed",
            Emotion::Docile => "Docile",
            Emotion::Hostile => "Hostile",
            Emotion::Disgust => "Disgust",
            Emotion::Anxious => "Anxious",
            Emotion::Bored => "Bored",
            Emotion::Depressed => "Depressed",
            Emotion::Neutral => "Neutral",
        }
    }

    /// Returns true if this emotion has positive valence.
    ///
    /// Positive valence emotions: Exuberant, Dependent, Relaxed, Docile.
    #[must_use]
    pub const fn is_positive(&self) -> bool {
        matches!(
            self,
            Emotion::Exuberant | Emotion::Dependent | Emotion::Relaxed | Emotion::Docile
        )
    }

    /// Returns true if this emotion has negative valence.
    ///
    /// Negative valence emotions: Hostile, Anxious, Bored, Depressed.
    #[must_use]
    pub const fn is_negative(&self) -> bool {
        matches!(
            self,
            Emotion::Hostile
                | Emotion::Disgust
                | Emotion::Anxious
                | Emotion::Bored
                | Emotion::Depressed
        )
    }

    /// Returns true if this is the Neutral emotion.
    #[must_use]
    pub const fn is_neutral(&self) -> bool {
        matches!(self, Emotion::Neutral)
    }

    /// Returns true if this emotion has high arousal.
    ///
    /// High arousal emotions: Exuberant, Dependent, Hostile, Disgust, Anxious.
    #[must_use]
    pub const fn is_high_arousal(&self) -> bool {
        matches!(
            self,
            Emotion::Exuberant
                | Emotion::Dependent
                | Emotion::Hostile
                | Emotion::Disgust
                | Emotion::Anxious
        )
    }

    /// Returns true if this emotion has high dominance.
    ///
    /// High dominance emotions: Exuberant, Relaxed, Hostile, Disgust, Bored.
    #[must_use]
    pub const fn is_high_dominance(&self) -> bool {
        matches!(
            self,
            Emotion::Exuberant
                | Emotion::Relaxed
                | Emotion::Hostile
                | Emotion::Disgust
                | Emotion::Bored
        )
    }
}

impl std::fmt::Display for Emotion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emotion_all_variants() {
        let all = Emotion::all();
        assert_eq!(all.len(), 10);
    }

    #[test]
    fn emotion_names() {
        assert_eq!(Emotion::Exuberant.name(), "Exuberant");
        assert_eq!(Emotion::Dependent.name(), "Dependent");
        assert_eq!(Emotion::Relaxed.name(), "Relaxed");
        assert_eq!(Emotion::Docile.name(), "Docile");
        assert_eq!(Emotion::Hostile.name(), "Hostile");
        assert_eq!(Emotion::Disgust.name(), "Disgust");
        assert_eq!(Emotion::Anxious.name(), "Anxious");
        assert_eq!(Emotion::Bored.name(), "Bored");
        assert_eq!(Emotion::Depressed.name(), "Depressed");
        assert_eq!(Emotion::Neutral.name(), "Neutral");
    }

    #[test]
    fn positive_emotions() {
        assert!(Emotion::Exuberant.is_positive());
        assert!(Emotion::Dependent.is_positive());
        assert!(Emotion::Relaxed.is_positive());
        assert!(Emotion::Docile.is_positive());

        assert!(!Emotion::Hostile.is_positive());
        assert!(!Emotion::Disgust.is_positive());
        assert!(!Emotion::Anxious.is_positive());
        assert!(!Emotion::Bored.is_positive());
        assert!(!Emotion::Depressed.is_positive());
        assert!(!Emotion::Neutral.is_positive());
    }

    #[test]
    fn negative_emotions() {
        assert!(Emotion::Hostile.is_negative());
        assert!(Emotion::Disgust.is_negative());
        assert!(Emotion::Anxious.is_negative());
        assert!(Emotion::Bored.is_negative());
        assert!(Emotion::Depressed.is_negative());

        assert!(!Emotion::Exuberant.is_negative());
        assert!(!Emotion::Dependent.is_negative());
        assert!(!Emotion::Relaxed.is_negative());
        assert!(!Emotion::Docile.is_negative());
        assert!(!Emotion::Neutral.is_negative());
    }

    #[test]
    fn neutral_emotion() {
        assert!(Emotion::Neutral.is_neutral());
        for emotion in Emotion::all() {
            if emotion != Emotion::Neutral {
                assert!(!emotion.is_neutral());
            }
        }
    }

    #[test]
    fn high_arousal_emotions() {
        assert!(Emotion::Exuberant.is_high_arousal());
        assert!(Emotion::Dependent.is_high_arousal());
        assert!(Emotion::Hostile.is_high_arousal());
        assert!(Emotion::Disgust.is_high_arousal());
        assert!(Emotion::Anxious.is_high_arousal());

        assert!(!Emotion::Relaxed.is_high_arousal());
        assert!(!Emotion::Docile.is_high_arousal());
        assert!(!Emotion::Bored.is_high_arousal());
        assert!(!Emotion::Depressed.is_high_arousal());
        assert!(!Emotion::Neutral.is_high_arousal());
    }

    #[test]
    fn high_dominance_emotions() {
        assert!(Emotion::Exuberant.is_high_dominance());
        assert!(Emotion::Relaxed.is_high_dominance());
        assert!(Emotion::Hostile.is_high_dominance());
        assert!(Emotion::Disgust.is_high_dominance());
        assert!(Emotion::Bored.is_high_dominance());

        assert!(!Emotion::Dependent.is_high_dominance());
        assert!(!Emotion::Docile.is_high_dominance());
        assert!(!Emotion::Anxious.is_high_dominance());
        assert!(!Emotion::Depressed.is_high_dominance());
        assert!(!Emotion::Neutral.is_high_dominance());
    }

    #[test]
    fn display_format() {
        assert_eq!(format!("{}", Emotion::Anxious), "Anxious");
        assert_eq!(format!("{}", Emotion::Neutral), "Neutral");
    }

    #[test]
    fn debug_format() {
        let debug = format!("{:?}", Emotion::Hostile);
        assert!(debug.contains("Hostile"));
    }

    #[test]
    fn clone_and_copy() {
        let e1 = Emotion::Relaxed;
        let e2 = e1; // Copy
        let e3 = e1.clone();
        assert_eq!(e1, e2);
        assert_eq!(e1, e3);
    }

    #[test]
    fn hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(Emotion::Anxious);
        set.insert(Emotion::Anxious);
        assert_eq!(set.len(), 1);

        set.insert(Emotion::Relaxed);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn positive_and_negative_are_mutually_exclusive() {
        for emotion in Emotion::all() {
            let is_pos = emotion.is_positive();
            let is_neg = emotion.is_negative();
            let is_neutral = emotion.is_neutral();

            // At most one can be true
            let mut count = 0;
            for value in [is_pos, is_neg, is_neutral] {
                if value {
                    count += 1;
                }
            }
            assert!(count <= 1);

            // Non-neutral emotions should be either positive or negative
            if emotion != Emotion::Neutral {
                assert_ne!(is_pos, is_neg);
            }
        }
    }
}
