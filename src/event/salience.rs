//! Arousal-modulated salience computation for memory encoding.
//!
//! When events are processed, their memorability (salience) is modulated
//! by the entity's arousal level at encoding time. This follows research
//! on emotional memory enhancement (McGaugh, Cahill, Kensinger).

use crate::enums::{EventCategory, Species};

/// Arousal weight for humans (moderate enhancement).
pub const AROUSAL_WEIGHT_HUMAN: f32 = 0.3;

/// Arousal weight for animals (higher enhancement due to survival-focus).
pub const AROUSAL_WEIGHT_ANIMAL: f32 = 0.4;

/// Arousal weight for robotic/stateless entities (no enhancement).
pub const AROUSAL_WEIGHT_ROBOTIC: f32 = 0.0;

/// Arousal threshold below which no enhancement occurs.
pub const AROUSAL_THRESHOLD: f32 = 0.2;

/// Arousal ceiling above which encoding is impaired (Yerkes-Dodson).
pub const AROUSAL_CEILING: f32 = 0.9;

/// Impairment factor for extreme arousal.
pub const EXTREME_AROUSAL_IMPAIRMENT: f32 = 0.3;

/// Multiplier for negative valence events (negativity bias).
pub const NEGATIVITY_BIAS_MULTIPLIER: f32 = 1.1;

/// Computes the arousal weight for a given species.
///
/// Different species have different arousal-memory coupling:
/// - Humans: moderate (0.3)
/// - Animals (Dog, Cat, etc.): higher (0.4-0.5)
/// - Robotic: none (0.0)
///
/// # Arguments
///
/// * `species` - The entity's species
///
/// # Returns
///
/// The arousal weight (0.0 to 1.0)
#[must_use]
pub fn arousal_weight_for_species(species: &Species) -> f32 {
    match species {
        Species::Human => AROUSAL_WEIGHT_HUMAN,
        Species::Dog | Species::Cat | Species::Mouse => AROUSAL_WEIGHT_ANIMAL,
        Species::Elephant | Species::Chimpanzee | Species::Dolphin | Species::Crow => {
            AROUSAL_WEIGHT_HUMAN
        }
        Species::Horse => AROUSAL_WEIGHT_ANIMAL,
        Species::Custom { .. } => AROUSAL_WEIGHT_HUMAN,
    }
}

/// Computes arousal-modulated salience for memory encoding.
///
/// The formula follows emotional memory research:
/// 1. Arousal below threshold (0.2) has no effect
/// 2. Moderate arousal enhances encoding linearly
/// 3. Extreme arousal (>0.9) impairs encoding (Yerkes-Dodson)
/// 4. Negative events get a 1.1x boost (negativity bias)
///
/// Exception: Trauma events (AC-building) bypass Yerkes-Dodson impairment
/// because painful experiences accumulate capability regardless of
/// dissociation.
///
/// # Arguments
///
/// * `base_salience` - The initial salience before modulation
/// * `arousal` - The entity's arousal at encoding (-1.0 to 1.0)
/// * `valence` - The event's valence (-1.0 to 1.0)
/// * `event_category` - The event's category for trauma check
/// * `species` - The entity's species for weight lookup
///
/// # Returns
///
/// The modulated salience (0.0 to 1.0)
///
/// # Examples
///
/// ```
/// use behavioral_pathways::event::compute_arousal_modulated_salience;
/// use behavioral_pathways::enums::{EventCategory, Species};
///
/// // High arousal enhances memory
/// let salience = compute_arousal_modulated_salience(
///     0.5, 0.7, 0.0, EventCategory::Social, &Species::Human
/// );
/// assert!(salience > 0.5);
///
/// // Low arousal has no effect
/// let low_arousal = compute_arousal_modulated_salience(
///     0.5, 0.1, 0.0, EventCategory::Social, &Species::Human
/// );
/// assert!((low_arousal - 0.5).abs() < 0.01);
/// ```
#[must_use]
pub fn compute_arousal_modulated_salience(
    base_salience: f32,
    arousal: f32,
    valence: f32,
    event_category: EventCategory,
    species: &Species,
) -> f32 {
    let arousal_weight = arousal_weight_for_species(species);

    // Use absolute arousal (both excitement and fear enhance consolidation)
    let effective_arousal = arousal.abs();

    // Below threshold: no effect
    if effective_arousal < AROUSAL_THRESHOLD {
        return apply_negativity_bias(base_salience, valence);
    }

    // Check for Yerkes-Dodson impairment
    let is_trauma = event_category == EventCategory::Trauma;

    if effective_arousal > AROUSAL_CEILING && !is_trauma {
        // Extreme arousal impairs encoding (except for trauma)
        let impaired = base_salience * (1.0 - EXTREME_AROUSAL_IMPAIRMENT);
        return apply_negativity_bias(impaired.clamp(0.0, 1.0), valence);
    }

    // Standard enhancement formula:
    // final_salience = base_salience + (effective_arousal * arousal_weight * (1 - base_salience))
    // This scales enhancement by how much "room" there is to improve
    let room_for_improvement = 1.0 - base_salience;
    let enhancement = effective_arousal * arousal_weight * room_for_improvement;
    let enhanced = base_salience + enhancement;

    apply_negativity_bias(enhanced.clamp(0.0, 1.0), valence)
}

