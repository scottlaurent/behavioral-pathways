//! Integration test: MentalHealth computes ITS factors from SocialCognition.
//!
//! Validates that MentalHealth correctly reads from SocialCognition to compute:
//! - Thwarted Belongingness (TB)
//! - Perceived Burdensomeness (PB)
//! - Suicidal Desire (requires TB, PB, and hopelessness above thresholds)

use behavioral_pathways::state::{MentalHealth, SocialCognition};
use behavioral_pathways::{HOPELESSNESS_THRESHOLD, PB_PRESENT_THRESHOLD, TB_PRESENT_THRESHOLD};

/// Tests that MentalHealth::compute_thwarted_belongingness correctly reads
/// loneliness and perceived_reciprocal_caring from SocialCognition.
///
/// Formula: TB = (loneliness + (1 - perceived_reciprocal_caring)) / 2
#[test]
fn mental_health_computes_thwarted_belongingness_from_social_cognition() {
    let mh = MentalHealth::new();

    // Test case 1: High loneliness (0.8), low caring (0.2)
    // TB = (0.8 + (1 - 0.2)) / 2 = (0.8 + 0.8) / 2 = 0.8
    let social_high_tb = SocialCognition::new()
        .with_loneliness_base(0.8)
        .with_perceived_reciprocal_caring_base(0.2);
    let tb_high = mh.compute_thwarted_belongingness(&social_high_tb);
    assert!(
        (tb_high - 0.8).abs() < 0.01,
        "Expected TB = 0.8, got {tb_high}"
    );

    // Test case 2: Low loneliness (0.2), high caring (0.8)
    // TB = (0.2 + (1 - 0.8)) / 2 = (0.2 + 0.2) / 2 = 0.2
    let social_low_tb = SocialCognition::new()
        .with_loneliness_base(0.2)
        .with_perceived_reciprocal_caring_base(0.8);
    let tb_low = mh.compute_thwarted_belongingness(&social_low_tb);
    assert!(
        (tb_low - 0.2).abs() < 0.01,
        "Expected TB = 0.2, got {tb_low}"
    );

    // Test case 3: Verify formula components - high loneliness, high caring
    // TB = (0.9 + (1 - 0.9)) / 2 = (0.9 + 0.1) / 2 = 0.5
    let social_mixed = SocialCognition::new()
        .with_loneliness_base(0.9)
        .with_perceived_reciprocal_caring_base(0.9);
    let tb_mixed = mh.compute_thwarted_belongingness(&social_mixed);
    assert!(
        (tb_mixed - 0.5).abs() < 0.01,
        "Expected TB = 0.5, got {tb_mixed}"
    );

    // Test case 4: Changes to SocialCognition flow through to TB computation
    let mut social_dynamic = SocialCognition::new()
        .with_loneliness_base(0.3)
        .with_perceived_reciprocal_caring_base(0.7);
    let initial_tb = mh.compute_thwarted_belongingness(&social_dynamic);
    // TB = (0.3 + (1 - 0.7)) / 2 = (0.3 + 0.3) / 2 = 0.3
    assert!(
        (initial_tb - 0.3).abs() < 0.01,
        "Expected initial TB = 0.3, got {initial_tb}"
    );

    // Increase loneliness via delta
    social_dynamic.add_loneliness_delta(0.4); // Now loneliness effective = 0.7
    let updated_tb = mh.compute_thwarted_belongingness(&social_dynamic);
    // TB = (0.7 + (1 - 0.7)) / 2 = (0.7 + 0.3) / 2 = 0.5
    assert!(
        (updated_tb - 0.5).abs() < 0.01,
        "Expected updated TB = 0.5, got {updated_tb}"
    );
}

