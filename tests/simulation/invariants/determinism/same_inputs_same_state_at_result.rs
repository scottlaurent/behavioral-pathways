//! Test: Same inputs produce same state_at results
//!
//! Scenario: Create identical simulations and query at same timestamp
//! Expected: Results are exactly equal

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::{EventType, MoodPath, Species, StatePath};
use behavioral_pathways::event::EventBuilder;
use behavioral_pathways::simulation::Simulation;
use behavioral_pathways::types::{Duration, EntityId, Timestamp};

/// Same inputs to state_at() produce identical results.
///
/// Stage 1: Create two identical simulations
/// Stage 2: Add identical entities and events to both
/// Stage 3: Query state at same timestamp in both
/// Stage 4: Verify results are exactly equal
#[test]
fn same_inputs_same_state_at_result() {
    // Stage 1 & 2: Create two identical simulations
    fn create_simulation() -> Simulation {
        let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let mut sim = Simulation::new(reference);

        let entity = EntityBuilder::new()
            .id("person_001")
            .species(Species::Human)
            .age(Duration::years(30))
            .build()
            .unwrap();

        sim.add_entity(entity, reference);

        // Add some events
        let event1 = EventBuilder::new(EventType::SocialExclusion)
            .target(EntityId::new("person_001").unwrap())
            .severity(0.7)
            .build()
            .unwrap();
        sim.add_event(event1, reference + Duration::days(10));

        let event2 = EventBuilder::new(EventType::SocialInclusion)
            .target(EntityId::new("person_001").unwrap())
            .severity(0.5)
            .build()
            .unwrap();
        sim.add_event(event2, reference + Duration::days(20));

        sim
    }

    let sim1 = create_simulation();
    let sim2 = create_simulation();

    let entity_id = EntityId::new("person_001").unwrap();
    let query_time = Timestamp::from_ymd_hms(2024, 2, 15, 12, 0, 0);

    // Stage 3: Query both simulations at the same timestamp
    let handle1 = sim1.entity(&entity_id).unwrap();
    let handle2 = sim2.entity(&entity_id).unwrap();

    let state1 = handle1.state_at(query_time);
    let state2 = handle2.state_at(query_time);

    // Stage 4: Verify results are exactly equal
    // Age
    assert_eq!(
        state1.age_at_timestamp(),
        state2.age_at_timestamp(),
        "Age should be identical"
    );

    // Life stage
    assert_eq!(
        state1.life_stage(),
        state2.life_stage(),
        "Life stage should be identical"
    );

    // Regression quality
    assert_eq!(
        state1.regression_quality(),
        state2.regression_quality(),
        "Regression quality should be identical"
    );

    // Mood dimensions - get_effective now returns f64 directly
    let v1 = state1.get_effective(StatePath::Mood(MoodPath::Valence));
    let v2 = state2.get_effective(StatePath::Mood(MoodPath::Valence));
    assert!(
        (v1 - v2).abs() < f64::EPSILON,
        "Valence should be identical: {} vs {}",
        v1,
        v2
    );

    let a1 = state1.get_effective(StatePath::Mood(MoodPath::Arousal));
    let a2 = state2.get_effective(StatePath::Mood(MoodPath::Arousal));
    assert!(
        (a1 - a2).abs() < f64::EPSILON,
        "Arousal should be identical: {} vs {}",
        a1,
        a2
    );

    let d1 = state1.get_effective(StatePath::Mood(MoodPath::Dominance));
    let d2 = state2.get_effective(StatePath::Mood(MoodPath::Dominance));
    assert!(
        (d1 - d2).abs() < f64::EPSILON,
        "Dominance should be identical: {} vs {}",
        d1,
        d2
    );
}

/// Repeated queries on the same simulation produce identical results.
#[test]
fn repeated_queries_produce_identical_results() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let entity = EntityBuilder::new()
        .id("person_001")
        .species(Species::Human)
        .age(Duration::years(30))
        .build()
        .unwrap();

    sim.add_entity(entity, reference);

    let entity_id = EntityId::new("person_001").unwrap();
    let query_time = Timestamp::from_ymd_hms(2024, 6, 15, 12, 0, 0);

    // Query the same timestamp multiple times
    let handle = sim.entity(&entity_id).unwrap();

    let result1 = handle.state_at(query_time);
    let result2 = handle.state_at(query_time);
    let result3 = handle.state_at(query_time);

    // All results should be identical
    assert_eq!(result1.age_at_timestamp(), result2.age_at_timestamp());
    assert_eq!(result2.age_at_timestamp(), result3.age_at_timestamp());

    // get_effective now returns f64 directly
    let v1 = result1.get_effective(StatePath::Mood(MoodPath::Valence));
    let v2 = result2.get_effective(StatePath::Mood(MoodPath::Valence));
    let v3 = result3.get_effective(StatePath::Mood(MoodPath::Valence));

    assert!((v1 - v2).abs() < f64::EPSILON);
    assert!((v2 - v3).abs() < f64::EPSILON);
}
