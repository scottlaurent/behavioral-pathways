//! Integration test: Duration + Species time_scale.

use behavioral_pathways::enums::Species;
use behavioral_pathways::types::Duration;

/// Tests that Duration works correctly with Species time scale.
#[test]
fn duration_with_species_time_scale() {
    let human = Species::Human;
    let dog = Species::Dog;

    let human_scale = human.time_scale();
    let dog_scale = dog.time_scale();

    assert!((human_scale - 1.0).abs() < f32::EPSILON);
    let expected_dog_scale = 80.0 / 12.0;
    assert!((dog_scale - expected_dog_scale).abs() < 0.01);

    let one_week = Duration::weeks(1);
    let human_effective_days = one_week.as_days_f64() * f64::from(human_scale);
    assert!((human_effective_days - 7.0).abs() < 0.01);

    let dog_effective_days = one_week.as_days_f64() * f64::from(dog_scale);
    assert!((dog_effective_days - 46.67).abs() < 0.5);
}
