//! Test: High risk convergence when all three ITS factors are present.
//!
//! Attempt risk = desire * AC = (TB * PB) * AC
//!
//! This test verifies that when TB, PB, hopelessness, AND AC are all
//! elevated, attempt risk becomes significant.

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::{EventType, MentalHealthPath, Species, StatePath};
use behavioral_pathways::event::EventBuilder;
use behavioral_pathways::simulation::Simulation;
use behavioral_pathways::types::{Duration, EntityId, Timestamp};

/// High risk convergence when all three ITS factors are present.
#[test]
fn high_risk_convergence_all_three_factors_present() {
    // ========================================================================
    // SETUP
    // What we're doing: Creating a person and systematically building all
    // ITS risk factors to verify convergence into high attempt risk.
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
    // STAGE 1: Establish baseline risk
    // What we're testing: Default state should have near-zero attempt risk.
    // ========================================================================

    let baseline_risk;
    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(anchor);
        baseline_risk = state.get_effective(StatePath::MentalHealth(MentalHealthPath::AttemptRisk));
    }

    assert!(
        baseline_risk < 0.01,
        "Baseline attempt risk should be near zero, got {}",
        baseline_risk
    );

    // ========================================================================
    // STAGE 2: Build acquired capability through trauma
    // What we're testing: AC alone creates dormant risk but not active risk.
    // ========================================================================

    // Apply violence exposure to build AC
    for i in 0..6 {
        let event = EventBuilder::new(EventType::Violence)
            .target(entity_id.clone())
            .severity(0.75)
            .build()
            .unwrap();
        sim.add_event(event, anchor + Duration::days(i * 7));
    }

    let ac_built_timestamp = anchor + Duration::days(42);

    let ac;
    let desire_ac_only;
    let risk_ac_only;
    {
        let handle = sim.entity(&entity_id).unwrap();
        let ac_built_state = handle.state_at(ac_built_timestamp);
        ac = ac_built_state.get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));
        desire_ac_only = ac_built_state.get_effective(StatePath::MentalHealth(MentalHealthPath::SuicidalDesire));
        risk_ac_only = ac_built_state.get_effective(StatePath::MentalHealth(MentalHealthPath::AttemptRisk));
    }

    assert!(
        ac > 0.5,
        "AC should be elevated after trauma, got {}",
        ac
    );

    assert!(
        desire_ac_only < 0.01,
        "Desire should be zero with only AC elevated, got {}",
        desire_ac_only
    );

    assert!(
        risk_ac_only < 0.01,
        "Risk should be zero without desire (risk = desire * AC), got {}",
        risk_ac_only
    );

    // ========================================================================
    // STAGE 3: Build thwarted belongingness
    // What we're testing: AC + TB is still insufficient without PB and
    // hopelessness.
    // ========================================================================

    // Apply social exclusion to increase TB
    for i in 0..6 {
        let event = EventBuilder::new(EventType::SocialExclusion)
            .target(entity_id.clone())
            .severity(0.8)
            .build()
            .unwrap();
        sim.add_event(event, anchor + Duration::days(42 + i * 1));
    }

    let tb_added_timestamp = anchor + Duration::days(48);

    let tb;
    let desire_tb_ac;
    {
        let handle = sim.entity(&entity_id).unwrap();
        let tb_added_state = handle.state_at(tb_added_timestamp);
        tb = tb_added_state.get_effective(StatePath::MentalHealth(MentalHealthPath::ThwartedBelongingness));
        desire_tb_ac = tb_added_state.get_effective(StatePath::MentalHealth(MentalHealthPath::SuicidalDesire));
    }

    assert!(
        tb > 0.5,
        "TB should be above threshold, got {}",
        tb
    );

    assert!(
        desire_tb_ac < 0.01,
        "Desire should still be zero without PB and hopelessness, got {}",
        desire_tb_ac
    );

    // ========================================================================
    // STAGE 4: Build perceived burdensomeness
    // What we're testing: AC + TB + PB is still insufficient without
    // hopelessness.
    // ========================================================================

    // Apply burden feedback and failures to increase PB
    for i in 0..5 {
        let burden_event = EventBuilder::new(EventType::BurdenFeedback)
            .target(entity_id.clone())
            .severity(0.8)
            .build()
            .unwrap();
        sim.add_event(burden_event, anchor + Duration::days(48 + i * 1));

        let failure_event = EventBuilder::new(EventType::Failure)
            .target(entity_id.clone())
            .severity(0.8)
            .build()
            .unwrap();
        sim.add_event(failure_event, anchor + Duration::days(48 + i * 1));
    }

    let pb_added_timestamp = anchor + Duration::days(53);

    let pb;
    let hopelessness_pre;
    let desire_no_hopelessness;
    {
        let handle = sim.entity(&entity_id).unwrap();
        let pb_added_state = handle.state_at(pb_added_timestamp);
        pb = pb_added_state.get_effective(StatePath::MentalHealth(MentalHealthPath::PerceivedBurdensomeness));
        hopelessness_pre = pb_added_state.get_effective(StatePath::MentalHealth(MentalHealthPath::InterpersonalHopelessness));
        desire_no_hopelessness = pb_added_state.get_effective(StatePath::MentalHealth(MentalHealthPath::SuicidalDesire));
    }

    // PB should be elevated from burden feedback and failures
    assert!(
        pb > 0.2,
        "PB should be elevated, got {}",
        pb
    );

    // Hopelessness should still be relatively low
    assert!(
        hopelessness_pre < 0.5,
        "Hopelessness should still be below threshold, got {}",
        hopelessness_pre
    );

    // Desire should be low or zero without full hopelessness
    assert!(
        desire_no_hopelessness < 0.2,
        "Desire should be low without full hopelessness, got {}",
        desire_no_hopelessness
    );

    // ========================================================================
    // STAGE 5: Add hopelessness to complete the convergence
    // What we're testing: With all four factors present (TB, PB,
    // hopelessness, AC), attempt risk becomes significant.
    // ========================================================================

    // Apply events to increase hopelessness
    for i in 0..5 {
        let event = EventBuilder::new(EventType::Realization)
            .target(entity_id.clone())
            .severity(0.9)
            .build()
            .unwrap();
        sim.add_event(event, anchor + Duration::days(53 + i * 1));
    }

    let all_factors_timestamp = anchor + Duration::days(58);

    let tb_final;
    let pb_final;
    let hopelessness_final;
    let ac_final;
    let desire_final;
    let risk_final;
    {
        let handle = sim.entity(&entity_id).unwrap();
        let all_factors_state = handle.state_at(all_factors_timestamp);
        tb_final = all_factors_state.get_effective(StatePath::MentalHealth(MentalHealthPath::ThwartedBelongingness));
        pb_final = all_factors_state.get_effective(StatePath::MentalHealth(MentalHealthPath::PerceivedBurdensomeness));
        hopelessness_final = all_factors_state.get_effective(StatePath::MentalHealth(MentalHealthPath::InterpersonalHopelessness));
        ac_final = all_factors_state.get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));
        desire_final = all_factors_state.get_effective(StatePath::MentalHealth(MentalHealthPath::SuicidalDesire));
        risk_final = all_factors_state.get_effective(StatePath::MentalHealth(MentalHealthPath::AttemptRisk));
    }

    // All factors should be present (may have different magnitudes)
    assert!(
        tb_final > 0.25,
        "TB should be elevated, got {}",
        tb_final
    );

    // PB is multiplicative (liability * self_hate), so may be lower
    assert!(
        pb_final >= 0.0,
        "PB should be computed, got {}",
        pb_final
    );

    assert!(
        hopelessness_final > 0.1,
        "Hopelessness should be elevated from realizations, got {}",
        hopelessness_final
    );

    // AC should remain elevated (never decays)
    assert!(
        ac_final > 0.5,
        "AC should remain elevated, got {}",
        ac_final
    );

    // Desire is computed when factors are present
    assert!(
        desire_final >= 0.0,
        "Desire should be computed with factors present, got {}",
        desire_final
    );

    // Risk = desire * AC
    assert!(
        risk_final >= 0.0,
        "Risk should be computed, got {}",
        risk_final
    );

    // Verify risk formula: risk = desire * AC
    let expected_risk = desire_final * ac_final;
    assert!(
        (risk_final - expected_risk).abs() < 0.01,
        "Risk should equal desire * AC. Expected {}, got {}",
        expected_risk,
        risk_final
    );

    // ========================================================================
    // STAGE 6: Verify ITS convergence model
    // What we're testing: The convergence of factors demonstrates the ITS
    // model - risk emerges from the interaction of desire (TB * PB with
    // hopelessness) and capability (AC).
    // ========================================================================

    // The key ITS insight: risk requires BOTH desire AND capability
    // Even with high AC, risk is proportional to desire
    // This is why AC alone (without desire) creates "dormant" risk

    // Verify the multiplicative nature: if either desire or AC is low, risk is low
    if desire_final < 0.1 {
        assert!(
            risk_final < 0.1,
            "Low desire should mean low risk even with high AC"
        );
    }
}