/// Applies negativity bias to salience.
///
/// Negative events (valence < 0) get a small boost to salience,
/// reflecting the well-documented negativity bias in memory.
fn apply_negativity_bias(salience: f32, valence: f32) -> f32 {
    if valence < 0.0 {
        (salience * NEGATIVITY_BIAS_MULTIPLIER).clamp(0.0, 1.0)
    } else {
        salience
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arousal_weight_human() {
        let weight = arousal_weight_for_species(&Species::Human);
        assert!((weight - AROUSAL_WEIGHT_HUMAN).abs() < f32::EPSILON);
    }

    #[test]
    fn arousal_weight_animal() {
        let dog_weight = arousal_weight_for_species(&Species::Dog);
        assert!((dog_weight - AROUSAL_WEIGHT_ANIMAL).abs() < f32::EPSILON);

        let cat_weight = arousal_weight_for_species(&Species::Cat);
        assert!((cat_weight - AROUSAL_WEIGHT_ANIMAL).abs() < f32::EPSILON);
    }

    #[test]
    fn arousal_weight_horse_uses_animal_weight() {
        let horse_weight = arousal_weight_for_species(&Species::Horse);
        assert!((horse_weight - AROUSAL_WEIGHT_ANIMAL).abs() < f32::EPSILON);
    }

    #[test]
    fn arousal_weight_custom_uses_human() {
        let custom = Species::custom("Custom", 80, 25, 1.0);
        let weight = arousal_weight_for_species(&custom);
        assert!((weight - AROUSAL_WEIGHT_HUMAN).abs() < f32::EPSILON);
    }

    #[test]
    fn arousal_boosts_salience() {
        let salience = compute_arousal_modulated_salience(
            0.5,
            0.7,
            0.0,
            EventCategory::Social,
            &Species::Human,
        );
        // Should be boosted above base
        assert!(salience > 0.5);
    }

    #[test]
    fn arousal_threshold_below_no_effect() {
        let salience = compute_arousal_modulated_salience(
            0.5,
            0.1, // Below threshold
            0.0,
            EventCategory::Social,
            &Species::Human,
        );
        // Should be unchanged
        assert!((salience - 0.5).abs() < 0.01);
    }

    #[test]
    fn arousal_ceiling_impairs_encoding() {
        let normal = compute_arousal_modulated_salience(
            0.5,
            0.7,
            0.0,
            EventCategory::Social,
            &Species::Human,
        );

        let extreme = compute_arousal_modulated_salience(
            0.5,
            0.95, // Above ceiling
            0.0,
            EventCategory::Social,
            &Species::Human,
        );

        // Extreme arousal should impair (lower salience than normal enhancement)
        assert!(extreme < normal);
    }

    #[test]
    fn trauma_events_bypass_yerkes_dodson() {
        let non_trauma = compute_arousal_modulated_salience(
            0.5,
            0.95,
            0.0,
            EventCategory::Social,
            &Species::Human,
        );

        let trauma = compute_arousal_modulated_salience(
            0.5,
            0.95,
            0.0,
            EventCategory::Trauma,
            &Species::Human,
        );

        // Trauma should not be impaired - should be higher than non-trauma
        assert!(trauma > non_trauma);
    }

    #[test]
    fn negativity_bias_increases_salience() {
        let neutral = compute_arousal_modulated_salience(
            0.5,
            0.5,
            0.0, // Neutral valence
            EventCategory::Social,
            &Species::Human,
        );

        let negative = compute_arousal_modulated_salience(
            0.5,
            0.5,
            -0.5, // Negative valence
            EventCategory::Social,
            &Species::Human,
        );

        // Negative events get a boost
        assert!(negative > neutral);
    }

    #[test]
    fn entity_model_arousal_weight() {
        // Human
        let human_salience = compute_arousal_modulated_salience(
            0.5,
            0.6,
            0.0,
            EventCategory::Social,
            &Species::Human,
        );

        // Dog (higher weight)
        let dog_salience =
            compute_arousal_modulated_salience(0.5, 0.6, 0.0, EventCategory::Social, &Species::Dog);

        // Dog should have higher enhancement due to higher weight
        assert!(dog_salience > human_salience);
    }

    #[test]
    fn salience_clamped_to_one() {
        // High base salience + high arousal should still be clamped
        let salience = compute_arousal_modulated_salience(
            0.9,
            0.8,
            -0.5, // Add negativity bias too
            EventCategory::Social,
            &Species::Human,
        );

        assert!(salience <= 1.0);
    }

    #[test]
    fn salience_clamped_to_zero() {
        // Extreme impairment should not go negative
        let salience = compute_arousal_modulated_salience(
            0.1,
            0.95,
            0.0,
            EventCategory::Social,
            &Species::Human,
        );

        assert!(salience >= 0.0);
    }

    #[test]
    fn negative_arousal_uses_absolute() {
        // Negative arousal (deactivation) should still enhance when absolute > threshold
        let salience = compute_arousal_modulated_salience(
            0.5,
            -0.7, // Negative arousal
            0.0,
            EventCategory::Social,
            &Species::Human,
        );

        // Should still be enhanced because abs(-0.7) = 0.7 > threshold
        assert!(salience > 0.5);
    }

    #[test]
    fn constants_have_expected_values() {
        assert!((AROUSAL_WEIGHT_HUMAN - 0.3).abs() < f32::EPSILON);
        assert!((AROUSAL_WEIGHT_ANIMAL - 0.4).abs() < f32::EPSILON);
        assert!(AROUSAL_WEIGHT_ROBOTIC.abs() < f32::EPSILON);
        assert!((AROUSAL_THRESHOLD - 0.2).abs() < f32::EPSILON);
        assert!((AROUSAL_CEILING - 0.9).abs() < f32::EPSILON);
        assert!((NEGATIVITY_BIAS_MULTIPLIER - 1.1).abs() < f32::EPSILON);
    }

    #[test]
    fn elephant_uses_human_weight() {
        let weight = arousal_weight_for_species(&Species::Elephant);
        assert!((weight - AROUSAL_WEIGHT_HUMAN).abs() < f32::EPSILON);
    }

    #[test]
    fn chimpanzee_uses_human_weight() {
        let weight = arousal_weight_for_species(&Species::Chimpanzee);
        assert!((weight - AROUSAL_WEIGHT_HUMAN).abs() < f32::EPSILON);
    }
}
