//! Integration test: StateValue decay mechanics.

use behavioral_pathways::state::StateValue;
use behavioral_pathways::types::Duration;

/// Tests that StateValue stores raw half-life without species coupling.
#[test]
fn state_value_decay_is_species_agnostic() {
    let human_stress = StateValue::new(0.3)
        .with_delta(0.5)
        .with_decay_half_life(Duration::days(3));

    let dog_stress = StateValue::new(0.3)
        .with_delta(0.5)
        .with_decay_half_life(Duration::days(3));

    assert_eq!(
        human_stress.decay_half_life().unwrap().as_days(),
        dog_stress.decay_half_life().unwrap().as_days()
    );
    assert_eq!(human_stress.decay_half_life().unwrap().as_days(), 3);
    assert_eq!(dog_stress.decay_half_life().unwrap().as_days(), 3);

    let mut human_stress = human_stress;
    let mut dog_stress = dog_stress;

    human_stress.apply_decay(Duration::days(3));
    dog_stress.apply_decay(Duration::days(3));

    assert!((human_stress.delta() - dog_stress.delta()).abs() < f32::EPSILON);
    assert!((human_stress.delta() - 0.25).abs() < 0.01);
}
