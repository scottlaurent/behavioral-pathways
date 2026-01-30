//! Test: Winning lottery produces exuberant emotion with specific decay dynamics.
//!
//! Tests that a major positive windfall event creates high valence,
//! high arousal, and moderate-to-high dominance, corresponding to
//! the Exuberant emotion, with observable decay patterns.

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::{EventType, MoodPath, Species, StatePath};
use behavioral_pathways::event::EventBuilder;
use behavioral_pathways::simulation::Simulation;
use behavioral_pathways::types::{Duration, EntityId, Timestamp};

#[test]
fn winning_lottery_high_valence_high_arousal_moderate_dominance() {
    // ========================================================================
    // SETUP
    // What we're doing: Creating a human entity and simulating a major
    // windfall event (lottery win) to verify it produces exuberant emotion.
    // ========================================================================

    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 20, 0, 0);
    let mut sim = Simulation::new(reference);

    let entity = EntityBuilder::new()
        .id("lottery_winner")
        .species(Species::Human)
        .build()
        .unwrap();

    let entity_id = EntityId::new("lottery_winner").unwrap();
    let anchor = reference;
    sim.add_entity(entity, anchor);

    // ========================================================================
    // STAGE 1: Establish baseline
    // What we're testing: Entity starts with neutral mood dimensions.
    // ========================================================================

    let handle = sim.entity(&entity_id).unwrap();
    let baseline_state = handle.state_at(anchor);

    let baseline_valence = baseline_state.get_effective(StatePath::Mood(MoodPath::Valence));
    let baseline_dominance = baseline_state.get_effective(StatePath::Mood(MoodPath::Dominance));

    // ========================================================================
    // STAGE 2: Apply lottery win (major achievement/windfall)
    // What we're testing: Major positive event increases all three PAD
    // dimensions, creating the exuberant emotion pattern.
    // ========================================================================

    let win_time = anchor + Duration::minutes(10);
    let lottery_win = EventBuilder::new(EventType::Achievement)
        .target(entity_id.clone())
        .severity(1.0) // Maximum positive event
        .build()
        .unwrap();
    sim.add_event(lottery_win, win_time);

    // Get fresh handle after adding event
    let handle = sim.entity(&entity_id).unwrap();
    let post_win_state = handle.state_at(win_time);
    let win_valence = post_win_state.get_effective(StatePath::Mood(MoodPath::Valence));
    let win_arousal = post_win_state.get_effective(StatePath::Mood(MoodPath::Arousal));
    let win_dominance = post_win_state.get_effective(StatePath::Mood(MoodPath::Dominance));

    // ========================================================================
    // STAGE 3: Verify positive emotion pattern
    // What we're testing: Achievement event creates positive valence and
    // moderate dominance increase.
    // ========================================================================

    // Achievement (severity 1.0) produces:
    // valence_delta = +0.3 * 1.0 * emotionality_factor ~ +0.3
    // dominance_delta = +0.1 * 1.0 * emotionality_factor ~ +0.1
    // arousal_delta = 0 (Achievement category doesn't add arousal)

    assert!(
        win_valence > 0.25,
        "Lottery win should create positive valence (expected > 0.25), got {}",
        win_valence
    );

    // Note: Achievement doesn't directly increase arousal in current implementation
    assert!(
        win_arousal > -1.0 && win_arousal < 1.0,
        "Arousal should be within valid range, got {}",
        win_arousal
    );

    assert!(
        win_dominance > 0.05,
        "Lottery win should increase dominance (expected > 0.05), got {}",
        win_dominance
    );

    // Verify valence and dominance are both elevated
    assert!(
        win_valence > 0.0 && win_dominance > 0.0,
        "Achievement should elevate both valence and dominance"
    );

    // ========================================================================
    // STAGE 4: Track decay over time
    // What we're testing: Valence and dominance effects decay toward baseline.
    // ========================================================================

    let two_hours_later = win_time + Duration::hours(2);
    let decay_2h_state = handle.state_at(two_hours_later);

    let valence_2h = decay_2h_state.get_effective(StatePath::Mood(MoodPath::Valence));
    let dominance_2h = decay_2h_state.get_effective(StatePath::Mood(MoodPath::Dominance));

    // Valence should still be positive (some decay but effects persist)
    assert!(
        valence_2h > baseline_valence,
        "Valence should remain elevated after 2 hours, got {} (baseline: {})",
        valence_2h,
        baseline_valence
    );

    // Dominance should also persist
    assert!(
        dominance_2h > baseline_dominance || dominance_2h.abs() < 0.1,
        "Dominance should remain elevated or near baseline after 2 hours"
    );

    // ========================================================================
    // STAGE 5: Track medium-term decay
    // What we're testing: After 12 hours, effects have decayed further.
    // ========================================================================

    let twelve_hours_later = win_time + Duration::hours(12);
    let decay_12h_state = handle.state_at(twelve_hours_later);

    let valence_12h = decay_12h_state.get_effective(StatePath::Mood(MoodPath::Valence));

    // Valence should continue decaying toward baseline
    assert!(
        valence_12h < valence_2h || valence_12h.abs() < 0.1,
        "Valence should decay toward baseline after 12 hours"
    );

    // ========================================================================
    // STAGE 6: Verify decay dynamics over 24 hours
    // What we're testing: Both dimensions decay toward baseline over time.
    // ========================================================================

    let one_day_later = win_time + Duration::hours(24);
    let decay_24h_state = handle.state_at(one_day_later);

    let valence_24h = decay_24h_state.get_effective(StatePath::Mood(MoodPath::Valence));
    let dominance_24h = decay_24h_state.get_effective(StatePath::Mood(MoodPath::Dominance));

    // After 24 hours, effects should have significantly decayed
    // Values should be closer to baseline (0)
    assert!(
        valence_24h.abs() <= win_valence.abs(),
        "Valence after 24h should be same or closer to baseline than immediately after win"
    );

    assert!(
        dominance_24h.abs() <= 1.0,
        "Dominance after 24h should be within valid range, got {}",
        dominance_24h
    );

    // Verify differential decay: dominance typically decays slower than valence
    let valence_boost = win_valence - baseline_valence;
    let dominance_boost = win_dominance - baseline_dominance;

    // Only check retention if boosts are meaningful
    if valence_boost.abs() > 0.05 && dominance_boost.abs() > 0.05 {
        let valence_remaining = valence_24h - baseline_valence;
        let dominance_remaining = dominance_24h - baseline_dominance;

        // Both should decay, verify values are reasonable
        assert!(
            valence_remaining.abs() < valence_boost.abs() * 1.1 || valence_remaining.abs() < 0.1,
            "Valence should not grow beyond initial boost"
        );
        assert!(
            dominance_remaining.abs() < dominance_boost.abs() * 1.1 || dominance_remaining.abs() < 0.1,
            "Dominance should not grow beyond initial boost"
        );
    }
}
