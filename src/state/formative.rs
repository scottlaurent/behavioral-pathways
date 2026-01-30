//! Formative events and personality base shifts.
//!
//! This module implements the formative events system that allows permanent
//! personality trait shifts based on significant life events. Grounded in
//! HEXACO personality theory and empirical research on personality change.
//!
//! # Theory Background
//!
//! Key researchers and theories:
//! - Roberts (Social Investment Theory): Personality changes through role investment
//! - Tedeschi & Calhoun (Post-Traumatic Growth): Trauma can shift traits
//! - Caspi (Cumulative Continuity): Age-based plasticity effects
//! - Bleidorn: Event-specific effect sizes
//!
//! # Key Concepts
//!
//! - **Trait Stability**: Each HEXACO trait has a resistance to change (0.60-0.85)
//! - **Age Plasticity**: Younger individuals show more personality change
//! - **Sensitive Periods**: Trait-specific windows of heightened plasticity
//! - **Diminishing Returns**: Repeated shifts approach asymptotic limits
//! - **Partial Recovery**: Severe shifts partially settle over time

use crate::enums::{HexacoPath, Species};
use crate::types::Duration;
use serde::{Deserialize, Serialize};

/// Maximum magnitude for a single formative event's base shift.
pub const MAX_SINGLE_EVENT_SHIFT: f32 = 0.30;

/// Threshold above which shifts experience partial recovery.
pub const SEVERE_SHIFT_THRESHOLD: f32 = 0.20;

/// Retention rate for severe shifts after settling (70%).
pub const SEVERE_SHIFT_RETENTION: f32 = 0.70;

/// Days for severe shifts to settle to their final value.
pub const SETTLING_DAYS: u32 = 180;

/// Saturation constant for diminishing returns calculation.
pub const SATURATION_CONSTANT: f32 = 0.50;

/// Cumulative maximum shift in any direction per trait.
pub const CUMULATIVE_CAP: f32 = 1.0;

/// A record of a personality base shift from a formative event.
///
/// Base shifts are stored as timestamped records rather than modifying
/// the anchor state directly. The effective base at any timestamp is
/// computed by summing applicable shifts.
///
/// # Settling Behavior
///
/// Shifts exceeding [`SEVERE_SHIFT_THRESHOLD`] have an immediate impact
/// that gradually settles to a lower permanent value over [`SETTLING_DAYS`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseShiftRecord {
    /// When the shift occurred.
    timestamp: Duration,

    /// Which HEXACO trait was shifted.
    trait_path: HexacoPath,

    /// Initial shift magnitude (what happens immediately).
    immediate: f32,

    /// Final settled magnitude (for severe shifts).
    settled: f32,

    /// Days to settle from immediate to settled (0 if no settling).
    settling_days: u32,
}

impl BaseShiftRecord {
    /// Creates a new base shift record.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - When the shift occurred
    /// * `trait_path` - Which HEXACO trait to shift
    /// * `shift_amount` - Raw shift amount (will be processed for settling)
    ///
    /// Shifts exceeding [`SEVERE_SHIFT_THRESHOLD`] will have settling behavior.
    #[must_use]
    pub fn new(timestamp: Duration, trait_path: HexacoPath, shift_amount: f32) -> Self {
        let abs_shift = shift_amount.abs();
        let is_severe = abs_shift > SEVERE_SHIFT_THRESHOLD;

        let (settled, settling_days) = if is_severe {
            (shift_amount * SEVERE_SHIFT_RETENTION, SETTLING_DAYS)
        } else {
            (shift_amount, 0)
        };

        BaseShiftRecord {
            timestamp,
            trait_path,
            immediate: shift_amount,
            settled,
            settling_days,
        }
    }

    /// Returns the timestamp when this shift occurred.
    #[must_use]
    pub fn timestamp(&self) -> Duration {
        self.timestamp
    }

    /// Returns which trait this shift affects.
    #[must_use]
    pub fn trait_path(&self) -> HexacoPath {
        self.trait_path
    }

