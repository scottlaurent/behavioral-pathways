//! Test: Events at anchor timestamp are excluded during forward projection.
//!
//! The anchor state represents the entity's state at the anchor timestamp,
//! which already incorporates any events that occurred at that exact time.
//! Therefore, when projecting forward from the anchor, we must exclude
//! events at the anchor timestamp to avoid double-counting their effects.

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::{EventType, MoodPath, Species, StatePath};
use behavioral_pathways::event::EventBuilder;
use behavioral_pathways::simulation::Simulation;
use behavioral_pathways::types::{Duration, EntityId, Timestamp};

/// Events occurring exactly at anchor timestamp should NOT be applied
/// during forward projection, as they are already reflected in the anchor state.
#[test]
fn event_at_anchor_excluded_forward() {
    // Setup: Create simulation with reference date
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    // Create entity with known initial state
    let entity = EntityBuilder::new()
        .id("person")
        .species(Species::Human)
        .build()
        .unwrap();

    let entity_id = EntityId::new("person").unwrap();
    let anchor = reference;
    sim.add_entity(entity, anchor);

    // Add an event at EXACTLY the anchor timestamp
    let event_at_anchor = EventBuilder::new(EventType::SocialExclusion)
        .target(entity_id.clone())
        .severity(0.8)
        .build()
        .unwrap();
    sim.add_event(event_at_anchor, anchor); // Same timestamp as anchor

    // Query state at anchor
    let handle = sim.entity(&entity_id).unwrap();
    let state_at_anchor = handle.state_at(anchor);
    // get_effective now returns f64 directly
    let valence_at_anchor = state_at_anchor.get_effective(StatePath::Mood(MoodPath::Valence));

    // Query state 1 hour after anchor
    let one_hour_later = anchor + Duration::hours(1);
    let state_forward = handle.state_at(one_hour_later);
    let valence_forward = state_forward.get_effective(StatePath::Mood(MoodPath::Valence));

    // The event at anchor should NOT be applied when projecting forward.
    // Both states should show the original valence (near 0), with only decay
    // applied for the forward state.
    // If the event WERE applied, valence would be significantly negative.

    // Anchor state should have valence near 0 (default)
    assert!(
        valence_at_anchor.abs() < 0.05,
        "Anchor state should have near-zero valence, got {}",
        valence_at_anchor
    );

    // Forward state should also have valence near 0 (only decay from anchor)
    // If event were applied, valence would be around -0.24 (0.8 * -0.3)
    assert!(
        valence_forward.abs() < 0.05,
        "Forward state should have near-zero valence (event at anchor excluded), got {}",
        valence_forward
    );
}
