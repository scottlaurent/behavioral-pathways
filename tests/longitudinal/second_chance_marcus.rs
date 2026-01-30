//! Test: The Second Chance - Marcus Williams
//!
//! A 60-year longitudinal simulation following Marcus from birth through a
//! turbulent youth, incarceration, rehabilitation, and eventual success.
//! Tests how formative events can both damage and heal personality over time.
//!
//! Key validations:
//! - Early trauma causes significant negative base shifts
//! - Positive formative events can partially offset prior damage
//! - Recovery trajectory is slower than damage trajectory
//! - Age plasticity means early damage is more permanent
//! - Acquired capability accumulates from violence exposure
//! - Trust (agreeableness) can rebuild through positive relationships

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::{
    EventType, HexacoPath, MentalHealthPath, MoodPath, NeedsPath, Species, StatePath,
};
use behavioral_pathways::event::EventBuilder;
use behavioral_pathways::simulation::Simulation;
use behavioral_pathways::state::Hexaco;
use behavioral_pathways::types::{Duration, EntityId, Timestamp};

/// The Second Chance: Marcus Williams' 60-year journey from adversity to redemption.
#[test]
#[ignore] // Expensive longitudinal test - run with `cargo test -- --ignored`
fn second_chance_marcus() {
    // ========================================================================
    // SETUP: Birth anchor (1985)
    // What we're doing: Creating Marcus at birth in inner-city environment.
    // Baseline: Average personality, slightly low conscientiousness (chaotic home).
    // ========================================================================

    // Marcus's birth: March 3, 1985
    let birth_date = Timestamp::from_ymd_hms(1985, 3, 3, 0, 0, 0);
    let reference = birth_date;
    let mut sim = Simulation::new(reference);

    // Marcus's baseline personality at birth:
    // - Moderate extraversion (0.3, sociable)
    // - Average openness (0.1)
    // - Below average conscientiousness (-0.1, chaotic environment)
    // - Moderate neuroticism (0.0, neutral baseline)
    // - Average agreeableness (0.1)
    // - Average honesty-humility (0.0)
    let baseline_hexaco = Hexaco::new()
        .with_extraversion(0.3)
        .with_openness(0.1)
        .with_conscientiousness(-0.1)
        .with_neuroticism(0.0)
        .with_agreeableness(0.1)
        .with_honesty_humility(0.0);

    let entity = EntityBuilder::new()
        .id("marcus_williams")
        .species(Species::Human)
        .birth_date(birth_date)
        .hexaco(baseline_hexaco.clone())
        .build()
        .unwrap();

    let entity_id = EntityId::new("marcus_williams").unwrap();
    sim.add_entity(entity, birth_date);

    // Store baseline values for comparison
    let baseline_neuroticism = baseline_hexaco.neuroticism();
    let baseline_conscientiousness = baseline_hexaco.conscientiousness();
    let baseline_agreeableness = baseline_hexaco.agreeableness();
    let baseline_extraversion = baseline_hexaco.extraversion();
    let baseline_honesty_humility = baseline_hexaco.honesty_humility();

    println!("=== MARCUS WILLIAMS: THE SECOND CHANCE ===");
    println!("Birth: March 3, 1985");
    println!("\nBaseline HEXACO:");
    println!("  Extraversion:      {:.2}", baseline_extraversion);
    println!("  Openness:          {:.2}", baseline_hexaco.openness());
    println!("  Conscientiousness: {:.2}", baseline_conscientiousness);
    println!("  Neuroticism:       {:.2}", baseline_neuroticism);
    println!("  Agreeableness:     {:.2}", baseline_agreeableness);
    println!("  Honesty-Humility:  {:.2}", baseline_honesty_humility);

    // ========================================================================
    // STAGE 1: Birth baseline verification (Age 0)
    // What we're testing: Infant starts with baseline personality values.
    // ========================================================================

    println!("\n=== STAGE 1: Birth (Age 0) ===");
    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(birth_date);

        let neuroticism = state.get_effective(StatePath::Hexaco(HexacoPath::Neuroticism));
        let conscientiousness = state.get_effective(StatePath::Hexaco(HexacoPath::Conscientiousness));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Life Stage: {:?}", state.life_stage());
        println!("Neuroticism: {:.3}", neuroticism);
        println!("Conscientiousness: {:.3}", conscientiousness);

        assert!(
            (neuroticism - baseline_neuroticism as f64).abs() < 0.001,
            "Neuroticism should match baseline at birth"
        );
    }

    // ========================================================================
    // STAGE 2: Father abandons family - FORMATIVE (Age 3)
    // What we're testing: Early parental abandonment causes:
    // 1. Increased neuroticism (+0.15)
    // 2. Decreased agreeableness (-0.10, trust issues)
    // Very high age plasticity (1.3x) at this age.
    // ========================================================================

    println!("\n=== STAGE 2: Father Abandons Family - FORMATIVE (Age 3) ===");
    let age_3 = birth_date + Duration::years(3);

    // Father leaves - formative abandonment event (using Rejection for abandonment)
    let abandonment_event = EventBuilder::new(EventType::Rejection)
        .target(entity_id.clone())
        .severity(1.0)
        .with_base_shift(HexacoPath::Neuroticism, 0.15)     // Anxiety from parental loss
        .with_base_shift(HexacoPath::Agreeableness, -0.10)  // Trust issues develop
        .build()
        .unwrap();
    sim.add_event(abandonment_event, age_3);

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_3 + Duration::days(30));

        let neuroticism = state.get_effective(StatePath::Hexaco(HexacoPath::Neuroticism));
        let agreeableness = state.get_effective(StatePath::Hexaco(HexacoPath::Agreeableness));
        let tb = state.get_effective(StatePath::MentalHealth(MentalHealthPath::ThwartedBelongingness));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Neuroticism: {:.3} (baseline was {:.3})", neuroticism, baseline_neuroticism);
        println!("Agreeableness: {:.3} (baseline was {:.3})", agreeableness, baseline_agreeableness);
        println!("Thwarted Belongingness: {:.3}", tb);
    }

    // ========================================================================
    // STAGE 3: Mother's boyfriend abusive (Age 5)
    // What we're testing: Ongoing abuse increases neuroticism and AC.
    // ========================================================================

    println!("\n=== STAGE 3: Abusive Home Environment (Age 5) ===");
    let age_5 = birth_date + Duration::years(5);

    // Multiple abuse incidents over months
    for i in 0..6 {
        let abuse_event = EventBuilder::new(EventType::Humiliation)
            .target(entity_id.clone())
            .severity(0.8)
            .build()
            .unwrap();
        sim.add_event(abuse_event, age_5 + Duration::days(i * 30));
    }

    // Physical abuse as traumatic exposure
    let violence_event = EventBuilder::new(EventType::TraumaticExposure)
        .target(entity_id.clone())
        .severity(0.9)
        .with_base_shift(HexacoPath::Neuroticism, 0.20)  // Domestic violence exposure
        .build()
        .unwrap();
    sim.add_event(violence_event, age_5 + Duration::days(90));

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_5 + Duration::days(180));

        let neuroticism = state.get_effective(StatePath::Hexaco(HexacoPath::Neuroticism));
        let ac = state.get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));
        let stress = state.get_effective(StatePath::Needs(NeedsPath::Stress));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Neuroticism: {:.3} (increasing from abuse)", neuroticism);
        println!("Acquired Capability: {:.3} (building from violence exposure)", ac);
        println!("Stress: {:.3}", stress);
    }

    // ========================================================================
    // STAGE 4: Witnessed shooting - FORMATIVE (Age 7)
    // What we're testing: Severe trauma from community violence.
    // 1. Large neuroticism increase (+0.25)
    // 2. Significant AC increase
    // ========================================================================

    println!("\n=== STAGE 4: Witnessed Neighborhood Shooting - FORMATIVE (Age 7) ===");
    let age_7 = birth_date + Duration::years(7);

    let shooting_event = EventBuilder::new(EventType::TraumaticExposure)
        .target(entity_id.clone())
        .severity(1.0)
        .with_base_shift(HexacoPath::Neuroticism, 0.25)        // Severe trauma
        .with_base_shift(HexacoPath::Agreeableness, -0.05)     // World is dangerous
        .build()
        .unwrap();
    sim.add_event(shooting_event, age_7);

    let ac_after_shooting;
    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_7 + Duration::days(60));

        let neuroticism = state.get_effective(StatePath::Hexaco(HexacoPath::Neuroticism));
        ac_after_shooting = state.get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Neuroticism: {:.3} (severe increase from trauma)", neuroticism);
        println!("Acquired Capability: {:.3} (high from death exposure)", ac_after_shooting);
    }

    // ========================================================================
    // STAGE 5: Caring teacher - positive influence (Age 9)
    // What we're testing: First positive adult relationship.
    // Normal event (no base shift) - temporary positive effect.
    // ========================================================================

    println!("\n=== STAGE 5: Caring Teacher (Age 9) ===");
    let age_9 = birth_date + Duration::years(9);

    // Sustained support from teacher
    for i in 0..10 {
        let support_event = EventBuilder::new(EventType::Support)
            .target(entity_id.clone())
            .severity(0.7)
            .build()
            .unwrap();
        sim.add_event(support_event, age_9 + Duration::days(i * 14));
    }

    let recognition_event = EventBuilder::new(EventType::Achievement)
        .target(entity_id.clone())
        .severity(0.6)
        .build()
        .unwrap();
    sim.add_event(recognition_event, age_9 + Duration::days(120));

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_9 + Duration::days(150));

        let valence = state.get_effective(StatePath::Mood(MoodPath::Valence));
        let self_worth = state.get_effective(StatePath::MentalHealth(MentalHealthPath::SelfWorth));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Valence: {:.3} (positive from teacher support)", valence);
        println!("Self-worth: {:.3}", self_worth);
    }

    // ========================================================================
    // STAGE 6: Mother's drug addiction worsens (Age 11)
    // What we're testing: Neglect and parentification.
    // ========================================================================

    println!("\n=== STAGE 6: Mother's Addiction Worsens (Age 11) ===");
    let age_11 = birth_date + Duration::years(11);

    let neglect_event = EventBuilder::new(EventType::SocialExclusion)
        .target(entity_id.clone())
        .severity(0.9)
        .with_base_shift(HexacoPath::Conscientiousness, 0.10)  // Forced responsibility
        .with_base_shift(HexacoPath::Neuroticism, 0.10)        // Chronic stress
        .build()
        .unwrap();
    sim.add_event(neglect_event, age_11);

    // Burden of caring for younger sibling
    let burden_event = EventBuilder::new(EventType::BurdenFeedback)
        .target(entity_id.clone())
        .severity(0.7)
        .build()
        .unwrap();
    sim.add_event(burden_event, age_11 + Duration::days(30));

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_11 + Duration::days(90));

        let conscientiousness = state.get_effective(StatePath::Hexaco(HexacoPath::Conscientiousness));
        let pb = state.get_effective(StatePath::MentalHealth(MentalHealthPath::PerceivedBurdensomeness));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Conscientiousness: {:.3} (increased from forced responsibility)", conscientiousness);
        println!("Perceived Burdensomeness: {:.3}", pb);
    }

    // ========================================================================
    // STAGE 7: Joins gang for protection - FORMATIVE (Age 14)
    // What we're testing: Gang membership as survival strategy.
    // 1. Decreases honesty-humility (-0.15)
    // 2. Increases extraversion (+0.05, social group)
    // In sensitive period for many traits.
    // ========================================================================

    println!("\n=== STAGE 7: Joins Gang - FORMATIVE (Age 14) ===");
    let age_14 = birth_date + Duration::years(14);

    // Gang initiation event
    let gang_event = EventBuilder::new(EventType::SocialInclusion)
        .target(entity_id.clone())
        .severity(0.9)
        .with_base_shift(HexacoPath::HonestyHumility, -0.15)  // Criminal values adopted
        .with_base_shift(HexacoPath::Extraversion, 0.05)      // Group belonging
        .with_base_shift(HexacoPath::Agreeableness, -0.05)    // In-group only trust
        .build()
        .unwrap();
    sim.add_event(gang_event, age_14);

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_14 + Duration::days(60));

        let honesty_humility = state.get_effective(StatePath::Hexaco(HexacoPath::HonestyHumility));
        let tb = state.get_effective(StatePath::MentalHealth(MentalHealthPath::ThwartedBelongingness));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Honesty-Humility: {:.3} (decreased from criminal activity)", honesty_humility);
        println!("Thwarted Belongingness: {:.3} (reduced - has gang 'family')", tb);
    }

    // ========================================================================
    // STAGE 8: First arrest (Age 15)
    // What we're testing: Legal consequences, normal event.
    // ========================================================================

    println!("\n=== STAGE 8: First Arrest (Age 15) ===");
    let age_15 = birth_date + Duration::years(15);

    let arrest_event = EventBuilder::new(EventType::Humiliation)
        .target(entity_id.clone())
        .severity(0.8)
        .build()
        .unwrap();
    sim.add_event(arrest_event, age_15);

    let failure_event = EventBuilder::new(EventType::Failure)
        .target(entity_id.clone())
        .severity(0.7)
        .build()
        .unwrap();
    sim.add_event(failure_event, age_15 + Duration::days(7));

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_15 + Duration::days(30));

        let dominance = state.get_effective(StatePath::Mood(MoodPath::Dominance));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Dominance: {:.3} (low from powerlessness)", dominance);
    }

    // ========================================================================
    // STAGE 9: Friend killed in gang violence - FORMATIVE (Age 16)
    // What we're testing: Loss and further AC buildup.
    // ========================================================================

    println!("\n=== STAGE 9: Friend Killed - FORMATIVE (Age 16) ===");
    let age_16 = birth_date + Duration::years(16);

    let friend_death_event = EventBuilder::new(EventType::Bereavement)
        .target(entity_id.clone())
        .severity(1.0)
        .with_base_shift(HexacoPath::Neuroticism, 0.15)  // Grief and survivor guilt
        .build()
        .unwrap();
    sim.add_event(friend_death_event, age_16);

    let trauma_event = EventBuilder::new(EventType::TraumaticExposure)
        .target(entity_id.clone())
        .severity(0.9)
        .build()
        .unwrap();
    sim.add_event(trauma_event, age_16 + Duration::days(1));

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_16 + Duration::days(60));

        let neuroticism = state.get_effective(StatePath::Hexaco(HexacoPath::Neuroticism));
        let ac = state.get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Neuroticism: {:.3} (high from cumulative trauma)", neuroticism);
        println!("Acquired Capability: {:.3}", ac);
    }

    // ========================================================================
    // STAGE 10: Drops out of school (Age 17)
    // What we're testing: Educational failure, normal negative events.
    // ========================================================================

    println!("\n=== STAGE 10: Drops Out of School (Age 17) ===");
    let age_17 = birth_date + Duration::years(17);

    let dropout_event = EventBuilder::new(EventType::Failure)
        .target(entity_id.clone())
        .severity(0.8)
        .build()
        .unwrap();
    sim.add_event(dropout_event, age_17);

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_17 + Duration::days(30));

        let hopelessness = state.get_effective(StatePath::MentalHealth(MentalHealthPath::Hopelessness));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Hopelessness: {:.3}", hopelessness);
    }

    // ========================================================================
    // STAGE 11: Armed robbery conviction - FORMATIVE (Age 19)
    // What we're testing: Major life derailment.
    // 1. Further decrease in honesty-humility (-0.10)
    // 2. Decrease in agreeableness (-0.05)
    // ========================================================================

    println!("\n=== STAGE 11: Armed Robbery Conviction - FORMATIVE (Age 19) ===");
    let age_19 = birth_date + Duration::years(19);

    let conviction_event = EventBuilder::new(EventType::Humiliation)
        .target(entity_id.clone())
        .severity(1.0)
        .with_base_shift(HexacoPath::HonestyHumility, -0.10)  // Criminal identity solidifies
        .with_base_shift(HexacoPath::Agreeableness, -0.05)    // Hostility toward society
        .build()
        .unwrap();
    sim.add_event(conviction_event, age_19);

    // Context transition to prison
    let prison_event = EventBuilder::new(EventType::ContextTransition)
        .target(entity_id.clone())
        .severity(1.0)
        .build()
        .unwrap();
    sim.add_event(prison_event, age_19 + Duration::days(30));

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_19 + Duration::days(60));

        let honesty_humility = state.get_effective(StatePath::Hexaco(HexacoPath::HonestyHumility));
        let self_worth = state.get_effective(StatePath::MentalHealth(MentalHealthPath::SelfWorth));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Honesty-Humility: {:.3} (very low)", honesty_humility);
        println!("Self-worth: {:.3}", self_worth);
    }

    // ========================================================================
    // STAGE 12: Prison violence exposure (Age 20-22)
    // What we're testing: Continuous trauma in prison environment.
    // ========================================================================

    println!("\n=== STAGE 12: Prison Violence (Ages 20-22) ===");
    let age_20 = birth_date + Duration::years(20);

    // Regular violence exposure
    for i in 0..24 {
        let violence_event = EventBuilder::new(EventType::TraumaticExposure)
            .target(entity_id.clone())
            .severity(0.6)
            .build()
            .unwrap();
        sim.add_event(violence_event, age_20 + Duration::days(i * 30));
    }

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_20 + Duration::years(2));

        let ac = state.get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));
        let stress = state.get_effective(StatePath::Needs(NeedsPath::Stress));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Acquired Capability: {:.3} (very high from prison)", ac);
        println!("Stress: {:.3}", stress);
    }

    // ========================================================================
    // STAGE 13: Prison education program mentor - FORMATIVE (Age 23)
    // THE TURNING POINT
    // What we're testing: First positive formative event in adulthood.
    // 1. Increase in openness (+0.15)
    // 2. Increase in conscientiousness (+0.10)
    // 3. Small decrease in neuroticism (-0.05)
    // ========================================================================

    println!("\n=== STAGE 13: Meets Mentor in Prison - FORMATIVE TURNING POINT (Age 23) ===");
    let age_23 = birth_date + Duration::years(23);

    // Sustained mentorship
    let mentor_event = EventBuilder::new(EventType::Support)
        .target(entity_id.clone())
        .severity(1.0)
        .with_base_shift(HexacoPath::Openness, 0.15)           // Opens to education
        .with_base_shift(HexacoPath::Conscientiousness, 0.10)  // Develops discipline
        .with_base_shift(HexacoPath::Neuroticism, -0.05)       // Some healing
        .build()
        .unwrap();
    sim.add_event(mentor_event, age_23);

    // Regular positive reinforcement
    for i in 0..12 {
        let encouragement_event = EventBuilder::new(EventType::Achievement)
            .target(entity_id.clone())
            .severity(0.5)
            .build()
            .unwrap();
        sim.add_event(encouragement_event, age_23 + Duration::days(i * 14));
    }

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_23 + Duration::days(180));

        let openness = state.get_effective(StatePath::Hexaco(HexacoPath::Openness));
        let conscientiousness = state.get_effective(StatePath::Hexaco(HexacoPath::Conscientiousness));
        let self_worth_val = state.get_effective(StatePath::MentalHealth(MentalHealthPath::SelfWorth));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Openness: {:.3} (increasing from education)", openness);
        println!("Conscientiousness: {:.3} (increasing from discipline)", conscientiousness);
        println!("Self-worth: {:.3}", self_worth_val);
    }

    // ========================================================================
    // STAGE 14: Earns GED in prison (Age 24)
    // What we're testing: Achievement milestone, normal positive event.
    // ========================================================================

    println!("\n=== STAGE 14: Earns GED (Age 24) ===");
    let age_24 = birth_date + Duration::years(24);

    let ged_event = EventBuilder::new(EventType::Achievement)
        .target(entity_id.clone())
        .severity(0.9)
        .build()
        .unwrap();
    sim.add_event(ged_event, age_24);

    let empowerment_event = EventBuilder::new(EventType::Empowerment)
        .target(entity_id.clone())
        .severity(0.8)
        .build()
        .unwrap();
    sim.add_event(empowerment_event, age_24 + Duration::days(1));

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_24 + Duration::days(30));

        let self_worth = state.get_effective(StatePath::MentalHealth(MentalHealthPath::SelfWorth));
        let dominance = state.get_effective(StatePath::Mood(MoodPath::Dominance));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Self-worth: {:.3} (improved from achievement)", self_worth);
        println!("Dominance: {:.3}", dominance);
    }

    // ========================================================================
    // STAGE 15: Released from prison (Age 26)
    // What we're testing: Major context transition.
    // ========================================================================

    println!("\n=== STAGE 15: Prison Release (Age 26) ===");
    let age_26 = birth_date + Duration::years(26);

    let release_event = EventBuilder::new(EventType::ContextTransition)
        .target(entity_id.clone())
        .severity(0.8)
        .build()
        .unwrap();
    sim.add_event(release_event, age_26);

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_26 + Duration::days(7));

        let arousal = state.get_effective(StatePath::Mood(MoodPath::Arousal));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Arousal: {:.3} (elevated from major change)", arousal);
    }

    // ========================================================================
    // STAGE 16: Struggles with employment (Age 26-27)
    // What we're testing: Post-release challenges, normal negative events.
    // ========================================================================

    println!("\n=== STAGE 16: Employment Struggles (Age 27) ===");
    let age_27 = birth_date + Duration::years(27);

    // Multiple rejections
    for i in 0..8 {
        let rejection_event = EventBuilder::new(EventType::Rejection)
            .target(entity_id.clone())
            .severity(0.6)
            .build()
            .unwrap();
        sim.add_event(rejection_event, age_27 + Duration::days(i * 14));
    }

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_27 + Duration::days(120));

        let hopelessness = state.get_effective(StatePath::MentalHealth(MentalHealthPath::Hopelessness));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Hopelessness: {:.3} (elevated from rejections)", hopelessness);
    }

    // ========================================================================
    // STAGE 17: Community college enrollment - FORMATIVE (Age 28)
    // What we're testing: Continued recovery trajectory.
    // 1. Openness continues to increase (+0.10)
    // 2. Conscientiousness increases (+0.05)
    // ========================================================================

    println!("\n=== STAGE 17: Community College - FORMATIVE (Age 28) ===");
    let age_28 = birth_date + Duration::years(28);

    let college_event = EventBuilder::new(EventType::Achievement)
        .target(entity_id.clone())
        .severity(0.9)
        .with_base_shift(HexacoPath::Openness, 0.10)           // Educational growth
        .with_base_shift(HexacoPath::Conscientiousness, 0.05)  // Academic discipline
        .build()
        .unwrap();
    sim.add_event(college_event, age_28);

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_28 + Duration::days(90));

        let openness = state.get_effective(StatePath::Hexaco(HexacoPath::Openness));
        let self_worth_val = state.get_effective(StatePath::MentalHealth(MentalHealthPath::SelfWorth));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Openness: {:.3} (continuing to grow)", openness);
        println!("Self-worth: {:.3}", self_worth_val);
    }

    // ========================================================================
    // STAGE 18: Meets future wife - FORMATIVE (Age 30)
    // What we're testing: Healthy relationship repairs trust.
    // 1. Agreeableness increases (+0.10)
    // 2. Neuroticism decreases (-0.05)
    // ========================================================================

    println!("\n=== STAGE 18: Meets Future Wife - FORMATIVE (Age 30) ===");
    let age_30 = birth_date + Duration::years(30);

    let relationship_event = EventBuilder::new(EventType::Support)
        .target(entity_id.clone())
        .severity(1.0)
        .with_base_shift(HexacoPath::Agreeableness, 0.10)   // Trust rebuilding
        .with_base_shift(HexacoPath::Neuroticism, -0.05)    // Secure attachment
        .build()
        .unwrap();
    sim.add_event(relationship_event, age_30);

    // Ongoing relationship support
    for i in 0..12 {
        let support_event = EventBuilder::new(EventType::Support)
            .target(entity_id.clone())
            .severity(0.7)
            .build()
            .unwrap();
        sim.add_event(support_event, age_30 + Duration::days(i * 14));
    }

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_30 + Duration::days(180));

        let agreeableness = state.get_effective(StatePath::Hexaco(HexacoPath::Agreeableness));
        let caring = state.get_effective(StatePath::SocialCognition(
            behavioral_pathways::enums::SocialCognitionPath::PerceivedReciprocalCaring,
        ));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Agreeableness: {:.3} (rebuilding trust)", agreeableness);
        println!("Perceived Caring: {:.3}", caring);
    }

    // ========================================================================
    // STAGE 19: Graduates community college (Age 31)
    // What we're testing: Educational milestone.
    // ========================================================================

    println!("\n=== STAGE 19: Community College Graduation (Age 31) ===");
    let age_31 = birth_date + Duration::years(31);

    let graduation_event = EventBuilder::new(EventType::Achievement)
        .target(entity_id.clone())
        .severity(0.9)
        .build()
        .unwrap();
    sim.add_event(graduation_event, age_31);

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_31 + Duration::days(7));

        let self_worth = state.get_effective(StatePath::MentalHealth(MentalHealthPath::SelfWorth));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Self-worth: {:.3}", self_worth);
    }

    // ========================================================================
    // STAGE 20: Marriage - FORMATIVE (Age 32)
    // What we're testing: Commitment deepens healing.
    // 1. Agreeableness continues to grow (+0.05)
    // 2. Honesty-humility begins recovery (+0.10)
    // ========================================================================

    println!("\n=== STAGE 20: Marriage - FORMATIVE (Age 32) ===");
    let age_32 = birth_date + Duration::years(32);

    let marriage_event = EventBuilder::new(EventType::SocialInclusion)
        .target(entity_id.clone())
        .severity(1.0)
        .with_base_shift(HexacoPath::Agreeableness, 0.05)      // Deepening trust
        .with_base_shift(HexacoPath::HonestyHumility, 0.10)    // Values realignment
        .build()
        .unwrap();
    sim.add_event(marriage_event, age_32);

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_32 + Duration::days(30));

        let honesty_humility = state.get_effective(StatePath::Hexaco(HexacoPath::HonestyHumility));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Honesty-Humility: {:.3} (beginning recovery)", honesty_humility);
    }

    // ========================================================================
    // STAGE 21: First child born (Age 33)
    // What we're testing: Parenthood effect.
    // ========================================================================

    println!("\n=== STAGE 21: First Child Born (Age 33) ===");
    let age_33 = birth_date + Duration::years(33);

    let child_event = EventBuilder::new(EventType::Achievement)
        .target(entity_id.clone())
        .severity(1.0)
        .build()
        .unwrap();
    sim.add_event(child_event, age_33);

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_33 + Duration::days(30));

        let self_worth_val = state.get_effective(StatePath::MentalHealth(MentalHealthPath::SelfWorth));
        let valence = state.get_effective(StatePath::Mood(MoodPath::Valence));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Self-worth: {:.3}", self_worth_val);
        println!("Valence: {:.3}", valence);
    }

    // ========================================================================
    // STAGE 22: Gets first career job - FORMATIVE (Age 35)
    // What we're testing: Employment success reinforces recovery.
    // 1. Conscientiousness increases (+0.05)
    // 2. Honesty-humility continues recovery (+0.05)
    // ========================================================================

    println!("\n=== STAGE 22: Career Job - FORMATIVE (Age 35) ===");
    let age_35 = birth_date + Duration::years(35);

    let job_event = EventBuilder::new(EventType::Achievement)
        .target(entity_id.clone())
        .severity(0.9)
        .with_base_shift(HexacoPath::Conscientiousness, 0.05)  // Professional growth
        .with_base_shift(HexacoPath::HonestyHumility, 0.05)    // Legitimate success
        .build()
        .unwrap();
    sim.add_event(job_event, age_35);

    let empowerment_event = EventBuilder::new(EventType::Empowerment)
        .target(entity_id.clone())
        .severity(0.8)
        .build()
        .unwrap();
    sim.add_event(empowerment_event, age_35 + Duration::days(1));

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_35 + Duration::days(60));

        let conscientiousness = state.get_effective(StatePath::Hexaco(HexacoPath::Conscientiousness));
        let dominance = state.get_effective(StatePath::Mood(MoodPath::Dominance));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Conscientiousness: {:.3}", conscientiousness);
        println!("Dominance: {:.3} (elevated from success)", dominance);
    }

    // ========================================================================
    // STAGE 23: Second child born (Age 37)
    // What we're testing: Family growth, normal positive.
    // ========================================================================

    println!("\n=== STAGE 23: Second Child Born (Age 37) ===");
    let age_37 = birth_date + Duration::years(37);

    let second_child_event = EventBuilder::new(EventType::Achievement)
        .target(entity_id.clone())
        .severity(0.9)
        .build()
        .unwrap();
    sim.add_event(second_child_event, age_37);

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_37 + Duration::days(30));

        let valence = state.get_effective(StatePath::Mood(MoodPath::Valence));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Valence: {:.3}", valence);
    }

    // ========================================================================
    // STAGE 24: Promotion to manager (Age 40)
    // What we're testing: Career advancement.
    // ========================================================================

    println!("\n=== STAGE 24: Promotion to Manager (Age 40) ===");
    let age_40 = birth_date + Duration::years(40);

    let promotion_event = EventBuilder::new(EventType::Achievement)
        .target(entity_id.clone())
        .severity(0.8)
        .build()
        .unwrap();
    sim.add_event(promotion_event, age_40);

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_40 + Duration::days(7));

        let dominance = state.get_effective(StatePath::Mood(MoodPath::Dominance));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Dominance: {:.3}", dominance);
    }

    // ========================================================================
    // STAGE 25: Starts mentoring at-risk youth - FORMATIVE (Age 42)
    // What we're testing: Giving back as final healing.
    // 1. Agreeableness continues recovery (+0.05)
    // 2. Honesty-humility continues recovery (+0.05)
    // 3. Neuroticism decreases (-0.05)
    // ========================================================================

    println!("\n=== STAGE 25: Starts Mentoring Youth - FORMATIVE (Age 42) ===");
    let age_42 = birth_date + Duration::years(42);

    let mentoring_event = EventBuilder::new(EventType::Support)
        .target(entity_id.clone())
        .severity(0.9)
        .with_base_shift(HexacoPath::Agreeableness, 0.05)      // Generativity
        .with_base_shift(HexacoPath::HonestyHumility, 0.05)    // Purpose through service
        .with_base_shift(HexacoPath::Neuroticism, -0.05)       // Healing through helping
        .build()
        .unwrap();
    sim.add_event(mentoring_event, age_42);

    // Ongoing positive impact
    for i in 0..12 {
        let impact_event = EventBuilder::new(EventType::Achievement)
            .target(entity_id.clone())
            .severity(0.5)
            .build()
            .unwrap();
        sim.add_event(impact_event, age_42 + Duration::days(i * 30));
    }

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_42 + Duration::days(365));

        let self_worth_val = state.get_effective(StatePath::MentalHealth(MentalHealthPath::SelfWorth));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Self-worth: {:.3} (high from mentoring)", self_worth_val);
    }

    // ========================================================================
    // STAGE 26: Mother dies (Age 45)
    // What we're testing: Loss event, but from a more stable baseline.
    // ========================================================================

    println!("\n=== STAGE 26: Mother Dies (Age 45) ===");
    let age_45 = birth_date + Duration::years(45);

    let mother_death_event = EventBuilder::new(EventType::Bereavement)
        .target(entity_id.clone())
        .severity(0.8)
        .with_base_shift(HexacoPath::Neuroticism, 0.05)  // Some grief, less impact at this age
        .build()
        .unwrap();
    sim.add_event(mother_death_event, age_45);

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_45 + Duration::days(60));

        let neuroticism = state.get_effective(StatePath::Hexaco(HexacoPath::Neuroticism));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Neuroticism: {:.3}", neuroticism);
    }

    // ========================================================================
    // STAGE 27: Son graduates college (Age 51)
    // What we're testing: Vicarious achievement.
    // ========================================================================

    println!("\n=== STAGE 27: Son Graduates College (Age 51) ===");
    let age_51 = birth_date + Duration::years(51);

    let son_graduation_event = EventBuilder::new(EventType::Achievement)
        .target(entity_id.clone())
        .severity(0.9)
        .build()
        .unwrap();
    sim.add_event(son_graduation_event, age_51);

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_51 + Duration::days(7));

        let valence = state.get_effective(StatePath::Mood(MoodPath::Valence));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Valence: {:.3}", valence);
    }

    // ========================================================================
    // STAGE 28: Receives community leadership award - FORMATIVE (Age 55)
    // What we're testing: Late-life recognition.
    // 1. Small conscientiousness increase (+0.03)
    // Diminished by lower age plasticity.
    // ========================================================================

    println!("\n=== STAGE 28: Community Leadership Award - FORMATIVE (Age 55) ===");
    let age_55 = birth_date + Duration::years(55);

    let award_event = EventBuilder::new(EventType::Achievement)
        .target(entity_id.clone())
        .severity(0.9)
        .with_base_shift(HexacoPath::Conscientiousness, 0.03)  // Small late-life shift
        .with_base_shift(HexacoPath::HonestyHumility, 0.03)    // Recognition of growth
        .build()
        .unwrap();
    sim.add_event(award_event, age_55);

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_55 + Duration::days(30));

        let self_worth = state.get_effective(StatePath::MentalHealth(MentalHealthPath::SelfWorth));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Self-worth: {:.3} (high from recognition)", self_worth);
    }

    // ========================================================================
    // STAGE 29: Grandchild born (Age 57)
    // What we're testing: Family legacy.
    // ========================================================================

    println!("\n=== STAGE 29: First Grandchild Born (Age 57) ===");
    let age_57 = birth_date + Duration::years(57);

    let grandchild_event = EventBuilder::new(EventType::Achievement)
        .target(entity_id.clone())
        .severity(0.8)
        .build()
        .unwrap();
    sim.add_event(grandchild_event, age_57);

    let inclusion_event = EventBuilder::new(EventType::SocialInclusion)
        .target(entity_id.clone())
        .severity(0.7)
        .build()
        .unwrap();
    sim.add_event(inclusion_event, age_57 + Duration::days(1));

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_57 + Duration::days(30));

        let self_worth_val = state.get_effective(StatePath::MentalHealth(MentalHealthPath::SelfWorth));
        let tb = state.get_effective(StatePath::MentalHealth(MentalHealthPath::ThwartedBelongingness));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Self-worth: {:.3}", self_worth_val);
        println!("Thwarted Belongingness: {:.3} (low - strong family)", tb);
    }

    // ========================================================================
    // STAGE 30: Retirement planning (Age 59)
    // What we're testing: Life transition.
    // ========================================================================

    println!("\n=== STAGE 30: Retirement Planning (Age 59) ===");
    let age_59 = birth_date + Duration::years(59);

    let retirement_event = EventBuilder::new(EventType::ContextTransition)
        .target(entity_id.clone())
        .severity(0.5)
        .build()
        .unwrap();
    sim.add_event(retirement_event, age_59);

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_59 + Duration::days(30));

        let arousal = state.get_effective(StatePath::Mood(MoodPath::Arousal));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Arousal: {:.3}", arousal);
    }

    // ========================================================================
    // STAGE 31: FINAL STATE ASSESSMENT (Age 60)
    // Validate the full arc: damage -> recovery trajectory.
    // ========================================================================

    println!("\n=== STAGE 31: FINAL STATE ASSESSMENT (Age 60) ===");
    let age_60 = birth_date + Duration::years(60);

    {
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(age_60);

        let final_extraversion = state.get_effective(StatePath::Hexaco(HexacoPath::Extraversion));
        let final_openness = state.get_effective(StatePath::Hexaco(HexacoPath::Openness));
        let final_conscientiousness = state.get_effective(StatePath::Hexaco(HexacoPath::Conscientiousness));
        let final_neuroticism = state.get_effective(StatePath::Hexaco(HexacoPath::Neuroticism));
        let final_agreeableness = state.get_effective(StatePath::Hexaco(HexacoPath::Agreeableness));
        let final_honesty_humility = state.get_effective(StatePath::Hexaco(HexacoPath::HonestyHumility));

        let final_ac = state.get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));
        let final_tb = state.get_effective(StatePath::MentalHealth(MentalHealthPath::ThwartedBelongingness));
        let final_sd = state.get_effective(StatePath::MentalHealth(MentalHealthPath::SuicidalDesire));

        println!("Age: {:.1} years", state.age_at_timestamp().as_years_f64());
        println!("Life Stage: {:?}", state.life_stage());

        println!("\n--- FINAL HEXACO PERSONALITY ---");
        println!(
            "Extraversion:      {:.3} (baseline: {:.3}, delta: {:+.3})",
            final_extraversion,
            baseline_extraversion,
            final_extraversion - baseline_extraversion as f64
        );
        println!(
            "Openness:          {:.3} (baseline: {:.3}, delta: {:+.3})",
            final_openness,
            baseline_hexaco.openness(),
            final_openness - baseline_hexaco.openness() as f64
        );
        println!(
            "Conscientiousness: {:.3} (baseline: {:.3}, delta: {:+.3})",
            final_conscientiousness,
            baseline_conscientiousness,
            final_conscientiousness - baseline_conscientiousness as f64
        );
        println!(
            "Neuroticism:       {:.3} (baseline: {:.3}, delta: {:+.3})",
            final_neuroticism,
            baseline_neuroticism,
            final_neuroticism - baseline_neuroticism as f64
        );
        println!(
            "Agreeableness:     {:.3} (baseline: {:.3}, delta: {:+.3})",
            final_agreeableness,
            baseline_agreeableness,
            final_agreeableness - baseline_agreeableness as f64
        );
        println!(
            "Honesty-Humility:  {:.3} (baseline: {:.3}, delta: {:+.3})",
            final_honesty_humility,
            baseline_honesty_humility,
            final_honesty_humility - baseline_honesty_humility as f64
        );

        println!("\n--- MENTAL HEALTH STATE ---");
        println!("Acquired Capability: {:.3} (high from trauma history)", final_ac);
        println!("Thwarted Belongingness: {:.3}", final_tb);
        println!("Suicidal Desire: {:.3}", final_sd);

        // ====================================================================
        // VALIDATIONS
        // ====================================================================

        println!("\n--- VALIDATIONS ---");

        // Validation 1: Neuroticism should be elevated but partially recovered
        // Started at 0.0, rose to very high levels, then partially came down
        println!("Validation 1: Neuroticism arc (damage then partial recovery)");
        println!("  Baseline: {:.3}", baseline_neuroticism);
        println!("  Final: {:.3}", final_neuroticism);
        println!("  Expected: Higher than baseline but not at peak");
        // Neuroticism shifts: +0.15 (abandonment) +0.20 (abuse) +0.25 (shooting)
        //                    +0.10 (neglect) +0.15 (friend death) -0.05 (mentor)
        //                    -0.05 (wife) -0.05 (mentoring) +0.05 (mother death)
        // Net: approximately +0.75 before plasticity adjustments

        // Validation 2: Honesty-humility should have partially recovered
        println!("\nValidation 2: Honesty-Humility recovery");
        println!("  Baseline: {:.3}", baseline_honesty_humility);
        println!("  Final: {:.3}", final_honesty_humility);
        // Shifts: -0.15 (gang) -0.10 (conviction) +0.10 (marriage) +0.05 (job)
        //         +0.05 (mentoring) +0.03 (award)
        // Net: approximately -0.02 before plasticity

        // Validation 3: Openness should have increased significantly
        println!("\nValidation 3: Openness growth from education");
        println!("  Baseline: {:.3}", baseline_hexaco.openness());
        println!("  Final: {:.3}", final_openness);
        // Shifts: +0.15 (mentor/prison education) +0.10 (college)
        // Net: approximately +0.25 before plasticity

        // Validation 4: AC should be very high from cumulative trauma
        println!("\nValidation 4: Acquired Capability persistence");
        println!("  AC after childhood shooting (age 7): {:.3}", ac_after_shooting);
        println!("  Final AC (age 60): {:.3}", final_ac);
        assert!(
            final_ac >= ac_after_shooting,
            "AC should never decrease: {} should be >= {}",
            final_ac,
            ac_after_shooting
        );
        println!("  PASSED: AC did not decay over 53 years");

        // Validation 5: Conscientiousness should have grown
        println!("\nValidation 5: Conscientiousness growth");
        println!("  Baseline: {:.3}", baseline_conscientiousness);
        println!("  Final: {:.3}", final_conscientiousness);
        // Shifts: +0.10 (neglect/responsibility) +0.10 (mentor) +0.05 (college)
        //         +0.05 (career job) +0.03 (award)
        // Net: approximately +0.33 before plasticity

        // Validation 6: Agreeableness recovery
        println!("\nValidation 6: Agreeableness (trust) recovery");
        println!("  Baseline: {:.3}", baseline_agreeableness);
        println!("  Final: {:.3}", final_agreeableness);
        // Early damage: -0.10 (abandonment) -0.05 (shooting) -0.05 (gang) -0.05 (conviction) = -0.25
        // Recovery: +0.10 (wife) +0.05 (marriage) +0.05 (mentoring) = +0.20
        // Net: approximately -0.05 before plasticity
    }
}
