//! State interpretation - converts psychological dimensions to human-readable text.

use crate::state::IndividualState;
use std::collections::HashMap;

/// Interprets psychological state as human-readable text.
pub struct StateInterpreter {
    interpretations: HashMap<String, String>,
    summary: String,
    delta_summary: Option<String>,
}

impl StateInterpreter {
    /// Creates a new interpreter from an individual state.
    pub fn from_state(state: &IndividualState) -> Self {
        let mut interpretations = HashMap::new();

        // Mood (PAD)
        let valence = state.mood().valence_effective();
        interpretations.insert(
            "valence".to_string(),
            Self::interpret_valence(valence),
        );

        let arousal = state.mood().arousal_effective();
        interpretations.insert(
            "arousal".to_string(),
            Self::interpret_arousal(arousal),
        );

        let dominance = state.mood().dominance_effective();
        interpretations.insert(
            "dominance".to_string(),
            Self::interpret_dominance(dominance),
        );

        // Needs
        let stress = state.needs().stress_effective();
        interpretations.insert(
            "stress".to_string(),
            Self::interpret_stress(stress),
        );

        let fatigue = state.needs().fatigue_effective();
        interpretations.insert(
            "fatigue".to_string(),
            Self::interpret_fatigue(fatigue),
        );

        let purpose = state.needs().purpose_effective();
        interpretations.insert(
            "purpose".to_string(),
            Self::interpret_purpose(purpose),
        );

        // Social Cognition
        let loneliness = state.social_cognition().loneliness_effective();
        interpretations.insert(
            "loneliness".to_string(),
            Self::interpret_loneliness(loneliness),
        );

        let prc = state.social_cognition().perceived_reciprocal_caring_effective();
        interpretations.insert(
            "perceived_reciprocal_caring".to_string(),
            Self::interpret_perceived_reciprocal_caring(prc),
        );

        // Mental Health
        let depression = state.mental_health().depression_effective();
        interpretations.insert(
            "depression".to_string(),
            Self::interpret_depression(depression),
        );

        // Build summary
        let summary = Self::build_summary(&interpretations);

        StateInterpreter {
            interpretations,
            summary,
            delta_summary: None,
        }
    }

    /// Creates a new interpreter with delta from baseline.
    pub fn from_state_with_baseline(state: &IndividualState, baseline: &IndividualState) -> Self {
        let mut interpretations = HashMap::new();
        let mut deltas = Vec::new();

        let valence = state.mood().valence_effective();
        let baseline_valence = baseline.mood().valence_effective();
        interpretations.insert("valence".to_string(), Self::interpret_valence(valence));
        if let Some(delta) = Self::delta_valence(valence, baseline_valence) { deltas.push(delta); }

        let arousal = state.mood().arousal_effective();
        let baseline_arousal = baseline.mood().arousal_effective();
        interpretations.insert("arousal".to_string(), Self::interpret_arousal(arousal));
        if let Some(delta) = Self::delta_arousal(arousal, baseline_arousal) { deltas.push(delta); }

        let dominance = state.mood().dominance_effective();
        let baseline_dominance = baseline.mood().dominance_effective();
        interpretations.insert("dominance".to_string(), Self::interpret_dominance(dominance));
        if let Some(delta) = Self::delta_dominance(dominance, baseline_dominance) { deltas.push(delta); }

        let stress = state.needs().stress_effective();
        let baseline_stress = baseline.needs().stress_effective();
        interpretations.insert("stress".to_string(), Self::interpret_stress(stress));
        if let Some(delta) = Self::delta_stress(stress, baseline_stress) { deltas.push(delta); }

        let fatigue = state.needs().fatigue_effective();
        let baseline_fatigue = baseline.needs().fatigue_effective();
        interpretations.insert("fatigue".to_string(), Self::interpret_fatigue(fatigue));
        if let Some(delta) = Self::delta_fatigue(fatigue, baseline_fatigue) { deltas.push(delta); }

        let purpose = state.needs().purpose_effective();
        let baseline_purpose = baseline.needs().purpose_effective();
        interpretations.insert("purpose".to_string(), Self::interpret_purpose(purpose));
        if let Some(delta) = Self::delta_purpose(purpose, baseline_purpose) { deltas.push(delta); }

        let loneliness = state.social_cognition().loneliness_effective();
        let baseline_loneliness = baseline.social_cognition().loneliness_effective();
        interpretations.insert("loneliness".to_string(), Self::interpret_loneliness(loneliness));
        if let Some(delta) = Self::delta_loneliness(loneliness, baseline_loneliness) { deltas.push(delta); }
        let prc = state.social_cognition().perceived_reciprocal_caring_effective();
        let baseline_prc = baseline.social_cognition().perceived_reciprocal_caring_effective();
        interpretations.insert("perceived_reciprocal_caring".to_string(), Self::interpret_perceived_reciprocal_caring(prc));
        if let Some(delta) = Self::delta_perceived_reciprocal_caring(prc, baseline_prc) { deltas.push(delta); }
        let depression = state.mental_health().depression_effective();
        let baseline_depression = baseline.mental_health().depression_effective();
        interpretations.insert("depression".to_string(), Self::interpret_depression(depression));
        if let Some(delta) = Self::delta_depression(depression, baseline_depression) { deltas.push(delta); }

        let summary = Self::build_summary(&interpretations);
        let delta_summary = if deltas.is_empty() {
            None
        } else {
            Some(deltas.join(". ") + ".")
        };

        StateInterpreter {
            interpretations,
            summary,
            delta_summary,
        }
    }

