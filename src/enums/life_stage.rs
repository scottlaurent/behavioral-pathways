//! Life stage definitions for developmental modeling.
//!
//! Life stages inform plasticity calculations and event impact multipliers
//! (how strongly events affect the entity).

use crate::enums::Species;

/// Developmental life stage based on age.
///
/// Each stage has associated event impact multipliers (how strongly events
/// affect the entity) and informs plasticity calculations.
///
/// Plasticity decreases with age as personality crystallizes.
/// Event impact also generally decreases, reflecting reduced sensitivity
/// to new experiences in later life.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::enums::{LifeStage, Species};
///
/// let stage = LifeStage::from_age_years_for_species(&Species::Human, 8.0);
/// assert_eq!(stage, LifeStage::Child);
/// assert_eq!(stage.name(), "Child");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum LifeStage {
    /// Ages 0-12: Highest plasticity, greatest event impact.
    Child,

    /// Ages 13-17: High plasticity, strong event impact.
    Adolescent,

    /// Ages 18-30: Moderate plasticity, moderate event impact.
    YoungAdult,

    /// Ages 31-55: Lower plasticity, baseline event impact.
    #[default]
    Adult,

    /// Ages 56-70: Low plasticity, reduced event impact.
    MatureAdult,

    /// Ages 71+: Lowest plasticity, minimal event impact.
    Elder,
}

impl LifeStage {
    /// Internal helper: Determines the life stage from human-equivalent age in years.
    ///
    /// This is a private helper used by `from_age_years_for_species()`.
    /// External callers should always use `from_age_years_for_species()` which
    /// requires an explicit Species parameter.
    fn from_human_equivalent_age(age_years: u16) -> Self {
        match age_years {
            0..=12 => LifeStage::Child,
            13..=17 => LifeStage::Adolescent,
            18..=30 => LifeStage::YoungAdult,
            31..=55 => LifeStage::Adult,
            56..=70 => LifeStage::MatureAdult,
            _ => LifeStage::Elder,
        }
    }

    /// Determines the life stage from age in years, scaled for the given species.
    ///
    /// This function scales the human-centric thresholds based on the species'
    /// maturity age relative to humans (25 years). For example, a dog matures
    /// at 2 years, so a 2-year-old dog is equivalent to a 25-year-old human
    /// (a Young Adult), and a 1-year-old dog is roughly equivalent to a
    /// 12.5-year-old human (a Child).
    ///
    /// # Scaling Formula
    ///
    /// The age is converted to "human-equivalent years" using:
    /// ```text
    /// human_equivalent = age_years * (human_maturity / species_maturity)
    ///                  = age_years * (25 / species.maturity_age_years())
    /// ```
    ///
    /// # Arguments
    ///
    /// * `species` - The species to scale thresholds for
    /// * `age_years` - The entity's age in years (as f64 for precision with short-lived species)
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::enums::{LifeStage, Species};
    ///
    /// // A 2-year-old dog has reached maturity (YoungAdult stage)
    /// assert_eq!(LifeStage::from_age_years_for_species(&Species::Dog, 2.0), LifeStage::YoungAdult);
    ///
    /// // A 1-year-old dog = 12.5 human years (boundary, still Child)
    /// assert_eq!(LifeStage::from_age_years_for_species(&Species::Dog, 1.0), LifeStage::Child);
    ///
    /// // A 1.1-year-old dog = 13.75 human years (Adolescent)
    /// assert_eq!(LifeStage::from_age_years_for_species(&Species::Dog, 1.1), LifeStage::Adolescent);
    ///
    /// // Human uses standard thresholds
    /// assert_eq!(LifeStage::from_age_years_for_species(&Species::Human, 25.0), LifeStage::YoungAdult);
    /// ```
    #[must_use]
    pub fn from_age_years_for_species(species: &Species, age_years: f64) -> Self {
        const HUMAN_MATURITY: f64 = 25.0;

        let species_maturity = f64::from(species.maturity_age_years());

        // Handle species with zero maturity age (e.g., Mouse with 6-week maturity stored as 0)
        // Use a small default to avoid division by zero while still giving fast development
        let effective_maturity = if species_maturity > 0.0 {
            species_maturity
        } else {
            // For species like Mouse where maturity is sub-year, use 0.12 (about 6 weeks)
            0.12
        };

        // Scale the age to human-equivalent years
        let scale_factor = HUMAN_MATURITY / effective_maturity;
        let human_equivalent_age = age_years * scale_factor;

        // Convert to u16 for the internal helper, clamping to valid range
        let clamped_age = human_equivalent_age.clamp(0.0, f64::from(u16::MAX)) as u16;

        Self::from_human_equivalent_age(clamped_age)
    }

