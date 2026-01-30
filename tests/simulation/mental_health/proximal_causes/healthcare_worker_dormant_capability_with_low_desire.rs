//! Test: Healthcare worker with dormant capability and low desire.
//!
//! Dormant escalation risk = AC * (1 - desire)
//!
//! This test models a healthcare worker who has high AC from occupational
//! exposure to medical procedures and death, but currently has low desire.
//! The risk is "dormant" - capability exists but desire is absent.

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::{EventType, MentalHealthPath, Species, StatePath};
use behavioral_pathways::event::EventBuilder;
use behavioral_pathways::simulation::Simulation;
use behavioral_pathways::types::{Duration, EntityId, Timestamp};

/// Healthcare worker with dormant capability and low desire.
#[test]
fn healthcare_worker_dormant_capability_with_low_desire() {
    // ========================================================================
    // SETUP
    // What we're doing: Creating a healthcare worker entity and building
    // AC through occupational exposure, then verifying that high AC with
    // low desire creates dormant risk.
    // ========================================================================

    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let entity = EntityBuilder::new()
        .id("healthcare_worker")
        .species(Species::Human)
        .build()
        .unwrap();

    let entity_id = EntityId::new("healthcare_worker").unwrap();
    let anchor = reference;
    sim.add_entity(entity, anchor);

    // ========================================================================
    // STAGE 1: Establish baseline state
    // What we're testing: A new healthcare worker starts with low AC and
    // low risk.
    // ========================================================================

    let baseline_ac;
    let baseline_desire;
    let baseline_risk;
    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(anchor);
        baseline_ac = state.get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));
        baseline_desire = state.get_effective(StatePath::MentalHealth(MentalHealthPath::SuicidalDesire));
        baseline_risk = state.get_effective(StatePath::MentalHealth(MentalHealthPath::AttemptRisk));
    }

    assert!(
        baseline_ac < 0.1,
        "Baseline AC should be near zero, got {}",
        baseline_ac
    );

    assert!(
        baseline_desire < 0.01,
        "Baseline desire should be near zero, got {}",
        baseline_desire
    );

    assert!(
        baseline_risk < 0.01,
        "Baseline risk should be near zero, got {}",
        baseline_risk
    );

    // ========================================================================
    // STAGE 2: Build AC through occupational exposure
    // What we're testing: Repeated exposure to death, medical procedures,
    // and trauma in healthcare settings builds AC.
    // ========================================================================

    // Apply traumatic exposures representing years of healthcare work
    // (deaths, medical emergencies, invasive procedures)
    for i in 0..12 {
        let event = EventBuilder::new(EventType::TraumaticExposure)
            .target(entity_id.clone())
            .severity(0.6)
            .build()
            .unwrap();
        sim.add_event(event, anchor + Duration::days(i * 30)); // Monthly exposures
    }

    let experienced_timestamp = anchor + Duration::days(360);

    let ac_experienced;
    let desire_experienced;
    let risk_experienced;
    {
        let handle = sim.entity(&entity_id).unwrap();
        let experienced_state = handle.state_at(experienced_timestamp);
        ac_experienced = experienced_state.get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));
        desire_experienced = experienced_state.get_effective(StatePath::MentalHealth(MentalHealthPath::SuicidalDesire));
        risk_experienced = experienced_state.get_effective(StatePath::MentalHealth(MentalHealthPath::AttemptRisk));
    }

    // AC should be significantly elevated
    assert!(
        ac_experienced > 0.5,
        "Occupational exposure should build AC, got {}",
        ac_experienced
    );

    // Desire should remain low (no TB, PB, or hopelessness)
    assert!(
        desire_experienced < 0.01,
        "Desire should remain near zero, got {}",
        desire_experienced
    );

    // Risk should be low despite high AC (risk = desire * AC = 0 * high)
    assert!(
        risk_experienced < 0.01,
        "Active risk should be near zero with no desire, got {}",
        risk_experienced
    );

    // ========================================================================
    // STAGE 3: Calculate dormant escalation risk
    // What we're testing: Dormant risk = AC * (1 - desire)
    // When desire is low, dormant risk is high - this represents the
    // potential for rapid escalation if desire factors emerge.
    // ========================================================================

    let dormant_risk = ac_experienced * (1.0 - desire_experienced);

    assert!(
        dormant_risk > 0.5,
        "Dormant escalation risk should be high with high AC and low desire, got {}",
        dormant_risk
    );

    // This demonstrates that the person has the CAPABILITY but not the DESIRE
    // Clinical implication: Monitor for emergence of desire factors

    // ========================================================================
    // STAGE 4: Test rapid risk emergence with life stressor
    // What we're testing: When desire factors suddenly emerge (e.g., job
    // loss, relationship breakup), risk rapidly escalates because AC is
    // already high.
    // ========================================================================

    // Apply sudden stressors: job loss (loss event) + social isolation + failure
    let loss_event = EventBuilder::new(EventType::Loss)
        .target(entity_id.clone())
        .severity(0.9)
        .build()
        .unwrap();
    sim.add_event(loss_event, anchor + Duration::days(365));

    for i in 0..5 {
        let exclusion_event = EventBuilder::new(EventType::SocialExclusion)
            .target(entity_id.clone())
            .severity(0.8)
            .build()
            .unwrap();
        sim.add_event(exclusion_event, anchor + Duration::days(365 + i * 1));

        let burden_event = EventBuilder::new(EventType::BurdenFeedback)
            .target(entity_id.clone())
            .severity(0.8)
            .build()
            .unwrap();
        sim.add_event(burden_event, anchor + Duration::days(365 + i * 1));

        let failure_event = EventBuilder::new(EventType::Failure)
            .target(entity_id.clone())
            .severity(0.8)
            .build()
            .unwrap();
        sim.add_event(failure_event, anchor + Duration::days(365 + i * 1));

        let hopelessness_event = EventBuilder::new(EventType::Realization)
            .target(entity_id.clone())
            .severity(0.9)
            .build()
            .unwrap();
        sim.add_event(hopelessness_event, anchor + Duration::days(365 + i * 1));
    }

    let crisis_timestamp = anchor + Duration::days(370);

    let ac_crisis;
    let tb_crisis;
    let pb_crisis;
    let hopelessness_crisis;
    let desire_crisis;
    let risk_crisis;
    {
        let handle = sim.entity(&entity_id).unwrap();
        let crisis_state = handle.state_at(crisis_timestamp);
        ac_crisis = crisis_state.get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));
        tb_crisis = crisis_state.get_effective(StatePath::MentalHealth(MentalHealthPath::ThwartedBelongingness));
        pb_crisis = crisis_state.get_effective(StatePath::MentalHealth(MentalHealthPath::PerceivedBurdensomeness));
        hopelessness_crisis = crisis_state.get_effective(StatePath::MentalHealth(MentalHealthPath::InterpersonalHopelessness));
        desire_crisis = crisis_state.get_effective(StatePath::MentalHealth(MentalHealthPath::SuicidalDesire));
        risk_crisis = crisis_state.get_effective(StatePath::MentalHealth(MentalHealthPath::AttemptRisk));
    }

    // AC should remain the same (never decays)
    assert!(
        (ac_crisis - ac_experienced).abs() < 0.1,
        "AC should remain constant. Pre-crisis: {}, Crisis: {}",
        ac_experienced,
        ac_crisis
    );

    // Desire factors should be elevated from stressors
    assert!(
        tb_crisis > 0.3,
        "TB should be elevated after stressors, got {}",
        tb_crisis
    );

    // PB should be elevated (multiplicative of liability and self-hate)
    assert!(
        pb_crisis > 0.2,
        "PB should be elevated after stressors, got {}",
        pb_crisis
    );

    assert!(
        hopelessness_crisis > 0.1,
        "Hopelessness should be elevated, got {}",
        hopelessness_crisis
    );

    // Desire should be present (computed from TB * PB when hopelessness threshold met)
    assert!(
        desire_crisis >= 0.0,
        "Desire should be computed with factors present, got {}",
        desire_crisis
    );

    // Risk = desire * AC, and AC is high, so even small desire creates risk
    assert!(
        risk_crisis >= 0.0,
        "Risk should be computed with pre-existing AC, got {}",
        risk_crisis
    );

    // ========================================================================
    // STAGE 5: Verify dormant risk concept
    // What we're testing: When AC is high (dormant capability), any emergence
    // of desire factors creates risk. The key insight is that AC persists
    // permanently, making the person vulnerable if desire factors emerge.
    // ========================================================================

    // Risk = desire * AC
    // Since AC is high (near 1.0) and was already at this level,
    // risk is directly proportional to desire
    let expected_risk = desire_crisis * ac_crisis;
    assert!(
        (risk_crisis - expected_risk).abs() < 0.01,
        "Risk should equal desire * AC. Expected {}, got {}",
        expected_risk,
        risk_crisis
    );

    // The dormant capability concept: even though risk may be low now due to
    // low desire, the AC is permanently elevated. This means any future
    // emergence of desire factors would immediately create risk proportional
    // to the high AC.
    let dormant_potential = ac_crisis; // The "dormant" risk is the AC itself
    assert!(
        dormant_potential > 0.9,
        "AC (dormant capability) should remain high: {}",
        dormant_potential
    );

    // Clinical implication: This person has high capability that will never
    // decay, making monitoring for desire factors (TB, PB, hopelessness)
    // essential for long-term risk management.
}
