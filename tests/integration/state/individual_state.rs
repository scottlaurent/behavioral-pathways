//! Integration test: IndividualState aggregate operations.
//!
//! Validates that IndividualState correctly:
//! - Propagates decay to all state modules
//! - Composes components via builder pattern

use behavioral_pathways::enums::PersonalityProfile;
use behavioral_pathways::state::{
    Disposition, EntityModelConfig, Hexaco, IndividualState, MentalHealth, Mood, Needs,
    PersonCharacteristics,
};
use behavioral_pathways::types::Duration;

/// Tests that IndividualState::apply_decay propagates to all state modules.
///
/// Validates:
/// - Decay is applied uniformly across all modules that support decay
/// - StateValue decay works when called on the aggregate
/// - Non-decaying values (Hexaco personality, Acquired Capability) are preserved
#[test]
fn individual_state_applies_decay_to_all_components() {
    let mut state = IndividualState::new();

    // Add deltas to all decaying components
    state.mood_mut().add_valence_delta(0.8);
    state.mood_mut().add_arousal_delta(0.8);
    state.mood_mut().add_dominance_delta(0.8);

    state.needs_mut().add_fatigue_delta(0.8);
    state.needs_mut().add_stress_delta(0.8);
    state.social_cognition_mut().add_loneliness_delta(0.8);

    state.mental_health_mut().add_depression_delta(0.8);
    state.mental_health_mut().add_hopelessness_delta(0.8);
    state.mental_health_mut().add_acquired_capability_delta(0.8); // Should NOT decay

    state.disposition_mut().add_aggression_delta(0.8);
    state.disposition_mut().add_empathy_delta(-0.3);

    state
        .person_characteristics_mut()
        .social_capital_mut()
        .add_delta(0.8);

    // Set a distinctive Hexaco value to verify it's preserved
    state.hexaco_mut().set_openness(0.9);

    // Apply significant decay (1 week)
    state.apply_decay(Duration::weeks(1));

    // Mood should have decayed significantly (6 hour half-life for valence)
    // After 1 week = 168 hours = 28 half-lives, delta should be essentially 0
    assert!(
        state.mood().valence_delta().abs() < 0.01,
        "Mood valence should decay, got delta = {}",
        state.mood().valence_delta()
    );
    assert!(
        state.mood().arousal_delta().abs() < 0.01,
        "Mood arousal should decay, got delta = {}",
        state.mood().arousal_delta()
    );

    // Needs should have decayed (fatigue has 8 hour half-life)
    assert!(
        state.needs().fatigue().delta() < 0.1,
        "Needs fatigue should decay, got delta = {}",
        state.needs().fatigue().delta()
    );
    assert!(
        state.needs().stress().delta() < 0.1,
        "Needs stress should decay, got delta = {}",
        state.needs().stress().delta()
    );

    // Mental health should have decayed (depression has 1 week half-life)
    // After exactly 1 half-life, delta should be halved
    assert!(
        (state.mental_health().depression().delta() - 0.4).abs() < 0.05,
        "Mental health depression should be halved, got delta = {}",
        state.mental_health().depression().delta()
    );

    // Acquired Capability should NOT decay
    assert!(
        (state.mental_health().acquired_capability().delta() - 0.8).abs() < f32::EPSILON,
        "Acquired capability should NOT decay, got delta = {}",
        state.mental_health().acquired_capability().delta()
    );

    // Disposition should have decayed (1 month half-life, 1 week elapsed = ~15% decay)
    assert!(
        state.disposition().aggression().delta() < 0.75,
        "Disposition aggression should decay, got delta = {}",
        state.disposition().aggression().delta()
    );

    // PersonCharacteristics should have decayed (1 month half-life, 1 week elapsed = ~15% decay)
    assert!(
        state.person_characteristics().social_capital().delta() < 0.75,
        "PersonCharacteristics social_capital should decay, got delta = {}",
        state.person_characteristics().social_capital().delta()
    );

    // Hexaco (personality) should be unchanged - it's stable, no decay
    assert!(
        (state.hexaco().openness() - 0.9).abs() < f32::EPSILON,
        "Hexaco personality should be stable, got openness = {}",
        state.hexaco().openness()
    );
}