    /// Returns the event impact multiplier for this life stage.
    ///
    /// This multiplier affects how strongly events affect the entity.
    /// Children are more affected by events than elders.
    ///
    /// | Stage | Impact Multiplier |
    /// |-------|-------------------|
    /// | Child | 2.0x |
    /// | Adolescent | 1.5x |
    /// | YoungAdult | 1.2x |
    /// | Adult | 1.0x (baseline) |
    /// | MatureAdult | 0.9x |
    /// | Elder | 0.8x |
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::enums::LifeStage;
    ///
    /// assert!((LifeStage::Child.event_impact_multiplier() - 2.0).abs() < f32::EPSILON);
    /// assert!((LifeStage::Adult.event_impact_multiplier() - 1.0).abs() < f32::EPSILON);
    /// ```
    #[must_use]
    pub const fn event_impact_multiplier(&self) -> f32 {
        match self {
            LifeStage::Child => 2.0,
            LifeStage::Adolescent => 1.5,
            LifeStage::YoungAdult => 1.2,
            LifeStage::Adult => 1.0,
            LifeStage::MatureAdult => 0.9,
            LifeStage::Elder => 0.8,
        }
    }

    /// Returns the age range (inclusive) for this life stage.
    ///
    /// # Returns
    ///
    /// A tuple of (min_age, max_age) in years. For Elder, max_age is `u16::MAX`.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::enums::LifeStage;
    ///
    /// assert_eq!(LifeStage::Child.age_range(), (0, 12));
    /// assert_eq!(LifeStage::Adolescent.age_range(), (13, 17));
    /// assert_eq!(LifeStage::Elder.age_range(), (71, u16::MAX));
    /// ```
    #[must_use]
    pub const fn age_range(&self) -> (u16, u16) {
        match self {
            LifeStage::Child => (0, 12),
            LifeStage::Adolescent => (13, 17),
            LifeStage::YoungAdult => (18, 30),
            LifeStage::Adult => (31, 55),
            LifeStage::MatureAdult => (56, 70),
            LifeStage::Elder => (71, u16::MAX),
        }
    }

    /// Returns a human-readable name for this life stage.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::enums::LifeStage;
    ///
    /// assert_eq!(LifeStage::YoungAdult.name(), "Young Adult");
    /// assert_eq!(LifeStage::MatureAdult.name(), "Mature Adult");
    /// ```
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            LifeStage::Child => "Child",
            LifeStage::Adolescent => "Adolescent",
            LifeStage::YoungAdult => "Young Adult",
            LifeStage::Adult => "Adult",
            LifeStage::MatureAdult => "Mature Adult",
            LifeStage::Elder => "Elder",
        }
    }

    /// Returns all life stages in order from youngest to oldest.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::enums::LifeStage;
    ///
    /// let stages = LifeStage::all();
    /// assert_eq!(stages[0], LifeStage::Child);
    /// assert_eq!(stages[5], LifeStage::Elder);
    /// ```
    #[must_use]
    pub const fn all() -> [LifeStage; 6] {
        [
            LifeStage::Child,
            LifeStage::Adolescent,
            LifeStage::YoungAdult,
            LifeStage::Adult,
            LifeStage::MatureAdult,
            LifeStage::Elder,
        ]
    }
}