    /// Returns the immediate shift magnitude.
    #[must_use]
    pub fn immediate(&self) -> f32 {
        self.immediate
    }

    /// Returns the final settled magnitude.
    #[must_use]
    pub fn settled(&self) -> f32 {
        self.settled
    }

    /// Returns the number of days to settle.
    #[must_use]
    pub fn settling_days(&self) -> u32 {
        self.settling_days
    }

    /// Returns true if this is a severe shift with settling behavior.
    #[must_use]
    pub fn is_severe(&self) -> bool {
        self.settling_days > 0
    }

    /// Computes the contribution of this shift at a given query timestamp.
    ///
    /// For non-severe shifts, returns the immediate value.
    /// For severe shifts, interpolates between immediate and settled based on time.
    ///
    /// Returns 0.0 if the query timestamp is before this shift occurred.
    #[must_use]
    pub fn contribution_at(&self, query_timestamp: Duration) -> f32 {
        // Shift hasn't happened yet
        if query_timestamp < self.timestamp {
            return 0.0;
        }

        // Non-severe shifts return immediate value
        if self.settling_days == 0 {
            return self.immediate;
        }

        // Severe shift: interpolate based on time elapsed
        let days_since = (query_timestamp - self.timestamp).as_days();
        let settling_days_u64 = u64::from(self.settling_days);

        // Fully settled
        if days_since >= settling_days_u64 {
            return self.settled;
        }

        // Interpolate: immediate -> settled over settling period
        let progress = days_since as f32 / self.settling_days as f32;
        let change = self.immediate - self.settled;
        self.immediate - (change * progress)
    }
}

/// Returns the stability coefficient for a HEXACO trait.
///
/// Higher values indicate greater resistance to change.
///
/// | Trait | Stability | Description |
/// |-------|-----------|-------------|
/// | Extraversion | 0.85 | Hardest to shift |
/// | Openness | 0.80 | Hard |
/// | Honesty-Humility | 0.75 | Moderate |
/// | Conscientiousness | 0.70 | Moderate |
/// | Agreeableness | 0.65 | Easier |
/// | Neuroticism | 0.60 | Easiest |
#[must_use]
pub fn stability_coefficient(trait_path: HexacoPath) -> f32 {
    match trait_path {
        HexacoPath::Extraversion => 0.85,
        HexacoPath::Openness => 0.80,
        HexacoPath::HonestyHumility => 0.75,
        HexacoPath::Conscientiousness => 0.70,
        HexacoPath::Agreeableness => 0.65,
        HexacoPath::Neuroticism => 0.60,
    }
}

/// Returns the trait modifier for personality change.
///
/// This is `1.0 - stability_coefficient`, representing how much
/// a trait can be influenced (inverse of resistance).
#[must_use]
pub fn trait_modifier(trait_path: HexacoPath) -> f32 {
    1.0 - stability_coefficient(trait_path)
}

/// Returns the age plasticity modifier for a given age.
///
/// | Age Range | Modifier |
/// |-----------|----------|
/// | < 18 | 1.3 |
/// | 18-29 | 1.0 (reference) |
/// | 30-49 | 0.8 |
/// | 50-69 | 0.7 |
/// | 70+ | 0.6 (soft floor) |
#[must_use]
pub fn age_plasticity(age_years: u16) -> f32 {
    match age_years {
        0..=17 => 1.3,
        18..=29 => 1.0,
        30..=49 => 0.8,
        50..=69 => 0.7,
        _ => 0.6, // 70+
    }
}

/// Returns the sensitive period modifier for a trait at a given age.
///
/// During sensitive periods, traits are more malleable.
///
/// | Trait | Sensitive Period | Multiplier |
/// |-------|------------------|------------|
/// | Neuroticism | 12-25 | 1.4x |
/// | Conscientiousness | 18-35 | 1.2x |
/// | Agreeableness | 25-40 | 1.2x |
/// | Extraversion | 13-22 | 1.2x |
/// | Openness | 15-30 | 1.2x |
/// | Honesty-Humility | 18-30 | 1.2x |
///
/// Returns 1.0 outside sensitive periods.
#[must_use]
pub fn sensitive_period_modifier(trait_path: HexacoPath, age_years: u16) -> f32 {
    let (start, end, multiplier) = sensitive_period_range(trait_path);

    if age_years >= start && age_years <= end {
        multiplier
    } else {
        1.0
    }
}

