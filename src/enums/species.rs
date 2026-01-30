//! Species definitions for entities.
//!
//! Each species defines lifespan characteristics and time scaling factors
//! that determine how quickly psychological processes occur relative to
//! real time.

use serde::{Deserialize, Serialize};

/// Species type defining biological characteristics that affect psychological processing.
///
/// Each species has:
/// - A typical lifespan in years
/// - A maturity age when personality stabilizes
/// - A time scale factor (relative to human baseline of 1.0)
///
/// # Time Scale
///
/// Time scale determines how quickly psychological processes occur.
/// A dog with time_scale 6.7 experiences ~7 psychological days per real day.
/// This affects decay rates, development speed, and memory consolidation.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::enums::Species;
///
/// let human = Species::Human;
/// assert_eq!(human.lifespan_years(), 80);
/// assert!((human.time_scale() - 1.0).abs() < f32::EPSILON);
///
/// let dog = Species::Dog;
/// assert_eq!(dog.lifespan_years(), 12);
/// assert!((dog.time_scale() - 6.67).abs() < 0.1);
/// ```
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum Species {
    /// Standard human psychology with full model support.
    /// Lifespan: 80 years, Maturity: 25 years, Time scale: 1.0x
    #[default]
    Human,

    /// Domestic dog with pack-oriented social behavior.
    /// Lifespan: 12 years, Maturity: 2 years, Time scale: 6.67x
    Dog,

    /// Domestic cat with solitary behavior patterns.
    /// Lifespan: 15 years, Maturity: 1 year, Time scale: 5.33x
    Cat,

    /// Dolphin with very high social complexity.
    /// Lifespan: 50 years, Maturity: 8 years, Time scale: 1.6x
    Dolphin,

    /// Horse with herd-based social structure.
    /// Lifespan: 30 years, Maturity: 4 years, Time scale: 2.67x
    Horse,

    /// Elephant with matriarchal social structure.
    /// Lifespan: 70 years, Maturity: 15 years, Time scale: 1.14x
    Elephant,

    /// Chimpanzee with very high social complexity.
    /// Lifespan: 50 years, Maturity: 13 years, Time scale: 1.6x
    Chimpanzee,

    /// Crow with family-based social structure.
    /// Lifespan: 15 years, Maturity: 2 years, Time scale: 5.33x
    Crow,

    /// Mouse with minimal psychology model.
    /// Lifespan: 2 years, Maturity: 6 weeks, Time scale: 40x
    Mouse,

    /// Custom species with user-defined parameters.
    Custom {
        /// Species identifier name.
        name: String,
        /// Expected lifespan in years.
        lifespan_years: u16,
        /// Age at which personality stabilizes, in years.
        maturity_age_years: u16,
        /// Social complexity factor (0.0 to 1.0).
        /// Affects which subsystems are active.
        social_complexity: f32,
    },
}

impl Species {
    /// Returns the expected lifespan in years for this species.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::enums::Species;
    ///
    /// assert_eq!(Species::Human.lifespan_years(), 80);
    /// assert_eq!(Species::Dog.lifespan_years(), 12);
    /// ```
    #[must_use]
    pub fn lifespan_years(&self) -> u16 {
        match self {
            Species::Human => 80,
            Species::Dog => 12,
            Species::Cat => 15,
            Species::Dolphin => 50,
            Species::Horse => 30,
            Species::Elephant => 70,
            Species::Chimpanzee => 50,
            Species::Crow => 15,
            Species::Mouse => 2,
            Species::Custom { lifespan_years, .. } => *lifespan_years,
        }
    }

    /// Returns the maturity age in years for this species.
    ///
    /// Maturity age is when personality traits stabilize and plasticity
    /// decreases significantly.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::enums::Species;
    ///
    /// assert_eq!(Species::Human.maturity_age_years(), 25);
    /// assert_eq!(Species::Dog.maturity_age_years(), 2);
    /// ```
    #[must_use]
    pub fn maturity_age_years(&self) -> u16 {
        match self {
            Species::Human => 25,
            Species::Dog => 2,
            Species::Cat => 1,
            Species::Dolphin => 8,
            Species::Horse => 4,
            Species::Elephant => 15,
            Species::Chimpanzee => 13,
            Species::Crow => 2,
            Species::Mouse => 0, // 6 weeks, represented as 0 years
            Species::Custom {
                maturity_age_years, ..
            } => *maturity_age_years,
        }
    }

