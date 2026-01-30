//! Demographical metadata for an individual.
//!
//! These traits describe stable identity and background attributes
//! used for ecological bias and discrimination effects.

use crate::types::{Duration, Timestamp};
use serde::{Deserialize, Serialize};

/// Demographical information for an individual.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Demographical {
    /// Display name or preferred name.
    pub name: String,

    /// Date of birth, if known.
    pub date_of_birth: Option<Timestamp>,

    /// Current age as a duration.
    pub age: Duration,

    /// Gender identity or presentation.
    pub gender: String,

    /// Ethnicity or cultural background.
    pub ethnicity: String,
}

impl Default for Demographical {
    fn default() -> Self {
        Demographical {
            name: String::new(),
            date_of_birth: None,
            age: Duration::zero(),
            gender: String::new(),
            ethnicity: String::new(),
        }
    }
}

impl Demographical {
    /// Creates a new Demographical record with empty defaults.
    #[must_use]
    pub fn new() -> Self {
        Demographical::default()
    }

    /// Sets the name field.
    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Sets the date of birth.
    #[must_use]
    pub fn with_date_of_birth(mut self, dob: Timestamp) -> Self {
        self.date_of_birth = Some(dob);
        self
    }

    /// Sets the age value.
    #[must_use]
    pub fn with_age(mut self, age: Duration) -> Self {
        self.age = age;
        self
    }

    /// Sets the gender value.
    #[must_use]
    pub fn with_gender(mut self, gender: impl Into<String>) -> Self {
        self.gender = gender.into();
        self
    }

    /// Sets the ethnicity value.
    #[must_use]
    pub fn with_ethnicity(mut self, ethnicity: impl Into<String>) -> Self {
        self.ethnicity = ethnicity.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Duration, Timestamp};

    #[test]
    fn demographical_defaults_empty_values() {
        let demo = Demographical::default();
        assert!(demo.name.is_empty());
        assert!(demo.date_of_birth.is_none());
        assert_eq!(demo.age, Duration::zero());
        assert!(demo.gender.is_empty());
        assert!(demo.ethnicity.is_empty());
    }

    #[test]
    fn demographical_builder_sets_fields() {
        let dob = Timestamp::from_ymd_hms(1990, 1, 1, 0, 0, 0);
        let demo = Demographical::new()
            .with_name("Alex")
            .with_date_of_birth(dob)
            .with_age(Duration::years(30))
            .with_gender("nonbinary")
            .with_ethnicity("Latinx");

        assert_eq!(demo.name, "Alex");
        assert_eq!(demo.date_of_birth, Some(dob));
        assert_eq!(demo.age, Duration::years(30));
        assert_eq!(demo.gender, "nonbinary");
        assert_eq!(demo.ethnicity, "Latinx");
    }
}
