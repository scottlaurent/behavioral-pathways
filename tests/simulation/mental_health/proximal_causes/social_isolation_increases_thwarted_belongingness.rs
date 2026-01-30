//! Test: Social isolation increases thwarted belongingness through the additive TB formula.
//!
//! TB = (loneliness + (1 - perceived_reciprocal_caring)) / 2
//!
//! This test verifies that social exclusion events increase loneliness
//! and decrease perceived caring, both contributing to elevated TB.

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::{
    EventType, MentalHealthPath, SocialCognitionPath, Species, StatePath,
};
use behavioral_pathways::event::EventBuilder;
use behavioral_pathways::simulation::Simulation;
use behavioral_pathways::types::{Duration, EntityId, Timestamp};

/// Social isolation increases thwarted belongingness through TB formula.
#[test]
fn social_isolation_increases_thwarted_belongingness() {
    // ========================================================================
    // SETUP
    // What we're doing: Creating a person with baseline social connection,
    // then applying social exclusion events to test TB formula.
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

    let baseline_loneliness;
    let baseline_caring;
    let baseline_tb;

    // ========================================================================
    // STAGE 1: Establish baseline TB
    // What we're testing: A socially connected person should have low TB.
    // ========================================================================
    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(anchor);

        baseline_loneliness = state.get_effective(StatePath::SocialCognition(
            SocialCognitionPath::Loneliness,
        ));
        baseline_caring = state.get_effective(StatePath::SocialCognition(
            SocialCognitionPath::PerceivedReciprocalCaring,
        ));
        baseline_tb = state.get_effective(StatePath::MentalHealth(MentalHealthPath::ThwartedBelongingness));

        // Default state should have low loneliness and high caring
        assert!(
            baseline_loneliness < 0.3,
            "Baseline loneliness should be low, got {}",
            baseline_loneliness
        );
        assert!(
            baseline_caring > 0.5,
            "Baseline perceived caring should be high, got {}",
            baseline_caring
        );

        // Verify formula: TB = (loneliness + (1 - caring)) / 2
        let expected_tb = (baseline_loneliness + (1.0 - baseline_caring)) / 2.0;
        assert!(
            (baseline_tb - expected_tb).abs() < 0.01,
            "TB formula mismatch. Expected {}, got {}",
            expected_tb,
            baseline_tb
        );
    } // handle dropped here

    // ========================================================================
    // STAGE 2: Apply social exclusion events
    // What we're testing: Social exclusion increases loneliness and
    // decreases perceived caring, both contributing to TB.
    // ========================================================================

    // Apply 5 social exclusion events over a week
    for i in 0..5 {
        let event = EventBuilder::new(EventType::SocialExclusion)
            .target(entity_id.clone())
            .severity(0.7)
            .build()
            .unwrap();
        sim.add_event(event, anchor + Duration::days(i * 1));
    }

    let isolated_timestamp = anchor + Duration::days(5);

    // ========================================================================
    // STAGE 3: Verify TB formula accuracy
    // What we're testing: TB accurately reflects the additive formula
    // with both loneliness and caring contributing.
    // ========================================================================
    {
        let handle = sim.entity(&entity_id).unwrap();
        let isolated_state = handle.state_at(isolated_timestamp);

        let isolated_loneliness = isolated_state.get_effective(StatePath::SocialCognition(
            SocialCognitionPath::Loneliness,
        ));
        let isolated_caring = isolated_state.get_effective(StatePath::SocialCognition(
            SocialCognitionPath::PerceivedReciprocalCaring,
        ));
        let isolated_tb = isolated_state.get_effective(StatePath::MentalHealth(MentalHealthPath::ThwartedBelongingness));

        // Loneliness should have increased
        assert!(
            isolated_loneliness > baseline_loneliness + 0.3,
            "Social exclusion should increase loneliness. Baseline: {}, Isolated: {}",
            baseline_loneliness,
            isolated_loneliness
        );

        // Perceived caring should have decreased
        assert!(
            isolated_caring < baseline_caring - 0.2,
            "Social exclusion should decrease perceived caring. Baseline: {}, Isolated: {}",
            baseline_caring,
            isolated_caring
        );

        let expected_isolated_tb = (isolated_loneliness + (1.0 - isolated_caring)) / 2.0;
        assert!(
            (isolated_tb - expected_isolated_tb).abs() < 0.01,
            "TB formula mismatch after isolation. Expected {}, got {}",
            expected_isolated_tb,
            isolated_tb
        );

        // TB should be significantly elevated
        assert!(
            isolated_tb > baseline_tb + 0.3,
            "TB should increase significantly after isolation. Baseline: {}, Isolated: {}",
            baseline_tb,
            isolated_tb
        );

        // ========================================================================
        // STAGE 4: Verify TB threshold
        // What we're testing: Severe isolation should push TB above the 0.5
        // threshold required for suicidal desire.
        // ========================================================================

        assert!(
            isolated_tb > 0.5,
            "Severe isolation should push TB above threshold. Got {}",
            isolated_tb
        );
    }
}