    /// Returns the time scale factor relative to human baseline.
    ///
    /// Time scale is calculated as: human_lifespan / species_lifespan
    ///
    /// A higher time scale means faster psychological processing:
    /// - Human: 1.0x (baseline)
    /// - Dog: 6.67x (experiences ~7 psychological days per real day)
    /// - Mouse: 40x (extremely rapid psychological cycling)
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::enums::Species;
    ///
    /// let human_scale = Species::Human.time_scale();
    /// assert!((human_scale - 1.0).abs() < f32::EPSILON);
    ///
    /// let dog_scale = Species::Dog.time_scale();
    /// assert!((dog_scale - 6.67).abs() < 0.1);
    /// ```
    #[must_use]
    pub fn time_scale(&self) -> f32 {
        const HUMAN_LIFESPAN: f32 = 80.0;
        let lifespan = f32::from(self.lifespan_years());
        if lifespan > 0.0 {
            HUMAN_LIFESPAN / lifespan
        } else {
            1.0 // Fallback for zero lifespan (should not happen in practice)
        }
    }

    /// Returns the social complexity rating for this species.
    ///
    /// Social complexity affects which subsystems are active:
    /// - Very high (0.8-1.0): Full model with all subsystems
    /// - High (0.6-0.8): Most subsystems, simplified ITS
    /// - Medium (0.4-0.6): Core subsystems only
    /// - Low (0.2-0.4): Simplified model
    /// - Minimal (0.0-0.2): State and memory only
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::enums::Species;
    ///
    /// assert!(Species::Human.social_complexity() > 0.8);
    /// assert!(Species::Dog.social_complexity() > 0.6);
    /// assert!(Species::Mouse.social_complexity() < 0.3);
    /// ```
    #[must_use]
    pub fn social_complexity(&self) -> f32 {
        match self {
            Species::Human => 1.0,
            Species::Dog => 0.7,
            Species::Cat => 0.3,
            Species::Dolphin => 0.9,
            Species::Horse => 0.5,
            Species::Elephant => 0.9,
            Species::Chimpanzee => 0.9,
            Species::Crow => 0.7,
            Species::Mouse => 0.1,
            Species::Custom {
                social_complexity, ..
            } => *social_complexity,
        }
    }

