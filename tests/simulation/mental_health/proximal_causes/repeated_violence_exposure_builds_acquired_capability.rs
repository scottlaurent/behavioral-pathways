//! Test: Repeated violence exposure builds acquired capability which never decays.
//!
//! AC accumulates permanently from traumatic exposure and NEVER decays.
//! This test verifies that AC increases with trauma events and persists
//! indefinitely, even after decades.

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::{EventType, MentalHealthPath, Species, StatePath};
use behavioral_pathways::event::EventBuilder;
use behavioral_pathways::simulation::Simulation;
use behavioral_pathways::types::{Duration, EntityId, Timestamp};

/// Repeated violence exposure builds acquired capability which never decays.
#[test]
fn repeated_violence_exposure_builds_acquired_capability() {
    // ========================================================================
    // SETUP
    // What we're doing: Creating a person and exposing them to repeated
    // violence to build AC, then verifying AC never decays.
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
    // STAGE 1: Establish baseline AC
    // What we're testing: A person with no trauma exposure should have
    // very low AC (near zero habituation to pain/fear).
    // ========================================================================

    let baseline_ac;
    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(anchor);
        baseline_ac = state.get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));
    }

    assert!(
        baseline_ac < 0.1,
        "Baseline AC should be near zero, got {}",
        baseline_ac
    );

    // ========================================================================
    // STAGE 2: Apply repeated violence exposure
    // What we're testing: Each traumatic exposure should incrementally
    // increase AC as habituation builds.
    // ========================================================================

    // Apply 8 violence events over 2 months
    for i in 0..8 {
        let event = EventBuilder::new(EventType::Violence)
            .target(entity_id.clone())
            .severity(0.7)
            .build()
            .unwrap();
        sim.add_event(event, anchor + Duration::days(i * 7)); // Weekly
    }

    let post_trauma_timestamp = anchor + Duration::days(56);

    let post_trauma_ac;
    {
        let handle = sim.entity(&entity_id).unwrap();
        let post_trauma_state = handle.state_at(post_trauma_timestamp);
        post_trauma_ac = post_trauma_state.get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));
    }

    // AC should be significantly elevated after repeated exposure
    assert!(
        post_trauma_ac > baseline_ac + 0.4,
        "Repeated violence should increase AC. Baseline: {}, Post-trauma: {}",
        baseline_ac,
        post_trauma_ac
    );

    assert!(
        post_trauma_ac > 0.5,
        "Severe repeated trauma should push AC above 0.5, got {}",
        post_trauma_ac
    );

    // ========================================================================
    // STAGE 3: Verify AC never decays (short term - 1 year)
    // What we're testing: After 1 year with no trauma, AC should remain
    // unchanged.
    // ========================================================================

    let one_year_later = post_trauma_timestamp + Duration::days(365);

    let ac_after_one_year;
    {
        let handle = sim.entity(&entity_id).unwrap();
        let one_year_state = handle.state_at(one_year_later);
        ac_after_one_year = one_year_state.get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));
    }

    assert!(
        (ac_after_one_year - post_trauma_ac).abs() < 0.01,
        "AC should not decay over 1 year. Post-trauma: {}, After 1 year: {}",
        post_trauma_ac,
        ac_after_one_year
    );

    // ========================================================================
    // STAGE 4: Verify AC never decays (long term - 20 years)
    // What we're testing: Even after decades, AC remains at the same level.
    // This is PERMANENT habituation.
    // ========================================================================

    let twenty_years_later = post_trauma_timestamp + Duration::days(365 * 20);

    let ac_after_twenty_years;
    {
        let handle = sim.entity(&entity_id).unwrap();
        let twenty_year_state = handle.state_at(twenty_years_later);
        ac_after_twenty_years = twenty_year_state.get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));
    }

    assert!(
        (ac_after_twenty_years - post_trauma_ac).abs() < 0.01,
        "AC should not decay over 20 years. Post-trauma: {}, After 20 years: {}",
        post_trauma_ac,
        ac_after_twenty_years
    );

    // ========================================================================
    // STAGE 5: Verify additional trauma maintains or increases AC
    // What we're testing: AC can only increase or stay the same, never
    // decrease. Additional trauma adds to existing AC (up to max 1.0).
    // ========================================================================

    // Apply more trauma after the long gap
    for i in 0..3 {
        let event = EventBuilder::new(EventType::TraumaticExposure)
            .target(entity_id.clone())
            .severity(0.8)
            .build()
            .unwrap();
        sim.add_event(event, twenty_years_later + Duration::days(i * 7));
    }

    let final_timestamp = twenty_years_later + Duration::days(21);

    let final_ac;
    {
        let handle = sim.entity(&entity_id).unwrap();
        let final_state = handle.state_at(final_timestamp);
        final_ac = final_state.get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));
    }

    // AC should be at or above previous level (capped at 1.0)
    assert!(
        final_ac >= ac_after_twenty_years,
        "Additional trauma should maintain or increase AC. Before: {}, After: {}",
        ac_after_twenty_years,
        final_ac
    );

    // If AC was already at max (1.0), it should stay at max
    if ac_after_twenty_years >= 0.99 {
        assert!(
            (final_ac - ac_after_twenty_years).abs() < 0.01,
            "AC at max should remain at max. Before: {}, After: {}",
            ac_after_twenty_years,
            final_ac
        );
    }
}
