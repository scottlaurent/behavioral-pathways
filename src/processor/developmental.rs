//! Developmental modifiers for event processing.
//!
//! This module applies developmental effects to event interpretation during
//! temporal state computation. Effects include:
//!
//! - Life stage plasticity modifiers (children more plastic than adults)
//! - Sensitive period amplification (attachment events amplified in childhood)
//! - Turning point effects (temporary plasticity increases)
//!
//! # Design
//!
//! All functions in this module are internal-only. Consumers interact with
//! developmental effects implicitly through `state_at()` - they don't call
//! these functions directly.
//!
//! # Theoretical Foundation
//!
//! Based on Erikson's psychosocial stages and personality development research:
//! - Plasticity decreases with age (personality crystallizes by ~25)
//! - Sensitive periods amplify specific event categories at specific life stages
//! - Turning points temporarily increase plasticity (major life transitions)

use crate::context::TurningPoint;
use crate::entity::Entity;
use crate::enums::{DevelopmentalCategory, LifeStage};
use crate::event::Event;
use crate::types::Timestamp;

/// Constants for plasticity computation.
const PLASTICITY_MAX: f64 = 2.0;
const PLASTICITY_MIN: f64 = 0.5;
const PLASTICITY_DECAY_RATE: f64 = 0.023;

/// Constants for turning point boost computation.
const TURNING_POINT_MAX_BOOST: f64 = 0.5;
const TURNING_POINT_HALF_LIFE_DAYS: f64 = 180.0;
const TURNING_POINT_DECAY_CONSTANT: f64 = 0.693; // ln(2)

/// Days per year for age conversion.
const DAYS_PER_YEAR: f64 = 365.25;

/// Applies developmental modifiers to event impact.
///
/// This function modifies the raw event impact based on:
/// 1. Age-based plasticity (younger = higher impact)
/// 2. Sensitive period multipliers (specific events at specific ages)
/// 3. Turning point boosts (recent major life changes)
///
/// # Arguments
///
/// * `entity` - The entity being affected
/// * `event` - The event being processed
/// * `event_impact` - The base impact of the event
/// * `current_age_days` - The entity's current age in days
/// * `current_timestamp` - The absolute timestamp for the current state
///
/// # Returns
///
/// The modified impact after applying developmental modifiers.
///
/// # Formula
///
/// ```text
/// modified_impact = event_impact * (plasticity + turning_point_boost) * sensitive_period_multiplier
/// ```
#[must_use]
pub(crate) fn apply_developmental_effects(
    entity: &Entity,
    event: &Event,
    event_impact: f64,
    current_age_days: u64,
    current_timestamp: Timestamp,
) -> f64 {
    // Get species for life stage and time scale calculations
    let species = entity.species();
    let time_scale = f64::from(species.time_scale());

    // Convert age to human-equivalent years for plasticity calculation
    let age_days_f64 = current_age_days as f64;
    let age_years = (age_days_f64 / DAYS_PER_YEAR) * time_scale;

    // Compute life stage at event time (not anchor time)
    // Use raw age in years for species-aware life stage lookup
    let raw_age_years = age_days_f64 / DAYS_PER_YEAR;
    let life_stage = LifeStage::from_age_years_for_species(species, raw_age_years);

    // Compute plasticity modifier
    let plasticity = get_plasticity_modifier(&life_stage, age_years);

    // Compute turning point boost
    let turning_points = entity.context().chronosystem().turning_points();
    let turning_point_boost = get_turning_point_boost(turning_points, current_timestamp);

    // Compute sensitive period multiplier
    let category = DevelopmentalCategory::from(&event.event_type());
    let sensitive_multiplier = get_sensitive_period_multiplier(&life_stage, &category);

    // Apply all modifiers
    let effective_plasticity = plasticity + turning_point_boost;
    event_impact * effective_plasticity * sensitive_multiplier
}

/// Returns the plasticity modifier based on age.
///
/// Plasticity follows a continuous decreasing curve from 2.0 at birth
/// to 0.5 at age 65+. This reflects research showing personality
/// crystallizes with age.
///
/// # Formula
///
/// ```text
/// plasticity = max(0.5, 2.0 - (age_years * 0.023))
/// ```
///
/// # Arguments
///
/// * `_life_stage` - The life stage (unused, included for API consistency)
/// * `age_years` - Human-equivalent age in years
///
/// # Returns
///
/// Plasticity modifier in range [0.5, 2.0].
#[must_use]
pub(crate) fn get_plasticity_modifier(_life_stage: &LifeStage, age_years: f64) -> f64 {
    // Clamp age to non-negative
    let age = age_years.max(0.0);

    // Linear decay from 2.0 at birth to 0.5 at ~65 years
    let plasticity = PLASTICITY_MAX - (age * PLASTICITY_DECAY_RATE);

    plasticity.max(PLASTICITY_MIN)
}

