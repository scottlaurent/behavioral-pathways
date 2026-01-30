//! Interaction pattern metadata for relationships.
//!
//! Captures how frequently and consistently entities interact.

use crate::types::Timestamp;

/// Interaction pattern for a relationship.
#[derive(Debug, Clone, PartialEq)]
pub struct InteractionPattern {
    /// How often the entities interact (0 = rarely, 1 = daily).
    pub frequency: f32,

    /// Regularity of interactions (0 = erratic, 1 = consistent).
    pub consistency: f32,

    /// Timestamp of the last interaction, if known.
    pub last_interaction: Option<Timestamp>,
}

impl Default for InteractionPattern {
    fn default() -> Self {
        InteractionPattern {
            frequency: 0.0,
            consistency: 0.0,
            last_interaction: None,
        }
    }
}

impl InteractionPattern {
    /// Creates a new InteractionPattern with empty defaults.
    #[must_use]
    pub fn new() -> Self {
        InteractionPattern::default()
    }

    /// Sets the interaction frequency.
    #[must_use]
    pub fn with_frequency(mut self, frequency: f32) -> Self {
        self.frequency = frequency.clamp(0.0, 1.0);
        self
    }

    /// Sets the interaction consistency.
    #[must_use]
    pub fn with_consistency(mut self, consistency: f32) -> Self {
        self.consistency = consistency.clamp(0.0, 1.0);
        self
    }

    /// Sets the last interaction timestamp.
    #[must_use]
    pub fn with_last_interaction(mut self, last_interaction: Timestamp) -> Self {
        self.last_interaction = Some(last_interaction);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interaction_pattern_defaults_empty() {
        let pattern = InteractionPattern::default();
        assert!((pattern.frequency - 0.0).abs() < f32::EPSILON);
        assert!((pattern.consistency - 0.0).abs() < f32::EPSILON);
        assert!(pattern.last_interaction.is_none());
    }

    #[test]
    fn interaction_pattern_builder_sets_values() {
        let ts = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let pattern = InteractionPattern::new()
            .with_frequency(1.5)
            .with_consistency(-0.2)
            .with_last_interaction(ts);

        assert!((pattern.frequency - 1.0).abs() < f32::EPSILON);
        assert!((pattern.consistency - 0.0).abs() < f32::EPSILON);
        assert_eq!(pattern.last_interaction, Some(ts));
    }
}
