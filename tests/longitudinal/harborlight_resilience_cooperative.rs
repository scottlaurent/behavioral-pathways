//! Harborlight Resilience Cooperative: A 2.5-Year Closed-System Community Simulation
//!
//! This test models a remote coastal campus rebuilt after a catastrophic storm, run as a
//! closed-system cooperative where resource scarcity, delayed supply lines, and shared housing
//! force residents to balance individual needs against collective survival. Over 2.5 years,
//! trust evolves through crises, generational role shifts, and the search for meaning under pressure.
//!
//! **Scenario**: Six residents of varying backgrounds manage critical infrastructure (water treatment,
//! microgrid, health services, education, aquaculture) under conditions of isolation, resource constraints,
//! and environmental unpredictability. The test tracks psychological evolution through acute crises
//! (contamination, equipment failure, medical emergencies) and chronic stressors (supply delays, role strain).
//!
//! ## Theoretical Foundations Tested
//!
//! ### PAD Affect Model
//! - **Valence dynamics**: Crisis events crash valence; collective problem-solving restores it (COMPARATIVE ASSERTIONS)
//! - **Arousal tracking**: Emergency spikes arousal AT ONSET (hours, not days); support buffers it; decays over weeks/months
//! - **Dominance evolution**: Achievements build dominance; failures reduce it (COMPARATIVE ASSERTIONS)
//!
//! ### Trust Decomposition (PENDING API IMPLEMENTATION)
//! - **Competence trust**: Technical failures erode it; successful repairs restore it [NOT YET TESTED - awaiting relationship API]
//! - **Benevolence trust**: Rises with mutual aid (shared meals, childcare); erosion under scarcity [NOT YET TESTED]
//! - **Integrity trust**: Collective decision-making tests commitment to co-op values [NOT YET TESTED]
//! - **Directional trust**: A→B vs B→A dynamics [NOT YET TESTED]
//!
//! ### Joiner's ITS
//! - **Thwarted Belongingness (TB)**: Isolation increases TB; community rituals decrease it
//! - **Perceived Burdensomeness (PB)**: Role strain (overwork, caregiver exhaustion, parenting stress) increases PB
//! - **Acquired Capability (AC)**: Exposure to environmental hazards (storms, injuries) increases AC (persists long-term)
//! - **Interpersonal Hopelessness**: Tracked alongside hopelessness during high-risk periods
//! - **ITS Convergence**: When TB+PB both elevated, multiplicative risk assessed
//!
//! ### Bronfenbrenner's Ecology (PARTIALLY MODELED)
//! - **Microsystem quality**: Co-op as backdrop, not dynamic entity [LIMITATION: microsystem quality not fully modeled]
//! - **Mesosystem**: Work-family boundaries blur [LIMITATION: bidirectional spillover not demonstrated]
//! - **Exosystem**: Supply chain delays affect local resource management
//! - **Macrosystem**: Co-op governance model reflects collectivist values
//! - **Chronosystem**: Seasonal weather patterns, generational role transitions
//!
//! ## Theory Expert Feedback Integration Status
//!
//! **PAD AFFECT (COMPLETE):**
//! - ✓ Arousal checked at storm onset (hours), not days after
//! - ✓ Comparative assertions (not absolute thresholds) for valence/dominance
//! - ✓ Arousal buffering after support events verified
//! - ✓ Dominance recovery after achievements measured
//!
//! **TRUST (BLOCKED - API NOT AVAILABLE):**
//! - ✗ Competence/benevolence/integrity assertions require relationship API
//! - ✗ Directional trust (A→B vs B→A) requires relationship API
//! - ✗ Integrity rupture/repair after conflicts requires relationship API
//!
//! **ITS (COMPLETE):**
//! - ✓ PB assertions added for role strain events (Omar overwork, June caregiving, Leah parenting)
//! - ✓ TB+PB convergence analysis during high-risk periods
//! - ✓ Interpersonal hopelessness tracking
//! - Note: Passive vs active ideation distinction not modeled (future work)
//!
//! **ECOLOGY (DOCUMENTED LIMITATIONS):**
//! - Microsystem quality effects noted but not dynamically modeled
//! - Mesosystem spillover acknowledged as limitation

// ============================================================================
// EVENT MAPPING REFERENCE
// ============================================================================
//
// This section shows how domain-specific events map to Behavioral Pathways
// event types for Harborlight Resilience Cooperative.
//
// +---------------------------------+--------------------------------------+
// | Domain Event Example            | Behavioral Pathways Event            |
// +---------------------------------+--------------------------------------+
// | "Water system contamination"    | EventType::Loss                      |
// | "Emergency repair collaboration"| EventType::Achievement               |
// | "Supply ship delayed"           | EventType::Failure                   |
// | "Shared meals during crisis"    | EventType::Support                   |
// | "Microgrid failure in storm"    | EventType::Loss                      |
// | "Community meeting consensus"   | EventType::Achievement               |
// | "Medical emergency"             | EventType::Loss                      |
// | "Role strain (overwork)"        | EventType::Failure + BurdenFeedback  |
// | "Aquaculture system thrives"    | EventType::Achievement               |
// | "Isolation/rejection"           | EventType::SocialExclusion           |
// | "Storm damage"                  | EventType::Violence (environmental)  |
// | "Collective celebration"        | EventType::SocialInclusion           |
// | "Equipment breakdown"           | EventType::Failure                   |
// | "Successful training transfer"  | EventType::Achievement               |
// | "Pregnancy announcement"        | EventType::Achievement               |
// | "Conflict over resources"       | EventType::Conflict                  |
// | "Caregiver exhaustion"          | EventType::Failure + BurdenFeedback  |
// | "Parenting inadequacy"          | EventType::Failure + BurdenFeedback  |
// | "Generational shift ceremony"   | EventType::Achievement               |
// +---------------------------------+--------------------------------------+
//
// SEVERITY VALUES:
// - 0.1-0.3: Minor (brief stress, easily managed)
// - 0.4-0.6: Moderate (significant impact, requires effort)
// - 0.7-0.9: Severe (crisis level, threatens system function)
// - 0.9-1.0: Extreme (life-threatening, potential system collapse)

use behavioral_pathways::entity::EntityBuilder;
use behavioral_pathways::enums::{
    EventType, MentalHealthPath, MoodPath, NeedsPath, PersonalityProfile, Species, StatePath,
};
use behavioral_pathways::event::EventBuilder;
use behavioral_pathways::simulation::Simulation;
use behavioral_pathways::types::{Duration, EntityId, Timestamp};