/// Returns (start_age, end_age, multiplier) for a trait's sensitive period.
#[must_use]
fn sensitive_period_range(trait_path: HexacoPath) -> (u16, u16, f32) {
    match trait_path {
        HexacoPath::Neuroticism => (12, 25, 1.4),
        HexacoPath::Conscientiousness => (18, 35, 1.2),
        HexacoPath::Agreeableness => (25, 40, 1.2),
        HexacoPath::Extraversion => (13, 22, 1.2),
        HexacoPath::Openness => (15, 30, 1.2),
        HexacoPath::HonestyHumility => (18, 30, 1.2),
    }
}

/// Returns the combined plasticity modifier for age and sensitive period.
///
/// Uses max (not multiplication) to prevent extreme stacking.
#[must_use]
pub fn combined_plasticity(trait_path: HexacoPath, age_years: u16) -> f32 {
    let age_mod = age_plasticity(age_years);
    let sensitive_mod = sensitive_period_modifier(trait_path, age_years);
    age_mod.max(sensitive_mod)
}

/// Computes the saturation factor for diminishing returns.
///
/// As cumulative shifts grow, additional shifts have less effect.
/// Formula: 1.0 / (1.0 + existing / SATURATION_CONSTANT)
#[must_use]
pub fn saturation_factor(existing_cumulative: f32) -> f32 {
    1.0 / (1.0 + existing_cumulative / SATURATION_CONSTANT)
}

/// Applies all formative modifiers to a raw shift request.
///
/// # Arguments
///
/// * `shift_request` - The raw shift amount requested
/// * `trait_path` - Which trait is being shifted
/// * `age_years` - Entity's age at the time of the shift
/// * `existing_cumulative` - Sum of existing shifts in the same direction
/// * `species` - Entity's species (affects base plasticity)
///
/// # Returns
///
/// The modified shift amount after applying all constraints.
#[must_use]
pub fn apply_formative_modifiers(
    shift_request: f32,
    trait_path: HexacoPath,
    age_years: u16,
    existing_cumulative: f32,
    species: &Species,
) -> f32 {
    // 1. Base plasticity from species
    let species_plasticity = species_plasticity_modifier(species);

    // 2. Combined age/sensitive period plasticity
    let plasticity = combined_plasticity(trait_path, age_years);

    // 3. Trait modifier (inverse of stability)
    let trait_mod = trait_modifier(trait_path);

    // 4. Saturation from existing shifts
    let saturation = saturation_factor(existing_cumulative);

    // 5. Apply all modifiers
    let modified = shift_request * species_plasticity * plasticity * trait_mod * saturation;

    // 6. Single-event cap
    let capped = modified.clamp(-MAX_SINGLE_EVENT_SHIFT, MAX_SINGLE_EVENT_SHIFT);

    // 7. Cumulative cap enforcement
    enforce_cumulative_cap(capped, existing_cumulative)
}

/// Returns the base plasticity modifier for a species.
///
/// | Species Type | Modifier |
/// |--------------|----------|
/// | Human | 1.0 (reference) |
/// | Animal | 1.2 (higher base plasticity) |
/// | Custom | Based on social complexity |
#[must_use]
pub fn species_plasticity_modifier(species: &Species) -> f32 {
    match species {
        Species::Human => 1.0,
        Species::Custom { social_complexity, .. } => {
            // Scale based on social complexity
            0.8 + (*social_complexity * 0.4)
        }
        // All animals have 1.2x base plasticity
        _ => 1.2,
    }
}