/// Tests that builder methods correctly set components without overwriting others.
///
/// Validates:
/// - Each with_* method sets only its target component
/// - Components set earlier are preserved when setting later ones
/// - Mutable accessors modify the correct component
#[test]
fn individual_state_builder_pattern_composes_components() {
    // Create distinct instances of each component
    let hexaco = Hexaco::from_profile(PersonalityProfile::Leader);
    let mood = Mood::new().with_valence_base(0.7).with_arousal_base(0.6);
    let needs = Needs::new().with_fatigue_base(0.4).with_stress_base(0.5);
    let mental_health = MentalHealth::new()
        .with_depression_base(0.2)
        .with_self_worth_base(0.8);
    let disposition = Disposition::new()
        .with_aggression_base(0.1)
        .with_empathy_base(0.9);
    let person_characteristics = PersonCharacteristics::new()
        .with_social_capital_base(0.7)
        .with_material_security_base(0.8);
    let config = EntityModelConfig::animal_simple();

    // Build state with all components
    let state = IndividualState::new()
        .with_hexaco(hexaco.clone())
        .with_mood(mood.clone())
        .with_needs(needs.clone())
        .with_mental_health(mental_health.clone())
        .with_disposition(disposition.clone())
        .with_person_characteristics(person_characteristics.clone())
        .with_config(config.clone());

    // Verify each component is correctly set
    assert_eq!(state.hexaco(), &hexaco, "Hexaco should match the set value");
    assert_eq!(state.mood(), &mood, "Mood should match the set value");
    assert_eq!(state.needs(), &needs, "Needs should match the set value");
    assert_eq!(
        state.mental_health(),
        &mental_health,
        "MentalHealth should match the set value"
    );
    assert_eq!(
        state.disposition(),
        &disposition,
        "Disposition should match the set value"
    );
    assert_eq!(
        state.person_characteristics(),
        &person_characteristics,
        "PersonCharacteristics should match the set value"
    );
    assert_eq!(
        state.config(),
        &config,
        "EntityModelConfig should match the set value"
    );

    // Verify specific values from each component to ensure no corruption
    // Leader profile has high extraversion (0.8 in 0-1 scale = 0.6 in -1 to 1 scale)
    assert!(
        state.hexaco().extraversion() > 0.5,
        "Leader profile should have high extraversion"
    );
    assert!(
        (state.mood().valence_base() - 0.7).abs() < f32::EPSILON,
        "Mood valence base should be 0.7"
    );
    assert!(
        (state.needs().fatigue_base() - 0.4).abs() < f32::EPSILON,
        "Needs fatigue base should be 0.4"
    );
    assert!(
        (state.mental_health().depression().base() - 0.2).abs() < f32::EPSILON,
        "MentalHealth depression base should be 0.2"
    );
    assert!(
        (state.disposition().empathy().base() - 0.9).abs() < f32::EPSILON,
        "Disposition empathy base should be 0.9"
    );
    assert!(
        (state.person_characteristics().social_capital().base() - 0.7).abs() < f32::EPSILON,
        "PersonCharacteristics social_capital base should be 0.7"
    );

    // Test that setting components in different orders produces same result
    let state_reversed = IndividualState::new()
        .with_config(config.clone())
        .with_person_characteristics(person_characteristics.clone())
        .with_disposition(disposition.clone())
        .with_mental_health(mental_health.clone())
        .with_needs(needs.clone())
        .with_mood(mood.clone())
        .with_hexaco(hexaco.clone());

    assert_eq!(
        state, state_reversed,
        "Builder order should not affect final state"
    );

    // Test mutable accessors modify correct component without affecting others
    let mut mutable_state = IndividualState::new()
        .with_hexaco(hexaco.clone())
        .with_mood(mood.clone())
        .with_needs(needs.clone());

    // Modify mood
    mutable_state.mood_mut().add_valence_delta(0.2);

    // Verify mood was modified
    assert!(
        (mutable_state.mood().valence_delta() - 0.2).abs() < f32::EPSILON,
        "Mood valence delta should be 0.2"
    );

    // Verify other components were NOT modified
    assert_eq!(
        mutable_state.hexaco(),
        &hexaco,
        "Hexaco should be unchanged after mood modification"
    );
    assert!(
        mutable_state.needs().fatigue().delta().abs() < f32::EPSILON,
        "Needs fatigue delta should be unchanged"
    );
}