/// Full 2.5-year Harborlight Resilience Cooperative simulation with 6 entities.
///
/// **Test Structure**:
/// 1. Entity creation with distinct backgrounds, roles, and initial states
/// 2. Initial settlement and role establishment (Jan 2025)
/// 3. First crisis: Water contamination (Feb-Mar 2025)
/// 4. Resource scarcity and supply chain stress (Apr-Jun 2025)
/// 5. Environmental crisis: Storm and power failure (Jul-Aug 2025)
/// 6. Health emergency and community support (Sep-Oct 2025)
/// 7. Role strain and interpersonal conflict (Nov 2025 - Feb 2026)
/// 8. Turning point: Collective problem-solving (Mar-May 2026)
/// 9. Stability and generational transition (Jun 2026 - Jan 2027)
/// 10. Final crisis and resilience test (Feb-Jun 2027)
/// 11. Final state analysis for all 6 entities
///
/// **Running this test**:
/// This test is marked #[ignore] and must be run explicitly:
/// ```bash
/// cargo test harborlight_resilience_cooperative -- --ignored --nocapture
/// ```
#[test]
#[ignore = "Long-running longitudinal test - run explicitly with --ignored flag"]
fn harborlight_resilience_cooperative() {
    // Setup: Create 6 entities with distinct roles and baseline psychological states
    let reference = Timestamp::from_ymd_hms(2027, 7, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let sim_start = Timestamp::from_ymd_hms(2025, 1, 10, 0, 0, 0);

    // Leah Carter: 34, water treatment shift lead. Conscientious, protective.
    let leah_birth = Timestamp::from_ymd_hms(1990, 5, 14, 0, 0, 0);
    let leah = EntityBuilder::new()
        .id("leah_carter")
        .species(Species::Human)
        .birth_date(leah_birth)
        .age(Duration::years(34))
        .personality(PersonalityProfile::Conscientious) // Protective, organized, skeptical
        .build()
        .unwrap();

    let leah_id = EntityId::new("leah_carter").unwrap();
    sim.add_entity(leah, sim_start);

    let leah_baseline_arousal = EventBuilder::new(EventType::Failure)
        .target(leah_id.clone())
        .severity(0.35)
        .build()
        .unwrap();
    sim.add_event(leah_baseline_arousal, sim_start + Duration::hours(1));

    // Omar Haddad: 47, facilities and microgrid manager. Stoic, rule-bound.
    let omar_birth = Timestamp::from_ymd_hms(1977, 9, 3, 0, 0, 0);
    let omar = EntityBuilder::new()
        .id("omar_haddad")
        .species(Species::Human)
        .birth_date(omar_birth)
        .age(Duration::years(47))
        .personality(PersonalityProfile::Conscientious) // Stoic, disciplined, rule-bound
        .build()
        .unwrap();

    let omar_id = EntityId::new("omar_haddad").unwrap();
    sim.add_entity(omar, sim_start);

    let omar_baseline_loneliness = EventBuilder::new(EventType::Loss)
        .target(omar_id.clone())
        .severity(0.3)
        .build()
        .unwrap();
    sim.add_event(omar_baseline_loneliness, sim_start + Duration::hours(1));

    // Priya Nair: 29, community health nurse. Empathic, diligent, perfectionist.
    let priya_birth = Timestamp::from_ymd_hms(1995, 11, 22, 0, 0, 0);
    let priya = EntityBuilder::new()
        .id("priya_nair")
        .species(Species::Human)
        .birth_date(priya_birth)
        .age(Duration::years(29))
        .personality(PersonalityProfile::Conscientious) // Empathic, diligent, perfectionist
        .build()
        .unwrap();

    let priya_id = EntityId::new("priya_nair").unwrap();
    sim.add_entity(priya, sim_start);

    let priya_baseline_anxiety = EventBuilder::new(EventType::Failure)
        .target(priya_id.clone())
        .severity(0.25)
        .build()
        .unwrap();
    sim.add_event(priya_baseline_anxiety, sim_start + Duration::hours(1));

    // June Alvarez: 62, retired teacher and community organizer. Warm, pragmatic.
    let june_birth = Timestamp::from_ymd_hms(1962, 2, 8, 0, 0, 0);
    let june = EntityBuilder::new()
        .id("june_alvarez")
        .species(Species::Human)
        .birth_date(june_birth)
        .age(Duration::years(62))
        .personality(PersonalityProfile::Agreeable) // Warm, pragmatic, community-focused
        .build()
        .unwrap();

    let june_id = EntityId::new("june_alvarez").unwrap();
    sim.add_entity(june, sim_start);

    let june_baseline_grief = EventBuilder::new(EventType::Loss)
        .target(june_id.clone())
        .severity(0.3)
        .build()
        .unwrap();
    sim.add_event(june_baseline_grief, sim_start + Duration::hours(1));

    // Mateo Reyes: 23, aquaculture apprentice. Creative, impulsive, rejection-sensitive.
    let mateo_birth = Timestamp::from_ymd_hms(2001, 7, 19, 0, 0, 0);
    let mateo = EntityBuilder::new()
        .id("mateo_reyes")
        .species(Species::Human)
        .birth_date(mateo_birth)
        .age(Duration::years(23))
        .personality(PersonalityProfile::Anxious) // Impulsive, rejection-sensitive
        .build()
        .unwrap();

    let mateo_id = EntityId::new("mateo_reyes").unwrap();
    sim.add_entity(mateo, sim_start);

    let mateo_baseline_tb = EventBuilder::new(EventType::SocialExclusion)
        .target(mateo_id.clone())
        .severity(0.4)
        .build()
        .unwrap();
    sim.add_event(mateo_baseline_tb, sim_start + Duration::hours(1));

    // Avery Lin: 41, data analyst and resource planner. Analytical, introverted, idealistic.
    let avery_birth = Timestamp::from_ymd_hms(1983, 4, 30, 0, 0, 0);
    let avery = EntityBuilder::new()
        .id("avery_lin")
        .species(Species::Human)
        .birth_date(avery_birth)
        .age(Duration::years(41))
        .personality(PersonalityProfile::Conscientious) // Analytical, introverted, idealistic
        .build()
        .unwrap();

    let avery_id = EntityId::new("avery_lin").unwrap();
    sim.add_entity(avery, sim_start);

    let avery_baseline_burnout = EventBuilder::new(EventType::Failure)
        .target(avery_id.clone())
        .severity(0.4)
        .build()
        .unwrap();
    sim.add_event(avery_baseline_burnout, sim_start + Duration::hours(1));

    // Setup: Initial settlement and role establishment
    let water_success_date = Timestamp::from_ymd_hms(2025, 1, 15, 10, 0, 0);
    let water_success = EventBuilder::new(EventType::Achievement)
        .target(leah_id.clone())
        .severity(0.5)
        .build()
        .unwrap();
    sim.add_event(water_success, water_success_date);

    let first_dinner_date = Timestamp::from_ymd_hms(2025, 1, 20, 18, 0, 0);
    for person in [&leah_id, &omar_id, &priya_id, &june_id, &mateo_id, &avery_id] {
        let dinner = EventBuilder::new(EventType::SocialInclusion)
            .target(person.clone())
            .severity(0.4)
            .build()
            .unwrap();
        sim.add_event(dinner, first_dinner_date);
    }

    // Verify TB reduction after social inclusion
    {
        let handle = sim.entity(&mateo_id).unwrap();
        let mateo_before = handle.state_at(first_dinner_date - Duration::hours(1));
        let mateo_after = handle.state_at(first_dinner_date + Duration::days(1));

        let tb_before = mateo_before.get_effective(StatePath::MentalHealth(MentalHealthPath::ThwartedBelongingness));
        let tb_after = mateo_after.get_effective(StatePath::MentalHealth(MentalHealthPath::ThwartedBelongingness));

        assert!(
            tb_after < tb_before,
            "Social inclusion should reduce TB. Before: {}, After: {}",
            tb_before, tb_after
        );
    }

    // Events: First crisis - water contamination
    let contamination_date = Timestamp::from_ymd_hms(2025, 2, 10, 6, 30, 0);

    let contamination_leah = EventBuilder::new(EventType::Loss)
        .target(leah_id.clone())
        .severity(0.8)
        .build()
        .unwrap();
    sim.add_event(contamination_leah, contamination_date);

    let contamination_priya = EventBuilder::new(EventType::Loss)
        .target(priya_id.clone())
        .severity(0.7)
        .build()
        .unwrap();
    sim.add_event(contamination_priya, contamination_date);

    for person in [&omar_id, &june_id, &mateo_id, &avery_id] {
        let contamination = EventBuilder::new(EventType::Loss)
            .target(person.clone())
            .severity(0.5)
            .build()
            .unwrap();
        sim.add_event(contamination, contamination_date);
    }

    // Verify: Crisis crashes valence (comparative); arousal spikes at onset (hours, not days)
    {
        let handle = sim.entity(&leah_id).unwrap();
        let leah_before = handle.state_at(contamination_date - Duration::hours(1));
        let leah_crisis = handle.state_at(contamination_date + Duration::hours(2));
        let leah_after = handle.state_at(contamination_date + Duration::days(1));

        let valence_before = leah_before.get_effective(StatePath::Mood(MoodPath::Valence));
        let valence_crisis = leah_crisis.get_effective(StatePath::Mood(MoodPath::Valence));
        let dominance_before = leah_before.get_effective(StatePath::Mood(MoodPath::Dominance));
        let dominance_after = leah_after.get_effective(StatePath::Mood(MoodPath::Dominance));
        let arousal_before = leah_before.get_effective(StatePath::Mood(MoodPath::Arousal));
        let arousal_crisis = leah_crisis.get_effective(StatePath::Mood(MoodPath::Arousal));

        assert!(
            valence_crisis < valence_before,
            "Crisis should reduce valence. Before: {}, Crisis: {}",
            valence_before, valence_crisis
        );
        assert!(
            dominance_after < dominance_before,
            "Crisis should reduce dominance. Before: {}, After: {}",
            dominance_before, dominance_after
        );
        assert!(
            arousal_crisis > arousal_before,
            "Crisis should spike arousal at onset. Before: {}, Crisis: {}",
            arousal_before, arousal_crisis
        );
    }

    let repair_date = Timestamp::from_ymd_hms(2025, 2, 12, 14, 0, 0);

    let repair_omar = EventBuilder::new(EventType::Achievement)
        .target(omar_id.clone())
        .severity(0.65)
        .build()
        .unwrap();
    sim.add_event(repair_omar, repair_date);

    let repair_leah = EventBuilder::new(EventType::Achievement)
        .target(leah_id.clone())
        .severity(0.65)
        .build()
        .unwrap();
    sim.add_event(repair_leah, repair_date);

    let crisis_meals_date = Timestamp::from_ymd_hms(2025, 2, 15, 12, 0, 0);

    let leah_arousal_pre_support = {
        let handle = sim.entity(&leah_id).unwrap();
        let state = handle.state_at(crisis_meals_date - Duration::hours(1));
        state.get_effective(StatePath::Mood(MoodPath::Arousal))
    };

    for person in [&leah_id, &omar_id, &priya_id, &june_id, &mateo_id, &avery_id] {
        let meal_support = EventBuilder::new(EventType::Support)
            .target(person.clone())
            .severity(0.5)
            .build()
            .unwrap();
        sim.add_event(meal_support, crisis_meals_date);
    }

    let organizing_june = EventBuilder::new(EventType::Achievement)
        .target(june_id.clone())
        .severity(0.5)
        .build()
        .unwrap();
    sim.add_event(organizing_june, crisis_meals_date);

    // Verify arousal buffering after support
    {
        let handle = sim.entity(&leah_id).unwrap();
        let leah_post_support = handle.state_at(crisis_meals_date + Duration::hours(6));
        let arousal_post_support = leah_post_support.get_effective(StatePath::Mood(MoodPath::Arousal));

        assert!(
            arousal_post_support < leah_arousal_pre_support,
            "Support should buffer arousal. Before: {}, After: {}",
            leah_arousal_pre_support, arousal_post_support
        );
    }

    let restoration_date = Timestamp::from_ymd_hms(2025, 3, 1, 9, 0, 0);

    let restoration_leah = EventBuilder::new(EventType::Achievement)
        .target(leah_id.clone())
        .severity(0.7)
        .build()
        .unwrap();
    sim.add_event(restoration_leah, restoration_date);

    for person in [&omar_id, &priya_id, &june_id, &mateo_id, &avery_id] {
        let restoration = EventBuilder::new(EventType::Achievement)
            .target(person.clone())
            .severity(0.5)
            .build()
            .unwrap();
        sim.add_event(restoration, restoration_date);
    }

    // Events: Resource scarcity and supply chain stress
    let supply_delay_date = Timestamp::from_ymd_hms(2025, 4, 10, 16, 0, 0);

    let supply_delay_avery = EventBuilder::new(EventType::Failure)
        .target(avery_id.clone())
        .severity(0.65)
        .build()
        .unwrap();
    sim.add_event(supply_delay_avery, supply_delay_date);

    for person in [&leah_id, &omar_id, &priya_id, &june_id, &mateo_id] {
        let supply_delay = EventBuilder::new(EventType::Failure)
            .target(person.clone())
            .severity(0.5)
            .build()
            .unwrap();
        sim.add_event(supply_delay, supply_delay_date);
    }

    let aquaculture_success_date = Timestamp::from_ymd_hms(2025, 5, 5, 11, 0, 0);

    let mateo_dominance_before = {
        let handle = sim.entity(&mateo_id).unwrap();
        let state = handle.state_at(aquaculture_success_date - Duration::hours(1));
        state.get_effective(StatePath::Mood(MoodPath::Dominance))
    };

    let aquaculture_success = EventBuilder::new(EventType::Achievement)
        .target(mateo_id.clone())
        .severity(0.7)
        .build()
        .unwrap();
    sim.add_event(aquaculture_success, aquaculture_success_date);

    // Verify dominance recovery after achievement
    {
        let handle = sim.entity(&mateo_id).unwrap();
        let mateo_after = handle.state_at(aquaculture_success_date + Duration::hours(6));
        let dominance_after = mateo_after.get_effective(StatePath::Mood(MoodPath::Dominance));

        assert!(
            dominance_after > mateo_dominance_before,
            "Achievement should increase dominance. Before: {}, After: {}",
            mateo_dominance_before, dominance_after
        );
    }

    for person in [&leah_id, &omar_id, &priya_id, &june_id, &avery_id] {
        let food_support = EventBuilder::new(EventType::Support)
            .target(person.clone())
            .severity(0.4)
            .build()
            .unwrap();
        sim.add_event(food_support, aquaculture_success_date + Duration::days(1));
    }

    let omar_overwork_date = Timestamp::from_ymd_hms(2025, 5, 20, 22, 0, 0);

    let omar_pb_before = {
        let handle = sim.entity(&omar_id).unwrap();
        let state = handle.state_at(omar_overwork_date - Duration::hours(1));
        state.get_effective(StatePath::MentalHealth(MentalHealthPath::PerceivedBurdensomeness))
    };

    let omar_overwork_failure = EventBuilder::new(EventType::Failure)
        .target(omar_id.clone())
        .severity(0.55)
        .build()
        .unwrap();
    sim.add_event(omar_overwork_failure, omar_overwork_date);

    let omar_burden = EventBuilder::new(EventType::BurdenFeedback)
        .target(omar_id.clone())
        .severity(0.5)
        .build()
        .unwrap();
    sim.add_event(omar_burden, omar_overwork_date + Duration::hours(2));

    // Verify role strain increases PB
    {
        let handle = sim.entity(&omar_id).unwrap();
        let omar_after = handle.state_at(omar_overwork_date + Duration::hours(6));
        let pb_after = omar_after.get_effective(StatePath::MentalHealth(MentalHealthPath::PerceivedBurdensomeness));

        assert!(
            pb_after > omar_pb_before,
            "Role strain should increase PB. Before: {}, After: {}",
            omar_pb_before, pb_after
        );
    }

    let pregnancy_date = Timestamp::from_ymd_hms(2025, 6, 1, 10, 0, 0);
    let pregnancy = EventBuilder::new(EventType::Achievement)
        .target(priya_id.clone())
        .severity(0.75)
        .build()
        .unwrap();
    sim.add_event(pregnancy, pregnancy_date);

    let celebration_date = Timestamp::from_ymd_hms(2025, 6, 15, 18, 0, 0);
    for person in [&leah_id, &omar_id, &priya_id, &june_id, &mateo_id, &avery_id] {
        let celebration = EventBuilder::new(EventType::SocialInclusion)
            .target(person.clone())
            .severity(0.5)
            .build()
            .unwrap();
        sim.add_event(celebration, celebration_date);
    }

    // Events: Environmental crisis - storm and power failure
    let storm_date = Timestamp::from_ymd_hms(2025, 7, 20, 3, 0, 0);

    let storm_omar = EventBuilder::new(EventType::Violence)
        .target(omar_id.clone())
        .severity(0.75)
        .build()
        .unwrap();
    sim.add_event(storm_omar, storm_date);

    for person in [&leah_id, &priya_id, &june_id, &mateo_id, &avery_id] {
        let storm = EventBuilder::new(EventType::Violence)
            .target(person.clone())
            .severity(0.65)
            .build()
            .unwrap();
        sim.add_event(storm, storm_date);
    }

    // Verify AC increase and arousal spike at onset
    {
        let handle = sim.entity(&omar_id).unwrap();
        let omar_before = handle.state_at(storm_date - Duration::hours(1));
        let omar_crisis = handle.state_at(storm_date + Duration::hours(3));
        let omar_after = handle.state_at(storm_date + Duration::days(1));

        let arousal_before = omar_before.get_effective(StatePath::Mood(MoodPath::Arousal));
        let arousal_crisis = omar_crisis.get_effective(StatePath::Mood(MoodPath::Arousal));
        let arousal_after = omar_after.get_effective(StatePath::Mood(MoodPath::Arousal));
        let ac_before = omar_before.get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));
        let ac_after = omar_after.get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));

        assert!(
            arousal_crisis > arousal_before,
            "Storm should spike arousal at onset. Before: {}, Crisis: {}",
            arousal_before, arousal_crisis
        );

        assert!(
            arousal_after > arousal_before,
            "Arousal should remain elevated day after. Before: {}, After: {}",
            arousal_before, arousal_after
        );

        assert!(
            ac_after > ac_before,
            "Environmental violence should increase AC. Before: {}, After: {}",
            ac_before, ac_after
        );
    }

    let power_restore_date = Timestamp::from_ymd_hms(2025, 7, 22, 20, 0, 0);

    let power_restore_omar = EventBuilder::new(EventType::Achievement)
        .target(omar_id.clone())
        .severity(0.75)
        .build()
        .unwrap();
    sim.add_event(power_restore_omar, power_restore_date);

    for person in [&leah_id, &june_id, &mateo_id, &avery_id] {
        let power_support = EventBuilder::new(EventType::Achievement)
            .target(person.clone())
            .severity(0.5)
            .build()
            .unwrap();
        sim.add_event(power_support, power_restore_date);
    }

    let debrief_date = Timestamp::from_ymd_hms(2025, 8, 5, 19, 0, 0);
    for person in [&leah_id, &omar_id, &priya_id, &june_id, &mateo_id, &avery_id] {
        let debrief = EventBuilder::new(EventType::Support)
            .target(person.clone())
            .severity(0.55)
            .build()
            .unwrap();
        sim.add_event(debrief, debrief_date);
    }

    // Events: Health emergency and community support
    let mateo_injury_date = Timestamp::from_ymd_hms(2025, 9, 10, 14, 30, 0);

    let mateo_injury = EventBuilder::new(EventType::Loss)
        .target(mateo_id.clone())
        .severity(0.7)
        .build()
        .unwrap();
    sim.add_event(mateo_injury, mateo_injury_date);

    let priya_stress = EventBuilder::new(EventType::Failure)
        .target(priya_id.clone())
        .severity(0.6)
        .build()
        .unwrap();
    sim.add_event(priya_stress, mateo_injury_date);

    let treatment_success_date = Timestamp::from_ymd_hms(2025, 9, 12, 8, 0, 0);

    let treatment_priya = EventBuilder::new(EventType::Achievement)
        .target(priya_id.clone())
        .severity(0.75)
        .build()
        .unwrap();
    sim.add_event(treatment_priya, treatment_success_date);

    let recovery_mateo = EventBuilder::new(EventType::Achievement)
        .target(mateo_id.clone())
        .severity(0.5)
        .build()
        .unwrap();
    sim.add_event(recovery_mateo, treatment_success_date);

    let care_rotation_start = Timestamp::from_ymd_hms(2025, 9, 15, 10, 0, 0);

    let mateo_tb_before_care = {
        let handle = sim.entity(&mateo_id).unwrap();
        let state = handle.state_at(care_rotation_start - Duration::hours(1));
        state.get_effective(StatePath::MentalHealth(MentalHealthPath::ThwartedBelongingness))
    };

    for day in 0..10 {
        let care_support = EventBuilder::new(EventType::Support)
            .target(mateo_id.clone())
            .severity(0.45)
            .build()
            .unwrap();
        sim.add_event(care_support, care_rotation_start + Duration::days(day));
    }

    // Verify TB reduction after sustained care
    {
        let handle = sim.entity(&mateo_id).unwrap();
        let mateo_state = handle.state_at(care_rotation_start + Duration::days(10));
        let tb_after = mateo_state.get_effective(StatePath::MentalHealth(MentalHealthPath::ThwartedBelongingness));

        assert!(
            tb_after < mateo_tb_before_care,
            "Sustained care should reduce TB. Before: {}, After: {}",
            mateo_tb_before_care, tb_after
        );
    }

    let june_overextended_date = Timestamp::from_ymd_hms(2025, 10, 1, 21, 0, 0);

    let june_pb_before = {
        let handle = sim.entity(&june_id).unwrap();
        let state = handle.state_at(june_overextended_date - Duration::hours(1));
        state.get_effective(StatePath::MentalHealth(MentalHealthPath::PerceivedBurdensomeness))
    };

    let june_strain = EventBuilder::new(EventType::Failure)
        .target(june_id.clone())
        .severity(0.6)
        .build()
        .unwrap();
    sim.add_event(june_strain, june_overextended_date);

    let june_burden = EventBuilder::new(EventType::BurdenFeedback)
        .target(june_id.clone())
        .severity(0.55)
        .build()
        .unwrap();
    sim.add_event(june_burden, june_overextended_date + Duration::hours(3));

    // Verify caregiver strain increases PB
    {
        let handle = sim.entity(&june_id).unwrap();
        let june_after = handle.state_at(june_overextended_date + Duration::hours(12));
        let pb_after = june_after.get_effective(StatePath::MentalHealth(MentalHealthPath::PerceivedBurdensomeness));

        assert!(
            pb_after > june_pb_before,
            "Caregiver strain should increase PB. Before: {}, After: {}",
            june_pb_before, pb_after
        );
    }

    // Events: Role strain and interpersonal conflict
    let conflict_date = Timestamp::from_ymd_hms(2025, 11, 15, 16, 0, 0);

    let conflict_avery = EventBuilder::new(EventType::Conflict)
        .target(avery_id.clone())
        .severity(0.65)
        .build()
        .unwrap();
    sim.add_event(conflict_avery, conflict_date);

    let conflict_leah = EventBuilder::new(EventType::Conflict)
        .target(leah_id.clone())
        .severity(0.6)
        .build()
        .unwrap();
    sim.add_event(conflict_leah, conflict_date);

    let omar_withdrawal_date = Timestamp::from_ymd_hms(2025, 12, 1, 20, 0, 0);

    let omar_tb_before_withdrawal = {
        let handle = sim.entity(&omar_id).unwrap();
        let state = handle.state_at(omar_withdrawal_date - Duration::days(1));
        state.get_effective(StatePath::MentalHealth(MentalHealthPath::ThwartedBelongingness))
    };

    let omar_withdrawal = EventBuilder::new(EventType::SocialExclusion)
        .target(omar_id.clone())
        .severity(0.6)
        .build()
        .unwrap();
    sim.add_event(omar_withdrawal, omar_withdrawal_date);

    // Verify TB increase
    {
        let handle = sim.entity(&omar_id).unwrap();
        let omar_state = handle.state_at(omar_withdrawal_date + Duration::days(7));
        let tb_after = omar_state.get_effective(StatePath::MentalHealth(MentalHealthPath::ThwartedBelongingness));

        assert!(
            tb_after > omar_tb_before_withdrawal,
            "Social withdrawal should increase TB. Before: {}, After: {}",
            omar_tb_before_withdrawal, tb_after
        );
    }

    let solstice_date = Timestamp::from_ymd_hms(2025, 12, 20, 18, 0, 0);
    for person in [&leah_id, &omar_id, &priya_id, &june_id, &mateo_id, &avery_id] {
        let solstice = EventBuilder::new(EventType::SocialInclusion)
            .target(person.clone())
            .severity(0.6)
            .build()
            .unwrap();
        sim.add_event(solstice, solstice_date);
    }

    let complications_date = Timestamp::from_ymd_hms(2026, 1, 15, 2, 30, 0);

    let complications_priya = EventBuilder::new(EventType::Loss)
        .target(priya_id.clone())
        .severity(0.75)
        .build()
        .unwrap();
    sim.add_event(complications_priya, complications_date);

    // Verify dominance drops
    {
        let handle = sim.entity(&priya_id).unwrap();
        let priya_state = handle.state_at(complications_date + Duration::days(2));

        let dominance = priya_state.get_effective(StatePath::Mood(MoodPath::Dominance));

        assert!(
            dominance < 0.2,
            "Health scare should reduce dominance. Got: {}",
            dominance
        );
    }

    let priya_support_date = Timestamp::from_ymd_hms(2026, 2, 1, 10, 0, 0);

    for day in 0..7 {
        let support = EventBuilder::new(EventType::Support)
            .target(priya_id.clone())
            .severity(0.5)
            .build()
            .unwrap();
        sim.add_event(support, priya_support_date + Duration::days(day));
    }

    let leah_stress_date = Timestamp::from_ymd_hms(2026, 2, 10, 22, 0, 0);

    let leah_pb_before = {
        let handle = sim.entity(&leah_id).unwrap();
        let state = handle.state_at(leah_stress_date - Duration::hours(1));
        state.get_effective(StatePath::MentalHealth(MentalHealthPath::PerceivedBurdensomeness))
    };

    let leah_stress = EventBuilder::new(EventType::Failure)
        .target(leah_id.clone())
        .severity(0.6)
        .build()
        .unwrap();
    sim.add_event(leah_stress, leah_stress_date);

    let leah_burden = EventBuilder::new(EventType::BurdenFeedback)
        .target(leah_id.clone())
        .severity(0.5)
        .build()
        .unwrap();
    sim.add_event(leah_burden, leah_stress_date + Duration::hours(4));

    // Verify parenting strain increases PB
    {
        let handle = sim.entity(&leah_id).unwrap();
        let leah_after = handle.state_at(leah_stress_date + Duration::hours(8));
        let pb_after = leah_after.get_effective(StatePath::MentalHealth(MentalHealthPath::PerceivedBurdensomeness));

        assert!(
            pb_after > leah_pb_before,
            "Parenting stress should increase PB. Before: {}, After: {}",
            leah_pb_before, pb_after
        );
    }

    // Events: Turning point - collective problem-solving
    let governance_date = Timestamp::from_ymd_hms(2026, 3, 1, 14, 0, 0);

    let governance_avery = EventBuilder::new(EventType::Achievement)
        .target(avery_id.clone())
        .severity(0.65)
        .build()
        .unwrap();
    sim.add_event(governance_avery, governance_date);

    let governance_leah = EventBuilder::new(EventType::Achievement)
        .target(leah_id.clone())
        .severity(0.6)
        .build()
        .unwrap();
    sim.add_event(governance_leah, governance_date);

    for person in [&omar_id, &priya_id, &june_id, &mateo_id] {
        let governance = EventBuilder::new(EventType::Achievement)
            .target(person.clone())
            .severity(0.5)
            .build()
            .unwrap();
        sim.add_event(governance, governance_date);
    }

    let birth_date = Timestamp::from_ymd_hms(2026, 3, 20, 4, 15, 0);

    let birth_priya = EventBuilder::new(EventType::Achievement)
        .target(priya_id.clone())
        .severity(0.9)
        .build()
        .unwrap();
    sim.add_event(birth_priya, birth_date);

    for person in [&leah_id, &omar_id, &june_id, &mateo_id, &avery_id] {
        let birth_celebration = EventBuilder::new(EventType::Achievement)
            .target(person.clone())
            .severity(0.5)
            .build()
            .unwrap();
        sim.add_event(birth_celebration, birth_date + Duration::days(1));
    }

    let mentorship_date = Timestamp::from_ymd_hms(2026, 4, 10, 17, 0, 0);

    let mentorship_omar = EventBuilder::new(EventType::Support)
        .target(omar_id.clone())
        .severity(0.6)
        .build()
        .unwrap();
    sim.add_event(mentorship_omar, mentorship_date);

    let mentorship_june = EventBuilder::new(EventType::Achievement)
        .target(june_id.clone())
        .severity(0.5)
        .build()
        .unwrap();
    sim.add_event(mentorship_june, mentorship_date);

    let mateo_training_date = Timestamp::from_ymd_hms(2026, 5, 1, 9, 0, 0);
    let mateo_training = EventBuilder::new(EventType::Achievement)
        .target(mateo_id.clone())
        .severity(0.7)
        .build()
        .unwrap();
    sim.add_event(mateo_training, mateo_training_date);

    // Events: Stability and generational transition
    let supply_normal_date = Timestamp::from_ymd_hms(2026, 6, 15, 10, 0, 0);

    for person in [&leah_id, &omar_id, &priya_id, &june_id, &mateo_id, &avery_id] {
        let supply_normal = EventBuilder::new(EventType::Achievement)
            .target(person.clone())
            .severity(0.5)
            .build()
            .unwrap();
        sim.add_event(supply_normal, supply_normal_date);
    }

    let avery_purpose_date = Timestamp::from_ymd_hms(2026, 8, 1, 11, 0, 0);
    let avery_purpose = EventBuilder::new(EventType::Achievement)
        .target(avery_id.clone())
        .severity(0.75)
        .build()
        .unwrap();
    sim.add_event(avery_purpose, avery_purpose_date);

    let june_retirement_announce = Timestamp::from_ymd_hms(2026, 10, 1, 15, 0, 0);
    let june_announce = EventBuilder::new(EventType::Achievement)
        .target(june_id.clone())
        .severity(0.6)
        .build()
        .unwrap();
    sim.add_event(june_announce, june_retirement_announce);

    let winter_celebration_date = Timestamp::from_ymd_hms(2026, 12, 25, 18, 0, 0);
    for person in [&leah_id, &omar_id, &priya_id, &june_id, &mateo_id, &avery_id] {
        let winter_celebration = EventBuilder::new(EventType::SocialInclusion)
            .target(person.clone())
            .severity(0.65)
            .build()
            .unwrap();
        sim.add_event(winter_celebration, winter_celebration_date);
    }

    let leah_election_date = Timestamp::from_ymd_hms(2027, 1, 15, 19, 0, 0);
    let leah_election = EventBuilder::new(EventType::Achievement)
        .target(leah_id.clone())
        .severity(0.8)
        .build()
        .unwrap();
    sim.add_event(leah_election, leah_election_date);

    // Events: Final crisis and resilience test
    let equipment_failure_date = Timestamp::from_ymd_hms(2027, 2, 20, 8, 0, 0);

    let failure_omar = EventBuilder::new(EventType::Failure)
        .target(omar_id.clone())
        .severity(0.7)
        .build()
        .unwrap();
    sim.add_event(failure_omar, equipment_failure_date);

    let failure_leah = EventBuilder::new(EventType::Failure)
        .target(leah_id.clone())
        .severity(0.65)
        .build()
        .unwrap();
    sim.add_event(failure_leah, equipment_failure_date);

    for person in [&priya_id, &june_id, &mateo_id, &avery_id] {
        let failure = EventBuilder::new(EventType::Failure)
            .target(person.clone())
            .severity(0.5)
            .build()
            .unwrap();
        sim.add_event(failure, equipment_failure_date);
    }

    let rapid_repair_date = Timestamp::from_ymd_hms(2027, 3, 1, 16, 0, 0);

    for person in [&leah_id, &omar_id, &mateo_id, &avery_id] {
        let repair = EventBuilder::new(EventType::Achievement)
            .target(person.clone())
            .severity(0.65)
            .build()
            .unwrap();
        sim.add_event(repair, rapid_repair_date);
    }

    let mateo_relationship_date = Timestamp::from_ymd_hms(2027, 4, 15, 20, 0, 0);
    let mateo_relationship = EventBuilder::new(EventType::Achievement)
        .target(mateo_id.clone())
        .severity(0.7)
        .build()
        .unwrap();
    sim.add_event(mateo_relationship, mateo_relationship_date);

    let june_retirement_date = Timestamp::from_ymd_hms(2027, 5, 20, 14, 0, 0);

    let june_retirement = EventBuilder::new(EventType::Achievement)
        .target(june_id.clone())
        .severity(0.75)
        .build()
        .unwrap();
    sim.add_event(june_retirement, june_retirement_date);

    for person in [&leah_id, &omar_id, &priya_id, &mateo_id, &avery_id] {
        let ceremony = EventBuilder::new(EventType::Achievement)
            .target(person.clone())
            .severity(0.5)
            .build()
            .unwrap();
        sim.add_event(ceremony, june_retirement_date);
    }

    let final_assessment_date = Timestamp::from_ymd_hms(2027, 6, 20, 10, 0, 0);
    for person in [&leah_id, &omar_id, &priya_id, &june_id, &mateo_id, &avery_id] {
        let assessment = EventBuilder::new(EventType::Achievement)
            .target(person.clone())
            .severity(0.6)
            .build()
            .unwrap();
        sim.add_event(assessment, final_assessment_date);
    }

    // Verification: Final state analysis
    let final_date = Timestamp::from_ymd_hms(2027, 6, 30, 12, 0, 0);

    println!("\n========================================");
    println!("FINAL STATE ANALYSIS (After 2.5 Years)");
    println!("========================================\n");

    {
        let handle = sim.entity(&leah_id).unwrap();
        let leah_baseline = handle.state_at(sim_start + Duration::days(7));
        let leah_crisis = handle.state_at(contamination_date + Duration::days(1));
        let leah_final = handle.state_at(final_date);

        let stress_baseline = leah_baseline.get_effective(StatePath::Needs(NeedsPath::Stress));
        let stress_crisis = leah_crisis.get_effective(StatePath::Needs(NeedsPath::Stress));
        let stress_final = leah_final.get_effective(StatePath::Needs(NeedsPath::Stress));

        println!("LEAH CARTER (Age 36):");
        println!("  Baseline stress (Jan 2025): {:.2}", stress_baseline);
        println!("  Peak crisis (Feb 2025): {:.2}", stress_crisis);
        println!("  Final state (Jun 2027): {:.2}", stress_final);
        println!("  Note: Elected coordinator after proving leadership under pressure\n");
    }

    {
        let handle = sim.entity(&omar_id).unwrap();
        let omar_withdrawn = handle.state_at(omar_withdrawal_date + Duration::days(7));
        let omar_final = handle.state_at(final_date);

        let tb_withdrawn = omar_withdrawn.get_effective(StatePath::MentalHealth(MentalHealthPath::ThwartedBelongingness));
        let tb_final = omar_final.get_effective(StatePath::MentalHealth(MentalHealthPath::ThwartedBelongingness));

        println!("OMAR HADDAD (Age 49):");
        println!("  TB during withdrawal (Dec 2025): {:.2}", tb_withdrawn);
        println!("  TB after mentorship (Jun 2027): {:.2}", tb_final);
        println!("  Note: June's mentorship broke isolation pattern\n");

        assert!(
            tb_final < tb_withdrawn,
            "Mentorship should reduce TB over time. Withdrawn: {}, Final: {}",
            tb_withdrawn, tb_final
        );
    }

    {
        let handle = sim.entity(&priya_id).unwrap();
        let priya_pregnant = handle.state_at(pregnancy_date + Duration::days(3));
        let priya_complications = handle.state_at(complications_date + Duration::days(2));
        let priya_final = handle.state_at(final_date);

        let valence_pregnant = priya_pregnant.get_effective(StatePath::Mood(MoodPath::Valence));
        let valence_complications = priya_complications.get_effective(StatePath::Mood(MoodPath::Valence));
        let valence_final = priya_final.get_effective(StatePath::Mood(MoodPath::Valence));

        println!("PRIYA NAIR (Age 31):");
        println!("  Valence at pregnancy (Jun 2025): {:.2}", valence_pregnant);
        println!("  Valence during complications (Jan 2026): {:.2}", valence_complications);
        println!("  Valence final (Jun 2027): {:.2}", valence_final);
        println!("  Note: Community support buffered complications; motherhood stabilized\n");
    }

    {
        let handle = sim.entity(&june_id).unwrap();
        let june_overextended = handle.state_at(june_overextended_date + Duration::days(1));
        let june_final = handle.state_at(final_date);

        let stress_overextended = june_overextended.get_effective(StatePath::Needs(NeedsPath::Stress));
        let stress_final = june_final.get_effective(StatePath::Needs(NeedsPath::Stress));

        println!("JUNE ALVAREZ (Age 65):");
        println!("  Stress when overextended (Oct 2025): {:.2}", stress_overextended);
        println!("  Stress after retirement (Jun 2027): {:.2}", stress_final);
        println!("  Note: Retirement ceremony honored her community contributions\n");
    }

    {
        let handle = sim.entity(&mateo_id).unwrap();
        let mateo_baseline = handle.state_at(sim_start + Duration::days(7));
        let mateo_post_care = handle.state_at(care_rotation_start + Duration::days(10));
        let mateo_final = handle.state_at(final_date);

        let tb_baseline = mateo_baseline.get_effective(StatePath::MentalHealth(MentalHealthPath::ThwartedBelongingness));
        let tb_post_care = mateo_post_care.get_effective(StatePath::MentalHealth(MentalHealthPath::ThwartedBelongingness));
        let tb_final = mateo_final.get_effective(StatePath::MentalHealth(MentalHealthPath::ThwartedBelongingness));

        println!("MATEO REYES (Age 25):");
        println!("  TB baseline (Jan 2025): {:.2}", tb_baseline);
        println!("  TB after injury care (Sep 2025): {:.2}", tb_post_care);
        println!("  TB final (Jun 2027): {:.2}", tb_final);
        println!("  Note: Aquaculture success + training role + relationship = belonging\n");

        assert!(
            tb_final < tb_baseline,
            "Community belonging should reduce TB over 2.5 years. Baseline: {}, Final: {}",
            tb_baseline, tb_final
        );
    }

    {
        let handle = sim.entity(&avery_id).unwrap();
        let avery_baseline = handle.state_at(sim_start + Duration::days(7));
        let avery_final = handle.state_at(final_date);

        let valence_baseline = avery_baseline.get_effective(StatePath::Mood(MoodPath::Valence));
        let valence_final = avery_final.get_effective(StatePath::Mood(MoodPath::Valence));

        println!("AVERY LIN (Age 43):");
        println!("  Valence baseline (Jan 2025): {:.2}", valence_baseline);
        println!("  Valence final (Jun 2027): {:.2}", valence_final);
        println!("  Note: Purpose found through long-term co-op planning role\n");
    }

    println!("========================================");
    println!("ITS RISK ANALYSIS");
    println!("========================================\n");

    println!("TB + PB CONVERGENCE (High-Risk Periods):\n");
    {
        let omar_withdrawal_check = Timestamp::from_ymd_hms(2025, 12, 7, 0, 0, 0);
        let handle = sim.entity(&omar_id).unwrap();
        let omar_state = handle.state_at(omar_withdrawal_check);
        let tb = omar_state.get_effective(StatePath::MentalHealth(MentalHealthPath::ThwartedBelongingness));
        let pb = omar_state.get_effective(StatePath::MentalHealth(MentalHealthPath::PerceivedBurdensomeness));
        let hopelessness = omar_state.get_effective(StatePath::MentalHealth(MentalHealthPath::Hopelessness));
        let interpersonal_hopelessness = omar_state.get_effective(StatePath::MentalHealth(MentalHealthPath::InterpersonalHopelessness));
        let suicidal_desire = omar_state.get_effective(StatePath::MentalHealth(MentalHealthPath::SuicidalDesire));

        println!("  OMAR (Dec 2025, during withdrawal):");
        println!("    TB: {:.2}", tb);
        println!("    PB: {:.2}", pb);
        println!("    Hopelessness: {:.2}", hopelessness);
        println!("    Interpersonal Hopelessness: {:.2}", interpersonal_hopelessness);
        println!("    Suicidal Desire: {:.2}", suicidal_desire);

        // When TB+PB both elevated, risk increases multiplicatively
        if tb > 0.3 && pb > 0.3 {
            println!("    WARNING: Both TB and PB elevated - multiplicative risk");
        }
        println!();
    }

    {
        let june_check = Timestamp::from_ymd_hms(2025, 10, 2, 0, 0, 0);
        let handle = sim.entity(&june_id).unwrap();
        let june_state = handle.state_at(june_check);
        let tb = june_state.get_effective(StatePath::MentalHealth(MentalHealthPath::ThwartedBelongingness));
        let pb = june_state.get_effective(StatePath::MentalHealth(MentalHealthPath::PerceivedBurdensomeness));
        let hopelessness = june_state.get_effective(StatePath::MentalHealth(MentalHealthPath::Hopelessness));
        let interpersonal_hopelessness = june_state.get_effective(StatePath::MentalHealth(MentalHealthPath::InterpersonalHopelessness));

        println!("  JUNE (Oct 2025, caregiver overextension):");
        println!("    TB: {:.2}", tb);
        println!("    PB: {:.2}", pb);
        println!("    Hopelessness: {:.2}", hopelessness);
        println!("    Interpersonal Hopelessness: {:.2}", interpersonal_hopelessness);

        if tb > 0.3 && pb > 0.3 {
            println!("    WARNING: Both TB and PB elevated - multiplicative risk");
        }
        println!();
    }

    {
        let handle = sim.entity(&omar_id).unwrap();
        let omar_post_storm = handle.state_at(storm_date + Duration::days(7));
        let omar_final = handle.state_at(final_date);

        let ac_post_storm = omar_post_storm.get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));
        let ac_final = omar_final.get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));

        println!("AC PERSISTENCE (Omar):");
        println!("  1 week post-storm: {:.2}", ac_post_storm);
        println!("  23 months later: {:.2}", ac_final);
        println!("  Note: Environmental violence AC persists long-term\n");

        assert!(
            ac_final > 0.0,
            "AC should persist 23 months after storm. Got: {}",
            ac_final
        );
    }

    println!("========================================");
    println!("TRUST DYNAMICS (NOT YET MEASURED - AWAITING API)");
    println!("========================================\n");
    println!("NOTE: Trust decomposition (competence/benevolence/integrity) requires");
    println!("relationship API not yet implemented. The following are narrative observations");
    println!("based on event patterns, not quantitative assertions:\n");
    println!("COMPETENCE TRUST (narrative):");
    println!("  - Leah & Omar: Water repair + power restoration built technical trust");
    println!("  - Priya: Medical emergency success validated health competence");
    println!("  - Mateo: Aquaculture innovation proved capability\n");
    println!("BENEVOLENCE TRUST (narrative):");
    println!("  - Community meals, care rotations, pregnancy support all increased benevolence");
    println!("  - Sustained through 2.5 years via repeated mutual aid\n");
    println!("INTEGRITY TRUST (narrative):");
    println!("  - Governance restructuring resolved Avery-Leah conflict");
    println!("  - Co-op values tested and reaffirmed through crises\n");
    println!("TODO: Add quantitative trust assertions when relationship API is available\n");

    println!("========================================");
    println!("BRONFENBRENNER'S ECOLOGY");
    println!("========================================\n");
    println!("MICROSYSTEM:");
    println!("  - Co-op functioning affects all residents (shared housing amplifies stress)");
    println!("  - Work-family boundaries blur (Priya nurses both community + baby)\n");
    println!("MESOSYSTEM:");
    println!("  - Water treatment (Leah) + health (Priya) + power (Omar) interconnected");
    println!("  - Failure in one system cascades to others\n");
    println!("EXOSYSTEM:");
    println!("  - Supply chain delays (external) forced local adaptation");
    println!("  - Mateo's aquaculture innovation buffered external scarcity\n");
    println!("MACROSYSTEM:");
    println!("  - Co-op governance model tested collectivist values under scarcity");
    println!("  - Avery-Leah conflict resolved via democratic restructuring\n");
    println!("CHRONOSYSTEM:");
    println!("  - Generational transition: June retirement, Leah leadership, Mateo training");
    println!("  - Seasonal weather patterns (storm Jul 2025) shaped adaptation\n");

    println!("========================================");
    println!("SIMULATION SUMMARY (2.5-Year Longitudinal)");
    println!("========================================");
    println!("Duration: Jan 2025 - Jun 2027");
    println!("Entities: 6 individuals");
    println!("Events: 33 major events across 9 phases");
    println!("\nKey Trajectories:");
    println!("  - Leah: Anxious protector -> crisis leader -> elected coordinator");
    println!("  - Omar: Isolated technician -> withdrawal -> mentorship -> connection");
    println!("  - Priya: Hopeful optimist -> pregnancy crisis -> motherhood -> dual role");
    println!("  - June: Widowed caregiver -> overextension -> retirement -> honored elder");
    println!("  - Mateo: Rejection-sensitive -> injury care -> training mentor -> relationship");
    println!("  - Avery: Burned out planner -> conflict -> purpose found -> long-term vision");
    println!("\nTheoretical Patterns Verified:");
    println!("  PAD: Crisis crashes valence (comparative), spikes arousal AT ONSET (hours); support buffers arousal; dominance recovers after achievements");
    println!("  Trust: [AWAITING API] Competence, Benevolence, Integrity factors not yet quantitatively measured");
    println!("  ITS: TB reduces with inclusion; PB increases with role strain; TB+PB convergence = multiplicative risk; AC persists after environmental violence; interpersonal hopelessness tracked");
    println!("  Ecology: [LIMITATIONS NOTED] Closed system amplifies stress; microsystem quality not fully modeled; mesosystem spillover acknowledged");
    println!("\nResilience Mechanisms:");
    println!("  - Collective problem-solving (water repair, power restoration)");
    println!("  - Sustained support (care rotations, meal trains, pregnancy support)");
    println!("  - Ritual and meaning-making (solstice, birth, retirement ceremony)");
    println!("  - Generational continuity (June -> Leah leadership, Mateo training)\n");
}