/// Returns the sensitive period multiplier for a developmental category.
///
/// During sensitive periods, specific event categories have amplified effects.
/// For example, attachment events have 2.0x impact during childhood.
///
/// # Sensitive Periods
///
/// | Life Stage | Category | Multiplier |
/// |------------|----------|------------|
/// | Child | Attachment, Autonomy | 2.0x |
/// | Child | Initiative, Industry | 1.5x |
/// | Adolescent | Identity | 1.8x |
/// | YoungAdult | Intimacy | 1.3x |
/// | Adult | Generativity | 1.3x |
/// | MatureAdult | Generativity | 1.3x |
/// | Elder | Integrity | 1.2x |
///
/// # Arguments
///
/// * `life_stage` - The entity's current life stage
/// * `category` - The developmental category of the event
///
/// # Returns
///
/// Multiplier in range [1.0, 2.0]. Returns 1.0 if not in a sensitive period.
#[must_use]
pub(crate) fn get_sensitive_period_multiplier(
    life_stage: &LifeStage,
    category: &DevelopmentalCategory,
) -> f64 {
    match (life_stage, category) {
        // Child stage - attachment, autonomy, initiative, industry
        (LifeStage::Child, DevelopmentalCategory::Attachment) => 2.0,
        (LifeStage::Child, DevelopmentalCategory::Autonomy) => 2.0,
        (LifeStage::Child, DevelopmentalCategory::Initiative) => 1.5,
        (LifeStage::Child, DevelopmentalCategory::Industry) => 1.5,

        // Adolescent stage - identity
        (LifeStage::Adolescent, DevelopmentalCategory::Identity) => 1.8,

        // Young adult stage - intimacy
        (LifeStage::YoungAdult, DevelopmentalCategory::Intimacy) => 1.3,

        // Adult stage - generativity
        (LifeStage::Adult, DevelopmentalCategory::Generativity) => 1.3,

        // Mature adult stage - generativity continues
        (LifeStage::MatureAdult, DevelopmentalCategory::Generativity) => 1.3,

        // Elder stage - integrity
        (LifeStage::Elder, DevelopmentalCategory::Integrity) => 1.2,

        // All other combinations - no amplification
        _ => 1.0,
    }
}