    /// Creates a custom species with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `name` - Identifier for the species
    /// * `lifespan_years` - Expected lifespan in years
    /// * `maturity_age_years` - Age when personality stabilizes
    /// * `social_complexity` - Social complexity factor (clamped to 0.0-1.0)
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::enums::Species;
    ///
    /// let parrot = Species::custom("Parrot", 60, 5, 0.6);
    /// assert_eq!(parrot.lifespan_years(), 60);
    /// assert_eq!(parrot.maturity_age_years(), 5);
    /// assert!((parrot.social_complexity() - 0.6).abs() < f32::EPSILON);
    /// ```
    #[must_use]
    pub fn custom(
        name: impl Into<String>,
        lifespan_years: u16,
        maturity_age_years: u16,
        social_complexity: f32,
    ) -> Self {
        Species::Custom {
            name: name.into(),
            lifespan_years,
            maturity_age_years,
            social_complexity: social_complexity.clamp(0.0, 1.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn human_lifespan_eighty_years() {
        assert_eq!(Species::Human.lifespan_years(), 80);
    }

    #[test]
    fn human_time_scale_is_one() {
        let scale = Species::Human.time_scale();
        assert!((scale - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn dog_time_scale_approximately_seven() {
        let scale = Species::Dog.time_scale();
        // 80 / 12 = 6.666...
        let expected = 80.0 / 12.0;
        assert!((scale - expected).abs() < 0.01);
        // Also verify it's approximately 7 (within 0.5)
        assert!((scale - 7.0).abs() < 0.5);
    }

    #[test]
    fn custom_species_parameters() {
        let parrot = Species::custom("Parrot", 60, 5, 0.65);

        assert_eq!(parrot.lifespan_years(), 60);
        assert_eq!(parrot.maturity_age_years(), 5);
        assert!((parrot.social_complexity() - 0.65).abs() < f32::EPSILON);
        assert!((parrot.time_scale() - (80.0 / 60.0)).abs() < 0.01);
    }

    #[test]
    fn custom_species_clamps_social_complexity() {
        let too_high = Species::custom("Test", 50, 5, 1.5);
        assert!((too_high.social_complexity() - 1.0).abs() < f32::EPSILON);

        let too_low = Species::custom("Test", 50, 5, -0.5);
        assert!(too_low.social_complexity().abs() < f32::EPSILON);
    }

    #[test]
    fn all_species_have_positive_time_scale() {
        assert!(Species::Human.time_scale() > 0.0);

        let species = [
            Species::Dog,
            Species::Cat,
            Species::Dolphin,
            Species::Horse,
            Species::Elephant,
            Species::Chimpanzee,
            Species::Crow,
            Species::Mouse,
        ];

        for s in species {
            assert!(s.time_scale() > 0.0);
        }
    }

    #[test]
    fn default_species_is_human() {
        assert_eq!(Species::default(), Species::Human);
    }

    #[test]
    fn species_equality() {
        assert_eq!(Species::Human, Species::Human);
        assert_ne!(Species::Human, Species::Dog);

        let parrot1 = Species::custom("Parrot", 60, 5, 0.6);
        let parrot2 = Species::custom("Parrot", 60, 5, 0.6);
        assert_eq!(parrot1, parrot2);
    }

    #[test]
    fn elephant_has_similar_time_scale_to_human() {
        let elephant_scale = Species::Elephant.time_scale();
        let human_scale = Species::Human.time_scale();

        // Elephant (70 years) should have scale close to human (80 years)
        // 80/70 = ~1.14
        assert!((elephant_scale - human_scale).abs() < 0.2);
    }

    #[test]
    fn mouse_has_highest_time_scale() {
        let mouse_scale = Species::Mouse.time_scale();
        // 80 / 2 = 40
        assert!((mouse_scale - 40.0).abs() < 0.1);
    }

    #[test]
    fn all_species_lifespan_values() {
        // Verify all species have expected lifespan values
        assert_eq!(Species::Cat.lifespan_years(), 15);
        assert_eq!(Species::Dolphin.lifespan_years(), 50);
        assert_eq!(Species::Horse.lifespan_years(), 30);
        assert_eq!(Species::Chimpanzee.lifespan_years(), 50);
        assert_eq!(Species::Crow.lifespan_years(), 15);
    }

    #[test]
    fn all_species_maturity_values() {
        // Verify all species have expected maturity values
        assert_eq!(Species::Human.maturity_age_years(), 25);
        assert_eq!(Species::Cat.maturity_age_years(), 1);
        assert_eq!(Species::Dolphin.maturity_age_years(), 8);
        assert_eq!(Species::Horse.maturity_age_years(), 4);
        assert_eq!(Species::Elephant.maturity_age_years(), 15);
        assert_eq!(Species::Chimpanzee.maturity_age_years(), 13);
        assert_eq!(Species::Crow.maturity_age_years(), 2);
        assert_eq!(Species::Mouse.maturity_age_years(), 0);
    }

    #[test]
    fn all_species_social_complexity_values() {
        // Verify all species have expected social complexity
        assert!((Species::Human.social_complexity() - 1.0).abs() < f32::EPSILON);
        assert!((Species::Dog.social_complexity() - 0.7).abs() < f32::EPSILON);
        assert!((Species::Cat.social_complexity() - 0.3).abs() < f32::EPSILON);
        assert!((Species::Dolphin.social_complexity() - 0.9).abs() < f32::EPSILON);
        assert!((Species::Horse.social_complexity() - 0.5).abs() < f32::EPSILON);
        assert!((Species::Elephant.social_complexity() - 0.9).abs() < f32::EPSILON);
        assert!((Species::Chimpanzee.social_complexity() - 0.9).abs() < f32::EPSILON);
        assert!((Species::Crow.social_complexity() - 0.7).abs() < f32::EPSILON);
        assert!((Species::Mouse.social_complexity() - 0.1).abs() < f32::EPSILON);
    }

    #[test]
    fn custom_species_with_zero_lifespan() {
        // Edge case: zero lifespan should return time_scale of 1.0 (fallback)
        let zero_lifespan = Species::custom("ZeroLife", 0, 0, 0.5);
        assert!((zero_lifespan.time_scale() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn clone_species() {
        let human = Species::Human;
        let cloned = human.clone();
        assert_eq!(human, cloned);

        let custom = Species::custom("Test", 50, 5, 0.5);
        let custom_cloned = custom.clone();
        assert_eq!(custom, custom_cloned);
    }

    #[test]
    fn debug_format() {
        let human = Species::Human;
        let debug = format!("{:?}", human);
        assert!(debug.contains("Human"));

        let custom = Species::custom("Parrot", 60, 5, 0.6);
        let custom_debug = format!("{:?}", custom);
        assert!(custom_debug.contains("Custom"));
        assert!(custom_debug.contains("Parrot"));
    }
}
