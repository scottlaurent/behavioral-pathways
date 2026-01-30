//! Birth era categories for cohort effects.

/// Era when an entity was born.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BirthEra {
    /// Born during a crisis period.
    Crisis,
    /// Born during a stable period.
    Stability,
    /// Born during a resource scarcity period.
    Scarcity,
    /// Born during a growth/expansion period.
    Expansion,
    /// Unknown or unspecified birth era.
    Unknown,
}

impl BirthEra {
    /// Parses a birth era from a label, returning None if unknown.
    #[must_use]
    pub fn from_label(label: &str) -> Option<Self> {
        match label.trim().to_lowercase().as_str() {
            "crisis" => Some(BirthEra::Crisis),
            "stability" => Some(BirthEra::Stability),
            "scarcity" => Some(BirthEra::Scarcity),
            "expansion" => Some(BirthEra::Expansion),
            _ => None,
        }
    }
}

impl Default for BirthEra {
    fn default() -> Self {
        BirthEra::Unknown
    }
}
