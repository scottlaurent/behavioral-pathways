//! Demand characteristics for an individual.
//!
//! These are observable social signals (gender, ethnicity, appearance)
//! that shape how others respond to the person in immediate interactions.

use serde::{Deserialize, Serialize};

/// Demand characteristics for observable social signals.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemandCharacteristics {
    /// Gender identity or presentation.
    pub gender: String,

    /// Ethnicity or cultural background.
    pub ethnicity: String,

    /// Visible appearance descriptors.
    pub appearance: String,

    /// Other observable signals (style, accent, markers).
    pub observable_signals: Vec<String>,
}

impl Default for DemandCharacteristics {
    fn default() -> Self {
        DemandCharacteristics {
            gender: String::new(),
            ethnicity: String::new(),
            appearance: String::new(),
            observable_signals: Vec::new(),
        }
    }
}

impl DemandCharacteristics {
    /// Creates a new demand characteristics record with empty defaults.
    #[must_use]
    pub fn new() -> Self {
        DemandCharacteristics::default()
    }

    /// Sets the gender field.
    #[must_use]
    pub fn with_gender(mut self, gender: impl Into<String>) -> Self {
        self.gender = gender.into();
        self
    }

    /// Sets the ethnicity field.
    #[must_use]
    pub fn with_ethnicity(mut self, ethnicity: impl Into<String>) -> Self {
        self.ethnicity = ethnicity.into();
        self
    }

    /// Sets the appearance field.
    #[must_use]
    pub fn with_appearance(mut self, appearance: impl Into<String>) -> Self {
        self.appearance = appearance.into();
        self
    }

    /// Adds an observable signal entry.
    pub fn add_signal(&mut self, signal: impl Into<String>) {
        self.observable_signals.push(signal.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demand_characteristics_defaults_empty() {
        let demand = DemandCharacteristics::default();
        assert!(demand.gender.is_empty());
        assert!(demand.ethnicity.is_empty());
        assert!(demand.appearance.is_empty());
        assert!(demand.observable_signals.is_empty());
    }

    #[test]
    fn demand_characteristics_builder_sets_appearance() {
        let demand = DemandCharacteristics::new()
            .with_gender("female")
            .with_ethnicity("Latinx")
            .with_appearance("casual");
        assert_eq!(demand.gender, "female");
        assert_eq!(demand.ethnicity, "Latinx");
        assert_eq!(demand.appearance, "casual");
    }

    #[test]
    fn demand_characteristics_adds_signal() {
        let mut demand = DemandCharacteristics::default();
        demand.add_signal("accent");
        assert_eq!(demand.observable_signals.len(), 1);
        assert_eq!(demand.observable_signals[0], "accent");
    }
}
