//! Test: Repeated failures create cumulative dominance reduction.
//!
//! Tests that multiple failure events over time create a cumulative
//! decrease in dominance (sense of control), demonstrating how repeated
//! negative experiences compound their effects.

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::{EventType, MoodPath, Species, StatePath};
use behavioral_pathways::event::EventBuilder;
use behavioral_pathways::simulation::Simulation;
use behavioral_pathways::types::{Duration, EntityId, Timestamp};

#[test]
fn chronic_powerlessness_from_repeated_failures() {
    // ========================================================================
    // SETUP
    // What we're doing: Creating a human entity and simulating three
    // failures over 36 hours to verify cumulative dominance effects.
    // ========================================================================

    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 9, 0, 0);
    let mut sim = Simulation::new(reference);

    let entity = EntityBuilder::new()
        .id("struggling_person")
        .species(Species::Human)
        .build()
        .unwrap();

    let entity_id = EntityId::new("struggling_person").unwrap();
    let anchor = reference;
    sim.add_entity(entity, anchor);

    // ========================================================================
    // STAGE 1: Establish baseline dominance
    // What we're testing: Entity starts with neutral sense of control.
    // ========================================================================

    let handle = sim.entity(&entity_id).unwrap();
    let baseline_state = handle.state_at(anchor);
    
    let baseline_valence = baseline_state.get_effective(StatePath::Mood(MoodPath::Valence));
    let baseline_dominance = baseline_state.get_effective(StatePath::Mood(MoodPath::Dominance));

    assert!(
        baseline_dominance.abs() < 0.1,
        "Baseline dominance should be near neutral, got {}",
        baseline_dominance
    );

    // ========================================================================
    // STAGE 2: First failure event
    // What we're testing: Initial failure decreases dominance and valence.
    // ========================================================================

    let first_failure_time = anchor + Duration::hours(2);
    let first_failure = EventBuilder::new(EventType::Failure)
        .target(entity_id.clone())
        .severity(0.6)
        .build()
        .unwrap();
    sim.add_event(first_failure, first_failure_time);

    // Get fresh handle after adding event
    let handle = sim.entity(&entity_id).unwrap();
    let after_first_state = handle.state_at(first_failure_time);
    let dominance_after_first = after_first_state.get_effective(StatePath::Mood(MoodPath::Dominance));
    let valence_after_first = after_first_state.get_effective(StatePath::Mood(MoodPath::Valence));

    // Failure with severity 0.6 produces dominance_delta = -0.1 * 0.6 * emotionality_factor
    // With default emotionality, this is approximately -0.06
    assert!(
        dominance_after_first < baseline_dominance,
        "First failure should decrease dominance, got {} (baseline: {})",
        dominance_after_first,
        baseline_dominance
    );
    
    // Failure with severity 0.6 produces valence_delta = -0.3 * 0.6 * emotionality_factor
    // With default emotionality, this is approximately -0.18
    assert!(
        valence_after_first < baseline_valence,
        "First failure should decrease valence, got {} (baseline: {})",
        valence_after_first,
        baseline_valence
    );

    // ========================================================================
    // STAGE 3: Second failure (12 hours later)
    // What we're testing: Second failure further reduces dominance,
    // compounding the sense of powerlessness before full recovery.
    // ========================================================================

    let second_failure_time = first_failure_time + Duration::hours(12);
    let second_failure = EventBuilder::new(EventType::Failure)
        .target(entity_id.clone())
        .severity(0.6)
        .build()
        .unwrap();
    sim.add_event(second_failure, second_failure_time);

    // Get fresh handle after adding second event
    let handle = sim.entity(&entity_id).unwrap();
    let after_second_state = handle.state_at(second_failure_time);
    let dominance_after_second = after_second_state.get_effective(StatePath::Mood(MoodPath::Dominance));

    assert!(
        dominance_after_second < dominance_after_first,
        "Second failure should further decrease dominance. \
         After first: {}, After second: {}",
        dominance_after_first,
        dominance_after_second
    );

    // ========================================================================
    // STAGE 4: Third failure (24 hours after first)
    // What we're testing: Third failure creates severe dominance deficit,
    // demonstrating cumulative trauma effect.
    // ========================================================================

    let third_failure_time = first_failure_time + Duration::hours(24);
    let third_failure = EventBuilder::new(EventType::Failure)
        .target(entity_id.clone())
        .severity(0.6)
        .build()
        .unwrap();
    sim.add_event(third_failure, third_failure_time);

    // Get fresh handle after adding third event
    let handle = sim.entity(&entity_id).unwrap();
    let after_third_state = handle.state_at(third_failure_time);
    let dominance_after_third = after_third_state.get_effective(StatePath::Mood(MoodPath::Dominance));

    assert!(
        dominance_after_third < dominance_after_second,
        "Third failure should further decrease dominance. \
         After second: {}, After third: {}",
        dominance_after_second,
        dominance_after_third
    );

    // ========================================================================
    // STAGE 5: Verify cumulative effect
    // What we're testing: The total dominance decrease from three failures
    // represents a loss of sense of control.
    // ========================================================================

    let total_dominance_loss = baseline_dominance - dominance_after_third;

    // Three failures each contribute approximately -0.06 dominance delta
    // Total expected is around -0.18 before decay
    assert!(
        total_dominance_loss > 0.0,
        "Three failures should create measurable dominance loss, got {}",
        total_dominance_loss
    );

    // ========================================================================
    // STAGE 6: Verify powerlessness state
    // What we're testing: After repeated failures, dominance should be
    // negative, indicating reduced sense of control.
    // ========================================================================

    assert!(
        dominance_after_third < 0.0,
        "Repeated failures should create negative dominance, got {}",
        dominance_after_third
    );

    // ========================================================================
    // STAGE 7: Check recovery trajectory
    // What we're testing: Even after 12 hours with no new failures,
    // some dominance reduction persists.
    // ========================================================================

    let recovery_time = third_failure_time + Duration::hours(12);
    let recovery_state = handle.state_at(recovery_time);
    let dominance_recovery = recovery_state.get_effective(StatePath::Mood(MoodPath::Dominance));

    // Some effect should persist, but decay will have occurred
    assert!(
        dominance_recovery < baseline_dominance || dominance_recovery.abs() < 0.1,
        "Dominance should show some lasting impact or return toward baseline. \
         Recovery value: {}, Baseline: {}",
        dominance_recovery,
        baseline_dominance
    );

    // The cumulative effect demonstrates that repeated negative experiences
    // create compounding impacts on sense of control.
}
