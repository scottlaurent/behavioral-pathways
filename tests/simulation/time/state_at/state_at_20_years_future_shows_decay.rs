//! Test: state_at 20 years in the future shows decay
//!
//! Scenario: Query an entity's state 20 years after anchor
//! Expected: All mood deltas should have fully decayed to baseline

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::{MoodPath, Species, StatePath};
use behavioral_pathways::simulation::Simulation;
use behavioral_pathways::types::{Duration, EntityId, Timestamp};

/// Querying state 20 years in the future shows complete mood decay.
///
/// Stage 1: Create entity with elevated mood deltas at anchor
/// Stage 2: Query state 20 years in the future
/// Stage 3: Verify all mood deltas have decayed to near-baseline
#[test]
fn state_at_20_years_future_shows_decay() {
    // Stage 1: Setup - entity at age 30 with elevated mood
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    // Entity born 30 years before reference (age 30)
    let birth_date = reference - Duration::years(30);
    let mut entity = EntityBuilder::new()
        .id("person_001")
        .species(Species::Human)
        .birth_date(birth_date)
        .age(Duration::years(30))
        .build()
        .unwrap();

    // Add significant mood deltas
    entity
        .individual_state_mut()
        .mood_mut()
        .add_valence_delta(0.8);
    entity
        .individual_state_mut()
        .mood_mut()
        .add_arousal_delta(0.6);
    entity
        .individual_state_mut()
        .mood_mut()
        .add_dominance_delta(0.4);

    sim.add_entity(entity, reference);

    let entity_id = EntityId::new("person_001").unwrap();
    let handle = sim.entity(&entity_id).unwrap();

    // Verify initial state has elevated deltas
    let initial_state = handle.state_at(reference);
    let initial_valence = initial_state.get_effective(StatePath::Mood(MoodPath::Valence));
    assert!(
        initial_valence.abs() > 0.1,
        "Initial valence delta should be significant"
    );

    // Stage 2: Query state 20 years later
    let future = reference + Duration::years(20);
    let future_state = handle.state_at(future);

    // Stage 3: Verify decay
    // After 20 years, mood deltas should have fully decayed
    // (mood has 6-hour half-life, so after 20 years = millions of half-lives)
    let valence_delta = future_state.individual_state().mood().valence_delta();
    let arousal_delta = future_state.individual_state().mood().arousal_delta();
    let dominance_delta = future_state.individual_state().mood().dominance_delta();

    // All deltas should be essentially zero
    assert!(
        valence_delta.abs() < 0.001,
        "Valence delta should be near zero after 20 years, got {}",
        valence_delta
    );
    assert!(
        arousal_delta.abs() < 0.001,
        "Arousal delta should be near zero after 20 years, got {}",
        arousal_delta
    );
    assert!(
        dominance_delta.abs() < 0.001,
        "Dominance delta should be near zero after 20 years, got {}",
        dominance_delta
    );

    // Age should reflect 20 years of aging
    let age = future_state.age_at_timestamp();
    assert_eq!(age.as_years(), 50);
}