/// Returns the temporary plasticity boost from recent turning points.
///
/// Turning points (major life events like marriage, divorce, job loss)
/// temporarily increase plasticity. The boost decays with a half-life
/// of 180 days (6 months).
///
/// # Formula
///
/// For each turning point:
/// ```text
/// boost = 0.5 * exp(-0.693 * days_since / 180)
/// ```
///
/// Total boost is the sum of individual boosts, capped at 0.5.
///
/// # Arguments
///
/// * `turning_points` - List of turning points from chronosystem
/// * `current_timestamp` - The absolute timestamp for the current state
///
/// # Returns
///
/// Boost in range [0.0, 0.5].
#[must_use]
pub(crate) fn get_turning_point_boost(
    turning_points: &[TurningPoint],
    current_timestamp: Timestamp,
) -> f64 {
    if turning_points.is_empty() {
        return 0.0;
    }

    let total_boost: f64 = turning_points
        .iter()
        .filter_map(|tp| {
            // Only consider turning points that occurred before current age
            if tp.timestamp <= current_timestamp {
                let days_since = (current_timestamp - tp.timestamp).as_days_f64();
                let decay_factor = (-TURNING_POINT_DECAY_CONSTANT * days_since
                    / TURNING_POINT_HALF_LIFE_DAYS)
                    .exp();
                Some(TURNING_POINT_MAX_BOOST * decay_factor)
            } else {
                None
            }
        })
        .sum();

    total_boost.min(TURNING_POINT_MAX_BOOST)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::{TurningPoint, TurningPointDomain};
    use crate::entity::EntityBuilder;
    use crate::enums::{EventType, Species};
    use crate::event::EventBuilder;
    use crate::types::{Duration, EventId, Timestamp};

    fn timestamp_for_days(days: u64) -> Timestamp {
        Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0) + Duration::days(days)
    }

    // === Plasticity Tests ===

    #[test]
    fn child_event_impact_higher_than_adult() {
        let child_plasticity = get_plasticity_modifier(&LifeStage::Child, 5.0);
        let adult_plasticity = get_plasticity_modifier(&LifeStage::Adult, 40.0);
        assert!(child_plasticity > adult_plasticity);
    }

    #[test]
    fn personality_crystallizes_by_age_25() {
        let age_0_plasticity = get_plasticity_modifier(&LifeStage::Child, 0.0);
        let age_25_plasticity = get_plasticity_modifier(&LifeStage::YoungAdult, 25.0);

        // At age 0: 2.0
        assert!((age_0_plasticity - 2.0).abs() < f64::EPSILON);

        // At age 25: 2.0 - (25 * 0.023) = 2.0 - 0.575 = 1.425
        let expected_25 = 2.0 - (25.0 * 0.023);
        assert!((age_25_plasticity - expected_25).abs() < 0.001);

        // Significant reduction (more than 25%)
        let reduction = (age_0_plasticity - age_25_plasticity) / age_0_plasticity;
        assert!(reduction > 0.25);
    }

    #[test]
    fn plasticity_modifier_returns_expected_range() {
        let test_ages = [0.0, 5.0, 12.0, 18.0, 25.0, 40.0, 65.0, 80.0, 100.0];
        for age in test_ages {
            let plasticity = get_plasticity_modifier(&LifeStage::Adult, age);
            assert!(plasticity >= 0.5 && plasticity <= 2.0);
        }
    }

    #[test]
    fn plasticity_at_age_zero_is_maximum() {
        let plasticity = get_plasticity_modifier(&LifeStage::Child, 0.0);
        assert!((plasticity - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn plasticity_at_age_65_is_minimum() {
        let plasticity = get_plasticity_modifier(&LifeStage::Elder, 65.0);
        // 2.0 - (65 * 0.023) = 2.0 - 1.495 = 0.505, but near the clamp
        assert!((plasticity - 0.505).abs() < 0.01);
    }

    #[test]
    fn plasticity_floors_at_minimum() {
        let plasticity_80 = get_plasticity_modifier(&LifeStage::Elder, 80.0);
        let plasticity_100 = get_plasticity_modifier(&LifeStage::Elder, 100.0);
        assert!((plasticity_80 - 0.5).abs() < f64::EPSILON);
        assert!((plasticity_100 - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn plasticity_negative_age_clamps_to_zero() {
        let plasticity = get_plasticity_modifier(&LifeStage::Child, -5.0);
        assert!((plasticity - 2.0).abs() < f64::EPSILON);
    }

    // === Sensitive Period Tests ===

    #[test]
    fn attachment_event_amplified_in_childhood() {
        let multiplier =
            get_sensitive_period_multiplier(&LifeStage::Child, &DevelopmentalCategory::Attachment);
        assert!((multiplier - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn identity_events_amplified_in_adolescence() {
        let multiplier = get_sensitive_period_multiplier(
            &LifeStage::Adolescent,
            &DevelopmentalCategory::Identity,
        );
        assert!((multiplier - 1.8).abs() < f64::EPSILON);
    }

    #[test]
    fn sensitive_period_returns_one_outside_period() {
        let adult_attachment =
            get_sensitive_period_multiplier(&LifeStage::Adult, &DevelopmentalCategory::Attachment);
        assert!((adult_attachment - 1.0).abs() < f64::EPSILON);

        let child_identity =
            get_sensitive_period_multiplier(&LifeStage::Child, &DevelopmentalCategory::Identity);
        assert!((child_identity - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn autonomy_amplified_in_childhood() {
        let multiplier =
            get_sensitive_period_multiplier(&LifeStage::Child, &DevelopmentalCategory::Autonomy);
        assert!((multiplier - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn initiative_amplified_in_childhood() {
        let multiplier =
            get_sensitive_period_multiplier(&LifeStage::Child, &DevelopmentalCategory::Initiative);
        assert!((multiplier - 1.5).abs() < f64::EPSILON);
    }

    #[test]
    fn industry_amplified_in_childhood() {
        let multiplier =
            get_sensitive_period_multiplier(&LifeStage::Child, &DevelopmentalCategory::Industry);
        assert!((multiplier - 1.5).abs() < f64::EPSILON);
    }

    #[test]
    fn intimacy_amplified_in_young_adult() {
        let multiplier = get_sensitive_period_multiplier(
            &LifeStage::YoungAdult,
            &DevelopmentalCategory::Intimacy,
        );
        assert!((multiplier - 1.3).abs() < f64::EPSILON);
    }

    #[test]
    fn generativity_amplified_in_adult() {
        let multiplier = get_sensitive_period_multiplier(
            &LifeStage::Adult,
            &DevelopmentalCategory::Generativity,
        );
        assert!((multiplier - 1.3).abs() < f64::EPSILON);
    }

    #[test]
    fn generativity_amplified_in_mature_adult() {
        let multiplier = get_sensitive_period_multiplier(
            &LifeStage::MatureAdult,
            &DevelopmentalCategory::Generativity,
        );
        assert!((multiplier - 1.3).abs() < f64::EPSILON);
    }

    #[test]
    fn integrity_amplified_in_elder() {
        let multiplier =
            get_sensitive_period_multiplier(&LifeStage::Elder, &DevelopmentalCategory::Integrity);
        assert!((multiplier - 1.2).abs() < f64::EPSILON);
    }

    #[test]
    fn neutral_category_returns_one() {
        for stage in LifeStage::all() {
            let multiplier =
                get_sensitive_period_multiplier(&stage, &DevelopmentalCategory::Neutral);
            assert!((multiplier - 1.0).abs() < f64::EPSILON);
        }
    }

    // === Turning Point Tests ===

    #[test]
    fn turning_point_increases_plasticity_temporarily() {
        let turning_points = vec![TurningPoint {
            event_id: EventId::new("tp_001").unwrap(),
            timestamp: timestamp_for_days(10000),
            domain: TurningPointDomain::Career,
            magnitude: 0.8,
        }];
        let boost = get_turning_point_boost(&turning_points, timestamp_for_days(10000));
        assert!((boost - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn turning_point_effect_decays_with_halflife() {
        let turning_points = vec![TurningPoint {
            event_id: EventId::new("tp_001").unwrap(),
            timestamp: timestamp_for_days(10000),
            domain: TurningPointDomain::Career,
            magnitude: 0.8,
        }];
        let boost_at_halflife =
            get_turning_point_boost(&turning_points, timestamp_for_days(10180));
        // Should be approximately 0.25 (half of 0.5)
        assert!((boost_at_halflife - 0.25).abs() < 0.01);
    }

    #[test]
    fn multiple_turning_points_compound() {
        let turning_points = vec![
            TurningPoint {
                event_id: EventId::new("tp_001").unwrap(),
                timestamp: timestamp_for_days(9970),
                domain: TurningPointDomain::Career,
                magnitude: 0.8,
            },
            TurningPoint {
                event_id: EventId::new("tp_002").unwrap(),
                timestamp: timestamp_for_days(9800),
                domain: TurningPointDomain::Relationship,
                magnitude: 0.6,
            },
        ];
        let boost = get_turning_point_boost(&turning_points, timestamp_for_days(10000));
        // Raw sum exceeds 0.5, so it's capped
        assert!((boost - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn turning_point_in_future_ignored() {
        let turning_points = vec![TurningPoint {
            event_id: EventId::new("tp_001").unwrap(),
            timestamp: timestamp_for_days(11000),
            domain: TurningPointDomain::Career,
            magnitude: 0.8,
        }];
        let boost = get_turning_point_boost(&turning_points, timestamp_for_days(10000));
        assert!(boost.abs() < f64::EPSILON);
    }

    #[test]
    fn empty_turning_points_returns_zero() {
        let turning_points: Vec<TurningPoint> = vec![];
        let boost = get_turning_point_boost(&turning_points, timestamp_for_days(10000));
        assert!(boost.abs() < f64::EPSILON);
    }

    #[test]
    fn turning_point_very_old_nearly_zero() {
        let turning_points = vec![TurningPoint {
            event_id: EventId::new("tp_001").unwrap(),
            timestamp: timestamp_for_days(5000),
            domain: TurningPointDomain::Career,
            magnitude: 0.8,
        }];
        let boost = get_turning_point_boost(&turning_points, timestamp_for_days(10000));
        assert!(boost < 0.01);
    }

    // === Species Scaling Tests ===

    #[test]
    fn dog_plasticity_uses_scaled_age() {
        let entity = EntityBuilder::new()
            .species(Species::Dog)
            .age(Duration::years(2))
            .build()
            .unwrap();

        assert_eq!(entity.life_stage(), LifeStage::YoungAdult);

        let event = EventBuilder::new(EventType::Betrayal).build().unwrap();
        let modified =
            apply_developmental_effects(&entity, &event, 1.0, 730, timestamp_for_days(730));

        // Dog at 2 years: time_scale ~6.67, human-equivalent ~13.3 years
        // Plasticity ~1.693, Intimacy category 1.3x in YoungAdult
        // Expected: 1.0 * 1.693 * 1.3 = ~2.2
        assert!(modified > 2.0 && modified < 2.5);
    }

    #[test]
    fn human_plasticity_uses_raw_age() {
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .age(Duration::years(25))
            .build()
            .unwrap();

        let event = EventBuilder::new(EventType::Violence).build().unwrap();
        let modified = apply_developmental_effects(
            &entity,
            &event,
            1.0,
            25 * 365,
            timestamp_for_days(25 * 365),
        );

        // Human at 25: time_scale 1.0, plasticity 1.425
        // Violence is Neutral category (1.0x)
        // Expected: 1.0 * 1.425 * 1.0 = 1.425
        assert!((modified - 1.425).abs() < 0.1);
    }

    // === Integration Tests ===

    #[test]
    fn apply_developmental_effects_modifies_event_impact() {
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .age(Duration::years(8))
            .build()
            .unwrap();

        let event = EventBuilder::new(EventType::Support).build().unwrap();
        let base_impact = 0.5;
        let modified = apply_developmental_effects(
            &entity,
            &event,
            base_impact,
            8 * 365,
            timestamp_for_days(8 * 365),
        );

        // Child at age 8: plasticity ~1.816, Attachment category 2.0x
        // Expected: 0.5 * 1.816 * 2.0 = 1.816
        assert!(modified > base_impact);
        assert!((modified - 1.816).abs() < 0.1);
    }

    #[test]
    fn developmental_effects_pure_function() {
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .age(Duration::years(30))
            .build()
            .unwrap();

        let event = EventBuilder::new(EventType::Achievement).build().unwrap();
        let original_age = entity.age();

        let result1 = apply_developmental_effects(
            &entity,
            &event,
            1.0,
            30 * 365,
            timestamp_for_days(30 * 365),
        );
        let result2 = apply_developmental_effects(
            &entity,
            &event,
            1.0,
            30 * 365,
            timestamp_for_days(30 * 365),
        );

        assert_eq!(entity.age(), original_age);
        assert!((result1 - result2).abs() < f64::EPSILON);
    }

    #[test]
    fn adolescent_identity_event_amplified() {
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .age(Duration::years(15))
            .build()
            .unwrap();

        let event = EventBuilder::new(EventType::Empowerment).build().unwrap();
        let modified = apply_developmental_effects(
            &entity,
            &event,
            1.0,
            15 * 365,
            timestamp_for_days(15 * 365),
        );

        // Adolescent at 15: plasticity ~1.655, Identity category 1.8x
        // Expected: 1.0 * 1.655 * 1.8 = ~2.98
        assert!(modified > 2.5 && modified < 3.5);
    }

    #[test]
    fn elder_integrity_event_amplified() {
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .age(Duration::years(75))
            .build()
            .unwrap();

        let event = EventBuilder::new(EventType::Realization).build().unwrap();
        let modified = apply_developmental_effects(
            &entity,
            &event,
            1.0,
            75 * 365,
            timestamp_for_days(75 * 365),
        );

        // Elder at 75: plasticity 0.5 (floor), Integrity category 1.2x
        // Expected: 1.0 * 0.5 * 1.2 = 0.6
        assert!((modified - 0.6).abs() < 0.1);
    }

    #[test]
    fn zero_impact_remains_zero() {
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .age(Duration::years(10))
            .build()
            .unwrap();

        let event = EventBuilder::new(EventType::Support).build().unwrap();
        let modified = apply_developmental_effects(
            &entity,
            &event,
            0.0,
            10 * 365,
            timestamp_for_days(10 * 365),
        );
        assert!(modified.abs() < f64::EPSILON);
    }

    #[test]
    fn negative_impact_scales_correctly() {
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .age(Duration::years(8))
            .build()
            .unwrap();

        let event = EventBuilder::new(EventType::Conflict).build().unwrap();
        let modified = apply_developmental_effects(
            &entity,
            &event,
            -0.5,
            8 * 365,
            timestamp_for_days(8 * 365),
        );

        // Child at 8: plasticity ~1.816, Attachment category 2.0x
        // Expected: -0.5 * 1.816 * 2.0 = -1.816
        assert!(modified < 0.0);
        assert!((modified - (-1.816)).abs() < 0.1);
    }
}