/// Tests that MentalHealth::compute_perceived_burdensomeness correctly reads
/// perceived_liability and self_hate from SocialCognition.
///
/// Formula: PB = perceived_liability * self_hate (multiplicative)
#[test]
fn mental_health_computes_perceived_burdensomeness_from_social_cognition() {
    let mh = MentalHealth::new();

    // Test case 1: High liability (0.8), high self-hate (0.8)
    // PB = 0.8 * 0.8 = 0.64
    let social_high_pb = SocialCognition::new()
        .with_perceived_liability_base(0.8)
        .with_self_hate_base(0.8);
    let pb_high = mh.compute_perceived_burdensomeness(&social_high_pb);
    assert!(
        (pb_high - 0.64).abs() < 0.01,
        "Expected PB = 0.64, got {pb_high}"
    );

    // Test case 2: High liability (0.8), low self-hate (0.1) - multiplicative means low PB
    // PB = 0.8 * 0.1 = 0.08
    let social_low_pb = SocialCognition::new()
        .with_perceived_liability_base(0.8)
        .with_self_hate_base(0.1);
    let pb_low = mh.compute_perceived_burdensomeness(&social_low_pb);
    assert!(
        (pb_low - 0.08).abs() < 0.01,
        "Expected PB = 0.08, got {pb_low}"
    );

    // Test case 3: Low liability (0.1), high self-hate (0.8) - multiplicative means low PB
    // PB = 0.1 * 0.8 = 0.08
    let social_inverse = SocialCognition::new()
        .with_perceived_liability_base(0.1)
        .with_self_hate_base(0.8);
    let pb_inverse = mh.compute_perceived_burdensomeness(&social_inverse);
    assert!(
        (pb_inverse - 0.08).abs() < 0.01,
        "Expected PB = 0.08, got {pb_inverse}"
    );

    // Test case 4: Zero liability means zero PB regardless of self-hate
    // PB = 0.0 * 0.9 = 0.0
    let social_zero_liability = SocialCognition::new()
        .with_perceived_liability_base(0.0)
        .with_self_hate_base(0.9);
    let pb_zero = mh.compute_perceived_burdensomeness(&social_zero_liability);
    assert!(
        pb_zero.abs() < f32::EPSILON,
        "Expected PB = 0.0, got {pb_zero}"
    );

    // Test case 5: Changes to SocialCognition flow through to PB computation
    let mut social_dynamic = SocialCognition::new()
        .with_perceived_liability_base(0.5)
        .with_self_hate_base(0.5);
    let initial_pb = mh.compute_perceived_burdensomeness(&social_dynamic);
    // PB = 0.5 * 0.5 = 0.25
    assert!(
        (initial_pb - 0.25).abs() < 0.01,
        "Expected initial PB = 0.25, got {initial_pb}"
    );

    // Increase self-hate via delta
    social_dynamic.add_self_hate_delta(0.4); // Now self_hate effective = 0.9
    let updated_pb = mh.compute_perceived_burdensomeness(&social_dynamic);
    // PB = 0.5 * 0.9 = 0.45
    assert!(
        (updated_pb - 0.45).abs() < 0.01,
        "Expected updated PB = 0.45, got {updated_pb}"
    );
}

