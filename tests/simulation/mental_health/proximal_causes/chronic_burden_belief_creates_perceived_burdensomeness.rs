//! Test: Chronic burden belief creates perceived burdensomeness through multiplicative formula.
//!
//! PB = perceived_liability * self_hate
//!
//! This test verifies that BOTH liability perception AND self-hate are required
//! for PB to be elevated. The multiplicative formula means either factor alone
//! is insufficient.

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::{
    EventType, MentalHealthPath, SocialCognitionPath, Species, StatePath,
};
use behavioral_pathways::event::EventBuilder;
use behavioral_pathways::simulation::Simulation;
use behavioral_pathways::types::{Duration, EntityId, Timestamp};

/// Chronic burden belief creates perceived burdensomeness through multiplicative formula.
#[test]
fn chronic_burden_belief_creates_perceived_burdensomeness() {
    // ========================================================================
    // SETUP
    // What we're doing: Creating a person and testing the multiplicative
    // PB formula by applying burden feedback and failure events.
    // ========================================================================

    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let entity = EntityBuilder::new()
        .id("person")
        .species(Species::Human)
        .build()
        .unwrap();

    let entity_id = EntityId::new("person").unwrap();
    let anchor = reference;
    sim.add_entity(entity, anchor);

    // ========================================================================
    // STAGE 1: Establish baseline PB
    // What we're testing: Default state should have low liability and
    // low self-hate, resulting in low PB.
    // ========================================================================

    let baseline_liability;
    let baseline_self_hate;
    let baseline_pb;
    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(anchor);
        baseline_liability = state.get_effective(StatePath::SocialCognition(
            SocialCognitionPath::PerceivedLiability,
        ));
        baseline_self_hate = state.get_effective(StatePath::SocialCognition(
            SocialCognitionPath::SelfHate,
        ));
        baseline_pb = state.get_effective(StatePath::MentalHealth(MentalHealthPath::PerceivedBurdensomeness));
    }

    // Verify formula: PB = liability * self_hate
    let expected_pb = baseline_liability * baseline_self_hate;
    assert!(
        (baseline_pb - expected_pb).abs() < 0.01,
        "PB formula mismatch. Expected {}, got {}",
        expected_pb,
        baseline_pb
    );

    assert!(
        baseline_pb < 0.3,
        "Baseline PB should be low, got {}",
        baseline_pb
    );

    // ========================================================================
    // STAGE 2: Test high liability alone (insufficient for PB)
    // What we're testing: High liability without self-hate keeps PB low
    // due to multiplicative formula.
    // ========================================================================

    // Apply burden feedback events to increase liability
    for i in 0..3 {
        let event = EventBuilder::new(EventType::BurdenFeedback)
            .target(entity_id.clone())
            .severity(0.8)
            .build()
            .unwrap();
        sim.add_event(event, anchor + Duration::days(i * 1));
    }

    let liability_only_timestamp = anchor + Duration::days(3);

    let liability_only;
    let self_hate_only;
    let pb_liability_only;
    {
        let handle = sim.entity(&entity_id).unwrap();
        let liability_only_state = handle.state_at(liability_only_timestamp);
        liability_only = liability_only_state.get_effective(StatePath::SocialCognition(
            SocialCognitionPath::PerceivedLiability,
        ));
        self_hate_only = liability_only_state.get_effective(StatePath::SocialCognition(
            SocialCognitionPath::SelfHate,
        ));
        pb_liability_only = liability_only_state.get_effective(StatePath::MentalHealth(MentalHealthPath::PerceivedBurdensomeness));
    }

    // Liability should be elevated above baseline
    assert!(
        liability_only > baseline_liability,
        "Burden feedback should increase liability above baseline. Baseline: {}, Now: {}",
        baseline_liability,
        liability_only
    );

    // But PB remains low because self-hate is low (multiplicative protection)
    let expected_pb_liability = liability_only * self_hate_only;
    assert!(
        (pb_liability_only - expected_pb_liability).abs() < 0.01,
        "PB formula mismatch with high liability. Expected {}, got {}",
        expected_pb_liability,
        pb_liability_only
    );

    assert!(
        pb_liability_only < 0.3,
        "PB should remain low without self-hate, got {}",
        pb_liability_only
    );

    // ========================================================================
    // STAGE 3: Add self-hate through failure events
    // What we're testing: When BOTH liability AND self-hate are elevated,
    // PB becomes significant through multiplication.
    // ========================================================================

    // Apply failure events to increase self-hate
    for i in 0..4 {
        let event = EventBuilder::new(EventType::Failure)
            .target(entity_id.clone())
            .severity(0.8)
            .build()
            .unwrap();
        sim.add_event(event, anchor + Duration::days(3 + i * 1));
    }

    let both_elevated_timestamp = anchor + Duration::days(7);

    let liability_both;
    let self_hate_both;
    let pb_both;
    {
        let handle = sim.entity(&entity_id).unwrap();
        let both_elevated_state = handle.state_at(both_elevated_timestamp);
        liability_both = both_elevated_state.get_effective(StatePath::SocialCognition(
            SocialCognitionPath::PerceivedLiability,
        ));
        self_hate_both = both_elevated_state.get_effective(StatePath::SocialCognition(
            SocialCognitionPath::SelfHate,
        ));
        pb_both = both_elevated_state.get_effective(StatePath::MentalHealth(MentalHealthPath::PerceivedBurdensomeness));
    }

    // Both factors should be elevated above baseline
    assert!(
        liability_both > baseline_liability,
        "Liability should be elevated above baseline. Baseline: {}, Now: {}",
        baseline_liability,
        liability_both
    );
    assert!(
        self_hate_both > baseline_self_hate,
        "Self-hate should be elevated after failures. Baseline: {}, Now: {}",
        baseline_self_hate,
        self_hate_both
    );

    // Now PB should be significantly elevated (multiplicative effect)
    let expected_pb_both = liability_both * self_hate_both;
    assert!(
        (pb_both - expected_pb_both).abs() < 0.01,
        "PB formula mismatch with both factors. Expected {}, got {}",
        expected_pb_both,
        pb_both
    );

    // PB should be higher than baseline since both factors increased
    assert!(
        pb_both > baseline_pb,
        "PB should be elevated above baseline. Baseline: {}, Now: {}",
        baseline_pb,
        pb_both
    );

    // ========================================================================
    // STAGE 4: Verify multiplicative formula
    // What we're testing: PB = liability * self_hate (multiplicative formula).
    // The key insight is that BOTH factors contribute - low self-hate keeps PB
    // low even with high liability.
    // ========================================================================

    // The multiplicative formula ensures that:
    // 1. With only liability elevated and self-hate near baseline, PB is limited
    // 2. With both factors elevated, PB reflects their product
    // The formula pb = liability * self_hate is verified above in both stages

    // Verify the fundamental multiplicative relationship
    assert!(
        (pb_both - (liability_both * self_hate_both)).abs() < 0.01,
        "PB should equal liability * self_hate. Expected {}, got {}",
        liability_both * self_hate_both,
        pb_both
    );
}