impl std::fmt::Display for LifeStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn age_eight_is_child() {
        // Use species-aware public API
        let stage = LifeStage::from_age_years_for_species(&Species::Human, 8.0);
        assert_eq!(stage, LifeStage::Child);
    }

    #[test]
    fn age_eighteen_is_young_adult() {
        // Use species-aware public API
        let stage = LifeStage::from_age_years_for_species(&Species::Human, 18.0);
        assert_eq!(stage, LifeStage::YoungAdult);
    }

    #[test]
    fn event_impact_decreases_with_age() {
        let stages = LifeStage::all();
        for i in 0..stages.len() - 1 {
            assert!(stages[i].event_impact_multiplier() >= stages[i + 1].event_impact_multiplier());
        }
    }

    #[test]
    fn age_boundaries() {
        // Test boundary values using internal helper (tests internal mapping logic)
        assert_eq!(LifeStage::from_human_equivalent_age(0), LifeStage::Child);
        assert_eq!(LifeStage::from_human_equivalent_age(12), LifeStage::Child);
        assert_eq!(
            LifeStage::from_human_equivalent_age(13),
            LifeStage::Adolescent
        );
        assert_eq!(
            LifeStage::from_human_equivalent_age(17),
            LifeStage::Adolescent
        );
        assert_eq!(
            LifeStage::from_human_equivalent_age(18),
            LifeStage::YoungAdult
        );
        assert_eq!(
            LifeStage::from_human_equivalent_age(30),
            LifeStage::YoungAdult
        );
        assert_eq!(LifeStage::from_human_equivalent_age(31), LifeStage::Adult);
        assert_eq!(LifeStage::from_human_equivalent_age(55), LifeStage::Adult);
        assert_eq!(
            LifeStage::from_human_equivalent_age(56),
            LifeStage::MatureAdult
        );
        assert_eq!(
            LifeStage::from_human_equivalent_age(70),
            LifeStage::MatureAdult
        );
        assert_eq!(LifeStage::from_human_equivalent_age(71), LifeStage::Elder);
        assert_eq!(LifeStage::from_human_equivalent_age(100), LifeStage::Elder);
    }

    #[test]
    fn age_range_consistency() {
        // Verify that internal helper is consistent with age_range
        for stage in LifeStage::all() {
            let (min, max) = stage.age_range();
            assert_eq!(LifeStage::from_human_equivalent_age(min), stage);
            if max < u16::MAX {
                assert_eq!(LifeStage::from_human_equivalent_age(max), stage);
            }
        }
    }

    #[test]
    fn display_format() {
        assert_eq!(format!("{}", LifeStage::Child), "Child");
        assert_eq!(format!("{}", LifeStage::YoungAdult), "Young Adult");
        assert_eq!(format!("{}", LifeStage::MatureAdult), "Mature Adult");
    }

    #[test]
    fn default_is_adult() {
        assert_eq!(LifeStage::default(), LifeStage::Adult);
    }

    #[test]
    fn all_stages() {
        let stages = LifeStage::all();
        assert_eq!(stages.len(), 6);
        assert_eq!(stages[0], LifeStage::Child);
        assert_eq!(stages[5], LifeStage::Elder);
    }

    #[test]
    fn equality_and_hash() {
        use std::collections::HashSet;

        assert_eq!(LifeStage::Child, LifeStage::Child);
        assert_ne!(LifeStage::Child, LifeStage::Adult);

        let mut set = HashSet::new();
        set.insert(LifeStage::Child);
        set.insert(LifeStage::Child);
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn copy_and_clone() {
        let original = LifeStage::Adult;
        let copied = original;
        let cloned = original.clone();

        assert_eq!(original, copied);
        assert_eq!(original, cloned);
    }

    #[test]
    fn all_stages_have_names() {
        for stage in LifeStage::all() {
            assert!(!stage.name().is_empty());
        }
    }

    #[test]
    fn all_stages_age_ranges_valid() {
        for stage in LifeStage::all() {
            let (min, max) = stage.age_range();
            assert!(min <= max);
        }
    }

    #[test]
    fn adolescent_properties() {
        let stage = LifeStage::Adolescent;
        assert_eq!(stage.name(), "Adolescent");
        assert!((stage.event_impact_multiplier() - 1.5).abs() < f32::EPSILON);
    }

    #[test]
    fn adult_properties() {
        let stage = LifeStage::Adult;
        assert_eq!(stage.name(), "Adult");
        assert!((stage.event_impact_multiplier() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn mature_adult_properties() {
        let stage = LifeStage::MatureAdult;
        assert_eq!(stage.name(), "Mature Adult");
        assert!((stage.event_impact_multiplier() - 0.9).abs() < f32::EPSILON);
    }

    #[test]
    fn elder_properties() {
        let stage = LifeStage::Elder;
        assert_eq!(stage.name(), "Elder");
        assert!((stage.event_impact_multiplier() - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn debug_format() {
        let stage = LifeStage::YoungAdult;
        let debug = format!("{:?}", stage);
        assert!(debug.contains("YoungAdult"));
    }

    #[test]
    fn child_properties() {
        let stage = LifeStage::Child;
        assert_eq!(stage.name(), "Child");
        assert!((stage.event_impact_multiplier() - 2.0).abs() < f32::EPSILON);
        assert_eq!(stage.age_range(), (0, 12));
    }

    #[test]
    fn young_adult_properties() {
        let stage = LifeStage::YoungAdult;
        assert_eq!(stage.name(), "Young Adult");
        assert!((stage.event_impact_multiplier() - 1.2).abs() < f32::EPSILON);
        assert_eq!(stage.age_range(), (18, 30));
    }

    // ==================== Species-Aware Tests ====================

    #[test]
    fn dog_two_years_is_young_adult() {
        // Dog maturity is 2 years, human maturity is 25 years
        // A 2-year-old dog = 25 human-equivalent years = YoungAdult
        let stage = LifeStage::from_age_years_for_species(&Species::Dog, 2.0);
        assert_eq!(stage, LifeStage::YoungAdult);
    }

    #[test]
    fn dog_one_year_is_child() {
        // 1-year-old dog = 12.5 human-equivalent years
        // scale_factor = 25 / 2 = 12.5
        // 1.0 * 12.5 = 12.5 -> truncates to 12 -> Child (0-12 range)
        let stage = LifeStage::from_age_years_for_species(&Species::Dog, 1.0);
        assert_eq!(stage, LifeStage::Child);
    }

    #[test]
    fn dog_one_point_one_years_is_adolescent() {
        // 1.1-year-old dog = 13.75 human-equivalent years
        // scale_factor = 25 / 2 = 12.5
        // 1.1 * 12.5 = 13.75 -> truncates to 13 -> Adolescent (13-17 range)
        let stage = LifeStage::from_age_years_for_species(&Species::Dog, 1.1);
        assert_eq!(stage, LifeStage::Adolescent);
    }

    #[test]
    fn dog_six_months_is_child() {
        // 0.5-year-old dog = 6.25 human-equivalent years = Child
        let stage = LifeStage::from_age_years_for_species(&Species::Dog, 0.5);
        assert_eq!(stage, LifeStage::Child);
    }

    #[test]
    fn dog_eight_years_is_elder() {
        // Dog lifespan is 12 years, maturity at 2
        // 8-year-old dog = 100 human-equivalent years = Elder
        let stage = LifeStage::from_age_years_for_species(&Species::Dog, 8.0);
        assert_eq!(stage, LifeStage::Elder);
    }

    #[test]
    fn human_uses_standard_thresholds() {
        // Human maturity is 25 years, so scale factor is 1.0
        assert_eq!(
            LifeStage::from_age_years_for_species(&Species::Human, 5.0),
            LifeStage::Child
        );
        assert_eq!(
            LifeStage::from_age_years_for_species(&Species::Human, 15.0),
            LifeStage::Adolescent
        );
        assert_eq!(
            LifeStage::from_age_years_for_species(&Species::Human, 25.0),
            LifeStage::YoungAdult
        );
        assert_eq!(
            LifeStage::from_age_years_for_species(&Species::Human, 40.0),
            LifeStage::Adult
        );
        assert_eq!(
            LifeStage::from_age_years_for_species(&Species::Human, 60.0),
            LifeStage::MatureAdult
        );
        assert_eq!(
            LifeStage::from_age_years_for_species(&Species::Human, 80.0),
            LifeStage::Elder
        );
    }

    #[test]
    fn cat_one_year_is_young_adult() {
        // Cat maturity is 1 year
        // scale_factor = 25 / 1 = 25
        // 1.0 * 25 = 25 -> YoungAdult (18-30)
        let stage = LifeStage::from_age_years_for_species(&Species::Cat, 1.0);
        assert_eq!(stage, LifeStage::YoungAdult);
    }

    #[test]
    fn cat_half_year_is_adolescent() {
        // 0.5-year-old cat = 12.5 human-equivalent years = Child (truncates to 12)
        // For Adolescent, need 13/25 = 0.52 years
        let stage = LifeStage::from_age_years_for_species(&Species::Cat, 0.55);
        assert_eq!(stage, LifeStage::Adolescent);
    }

    #[test]
    fn elephant_at_maturity_is_young_adult() {
        // Elephant maturity is 15 years
        // scale_factor = 25 / 15 = 1.67
        // 15.0 * 1.67 = 25 -> YoungAdult
        let stage = LifeStage::from_age_years_for_species(&Species::Elephant, 15.0);
        assert_eq!(stage, LifeStage::YoungAdult);
    }

    #[test]
    fn mouse_zero_maturity_uses_fallback() {
        // Mouse has 0 maturity_age_years (6 weeks is sub-year)
        // Fallback is 0.12 years (~6 weeks)
        // scale_factor = 25 / 0.12 = 208.33
        // 0.12 * 208.33 = 25 -> YoungAdult
        let stage = LifeStage::from_age_years_for_species(&Species::Mouse, 0.12);
        assert_eq!(stage, LifeStage::YoungAdult);
    }

    #[test]
    fn mouse_one_week_is_child() {
        // 1 week = ~0.019 years
        // 0.019 * 208.33 = ~4 -> Child
        let stage = LifeStage::from_age_years_for_species(&Species::Mouse, 0.019);
        assert_eq!(stage, LifeStage::Child);
    }

    #[test]
    fn custom_species_uses_custom_maturity() {
        // Custom parrot with 5-year maturity
        let parrot = Species::custom("Parrot", 60, 5, 0.6);
        // scale_factor = 25 / 5 = 5
        // 5.0 * 5 = 25 -> YoungAdult
        let stage = LifeStage::from_age_years_for_species(&parrot, 5.0);
        assert_eq!(stage, LifeStage::YoungAdult);
    }

    #[test]
    fn custom_species_zero_maturity_uses_fallback() {
        // Custom species with 0 maturity (edge case)
        let fast_critter = Species::custom("FastCritter", 1, 0, 0.2);
        // Should use 0.12 fallback
        // scale_factor = 25 / 0.12 = 208.33
        // 0.12 * 208.33 = 25 -> YoungAdult
        let stage = LifeStage::from_age_years_for_species(&fast_critter, 0.12);
        assert_eq!(stage, LifeStage::YoungAdult);
    }

    #[test]
    fn zero_age_is_child_for_all_species() {
        // Age 0 should always be Child regardless of species
        assert_eq!(
            LifeStage::from_age_years_for_species(&Species::Human, 0.0),
            LifeStage::Child
        );
        assert_eq!(
            LifeStage::from_age_years_for_species(&Species::Dog, 0.0),
            LifeStage::Child
        );
        assert_eq!(
            LifeStage::from_age_years_for_species(&Species::Mouse, 0.0),
            LifeStage::Child
        );
    }

    #[test]
    fn very_old_age_is_elder() {
        // Very old ages should be Elder
        // Dog: 12 years * 12.5 = 150 human years -> Elder
        assert_eq!(
            LifeStage::from_age_years_for_species(&Species::Dog, 12.0),
            LifeStage::Elder
        );
        // Human: 100 years -> Elder
        assert_eq!(
            LifeStage::from_age_years_for_species(&Species::Human, 100.0),
            LifeStage::Elder
        );
    }

    #[test]
    fn chimpanzee_at_maturity_is_young_adult() {
        // Chimpanzee maturity is 13 years
        // scale_factor = 25 / 13 = 1.92
        // 13.0 * 1.92 = 25 -> YoungAdult
        let stage = LifeStage::from_age_years_for_species(&Species::Chimpanzee, 13.0);
        assert_eq!(stage, LifeStage::YoungAdult);
    }

    #[test]
    fn horse_at_maturity_is_young_adult() {
        // Horse maturity is 4 years
        // scale_factor = 25 / 4 = 6.25
        // 4.0 * 6.25 = 25 -> YoungAdult
        let stage = LifeStage::from_age_years_for_species(&Species::Horse, 4.0);
        assert_eq!(stage, LifeStage::YoungAdult);
    }

    #[test]
    fn dolphin_at_maturity_is_young_adult() {
        // Dolphin maturity is 8 years
        // scale_factor = 25 / 8 = 3.125
        // 8.0 * 3.125 = 25 -> YoungAdult
        let stage = LifeStage::from_age_years_for_species(&Species::Dolphin, 8.0);
        assert_eq!(stage, LifeStage::YoungAdult);
    }

    #[test]
    fn crow_at_maturity_is_young_adult() {
        // Crow maturity is 2 years
        // scale_factor = 25 / 2 = 12.5
        // 2.0 * 12.5 = 25 -> YoungAdult
        let stage = LifeStage::from_age_years_for_species(&Species::Crow, 2.0);
        assert_eq!(stage, LifeStage::YoungAdult);
    }

    #[test]
    fn species_aware_boundary_child_to_adolescent() {
        // Test the boundary between Child (0-12) and Adolescent (13-17)
        // For Dog (maturity 2), boundary is at 12/12.5 = 0.96 years
        // At 0.96 years: 0.96 * 12.5 = 12 -> Child
        // At 1.04 years: 1.04 * 12.5 = 13 -> Adolescent
        assert_eq!(
            LifeStage::from_age_years_for_species(&Species::Dog, 0.96),
            LifeStage::Child
        );
        assert_eq!(
            LifeStage::from_age_years_for_species(&Species::Dog, 1.04),
            LifeStage::Adolescent
        );
    }

    #[test]
    fn species_aware_boundary_adolescent_to_young_adult() {
        // For Dog, boundary between Adolescent (13-17) and YoungAdult (18-30)
        // At 17/12.5 = 1.36 years: still Adolescent
        // At 18/12.5 = 1.44 years: YoungAdult
        assert_eq!(
            LifeStage::from_age_years_for_species(&Species::Dog, 1.36),
            LifeStage::Adolescent
        );
        assert_eq!(
            LifeStage::from_age_years_for_species(&Species::Dog, 1.44),
            LifeStage::YoungAdult
        );
    }
}
