//! Test: Job promotion increases valence and dominance with differential decay.
//!
//! Tests that a positive achievement (job promotion) increases both valence
//! and dominance dimensions, and that these dimensions decay at different rates
//! according to entity model settings.

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::{EventType, MoodPath, Species, StatePath};
use behavioral_pathways::event::EventBuilder;
use behavioral_pathways::simulation::Simulation;
use behavioral_pathways::types::{Duration, EntityId, Timestamp};

#[test]
fn job_promotion_elevates_valence_and_dominance() {
    // ========================================================================
    // SETUP
    // What we're doing: Creating a human entity and simulating a job promotion
    // event to verify it affects both valence and dominance dimensions.
    // ========================================================================

    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 9, 0, 0);
    let mut sim = Simulation::new(reference);

    let entity = EntityBuilder::new()
        .id("professional")
        .species(Species::Human)
        .build()
        .unwrap();

    let entity_id = EntityId::new("professional").unwrap();
    let anchor = reference;
    sim.add_entity(entity, anchor);

    // ========================================================================
    // STAGE 1: Establish baseline PAD values
    // What we're testing: Entity starts with neutral mood dimensions.
    // ========================================================================

    let handle = sim.entity(&entity_id).unwrap();
    let baseline_state = handle.state_at(anchor);
    
    let baseline_valence = baseline_state.get_effective(StatePath::Mood(MoodPath::Valence));
    let baseline_dominance = baseline_state.get_effective(StatePath::Mood(MoodPath::Dominance));

    assert!(
        baseline_valence.abs() < 0.1,
        "Baseline valence should be near neutral, got {}",
        baseline_valence
    );
    assert!(
        baseline_dominance.abs() < 0.1,
        "Baseline dominance should be near neutral, got {}",
        baseline_dominance
    );

    // ========================================================================
    // STAGE 2: Apply job promotion event
    // What we're testing: Achievement event increases valence (+0.4) and
    // dominance (+0.3), with arousal potentially affected.
    // ========================================================================

    let promotion_time = anchor + Duration::hours(1);
    let promotion_event = EventBuilder::new(EventType::Achievement)
        .target(entity_id.clone())
        .severity(0.7) // Strong positive event
        .build()
        .unwrap();
    sim.add_event(promotion_event, promotion_time);

    // Get fresh handle after adding event
    let handle = sim.entity(&entity_id).unwrap();
    let post_promotion_state = handle.state_at(promotion_time);
    let post_valence = post_promotion_state.get_effective(StatePath::Mood(MoodPath::Valence));
    let post_dominance = post_promotion_state.get_effective(StatePath::Mood(MoodPath::Dominance));

    assert!(
        post_valence > baseline_valence + 0.3,
        "Valence should increase by at least 0.3 from promotion, got {} (baseline {})",
        post_valence,
        baseline_valence
    );
    
    assert!(
        post_dominance > baseline_dominance + 0.2,
        "Dominance should increase by at least 0.2 from promotion, got {} (baseline {})",
        post_dominance,
        baseline_dominance
    );

    // ========================================================================
    // STAGE 3: Verify differential decay after 6 hours
    // What we're testing: Valence decays faster than dominance. After 6 hours,
    // valence should have decayed more significantly than dominance.
    // ========================================================================

    let six_hours_later = promotion_time + Duration::hours(6);
    let decay_state_6h = handle.state_at(six_hours_later);
    
    let valence_6h = decay_state_6h.get_effective(StatePath::Mood(MoodPath::Valence));
    let dominance_6h = decay_state_6h.get_effective(StatePath::Mood(MoodPath::Dominance));

    // Valence should have decayed more than dominance
    let valence_decay_amount = post_valence - valence_6h;
    let dominance_decay_amount = post_dominance - dominance_6h;

    assert!(
        valence_decay_amount > 0.0,
        "Valence should have decayed after 6 hours"
    );
    
    assert!(
        dominance_decay_amount >= 0.0,
        "Dominance should have decayed or remained stable"
    );

    // ========================================================================
    // STAGE 4: Verify continued decay after 12 hours
    // What we're testing: Both dimensions continue to decay toward baseline,
    // with dominance showing slower decay rate over the full 12-hour period.
    // ========================================================================

    let twelve_hours_later = promotion_time + Duration::hours(12);
    let decay_state_12h = handle.state_at(twelve_hours_later);
    
    let valence_12h = decay_state_12h.get_effective(StatePath::Mood(MoodPath::Valence));
    let dominance_12h = decay_state_12h.get_effective(StatePath::Mood(MoodPath::Dominance));

    assert!(
        valence_12h < post_valence,
        "Valence should have decayed significantly after 12 hours"
    );
    
    assert!(
        dominance_12h < post_dominance,
        "Dominance should have decayed after 12 hours"
    );

    // Dominance should retain more of its elevation relative to valence
    let valence_retention = (valence_12h - baseline_valence) / (post_valence - baseline_valence);
    let dominance_retention = (dominance_12h - baseline_dominance) / (post_dominance - baseline_dominance);

    assert!(
        dominance_retention > valence_retention,
        "Dominance should retain more of its boost than valence after 12 hours. \
         Valence retention: {:.2}, Dominance retention: {:.2}",
        valence_retention,
        dominance_retention
    );
}