/// Enforces the cumulative cap, preventing shifts that would exceed the limit.
#[must_use]
fn enforce_cumulative_cap(proposed_shift: f32, existing_cumulative: f32) -> f32 {
    let new_cumulative = existing_cumulative + proposed_shift.abs();

    if new_cumulative > CUMULATIVE_CAP {
        let remaining = CUMULATIVE_CAP - existing_cumulative;
        proposed_shift.signum() * remaining.max(0.0)
    } else {
        proposed_shift
    }
}

/// Computes the effective base value for a trait at a given timestamp.
///
/// This is the core function for querying personality state. It sums
/// all applicable base shifts with the anchor value.
///
/// # Arguments
///
/// * `anchor_value` - The original personality trait value
/// * `shifts` - All base shift records for this trait
/// * `query_timestamp` - The timestamp to compute the value for
///
/// # Returns
///
/// The effective base value, clamped to -1.0 to 1.0.
#[must_use]
pub fn effective_base_at(
    anchor_value: f32,
    shifts: &[BaseShiftRecord],
    query_timestamp: Duration,
) -> f32 {
    let total_shift: f32 = shifts
        .iter()
        .map(|shift| shift.contribution_at(query_timestamp))
        .sum();

    (anchor_value + total_shift).clamp(-1.0, 1.0)
}