    /// Returns all interpretations.
    pub fn interpretations(&self) -> &HashMap<String, String> { &self.interpretations }

    /// Returns the summary paragraph.
    pub fn summary(&self) -> &str { &self.summary }

    /// Returns the delta summary (changes from baseline).
    pub fn delta_summary(&self) -> Option<&str> { self.delta_summary.as_deref() }

    fn interpret_valence(value: f32) -> String {
        let normalized = value; // -1.0 to 1.0
        match normalized {
            v if v > 0.6 => "feeling very positive".to_string(),
            v if v > 0.2 => "feeling moderately positive".to_string(),
            v if v > -0.2 => "feeling neutral".to_string(),
            v if v > -0.6 => "feeling moderately negative".to_string(),
            _ => "feeling very negative".to_string(),
        }
    }

    fn interpret_arousal(value: f32) -> String {
        let normalized = value; // -1.0 to 1.0
        match normalized {
            v if v > 0.6 => "highly energized".to_string(),
            v if v > 0.2 => "moderately energized".to_string(),
            v if v > -0.2 => "neutral energy level".to_string(),
            v if v > -0.6 => "low energy".to_string(),
            _ => "very low energy".to_string(),
        }
    }

    fn interpret_dominance(value: f32) -> String {
        let normalized = value; // -1.0 to 1.0
        match normalized {
            v if v > 0.6 => "feeling very in control".to_string(),
            v if v > 0.2 => "feeling somewhat in control".to_string(),
            v if v > -0.2 => "feeling neutral control".to_string(),
            v if v > -0.6 => "feeling somewhat out of control".to_string(),
            _ => "feeling very out of control".to_string(),
        }
    }

    fn interpret_stress(value: f32) -> String {
        let normalized = value; // 0.0 to 1.0
        match normalized {
            v if v > 0.75 => "experiencing severe stress".to_string(),
            v if v > 0.5 => "experiencing elevated stress".to_string(),
            v if v > 0.25 => "experiencing mild stress".to_string(),
            _ => "feeling calm".to_string(),
        }
    }

    fn interpret_fatigue(value: f32) -> String {
        let normalized = value; // 0.0 to 1.0
        match normalized {
            v if v > 0.75 => "extremely fatigued".to_string(),
            v if v > 0.5 => "moderately fatigued".to_string(),
            v if v > 0.25 => "mildly fatigued".to_string(),
            _ => "well-rested".to_string(),
        }
    }

    fn interpret_purpose(value: f32) -> String {
        let normalized = value; // 0.0 to 1.0
        match normalized {
            v if v > 0.75 => "has strong sense of purpose".to_string(),
            v if v > 0.5 => "has moderate sense of purpose".to_string(),
            v if v > 0.25 => "has weak sense of purpose".to_string(),
            _ => "lacks sense of purpose".to_string(),
        }
    }

    fn interpret_loneliness(value: f32) -> String {
        let normalized = value; // 0.0 to 1.0
        match normalized {
            v if v > 0.75 => "feeling very lonely".to_string(),
            v if v > 0.5 => "feeling moderately lonely".to_string(),
            v if v > 0.25 => "feeling mildly lonely".to_string(),
            _ => "feeling well-connected".to_string(),
        }
    }

    fn interpret_perceived_reciprocal_caring(value: f32) -> String {
        let normalized = value; // 0.0 to 1.0
        match normalized {
            v if v > 0.75 => "feels deeply cared for by others".to_string(),
            v if v > 0.5 => "feels moderately cared for by others".to_string(),
            v if v > 0.25 => "feels somewhat cared for by others".to_string(),
            _ => "feels uncared for by others".to_string(),
        }
    }

    fn interpret_depression(value: f32) -> String {
        let normalized = value; // 0.0 to 1.0
        match normalized {
            v if v > 0.75 => "experiencing severe depression".to_string(),
            v if v > 0.5 => "experiencing moderate depression".to_string(),
            v if v > 0.25 => "experiencing mild depression".to_string(),
            _ => "not depressed".to_string(),
        }
    }

