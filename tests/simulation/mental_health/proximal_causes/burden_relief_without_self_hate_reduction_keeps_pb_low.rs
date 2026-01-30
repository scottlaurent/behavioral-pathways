//! Test: Burden relief without self-hate reduction keeps PB low.
//!
//! PB = liability * self_hate (multiplicative)
//!
//! This test verifies that reducing liability alone is sufficient to reduce
//! PB, even if self-hate remains elevated, due to the multiplicative formula.

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::{
    EventType, MentalHealthPath, SocialCognitionPath, Species, StatePath,
};
use behavioral_pathways::event::EventBuilder;
use behavioral_pathways::simulation::Simulation;
use behavioral_pathways::types::{Duration, EntityId, Timestamp};

/// Burden relief without self-hate reduction keeps PB low.
#[test]
fn burden_relief_without_self_hate_reduction_keeps_pb_low() {
    // ========================================================================
    // SETUP
    // What we're doing: Creating a person with elevated PB (high liability
    // and high self-hate), then testing whether reducing liability alone
    // reduces PB despite persistent self-hate.
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
    // STAGE 1: Build PB through liability and self-hate
    // What we're testing: High liability and high self-hate create high PB.
    // ========================================================================

    // Apply burden feedback to increase liability
    for i in 0..5 {
        let event = EventBuilder::new(EventType::BurdenFeedback)
            .target(entity_id.clone())
            .severity(0.8)
            .build()
            .unwrap();
        sim.add_event(event, anchor + Duration::days(i * 1));
    }

    // Apply failures to increase self-hate
    for i in 0..5 {
        let event = EventBuilder::new(EventType::Failure)
            .target(entity_id.clone())
            .severity(0.8)
            .build()
            .unwrap();
        sim.add_event(event, anchor + Duration::days(i * 1));
    }

    let pb_elevated_timestamp = anchor + Duration::days(5);

    let liability_elevated;
    let self_hate_elevated;
    let pb_elevated;
    {
        let handle = sim.entity(&entity_id).unwrap();
        let pb_elevated_state = handle.state_at(pb_elevated_timestamp);
        liability_elevated = pb_elevated_state.get_effective(StatePath::SocialCognition(
            SocialCognitionPath::PerceivedLiability,
        ));
        self_hate_elevated = pb_elevated_state.get_effective(StatePath::SocialCognition(
            SocialCognitionPath::SelfHate,
        ));
        pb_elevated = pb_elevated_state.get_effective(StatePath::MentalHealth(MentalHealthPath::PerceivedBurdensomeness));
    }

    assert!(
        liability_elevated > 0.2,
        "Liability should be elevated, got {}",
        liability_elevated
    );

    assert!(
        self_hate_elevated > 0.2,
        "Self-hate should be elevated, got {}",
        self_hate_elevated
    );

    // PB should be elevated above default since both factors increased
    assert!(
        pb_elevated > 0.05,
        "PB should be elevated with both factors high, got {}",
        pb_elevated
    );

    // Verify formula: PB = liability * self_hate
    let expected_pb = liability_elevated * self_hate_elevated;
    assert!(
        (pb_elevated - expected_pb).abs() < 0.01,
        "PB formula mismatch. Expected {}, got {}",
        expected_pb,
        pb_elevated
    );

    // ========================================================================
    // STAGE 2: Apply burden relief (achievements/successes)
    // What we're testing: Achievements reduce perceived liability by
    // demonstrating competence and value.
    // ========================================================================

    // Apply achievements to reduce liability perception
    for i in 0..8 {
        let event = EventBuilder::new(EventType::Achievement)
            .target(entity_id.clone())
            .severity(0.7)
            .build()
            .unwrap();
        sim.add_event(event, anchor + Duration::days(5 + i * 1));
    }

    let post_relief_timestamp = anchor + Duration::days(13);

    let liability_post;
    let self_hate_post;
    let pb_post;
    {
        let handle = sim.entity(&entity_id).unwrap();
        let post_relief_state = handle.state_at(post_relief_timestamp);
        liability_post = post_relief_state.get_effective(StatePath::SocialCognition(
            SocialCognitionPath::PerceivedLiability,
        ));
        self_hate_post = post_relief_state.get_effective(StatePath::SocialCognition(
            SocialCognitionPath::SelfHate,
        ));
        pb_post = post_relief_state.get_effective(StatePath::MentalHealth(MentalHealthPath::PerceivedBurdensomeness));
    }

    // Liability should be significantly reduced
    assert!(
        liability_post < liability_elevated - 0.3,
        "Achievements should reduce liability. Pre: {}, Post: {}",
        liability_elevated,
        liability_post
    );

    // Self-hate may remain elevated (achievements alone don't necessarily fix self-hate)
    // But even with high self-hate, reduced liability means reduced PB (multiplicative)

    // PB should be significantly reduced due to reduced liability
    assert!(
        pb_post < pb_elevated - 0.2,
        "Reduced liability should reduce PB. Pre: {}, Post: {}",
        pb_elevated,
        pb_post
    );

    // Verify formula still holds
    let expected_pb_post = liability_post * self_hate_post;
    assert!(
        (pb_post - expected_pb_post).abs() < 0.01,
        "PB formula mismatch after burden relief. Expected {}, got {}",
        expected_pb_post,
        pb_post
    );

    // ========================================================================
    // STAGE 3: Verify multiplicative protection
    // What we're testing: Even if self-hate remains high, low liability
    // keeps PB low (one factor being low is sufficient).
    // ========================================================================

    // Wait for liability to decay further while self-hate may persist
    let long_term_timestamp = anchor + Duration::days(30);

    let liability_long_term;
    let self_hate_long_term;
    let pb_long_term;
    {
        let handle = sim.entity(&entity_id).unwrap();
        let long_term_state = handle.state_at(long_term_timestamp);
        liability_long_term = long_term_state.get_effective(StatePath::SocialCognition(
            SocialCognitionPath::PerceivedLiability,
        ));
        self_hate_long_term = long_term_state.get_effective(StatePath::SocialCognition(
            SocialCognitionPath::SelfHate,
        ));
        pb_long_term = long_term_state.get_effective(StatePath::MentalHealth(MentalHealthPath::PerceivedBurdensomeness));
    }

    // Even if self-hate remains somewhat elevated
    if self_hate_long_term > 0.4 {
        // Low liability should keep PB low
        assert!(
            pb_long_term < 0.3,
            "Low liability should keep PB low despite elevated self-hate. Liability: {}, Self-hate: {}, PB: {}",
            liability_long_term,
            self_hate_long_term,
            pb_long_term
        );
    }

    // Verify formula
    let expected_pb_long_term = liability_long_term * self_hate_long_term;
    assert!(
        (pb_long_term - expected_pb_long_term).abs() < 0.01,
        "PB formula mismatch long-term. Expected {}, got {}",
        expected_pb_long_term,
        pb_long_term
    );

    // ========================================================================
    // STAGE 4: Demonstrate protection against PB elevation
    // What we're testing: With low liability, even deliberate attempts to
    // increase self-hate don't create high PB (multiplicative protection).
    // ========================================================================

    // Apply more failures to test if PB can be elevated despite low liability
    for i in 0..3 {
        let event = EventBuilder::new(EventType::Failure)
            .target(entity_id.clone())
            .severity(0.9)
            .build()
            .unwrap();
        sim.add_event(event, anchor + Duration::days(30 + i * 1));
    }

    let final_timestamp = anchor + Duration::days(33);

    let liability_final;
    let self_hate_final;
    let pb_final;
    {
        let handle = sim.entity(&entity_id).unwrap();
        let final_state = handle.state_at(final_timestamp);
        liability_final = final_state.get_effective(StatePath::SocialCognition(
            SocialCognitionPath::PerceivedLiability,
        ));
        self_hate_final = final_state.get_effective(StatePath::SocialCognition(
            SocialCognitionPath::SelfHate,
        ));
        pb_final = final_state.get_effective(StatePath::MentalHealth(MentalHealthPath::PerceivedBurdensomeness));
    }

    // Self-hate may increase
    // But if liability is low, PB remains low (multiplicative protection)

    assert!(
        liability_final < 0.4,
        "Liability should remain low, got {}",
        liability_final
    );

    // Even if self-hate increases, PB should remain low
    assert!(
        pb_final < 0.4,
        "PB should remain low due to low liability, despite self-hate. Liability: {}, Self-hate: {}, PB: {}",
        liability_final,
        self_hate_final,
        pb_final
    );

    // Verify formula
    let expected_pb_final = liability_final * self_hate_final;
    assert!(
        (pb_final - expected_pb_final).abs() < 0.01,
        "PB formula mismatch final. Expected {}, got {}",
        expected_pb_final,
        pb_final
    );
}