/// Computes cumulative shifts in a specific direction (positive or negative).
///
/// Used for diminishing returns calculation which tracks directions independently.
#[must_use]
pub fn cumulative_in_direction(shifts: &[BaseShiftRecord], positive: bool) -> f32 {
    shifts
        .iter()
        .filter(|s| (s.settled > 0.0) == positive)
        .map(|s| s.settled.abs())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    // BaseShiftRecord tests

    #[test]
    fn base_shift_record_new_non_severe() {
        let shift = BaseShiftRecord::new(
            Duration::seconds(0),
            HexacoPath::Agreeableness,
            0.15,
        );

        assert!((shift.immediate() - 0.15).abs() < f32::EPSILON);
        assert!((shift.settled() - 0.15).abs() < f32::EPSILON);
        assert_eq!(shift.settling_days(), 0);
        assert!(!shift.is_severe());
    }

    #[test]
    fn base_shift_record_new_severe() {
        let shift = BaseShiftRecord::new(
            Duration::seconds(0),
            HexacoPath::Neuroticism,
            0.25,
        );

        assert!((shift.immediate() - 0.25).abs() < f32::EPSILON);
        assert!((shift.settled() - 0.175).abs() < f32::EPSILON); // 0.25 * 0.70
        assert_eq!(shift.settling_days(), SETTLING_DAYS);
        assert!(shift.is_severe());
    }

    #[test]
    fn base_shift_record_new_severe_negative() {
        let shift = BaseShiftRecord::new(
            Duration::seconds(0),
            HexacoPath::Agreeableness,
            -0.25,
        );

        assert!((shift.immediate() - (-0.25)).abs() < f32::EPSILON);
        assert!((shift.settled() - (-0.175)).abs() < f32::EPSILON);
        assert!(shift.is_severe());
    }

    #[test]
    fn base_shift_record_at_threshold_not_severe() {
        let shift = BaseShiftRecord::new(
            Duration::seconds(0),
            HexacoPath::Openness,
            SEVERE_SHIFT_THRESHOLD,
        );

        assert!(!shift.is_severe());
    }

    #[test]
    fn base_shift_record_accessors() {
        let ts = Duration::seconds(1000);
        let shift = BaseShiftRecord::new(ts, HexacoPath::Extraversion, 0.10);

        assert_eq!(shift.timestamp(), ts);
        assert_eq!(shift.trait_path(), HexacoPath::Extraversion);
    }

    #[test]
    fn contribution_before_shift_is_zero() {
        let shift = BaseShiftRecord::new(
            Duration::seconds(1000),
            HexacoPath::Agreeableness,
            0.15,
        );

        let contribution = shift.contribution_at(Duration::seconds(500));
        assert!(contribution.abs() < f32::EPSILON);
    }

    #[test]
    fn contribution_at_shift_time_is_immediate() {
        let shift = BaseShiftRecord::new(
            Duration::seconds(1000),
            HexacoPath::Agreeableness,
            0.15,
        );

        let contribution = shift.contribution_at(Duration::seconds(1000));
        assert!((contribution - 0.15).abs() < f32::EPSILON);
    }

    #[test]
    fn contribution_non_severe_always_immediate() {
        let shift = BaseShiftRecord::new(
            Duration::seconds(0),
            HexacoPath::Agreeableness,
            0.15,
        );

        let day_1 = Duration::seconds(86400);
        let day_100 = Duration::seconds(86400 * 100);

        assert!((shift.contribution_at(day_1) - 0.15).abs() < f32::EPSILON);
        assert!((shift.contribution_at(day_100) - 0.15).abs() < f32::EPSILON);
    }

    #[test]
    fn contribution_severe_settles_over_time() {
        let shift = BaseShiftRecord::new(
            Duration::seconds(0),
            HexacoPath::Neuroticism,
            0.25,
        );

        // At day 0: immediate (0.25)
        let at_start = shift.contribution_at(Duration::seconds(0));
        assert!((at_start - 0.25).abs() < f32::EPSILON);

        // At day 90 (halfway): midpoint between 0.25 and 0.175
        let day_90 = Duration::seconds(86400 * 90);
        let at_mid = shift.contribution_at(day_90);
        let expected_mid = 0.25 - (0.25 - 0.175) * 0.5;
        assert!((at_mid - expected_mid).abs() < 0.01);

        // At day 180+: settled (0.175)
        let day_200 = Duration::seconds(86400 * 200);
        let at_end = shift.contribution_at(day_200);
        assert!((at_end - 0.175).abs() < f32::EPSILON);
    }

    // Stability coefficient tests

    #[test]
    fn stability_coefficients_correct() {
        assert!((stability_coefficient(HexacoPath::Extraversion) - 0.85).abs() < f32::EPSILON);
        assert!((stability_coefficient(HexacoPath::Openness) - 0.80).abs() < f32::EPSILON);
        assert!((stability_coefficient(HexacoPath::HonestyHumility) - 0.75).abs() < f32::EPSILON);
        assert!((stability_coefficient(HexacoPath::Conscientiousness) - 0.70).abs() < f32::EPSILON);
        assert!((stability_coefficient(HexacoPath::Agreeableness) - 0.65).abs() < f32::EPSILON);
        assert!((stability_coefficient(HexacoPath::Neuroticism) - 0.60).abs() < f32::EPSILON);
    }

    #[test]
    fn trait_modifiers_are_inverse() {
        for path in HexacoPath::all() {
            let stability = stability_coefficient(path);
            let modifier = trait_modifier(path);
            assert!((stability + modifier - 1.0).abs() < f32::EPSILON);
        }
    }

    // Age plasticity tests

    #[test]
    fn age_plasticity_under_18() {
        assert!((age_plasticity(0) - 1.3).abs() < f32::EPSILON);
        assert!((age_plasticity(10) - 1.3).abs() < f32::EPSILON);
        assert!((age_plasticity(17) - 1.3).abs() < f32::EPSILON);
    }

    #[test]
    fn age_plasticity_18_to_29() {
        assert!((age_plasticity(18) - 1.0).abs() < f32::EPSILON);
        assert!((age_plasticity(25) - 1.0).abs() < f32::EPSILON);
        assert!((age_plasticity(29) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn age_plasticity_30_to_49() {
        assert!((age_plasticity(30) - 0.8).abs() < f32::EPSILON);
        assert!((age_plasticity(40) - 0.8).abs() < f32::EPSILON);
        assert!((age_plasticity(49) - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn age_plasticity_50_to_69() {
        assert!((age_plasticity(50) - 0.7).abs() < f32::EPSILON);
        assert!((age_plasticity(60) - 0.7).abs() < f32::EPSILON);
        assert!((age_plasticity(69) - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn age_plasticity_70_plus() {
        assert!((age_plasticity(70) - 0.6).abs() < f32::EPSILON);
        assert!((age_plasticity(80) - 0.6).abs() < f32::EPSILON);
        assert!((age_plasticity(100) - 0.6).abs() < f32::EPSILON);
    }

    // Sensitive period tests

    #[test]
    fn sensitive_period_neuroticism() {
        assert!((sensitive_period_modifier(HexacoPath::Neuroticism, 11) - 1.0).abs() < f32::EPSILON);
        assert!((sensitive_period_modifier(HexacoPath::Neuroticism, 12) - 1.4).abs() < f32::EPSILON);
        assert!((sensitive_period_modifier(HexacoPath::Neuroticism, 20) - 1.4).abs() < f32::EPSILON);
        assert!((sensitive_period_modifier(HexacoPath::Neuroticism, 25) - 1.4).abs() < f32::EPSILON);
        assert!((sensitive_period_modifier(HexacoPath::Neuroticism, 26) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn sensitive_period_conscientiousness() {
        assert!((sensitive_period_modifier(HexacoPath::Conscientiousness, 17) - 1.0).abs() < f32::EPSILON);
        assert!((sensitive_period_modifier(HexacoPath::Conscientiousness, 18) - 1.2).abs() < f32::EPSILON);
        assert!((sensitive_period_modifier(HexacoPath::Conscientiousness, 35) - 1.2).abs() < f32::EPSILON);
        assert!((sensitive_period_modifier(HexacoPath::Conscientiousness, 36) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn sensitive_period_agreeableness() {
        assert!((sensitive_period_modifier(HexacoPath::Agreeableness, 24) - 1.0).abs() < f32::EPSILON);
        assert!((sensitive_period_modifier(HexacoPath::Agreeableness, 25) - 1.2).abs() < f32::EPSILON);
        assert!((sensitive_period_modifier(HexacoPath::Agreeableness, 40) - 1.2).abs() < f32::EPSILON);
        assert!((sensitive_period_modifier(HexacoPath::Agreeableness, 41) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn sensitive_period_extraversion() {
        assert!((sensitive_period_modifier(HexacoPath::Extraversion, 12) - 1.0).abs() < f32::EPSILON);
        assert!((sensitive_period_modifier(HexacoPath::Extraversion, 13) - 1.2).abs() < f32::EPSILON);
        assert!((sensitive_period_modifier(HexacoPath::Extraversion, 22) - 1.2).abs() < f32::EPSILON);
        assert!((sensitive_period_modifier(HexacoPath::Extraversion, 23) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn sensitive_period_openness() {
        assert!((sensitive_period_modifier(HexacoPath::Openness, 14) - 1.0).abs() < f32::EPSILON);
        assert!((sensitive_period_modifier(HexacoPath::Openness, 15) - 1.2).abs() < f32::EPSILON);
        assert!((sensitive_period_modifier(HexacoPath::Openness, 30) - 1.2).abs() < f32::EPSILON);
        assert!((sensitive_period_modifier(HexacoPath::Openness, 31) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn sensitive_period_honesty_humility() {
        assert!((sensitive_period_modifier(HexacoPath::HonestyHumility, 17) - 1.0).abs() < f32::EPSILON);
        assert!((sensitive_period_modifier(HexacoPath::HonestyHumility, 18) - 1.2).abs() < f32::EPSILON);
        assert!((sensitive_period_modifier(HexacoPath::HonestyHumility, 30) - 1.2).abs() < f32::EPSILON);
        assert!((sensitive_period_modifier(HexacoPath::HonestyHumility, 31) - 1.0).abs() < f32::EPSILON);
    }

    // Combined plasticity tests

    #[test]
    fn combined_plasticity_uses_max() {
        // Age 15 with Neuroticism: age=1.3, sensitive=1.4, max=1.4
        let result = combined_plasticity(HexacoPath::Neuroticism, 15);
        assert!((result - 1.4).abs() < f32::EPSILON);

        // Age 25 with Agreeableness: age=1.0, sensitive=1.2, max=1.2
        let result2 = combined_plasticity(HexacoPath::Agreeableness, 25);
        assert!((result2 - 1.2).abs() < f32::EPSILON);

        // Age 50 with any trait outside sensitive: age=0.7, sensitive=1.0, max=1.0
        let result3 = combined_plasticity(HexacoPath::Openness, 50);
        assert!((result3 - 1.0).abs() < f32::EPSILON);
    }

    // Saturation tests

    #[test]
    fn saturation_factor_zero_existing() {
        let factor = saturation_factor(0.0);
        assert!((factor - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn saturation_factor_decreases_with_existing() {
        let factor_0 = saturation_factor(0.0);
        let factor_25 = saturation_factor(0.25);
        let factor_50 = saturation_factor(0.50);

        assert!(factor_25 < factor_0);
        assert!(factor_50 < factor_25);
    }

    #[test]
    fn saturation_factor_at_constant() {
        // At existing = SATURATION_CONSTANT (0.50), factor = 1/(1+1) = 0.5
        let factor = saturation_factor(SATURATION_CONSTANT);
        assert!((factor - 0.5).abs() < f32::EPSILON);
    }

    // Species plasticity tests

    #[test]
    fn species_plasticity_human() {
        assert!((species_plasticity_modifier(&Species::Human) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn species_plasticity_animals() {
        assert!((species_plasticity_modifier(&Species::Dog) - 1.2).abs() < f32::EPSILON);
        assert!((species_plasticity_modifier(&Species::Cat) - 1.2).abs() < f32::EPSILON);
        assert!((species_plasticity_modifier(&Species::Elephant) - 1.2).abs() < f32::EPSILON);
    }

    #[test]
    fn species_plasticity_custom() {
        let custom = Species::custom("Test", 50, 5, 0.5);
        // 0.8 + (0.5 * 0.4) = 1.0
        assert!((species_plasticity_modifier(&custom) - 1.0).abs() < f32::EPSILON);

        let custom_high = Species::custom("HighSocial", 50, 5, 1.0);
        // 0.8 + (1.0 * 0.4) = 1.2
        assert!((species_plasticity_modifier(&custom_high) - 1.2).abs() < f32::EPSILON);
    }

    // Cumulative cap tests

    #[test]
    fn enforce_cap_within_limit() {
        let result = enforce_cumulative_cap(0.2, 0.5);
        assert!((result - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn enforce_cap_at_limit() {
        let result = enforce_cumulative_cap(0.3, 0.8);
        // Would exceed cap, reduced to remaining (0.2)
        assert!((result - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn enforce_cap_negative_at_limit() {
        let result = enforce_cumulative_cap(-0.3, 0.8);
        assert!((result - (-0.2)).abs() < f32::EPSILON);
    }

    #[test]
    fn enforce_cap_already_at_cap() {
        let result = enforce_cumulative_cap(0.1, CUMULATIVE_CAP);
        assert!(result.abs() < f32::EPSILON);
    }

    // Apply formative modifiers tests

    #[test]
    fn apply_modifiers_human_reference() {
        let result = apply_formative_modifiers(
            0.1,
            HexacoPath::Agreeableness,
            25,
            0.0,
            &Species::Human,
        );

        // 0.1 * 1.0 (species) * 1.2 (combined=max(age=1.0, sensitive=1.2)) * 0.35 (trait) * 1.0 (saturation) = 0.042
        assert!((result - 0.042).abs() < 0.001);
    }

    #[test]
    fn apply_modifiers_respects_single_event_cap() {
        let result = apply_formative_modifiers(
            1.0, // Very large request
            HexacoPath::Neuroticism,
            15, // High plasticity
            0.0,
            &Species::Human,
        );

        assert!(result.abs() <= MAX_SINGLE_EVENT_SHIFT);
    }

    #[test]
    fn apply_modifiers_respects_cumulative_cap() {
        let result = apply_formative_modifiers(
            1.0,
            HexacoPath::Neuroticism,
            15,
            0.95, // Already near cap
            &Species::Human,
        );

        // Should be limited by remaining room to cap
        assert!(result.abs() <= CUMULATIVE_CAP - 0.95 + f32::EPSILON);
    }

    // Effective base tests

    #[test]
    fn effective_base_no_shifts() {
        let result = effective_base_at(0.5, &[], Duration::seconds(1000));
        assert!((result - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn effective_base_single_shift() {
        let shifts = vec![BaseShiftRecord::new(
            Duration::seconds(0),
            HexacoPath::Agreeableness,
            0.1,
        )];

        let result = effective_base_at(0.5, &shifts, Duration::seconds(1000));
        assert!((result - 0.6).abs() < f32::EPSILON);
    }

    #[test]
    fn effective_base_multiple_shifts() {
        let shifts = vec![
            BaseShiftRecord::new(Duration::seconds(0), HexacoPath::Agreeableness, 0.1),
            BaseShiftRecord::new(Duration::seconds(100), HexacoPath::Agreeableness, -0.05),
        ];

        let result = effective_base_at(0.5, &shifts, Duration::seconds(1000));
        assert!((result - 0.55).abs() < f32::EPSILON);
    }

    #[test]
    fn effective_base_before_shift() {
        let shifts = vec![BaseShiftRecord::new(
            Duration::seconds(1000),
            HexacoPath::Agreeableness,
            0.1,
        )];

        let result = effective_base_at(0.5, &shifts, Duration::seconds(500));
        assert!((result - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn effective_base_clamped_high() {
        let shifts = vec![BaseShiftRecord::new(
            Duration::seconds(0),
            HexacoPath::Agreeableness,
            0.8,
        )];

        let result = effective_base_at(0.9, &shifts, Duration::seconds(1000));
        assert!((result - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn effective_base_clamped_low() {
        let shifts = vec![BaseShiftRecord::new(
            Duration::seconds(0),
            HexacoPath::Agreeableness,
            -0.8,
        )];

        let result = effective_base_at(-0.9, &shifts, Duration::seconds(1000));
        assert!((result - (-1.0)).abs() < f32::EPSILON);
    }

    // Cumulative in direction tests

    #[test]
    fn cumulative_in_direction_positive() {
        let shifts = vec![
            BaseShiftRecord::new(Duration::seconds(0), HexacoPath::Agreeableness, 0.1),
            BaseShiftRecord::new(Duration::seconds(100), HexacoPath::Agreeableness, 0.15),
            BaseShiftRecord::new(Duration::seconds(200), HexacoPath::Agreeableness, -0.05),
        ];

        let positive_sum = cumulative_in_direction(&shifts, true);
        assert!((positive_sum - 0.25).abs() < f32::EPSILON);
    }

    #[test]
    fn cumulative_in_direction_negative() {
        let shifts = vec![
            BaseShiftRecord::new(Duration::seconds(0), HexacoPath::Agreeableness, 0.1),
            BaseShiftRecord::new(Duration::seconds(100), HexacoPath::Agreeableness, -0.15),
            BaseShiftRecord::new(Duration::seconds(200), HexacoPath::Agreeableness, -0.05),
        ];

        let negative_sum = cumulative_in_direction(&shifts, false);
        assert!((negative_sum - 0.20).abs() < f32::EPSILON);
    }

    #[test]
    fn cumulative_in_direction_empty() {
        let positive_sum = cumulative_in_direction(&[], true);
        assert!(positive_sum.abs() < f32::EPSILON);
    }

    // Clone and serialize tests

    #[test]
    fn base_shift_record_clone() {
        let shift = BaseShiftRecord::new(
            Duration::seconds(1000),
            HexacoPath::Openness,
            0.15,
        );
        let cloned = shift.clone();
        assert_eq!(shift, cloned);
    }

    #[test]
    fn base_shift_record_debug() {
        let shift = BaseShiftRecord::new(
            Duration::seconds(0),
            HexacoPath::Extraversion,
            0.1,
        );
        let debug = format!("{:?}", shift);
        assert!(debug.contains("BaseShiftRecord"));
    }
}