    /// Minimum delta threshold to report any change (avoids floating point noise).
    const MIN_DELTA_THRESHOLD: f32 = 0.01;

    /// Returns magnitude descriptor based on absolute delta value.
    /// Thresholds tuned for 0-1 or -1 to 1 scales.
    fn magnitude_word(delta: f32) -> &'static str {
        let abs_delta = delta.abs();
        if abs_delta < 0.05 {
            "slightly"
        } else if abs_delta < 0.15 {
            "somewhat"
        } else if abs_delta < 0.30 {
            "noticeably"
        } else {
            "much"
        }
    }

    /// Builds a concise delta description: "[magnitude] [comparative]"
    fn build_delta_description(
        current: f32,
        baseline: f32,
        comparative_more: &str,
        comparative_less: &str,
    ) -> Option<String> {
        let delta = current - baseline;

        // Ignore noise
        if delta.abs() < Self::MIN_DELTA_THRESHOLD {
            return None;
        }

        let magnitude = Self::magnitude_word(delta);
        let comparative = if delta > 0.0 { comparative_more } else { comparative_less };
        Some(format!("{} {}", magnitude, comparative))
    }

    fn delta_valence(current: f32, baseline: f32) -> Option<String> { Self::build_delta_description(current, baseline, "happier", "sadder") }
    fn delta_arousal(current: f32, baseline: f32) -> Option<String> { Self::build_delta_description(current, baseline, "more energized", "less energized") }
    fn delta_dominance(current: f32, baseline: f32) -> Option<String> { Self::build_delta_description(current, baseline, "more in control", "less in control") }
    fn delta_stress(current: f32, baseline: f32) -> Option<String> { Self::build_delta_description(current, baseline, "more stressed", "less stressed") }
    fn delta_fatigue(current: f32, baseline: f32) -> Option<String> { Self::build_delta_description(current, baseline, "more fatigued", "less fatigued") }
    fn delta_purpose(current: f32, baseline: f32) -> Option<String> { Self::build_delta_description(current, baseline, "stronger sense of purpose", "weaker sense of purpose") }
    fn delta_loneliness(current: f32, baseline: f32) -> Option<String> { Self::build_delta_description(current, baseline, "lonelier", "less lonely") }
    fn delta_perceived_reciprocal_caring(current: f32, baseline: f32) -> Option<String> { Self::build_delta_description(current, baseline, "more cared for", "less cared for") }
    fn delta_depression(current: f32, baseline: f32) -> Option<String> { Self::build_delta_description(current, baseline, "more depressed", "less depressed") }

    fn build_summary(interpretations: &HashMap<String, String>) -> String {
        // Build in a consistent order
        let order = vec![
            "valence",
            "arousal",
            "dominance",
            "stress",
            "fatigue",
            "purpose",
            "loneliness",
            "perceived_reciprocal_caring",
            "depression",
        ];

        let sentences: Vec<String> = order
            .iter()
            .filter_map(|key| interpretations.get(*key))
            .map(|s| { let mut chars = s.chars(); chars.next().map_or_else(String::new, |first| first.to_uppercase().chain(chars).collect()) })
            .collect();
        sentences.join(". ") + "."
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::IndividualState;

    #[test]
    fn build_summary_handles_empty_string() {
        let mut interpretations = HashMap::new();
        interpretations.insert("valence".to_string(), "".to_string());
        let summary = StateInterpreter::build_summary(&interpretations);
        assert_eq!(summary, ".");
    }

    #[test]
    fn build_summary_capitalizes_first_letter() {
        let mut interpretations = HashMap::new();
        interpretations.insert("valence".to_string(), "feeling good".to_string());
        let summary = StateInterpreter::build_summary(&interpretations);
        assert_eq!(summary, "Feeling good.");
    }

    #[test]
    fn from_state_creates_interpretations() {
        let state = IndividualState::new();
        let interpreter = StateInterpreter::from_state(&state);
        assert!(!interpreter.interpretations().is_empty());
        assert!(!interpreter.summary().is_empty());
        assert!(interpreter.delta_summary().is_none());
    }

    #[test]
    fn from_state_with_baseline_no_changes() {
        let state = IndividualState::new();
        let baseline = IndividualState::new();
        let interpreter = StateInterpreter::from_state_with_baseline(&state, &baseline);
        assert!(!interpreter.interpretations().is_empty());
        assert!(!interpreter.summary().is_empty());
        assert!(interpreter.delta_summary().is_none());
    }

    #[test]
    fn from_state_with_baseline_includes_social_and_mental_keys() {
        let state = IndividualState::new();
        let baseline = IndividualState::new();
        let interpreter = StateInterpreter::from_state_with_baseline(&state, &baseline);
        let interpretations = interpreter.interpretations();
        assert!(interpretations.contains_key("loneliness"));
        assert!(interpretations.contains_key("perceived_reciprocal_caring"));
        assert!(interpretations.contains_key("depression"));
    }

    #[test]
    fn interpretations_accessor_returns_map() {
        let state = IndividualState::new();
        let interpreter = StateInterpreter::from_state(&state);
        let interpretations = interpreter.interpretations();
        assert!(interpretations.contains_key("valence"));
        assert!(interpretations.contains_key("arousal"));
    }

    #[test]
    fn summary_accessor_returns_string() {
        let state = IndividualState::new();
        let interpreter = StateInterpreter::from_state(&state);
        let summary = interpreter.summary();
        assert!(!summary.is_empty());
    }

    #[test]
    fn delta_summary_accessor_returns_none_without_baseline() {
        let state = IndividualState::new();
        let interpreter = StateInterpreter::from_state(&state);
        assert!(interpreter.delta_summary().is_none());
    }

    #[test]
    fn from_state_with_baseline_with_changes() {
        let mut state = IndividualState::new();
        let baseline = IndividualState::new();

        state.mood_mut().add_valence_delta(0.8);

        let interpreter = StateInterpreter::from_state_with_baseline(&state, &baseline);
        assert!(!interpreter.interpretations().is_empty());
        assert!(!interpreter.summary().is_empty());
        assert!(interpreter.delta_summary().is_some());
        assert!(interpreter.delta_summary().unwrap().contains("happier"));
    }

    #[test]
    fn from_state_with_baseline_includes_needs_and_mental_health_deltas() {
        let mut state = IndividualState::new();
        let baseline = IndividualState::new();

        state.needs_mut().add_fatigue_delta(0.2);
        state.needs_mut().add_purpose_delta(-0.3);
        state.mental_health_mut().add_depression_delta(0.4);

        let interpreter = StateInterpreter::from_state_with_baseline(&state, &baseline);
        let delta_summary = interpreter.delta_summary().expect("delta summary missing");

        assert!(delta_summary.contains("more fatigued"));
        assert!(delta_summary.contains("weaker sense of purpose"));
        assert!(delta_summary.contains("more depressed"));
    }

    #[test]
    fn magnitude_word_returns_correct_descriptors() {
        assert_eq!(StateInterpreter::magnitude_word(0.02), "slightly");
        assert_eq!(StateInterpreter::magnitude_word(0.04), "slightly");
        assert_eq!(StateInterpreter::magnitude_word(0.08), "somewhat");
        assert_eq!(StateInterpreter::magnitude_word(0.14), "somewhat");
        assert_eq!(StateInterpreter::magnitude_word(0.20), "noticeably");
        assert_eq!(StateInterpreter::magnitude_word(0.29), "noticeably");
        assert_eq!(StateInterpreter::magnitude_word(0.35), "much");
        assert_eq!(StateInterpreter::magnitude_word(0.80), "much");
        // Negative deltas use absolute value
        assert_eq!(StateInterpreter::magnitude_word(-0.02), "slightly");
        assert_eq!(StateInterpreter::magnitude_word(-0.35), "much");
    }

    #[test]
    fn delta_below_threshold_returns_none() {
        // 0.005 is below MIN_DELTA_THRESHOLD of 0.01
        let result = StateInterpreter::delta_stress(0.505, 0.50);
        assert!(result.is_none());
    }

    #[test]
    fn delta_small_change_says_slightly() {
        // 0.03 delta = "slightly"
        let result = StateInterpreter::delta_stress(0.55, 0.52);
        assert_eq!(result, Some("slightly more stressed".to_string()));
    }

    #[test]
    fn delta_large_change_says_much() {
        // 0.50 delta = "much"
        let result = StateInterpreter::delta_stress(0.80, 0.30);
        assert_eq!(result, Some("much more stressed".to_string()));
    }

    #[test]
    fn delta_medium_change_says_noticeably() {
        // 0.20 delta = "noticeably"
        let result = StateInterpreter::delta_stress(0.50, 0.30);
        assert_eq!(result, Some("noticeably more stressed".to_string()));
    }

    #[test]
    fn delta_decrease_uses_correct_language() {
        // -0.30 delta = "much less" (>= 0.30 threshold)
        let result = StateInterpreter::delta_stress(0.30, 0.60);
        assert_eq!(result, Some("much less stressed".to_string()));
    }

    #[test]
    fn delta_valence_reports_increase() {
        let result = StateInterpreter::delta_valence(0.7, 0.0);
        assert_eq!(result, Some("much happier".to_string()));
    }
}