/// Tests that compute_suicidal_desire requires TB, PB, and interpersonal_hopelessness
/// all above their respective thresholds.
///
/// Thresholds tested:
/// - TB_PRESENT_THRESHOLD (0.5)
/// - PB_PRESENT_THRESHOLD (0.5)
/// - HOPELESSNESS_THRESHOLD (0.5)
#[test]
fn mental_health_compute_suicidal_desire_requires_all_thresholds() {
    // Verify threshold constants
    assert!(
        (TB_PRESENT_THRESHOLD - 0.5).abs() < f32::EPSILON,
        "TB_PRESENT_THRESHOLD should be 0.5"
    );
    assert!(
        (PB_PRESENT_THRESHOLD - 0.5).abs() < f32::EPSILON,
        "PB_PRESENT_THRESHOLD should be 0.5"
    );
    assert!(
        (HOPELESSNESS_THRESHOLD - 0.5).abs() < f32::EPSILON,
        "HOPELESSNESS_THRESHOLD should be 0.5"
    );

    // Create social cognition that produces high TB and PB
    // TB = (0.9 + (1 - 0.1)) / 2 = (0.9 + 0.9) / 2 = 0.9
    // PB = 0.9 * 0.9 = 0.81
    let high_risk_social = SocialCognition::new()
        .with_loneliness_base(0.9)
        .with_perceived_reciprocal_caring_base(0.1)
        .with_perceived_liability_base(0.9)
        .with_self_hate_base(0.9);

    // Test case 1: All conditions met - desire should be non-zero
    let mh_all_met = MentalHealth::new().with_interpersonal_hopelessness_base(0.7);
    let desire_all_met = mh_all_met.compute_suicidal_desire(&high_risk_social);
    assert!(
        desire_all_met > 0.0,
        "Expected non-zero desire when all conditions met, got {desire_all_met}"
    );

    // Test case 2: Hopelessness below threshold - desire should be zero
    let mh_low_hopelessness = MentalHealth::new().with_interpersonal_hopelessness_base(0.3);
    let desire_low_hopelessness =
        mh_low_hopelessness.compute_suicidal_desire(&high_risk_social);
    assert!(
        desire_low_hopelessness.abs() < f32::EPSILON,
        "Expected zero desire when hopelessness below threshold, got {desire_low_hopelessness}"
    );

    // Test case 3: TB below threshold - desire should be zero
    // TB = (0.2 + (1 - 0.8)) / 2 = (0.2 + 0.2) / 2 = 0.2 (below 0.5)
    let low_tb_social = SocialCognition::new()
        .with_loneliness_base(0.2)
        .with_perceived_reciprocal_caring_base(0.8)
        .with_perceived_liability_base(0.9)
        .with_self_hate_base(0.9);
    let mh_with_hopelessness = MentalHealth::new().with_interpersonal_hopelessness_base(0.7);
    let desire_low_tb = mh_with_hopelessness.compute_suicidal_desire(&low_tb_social);
    assert!(
        desire_low_tb.abs() < f32::EPSILON,
        "Expected zero desire when TB below threshold, got {desire_low_tb}"
    );

    // Test case 4: PB below threshold - desire should be zero
    // PB = 0.2 * 0.2 = 0.04 (below 0.5)
    let low_pb_social = SocialCognition::new()
        .with_loneliness_base(0.9)
        .with_perceived_reciprocal_caring_base(0.1)
        .with_perceived_liability_base(0.2)
        .with_self_hate_base(0.2);
    let desire_low_pb = mh_with_hopelessness.compute_suicidal_desire(&low_pb_social);
    assert!(
        desire_low_pb.abs() < f32::EPSILON,
        "Expected zero desire when PB below threshold, got {desire_low_pb}"
    );

    // Test case 5: At exact thresholds - should produce desire
    // TB = (0.5 + (1 - 0.5)) / 2 = (0.5 + 0.5) / 2 = 0.5 (exactly at threshold)
    // For PB = 0.5, we need sqrt(0.5) * sqrt(0.5) = 0.5, so liability = 0.707..., self_hate = 0.707...
    // Actually simpler: PB = 1.0 * 0.5 = 0.5 (exactly at threshold)
    let at_threshold_social = SocialCognition::new()
        .with_loneliness_base(0.5)
        .with_perceived_reciprocal_caring_base(0.5)
        .with_perceived_liability_base(1.0)
        .with_self_hate_base(0.5);
    let mh_at_threshold = MentalHealth::new().with_interpersonal_hopelessness_base(0.5);
    let desire_at_threshold = mh_at_threshold.compute_suicidal_desire(&at_threshold_social);
    // At exact thresholds (>=), should produce desire
    assert!(
        desire_at_threshold > 0.0,
        "Expected non-zero desire at exact thresholds, got {desire_at_threshold}"
    );
}
