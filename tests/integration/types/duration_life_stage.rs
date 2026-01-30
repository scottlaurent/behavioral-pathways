//! Integration test: Duration + LifeStage determination.

use behavioral_pathways::enums::{LifeStage, Species};
use behavioral_pathways::types::Duration;

/// Tests that LifeStage can be determined from Duration age using the species-aware API.
#[test]
fn life_stage_from_duration_age() {
    let child_age = Duration::years(8);
    let adolescent_age = Duration::years(15);
    let young_adult_age = Duration::years(25);
    let adult_age = Duration::years(40);
    let mature_adult_age = Duration::years(60);
    let elder_age = Duration::years(80);

    assert_eq!(
        LifeStage::from_age_years_for_species(&Species::Human, child_age.as_years_f64()),
        LifeStage::Child
    );
    assert_eq!(
        LifeStage::from_age_years_for_species(&Species::Human, adolescent_age.as_years_f64()),
        LifeStage::Adolescent
    );
    assert_eq!(
        LifeStage::from_age_years_for_species(&Species::Human, young_adult_age.as_years_f64()),
        LifeStage::YoungAdult
    );
    assert_eq!(
        LifeStage::from_age_years_for_species(&Species::Human, adult_age.as_years_f64()),
        LifeStage::Adult
    );
    assert_eq!(
        LifeStage::from_age_years_for_species(&Species::Human, mature_adult_age.as_years_f64()),
        LifeStage::MatureAdult
    );
    assert_eq!(
        LifeStage::from_age_years_for_species(&Species::Human, elder_age.as_years_f64()),
        LifeStage::Elder
    );
}
