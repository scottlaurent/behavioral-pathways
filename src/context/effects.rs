//! Context effects for state evolution.
//!
//! This module applies ecological context effects to entity state during
//! temporal state computation. Effects include microsystem influences,
//! mesosystem spillover, and macrosystem cultural constraints.
//!
//! # Phase 11 Implementation
//!
//! This is currently a stub that returns state unchanged. In Phase 11,
//! this will implement:
//!
//! - Microsystem effects (direct environment influences)
//! - Mesosystem effects (cross-context spillover and role conflict)
//! - Exosystem effects (indirect influences like parent work stress)
//! - Macrosystem effects (cultural stress, collective trauma)
//! - Chronosystem effects (historical period impacts)

use crate::context::mesosystem::{
    check_proximal_process_gate, MesosystemState, INTERACTION_COMPLEXITY_THRESHOLD,
    INTERACTION_FREQUENCY_THRESHOLD,
};
use crate::context::EcologicalContext;
use crate::enums::{BirthEra, LifeStage};
use crate::state::IndividualState;
use crate::types::{Duration, Timestamp};

/// Applies ecological context effects to state.
///
/// This function applies Bronfenbrenner's ecological layers effects to the
/// state during temporal evolution. It is called by `state_at()` AFTER
/// decay and event processing, and AFTER developmental modifiers.
///
/// # Arguments
///
/// * `state` - The current state after decay, events, and developmental modifiers
/// * `context` - The ecological context (microsystem through chronosystem)
/// * `relationship_quality` - Average relationship quality (0.0 to 1.0)
/// * `duration` - The time elapsed
/// * `current_timestamp` - Absolute timestamp for the current state
/// * `life_stage` - Life stage at the target timestamp
///
/// # Returns
///
/// The state with context effects applied.
/// Currently returns state unchanged (stub behavior).
///
/// # Examples
///
/// ```ignore
/// use behavioral_pathways::context::apply_context_effects;
/// use behavioral_pathways::state::IndividualState;
/// use behavioral_pathways::types::{Duration, Timestamp};
///
/// let state = IndividualState::new();
/// # use behavioral_pathways::context::EcologicalContext;
/// let context = EcologicalContext::default();
/// use behavioral_pathways::enums::LifeStage;
/// let modified = apply_context_effects(
///     state.clone(),
///     &context,
///     0.5,
///     Duration::days(30),
///     LifeStage::Adult,
///     Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0),
/// );
///
/// // Currently a stub - returns unchanged
/// assert_eq!(modified.mood().valence_delta(), state.mood().valence_delta());
/// ```
#[must_use]
pub(crate) fn apply_context_effects(
    mut state: IndividualState,
    context: &EcologicalContext,
    relationship_quality: f64,
    duration: Duration,
    life_stage: LifeStage,
    current_timestamp: Timestamp,
) -> IndividualState {
    let time_scale = duration_scale(duration);

    if time_scale <= 0.0 {
        return state;
    }

    let (avg_frequency, avg_complexity) = compute_aggregate_interaction_metrics(context);
    let microsystem_multiplier = match check_proximal_process_gate(
        avg_frequency,
        avg_complexity,
        INTERACTION_FREQUENCY_THRESHOLD,
        INTERACTION_COMPLEXITY_THRESHOLD,
    ) {
        Ok(()) => 1.0,
        Err(_) => 0.0,
    };

    if microsystem_multiplier > 0.0 {
        apply_microsystem_effects(&mut state, context, time_scale, microsystem_multiplier);
        apply_mesosystem_spillover(&mut state, context, time_scale, microsystem_multiplier);

        let (stress_adj, loneliness_adj) =
            context.compute_context_to_person_effects(relationship_quality);
        state
            .needs_mut()
            .add_stress_delta(stress_adj * time_scale);
        state
            .social_cognition_mut()
            .add_loneliness_delta(loneliness_adj * time_scale);
    }

    apply_exosystem_effects(&mut state, context, time_scale, life_stage);
    apply_macrosystem_effects(&mut state, context, time_scale, relationship_quality);
    apply_chronosystem_effects(&mut state, context, time_scale, current_timestamp);
    state
}

fn duration_scale(duration: Duration) -> f32 {
    let days = duration.as_days_f64();
    if days <= 0.0 {
        return 0.0;
    }
    (days / 30.0) as f32
}

fn compute_aggregate_interaction_metrics(context: &EcologicalContext) -> (f64, f64) {
    let mut total_frequency = 0.0;
    let mut total_complexity = 0.0;
    let mut count = 0.0;

    for (_, microsystem) in context.microsystems_iter() {
        total_frequency += microsystem.interaction_frequency();
        total_complexity += microsystem.interaction_complexity();
        count += 1.0;
    }

    if count == 0.0 {
        return (0.0, 0.0);
    }

    (total_frequency / count, total_complexity / count)
}

fn apply_microsystem_effects(
    state: &mut IndividualState,
    context: &EcologicalContext,
    time_scale: f32,
    microsystem_multiplier: f64,
) {
    for (_, microsystem) in context.microsystems_iter() {
        if let Some(work) = microsystem.work() {
            let workload_excess = (work.workload_stress - 0.7).max(0.0);
            let stress_delta = (workload_excess * 0.15 * microsystem_multiplier) as f32;
            let fatigue_delta = (workload_excess * 0.1 * microsystem_multiplier) as f32;
            state
                .needs_mut()
                .add_stress_delta(stress_delta * time_scale);
            state
                .needs_mut()
                .add_fatigue_delta(fatigue_delta * time_scale);
        }

        if let Some(family) = microsystem.family() {
            let role_multiplier = family.family_role.effect_multiplier();
            let support_level = (family.family_satisfaction + family.warmth) / 2.0;
            let support_buffer = (support_level - 0.6).max(0.0) * 0.1 * role_multiplier;
            let buffer_delta = (support_buffer * microsystem_multiplier) as f32;
            state
                .needs_mut()
                .add_stress_delta(-buffer_delta * time_scale);
            state
                .social_cognition_mut()
                .add_loneliness_delta(-buffer_delta * time_scale);

            let caregiving_load = (family.caregiving_burden - 0.5).max(0.0) * role_multiplier;
            let burden_stress = (caregiving_load * 0.1 * microsystem_multiplier) as f32;
            let burden_fatigue = (caregiving_load * 0.08 * microsystem_multiplier) as f32;
            state
                .needs_mut()
                .add_stress_delta(burden_stress * time_scale);
            state
                .needs_mut()
                .add_fatigue_delta(burden_fatigue * time_scale);

            let hostility_load = (family.hostility - 0.5).max(0.0) * role_multiplier;
            let hostility_stress = (hostility_load * 0.05 * microsystem_multiplier) as f32;
            state
                .needs_mut()
                .add_stress_delta(hostility_stress * time_scale);

            let warmth_boost = (family.warmth - 0.5).max(0.0) * 0.05 * role_multiplier;
            let valence_delta = (warmth_boost * microsystem_multiplier) as f32;
            state.mood_mut().add_valence_delta(valence_delta * time_scale);
        }
    }
}

fn apply_mesosystem_spillover(
    state: &mut IndividualState,
    context: &EcologicalContext,
    time_scale: f32,
    microsystem_multiplier: f64,
) {
    let mesosystem_state = MesosystemState::compute(&context.microsystems);
    let shared_membership_benefit = (mesosystem_state.shared_membership_strength * 0.5).clamp(0.0, 1.0);
    let shared_membership_multiplier = 1.0 - shared_membership_benefit;

    for (from_id, to_id) in context.list_linkages() {
        let spillover_ab = context.get_spillover(&from_id, &to_id);
        let spillover_ba = context.get_spillover(&to_id, &from_id);
        let total_spillover = spillover_ab + spillover_ba;

        if total_spillover <= 0.0 {
            continue;
        }

        let stress_delta =
            (total_spillover * 0.1 * shared_membership_multiplier * microsystem_multiplier)
                as f32;
        state
            .needs_mut()
            .add_stress_delta(stress_delta * time_scale);
    }

    let role_conflict = compute_role_conflict(context);
    if role_conflict > 0.0 {
        let conflict_delta = (role_conflict * 0.1 * microsystem_multiplier) as f32;
        state
            .needs_mut()
            .add_stress_delta(conflict_delta * time_scale);
    }

    let inconsistency = (1.0 - mesosystem_state.mesosystem_consistency).max(0.0);
    if inconsistency > 0.0 {
        let consistency_delta = (inconsistency * 0.05 * microsystem_multiplier) as f32;
        state
            .needs_mut()
            .add_stress_delta(consistency_delta * time_scale);
    }
}

fn compute_role_conflict(context: &EcologicalContext) -> f64 {
    let work_contexts: Vec<_> = context
        .microsystems_iter()
        .filter_map(|(_, microsystem)| microsystem.work())
        .collect();
    let family_contexts: Vec<_> = context
        .microsystems_iter()
        .filter_map(|(_, microsystem)| microsystem.family())
        .collect();

    let mut conflict: f64 = 0.0;
    for work in &work_contexts {
        for family in &family_contexts {
            if work.workload_stress > 0.6 && family.caregiving_burden > 0.4 {
                conflict += 0.3;
            }
        }
    }

    conflict.clamp(0.0, 1.0)
}

fn apply_exosystem_effects(
    state: &mut IndividualState,
    context: &EcologicalContext,
    time_scale: f32,
    life_stage: LifeStage,
) {
    let exosystem = context.exosystem();
    let rule_of_law = context.macrosystem().institutional_structure.rule_of_law;
    let reliability_multiplier = 1.0 + (1.0 - rule_of_law);

    let resource_availability = exosystem.resource_availability;
    let deficit = (0.5 - resource_availability).max(0.0) * reliability_multiplier;
    if deficit > 0.0 {
        let coping_delta = (deficit * 0.1) as f32;
        state
            .person_characteristics_mut()
            .emotional_regulation_assets_mut()
            .add_delta(-coping_delta * time_scale);

        let stability_stress = (deficit * 0.1) as f32;
        let cohesion_loneliness = (deficit * 0.05) as f32;
        state
            .needs_mut()
            .add_stress_delta(stability_stress * time_scale);
        state
            .social_cognition_mut()
            .add_loneliness_delta(cohesion_loneliness * time_scale);
    }

    let institutional_support = exosystem.institutional_support;
    if institutional_support < 0.4 {
        let pressure = (0.4 - institutional_support) * 0.5 * reliability_multiplier;
        let reactance_delta = (pressure * 0.05) as f32;
        state
            .disposition_mut()
            .add_reactance_delta(reactance_delta * time_scale);
    }

    if matches!(life_stage, LifeStage::Child | LifeStage::Adolescent) {
        if let Some(capacity) = exosystem.parent_capacity() {
            if capacity < 0.5 {
                let deficit = 0.5 - capacity;
                let anxiety_delta = (deficit * 0.02) as f32;
                let loneliness_delta = (deficit * 0.03) as f32;
                state
                    .social_cognition_mut()
                    .add_perceived_reciprocal_caring_delta(-anxiety_delta * time_scale);
                state
                    .social_cognition_mut()
                    .add_loneliness_delta(loneliness_delta * time_scale);
            }
        }
    }
}

fn apply_macrosystem_effects(
    state: &mut IndividualState,
    context: &EcologicalContext,
    time_scale: f32,
    relationship_quality: f64,
) {
    let belonging_weight = context.macrosystem().belonging_need_weight();
    let loneliness_adjustment = ((1.0 - belonging_weight) * 0.05) as f32;
    state
        .social_cognition_mut()
        .add_loneliness_delta(loneliness_adjustment * time_scale);

    let cultural_stress = context.macrosystem().cultural_stress;
    let stress_adjustment = (cultural_stress * 0.05) as f32;
    state
        .needs_mut()
        .add_stress_delta(stress_adjustment * time_scale);

    let hierarchy_penalty = context
        .macrosystem()
        .constraint_set()
        .hierarchy_violation_penalty;
    if hierarchy_penalty > 0.0 {
        let violation_factor = (1.0 - relationship_quality).max(0.0);
        let enforcement_delta = (hierarchy_penalty * violation_factor * 0.05) as f32;
        state
            .needs_mut()
            .add_stress_delta(enforcement_delta * time_scale);
    }

    let uncertainty_avoidance = context
        .macrosystem()
        .cultural_orientation
        .uncertainty_avoidance;
    let mut total_predictability = 0.0;
    let mut count = 0.0;
    for (_, microsystem) in context.microsystems_iter() {
        if let Some(work) = microsystem.work() {
            total_predictability += work.predictability;
            count += 1.0;
        }
    }

    if count > 0.0 {
        let avg_predictability = total_predictability / count;
        if avg_predictability < uncertainty_avoidance {
            let stress_delta = ((uncertainty_avoidance - avg_predictability) * 0.05) as f32;
            state
                .needs_mut()
                .add_stress_delta(stress_delta * time_scale);
        }
    }
}

fn apply_chronosystem_effects(
    state: &mut IndividualState,
    context: &EcologicalContext,
    time_scale: f32,
    current_timestamp: Timestamp,
) {
    let chronosystem = context.chronosystem();

    let off_time_stress = chronosystem.total_off_time_stress();
    let off_time_delta = (off_time_stress * 0.1) as f32;
    state
        .needs_mut()
        .add_stress_delta(off_time_delta * time_scale);

    let historical = chronosystem.historical_period();
    let instability = (0.5 - historical.stability_level).max(0.0);
    let instability_delta = (instability * 0.03) as f32;
    state
        .needs_mut()
        .add_stress_delta(instability_delta * time_scale);

    let scarcity = (historical.resource_scarcity - 0.5).max(0.0);
    let scarcity_delta = (scarcity * 0.02) as f32;
    state
        .disposition_mut()
        .add_grievance_delta(scarcity_delta * time_scale);

    let cohort_effects = chronosystem.cohort_effects();
    let birth_era = cohort_effects.birth_era;
    let current_era =
        BirthEra::from_label(&historical.era_name).unwrap_or(BirthEra::Unknown);

    if birth_era != BirthEra::Unknown {
        if birth_era == BirthEra::Crisis && historical.stability_level < 0.7 {
            state.needs_mut().add_stress_delta(0.1 * time_scale);
        }

        let cohort_affinity: f64 = if current_era == birth_era { 1.0 } else { 0.8 };

        if (cohort_affinity - 1.0).abs() > f64::EPSILON {
            let cultural_stress = context.macrosystem().cultural_stress;
            let stress_delta = (cultural_stress * cohort_affinity - cultural_stress) * 0.05;
            state
                .needs_mut()
                .add_stress_delta((stress_delta as f32) * time_scale);
        }
    }

    let plasticity_boost = chronosystem.turning_point_plasticity_boost(current_timestamp);
    if plasticity_boost > 0.0 {
        let boost_delta = (plasticity_boost * 0.05) as f32;
        state
            .person_characteristics_mut()
            .curiosity_tendency_mut()
            .add_delta(boost_delta * time_scale);
        state
            .person_characteristics_mut()
            .experience_diversity_mut()
            .add_delta(boost_delta * time_scale);
    }

    let cohort_weight = chronosystem.cohort_effect_weight(current_era);
    // Apply cohort effects based on birth era (skip for Unknown or zero weight)
    if cohort_weight > 0.0 {
        let weight = cohort_weight as f32;
        let hexaco = state.hexaco_mut();
        // Use if-let chains to avoid unreachable match arm for Unknown
        if birth_era == BirthEra::Crisis {
            let value = hexaco.neuroticism();
            hexaco.set_neuroticism((value + 0.1 * weight).clamp(-1.0, 1.0));
        } else if birth_era == BirthEra::Stability {
            let value = hexaco.openness();
            hexaco.set_openness((value + 0.05 * weight).clamp(-1.0, 1.0));
        } else if birth_era == BirthEra::Scarcity {
            let value = hexaco.conscientiousness();
            hexaco.set_conscientiousness((value + 0.1 * weight).clamp(-1.0, 1.0));
        } else {
            let openness = hexaco.openness();
            let extraversion = hexaco.extraversion();
            hexaco.set_openness((openness + 0.1 * weight).clamp(-1.0, 1.0));
            hexaco.set_extraversion((extraversion + 0.05 * weight).clamp(-1.0, 1.0));
        }
        // BirthEra::Unknown: no cohort effects applied
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::{
        EcologicalContext, EducationContext, FamilyContext, FamilyRole, Microsystem,
        ParentWorkQuality, SocialContext, TurningPoint, TurningPointDomain, WorkContext,
    };
    use crate::enums::{BirthEra, LifeStage};
    use crate::types::{EventId, MicrosystemId, Timestamp};

    const TEST_AGE_DAYS: u64 = 365 * 30;

    fn test_timestamp() -> Timestamp {
        Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0) + Duration::days(TEST_AGE_DAYS)
    }

    #[test]
    fn high_family_support_buffers_stress() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;
        let family_id = MicrosystemId::new("family").unwrap();
        let mut family = FamilyContext::default();
        family.family_satisfaction = 0.9;
        family.warmth = 0.9;
        family.interaction_profile.interaction_frequency = 0.8;
        family.interaction_profile.interaction_complexity = 0.8;
        context.add_microsystem(family_id, Microsystem::new_family(family));

        let state = IndividualState::new();
        let result = apply_context_effects(
            state,
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(result.needs().stress().delta() < 0.0);
        assert!(result.social_cognition().loneliness().delta() < 0.0);
    }

    #[test]
    fn zero_duration_returns_unchanged_state() {
        let context = EcologicalContext::default();
        let state = IndividualState::new();
        let result = apply_context_effects(
            state.clone(),
            &context,
            0.5,
            Duration::zero(),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert_eq!(result, state);
    }

    #[test]
    fn aggregate_metrics_empty_context() {
        let context = EcologicalContext::default();
        let (frequency, complexity) = compute_aggregate_interaction_metrics(&context);
        assert!((frequency - 0.0).abs() < f64::EPSILON);
        assert!((complexity - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn work_stress_spills_to_home() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;
        let work_id = MicrosystemId::new("work").unwrap();
        let family_id = MicrosystemId::new("family").unwrap();

        let mut work = WorkContext::default();
        work.workload_stress = 0.9;
        work.interaction_profile.interaction_frequency = 0.8;
        work.interaction_profile.interaction_complexity = 0.8;
        context.add_microsystem(work_id.clone(), Microsystem::new_work(work));

        let mut family = FamilyContext::default();
        family.predictability = 0.3;
        family.stability = 0.3;
        family.interaction_profile.interaction_frequency = 0.8;
        family.interaction_profile.interaction_complexity = 0.8;
        context.add_microsystem(family_id.clone(), Microsystem::new_family(family));

        let state = IndividualState::new();
        let result = apply_context_effects(
            state,
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(result.needs().stress().delta() > 0.0);
    }

    #[test]
    fn mesosystem_spillover_applies_between_work_and_family() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;

        let work_id = MicrosystemId::new("work").unwrap();
        let family_id = MicrosystemId::new("family").unwrap();

        let mut work = WorkContext::default();
        work.workload_stress = 0.9;
        work.interaction_profile.interaction_frequency = 0.8;
        work.interaction_profile.interaction_complexity = 0.8;
        context.add_microsystem(work_id, Microsystem::new_work(work));

        let mut family = FamilyContext::default();
        family.predictability = 0.2;
        family.stability = 0.2;
        family.interaction_profile.interaction_frequency = 0.8;
        family.interaction_profile.interaction_complexity = 0.8;
        context.add_microsystem(family_id, Microsystem::new_family(family));

        let result = apply_context_effects(
            IndividualState::new(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(result.needs().stress().delta() > 0.0);
    }

    #[test]
    fn mesosystem_spillover_applies_between_family_and_work() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;

        let alpha_id = MicrosystemId::new("alpha").unwrap();
        let beta_id = MicrosystemId::new("beta").unwrap();

        let mut placeholder_a = WorkContext::default();
        placeholder_a.interaction_profile.interaction_frequency = 0.8;
        placeholder_a.interaction_profile.interaction_complexity = 0.8;
        context.add_microsystem(alpha_id.clone(), Microsystem::new_work(placeholder_a));

        let mut placeholder_b = WorkContext::default();
        placeholder_b.interaction_profile.interaction_frequency = 0.8;
        placeholder_b.interaction_profile.interaction_complexity = 0.8;
        context.add_microsystem(beta_id.clone(), Microsystem::new_work(placeholder_b));

        let linkages = context.list_linkages();
        assert_eq!(linkages.len(), 1);
        let (from_id, to_id) = (linkages[0].0.clone(), linkages[0].1.clone());

        let mut family = FamilyContext::default();
        family.caregiving_burden = 0.9;
        family.hostility = 0.6;
        family.interaction_profile.interaction_frequency = 0.8;
        family.interaction_profile.interaction_complexity = 0.8;

        let mut work = WorkContext::default();
        work.predictability = 0.2;
        work.stability = 0.2;
        work.interaction_profile.interaction_frequency = 0.8;
        work.interaction_profile.interaction_complexity = 0.8;

        context.add_microsystem(from_id.clone(), Microsystem::new_family(family));
        context.add_microsystem(to_id.clone(), Microsystem::new_work(work));

        let updated_linkages = context.list_linkages();
        assert_eq!(updated_linkages.len(), 1);
        assert_eq!(updated_linkages[0].0, from_id);
        assert_eq!(updated_linkages[0].1, to_id);

        let mut state = IndividualState::new();
        apply_mesosystem_spillover(&mut state, &context, 1.0, 1.0);

        assert!(state.needs().stress().delta() > 0.0);
    }

    #[test]
    fn mesosystem_spillover_applies_to_non_work_family_linkages() {
        let mut context = EcologicalContext::default();

        let social_id = MicrosystemId::new("social").unwrap();
        let education_id = MicrosystemId::new("education").unwrap();

        let mut social = SocialContext::default();
        social.hostility = 0.8;
        social.group_standing = 0.2;
        social.interaction_profile.interaction_frequency = 0.8;
        social.interaction_profile.interaction_complexity = 0.8;
        context.add_microsystem(social_id.clone(), Microsystem::new_social(social));

        let mut education = EducationContext::default();
        education.cognitive_demand = 0.7;
        education.hostility = 0.7;
        education.interaction_profile.interaction_frequency = 0.8;
        education.interaction_profile.interaction_complexity = 0.8;
        context.add_microsystem(education_id.clone(), Microsystem::new_education(education));

        assert_eq!(context.list_linkages().len(), 1);
        assert!(context.get_spillover(&social_id, &education_id) > 0.0);

        let mut state = IndividualState::new();
        apply_mesosystem_spillover(&mut state, &context, 1.0, 1.0);

        assert!(state.needs().stress().delta() > 0.0);
    }

    #[test]
    fn mesosystem_role_conflict_adds_stress() {
        let mut baseline = EcologicalContext::default();

        let mut work = WorkContext::default();
        work.workload_stress = 0.7;
        work.role_clarity = 0.5;
        work.predictability = 0.5;
        work.warmth = 0.5;
        work.hostility = 0.2;
        work.interaction_profile.interaction_frequency = 0.0;

        let mut family = FamilyContext::default();
        family.caregiving_burden = 0.3;
        family.role_clarity = 0.5;
        family.predictability = 0.5;
        family.warmth = 0.5;
        family.hostility = 0.2;
        family.interaction_profile.interaction_frequency = 0.0;

        baseline.add_microsystem(
            MicrosystemId::new("work").unwrap(),
            Microsystem::new_work(work),
        );
        baseline.add_microsystem(
            MicrosystemId::new("family").unwrap(),
            Microsystem::new_family(family),
        );

        let mut baseline_state = IndividualState::new();
        apply_mesosystem_spillover(&mut baseline_state, &baseline, 1.0, 1.0);

        let mut conflict = baseline.clone();
        let family = conflict
            .get_microsystem_mut(&MicrosystemId::new("family").unwrap())
            .and_then(Microsystem::family_mut)
            .expect("family microsystem missing");
        family.caregiving_burden = 0.6;

        let mut conflict_state = IndividualState::new();
        apply_mesosystem_spillover(&mut conflict_state, &conflict, 1.0, 1.0);

        assert!(conflict_state.needs().stress().delta() > baseline_state.needs().stress().delta());
    }

    #[test]
    fn mesosystem_consistency_buffers_stress() {
        let mut consistent = EcologicalContext::default();

        let mut work = WorkContext::default();
        work.workload_stress = 0.4;
        work.role_clarity = 0.7;
        work.predictability = 0.7;
        work.warmth = 0.6;
        work.hostility = 0.1;
        work.interaction_profile.interaction_frequency = 0.0;

        let mut family = FamilyContext::default();
        family.caregiving_burden = 0.3;
        family.role_clarity = 0.7;
        family.predictability = 0.7;
        family.warmth = 0.6;
        family.hostility = 0.1;
        family.interaction_profile.interaction_frequency = 0.0;

        let mut social = SocialContext::default();
        social.predictability = 0.7;
        social.warmth = 0.6;
        social.hostility = 0.1;
        social.interaction_profile.interaction_frequency = 0.0;

        consistent.add_microsystem(
            MicrosystemId::new("work").unwrap(),
            Microsystem::new_work(work),
        );
        consistent.add_microsystem(
            MicrosystemId::new("family").unwrap(),
            Microsystem::new_family(family),
        );
        consistent.add_microsystem(
            MicrosystemId::new("social").unwrap(),
            Microsystem::new_social(social),
        );

        let mut consistent_state = IndividualState::new();
        apply_mesosystem_spillover(&mut consistent_state, &consistent, 1.0, 1.0);

        let mut inconsistent = EcologicalContext::default();

        let mut work = WorkContext::default();
        work.workload_stress = 0.4;
        work.role_clarity = 0.2;
        work.predictability = 0.2;
        work.warmth = 0.2;
        work.hostility = 0.7;
        work.interaction_profile.interaction_frequency = 0.0;

        let mut family = FamilyContext::default();
        family.caregiving_burden = 0.3;
        family.role_clarity = 0.9;
        family.predictability = 0.9;
        family.warmth = 0.9;
        family.hostility = 0.1;
        family.interaction_profile.interaction_frequency = 0.0;

        let mut social = SocialContext::default();
        social.predictability = 0.1;
        social.warmth = 0.9;
        social.hostility = 0.1;
        social.interaction_profile.interaction_frequency = 0.0;

        inconsistent.add_microsystem(
            MicrosystemId::new("work").unwrap(),
            Microsystem::new_work(work),
        );
        inconsistent.add_microsystem(
            MicrosystemId::new("family").unwrap(),
            Microsystem::new_family(family),
        );
        inconsistent.add_microsystem(
            MicrosystemId::new("social").unwrap(),
            Microsystem::new_social(social),
        );

        let mut inconsistent_state = IndividualState::new();
        apply_mesosystem_spillover(&mut inconsistent_state, &inconsistent, 1.0, 1.0);

        assert!(inconsistent_state.needs().stress().delta() > consistent_state.needs().stress().delta());
    }

    #[test]
    fn shared_membership_buffers_spillover() {
        use crate::types::EntityId;

        let mut no_overlap = EcologicalContext::default();
        let mut overlap = EcologicalContext::default();

        let mut work = WorkContext::default();
        work.workload_stress = 0.8;
        work.role_clarity = 0.6;
        work.predictability = 0.6;
        work.warmth = 0.6;
        work.hostility = 0.1;
        work.peer_ids = vec![EntityId::new("coworker").unwrap()];
        work.interaction_profile.interaction_frequency = 0.8;

        let mut family = FamilyContext::default();
        family.caregiving_burden = 0.3;
        family.role_clarity = 0.6;
        family.predictability = 0.6;
        family.warmth = 0.6;
        family.hostility = 0.1;
        family.family_unit = vec![EntityId::new("relative").unwrap()];
        family.interaction_profile.interaction_frequency = 0.8;

        no_overlap.add_microsystem(
            MicrosystemId::new("work").unwrap(),
            Microsystem::new_work(work.clone()),
        );
        no_overlap.add_microsystem(
            MicrosystemId::new("family").unwrap(),
            Microsystem::new_family(family.clone()),
        );

        let shared_id = EntityId::new("shared_member").unwrap();
        let mut overlap_work = work;
        overlap_work.peer_ids = vec![shared_id.clone()];
        let mut overlap_family = family;
        overlap_family.family_unit = vec![shared_id];

        overlap.add_microsystem(
            MicrosystemId::new("work").unwrap(),
            Microsystem::new_work(overlap_work),
        );
        overlap.add_microsystem(
            MicrosystemId::new("family").unwrap(),
            Microsystem::new_family(overlap_family),
        );

        let mut no_overlap_state = IndividualState::new();
        apply_mesosystem_spillover(&mut no_overlap_state, &no_overlap, 1.0, 1.0);

        let mut overlap_state = IndividualState::new();
        apply_mesosystem_spillover(&mut overlap_state, &overlap, 1.0, 1.0);

        assert!(overlap_state.needs().stress().delta() < no_overlap_state.needs().stress().delta());
    }

    #[test]
    fn low_resources_reduce_coping() {
        let mut context = EcologicalContext::default();
        context.exosystem_mut().resource_availability = 0.2;

        let state = IndividualState::new();
        let result = apply_context_effects(
            state,
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(
            result
                .person_characteristics()
                .emotional_regulation_assets()
                .delta()
                < 0.0
        );
    }

    #[test]
    fn low_institutional_support_increases_reactance() {
        let mut context = EcologicalContext::default();
        context.exosystem_mut().institutional_support = 0.2;
        context.exosystem_mut().resource_availability = 0.6;

        let state = IndividualState::new();
        let result = apply_context_effects(
            state,
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(result.disposition().reactance().delta() > 0.0);
    }

    #[test]
    fn parent_capacity_affects_child_attachment() {
        let mut context = EcologicalContext::default();
        context.exosystem_mut().resource_availability = 0.6;
        context.exosystem_mut().institutional_support = 0.6;
        context.exosystem_mut().parent_work_environment = Some(ParentWorkQuality {
            stress_level: 1.0,
            schedule_flexibility: 0.0,
            income_stability: 0.5,
        });

        let child_state = apply_context_effects(
            IndividualState::new(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Child,
            test_timestamp(),
        );
        assert!(child_state.social_cognition().loneliness().delta() > 0.0);
        assert!(
            child_state
                .social_cognition()
                .perceived_reciprocal_caring()
                .delta()
                < 0.0
        );

        let adult_state = apply_context_effects(
            IndividualState::new(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );
        assert!(adult_state.social_cognition().loneliness().delta().abs() < f32::EPSILON);
        assert!(
            adult_state
                .social_cognition()
                .perceived_reciprocal_caring()
                .delta()
                .abs()
                < f32::EPSILON
        );
    }

    #[test]
    fn collectivist_culture_increases_belonging() {
        let mut context = EcologicalContext::default();
        context
            .macrosystem_mut()
            .cultural_orientation
            .individualism_collectivism = -0.6;

        let state = IndividualState::new();
        let result = apply_context_effects(
            state,
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(result.social_cognition().loneliness().delta() < 0.0);
    }

    #[test]
    fn high_power_distance_penalizes_low_quality_relationships() {
        let mut high_pd = EcologicalContext::default();
        high_pd.macrosystem_mut().cultural_stress = 0.0;
        high_pd
            .macrosystem_mut()
            .cultural_orientation
            .power_distance = 0.9;

        let mut low_pd = high_pd.clone();
        low_pd
            .macrosystem_mut()
            .cultural_orientation
            .power_distance = 0.2;

        let high_state = apply_context_effects(
            IndividualState::new(),
            &high_pd,
            0.3,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );
        let low_state = apply_context_effects(
            IndividualState::new(),
            &low_pd,
            0.3,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(high_state.needs().stress().delta() > low_state.needs().stress().delta());
    }

    #[test]
    fn rule_of_law_scales_exosystem_deficit() {
        let mut high_rule = EcologicalContext::default();
        high_rule.exosystem_mut().resource_availability = 0.2;
        high_rule.macrosystem_mut().cultural_stress = 0.0;
        high_rule
            .macrosystem_mut()
            .institutional_structure
            .rule_of_law = 1.0;

        let mut low_rule = high_rule.clone();
        low_rule
            .macrosystem_mut()
            .institutional_structure
            .rule_of_law = 0.2;

        let high_state = apply_context_effects(
            IndividualState::new(),
            &high_rule,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );
        let low_state = apply_context_effects(
            IndividualState::new(),
            &low_rule,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(low_state.needs().stress().delta() > high_state.needs().stress().delta());
    }

    #[test]
    fn uncertainty_avoidance_increases_stress_with_low_predictability() {
        let mut high_ua = EcologicalContext::default();
        high_ua.macrosystem_mut().cultural_stress = 0.0;
        high_ua
            .macrosystem_mut()
            .cultural_orientation
            .uncertainty_avoidance = 0.8;

        let work_id = MicrosystemId::new("work").unwrap();
        let mut work = WorkContext::default();
        work.predictability = 0.2;
        work.interaction_profile.interaction_frequency = 0.0;
        work.interaction_profile.interaction_complexity = 0.0;
        high_ua.add_microsystem(work_id, Microsystem::new_work(work));

        let mut low_ua = high_ua.clone();
        low_ua
            .macrosystem_mut()
            .cultural_orientation
            .uncertainty_avoidance = 0.1;

        let high_state = apply_context_effects(
            IndividualState::new(),
            &high_ua,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );
        let low_state = apply_context_effects(
            IndividualState::new(),
            &low_ua,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(high_state.needs().stress().delta() > low_state.needs().stress().delta());
    }

    #[test]
    fn economic_crisis_period_raises_baseline_stress() {
        let mut context = EcologicalContext::default();
        context
            .chronosystem_mut()
            .historical_period_mut()
            .resource_scarcity = 0.9;
        context
            .chronosystem_mut()
            .historical_period_mut()
            .stability_level = 0.4;
        context.macrosystem_mut().cultural_stress = 0.0;

        let state = IndividualState::new();
        let result = apply_context_effects(
            state,
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(result.needs().stress().delta() > 0.0);
    }

    #[test]
    fn historical_resource_scarcity_increases_grievance() {
        let mut context = EcologicalContext::default();
        context
            .chronosystem_mut()
            .historical_period_mut()
            .resource_scarcity = 0.8;
        context.macrosystem_mut().cultural_stress = 0.0;

        let result = apply_context_effects(
            IndividualState::new(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(result.disposition().grievance().delta() > 0.0);
    }

    #[test]
    fn turning_point_boost_increases_curiosity() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;
        let tp = TurningPoint {
            event_id: EventId::new("tp_001").unwrap(),
            timestamp: test_timestamp() - Duration::days(200),
            domain: TurningPointDomain::Identity,
            magnitude: 1.0,
        };
        context.chronosystem_mut().add_turning_point(tp);

        let result = apply_context_effects(
            IndividualState::new(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(result
            .person_characteristics()
            .curiosity_tendency()
            .delta()
            > 0.0);
    }

    #[test]
    fn crisis_cohort_increases_neuroticism() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;
        context
            .chronosystem_mut()
            .historical_period_mut()
            .era_name = "Crisis".to_string();
        context
            .chronosystem_mut()
            .historical_period_mut()
            .stability_level = 0.6;
        context.chronosystem_mut().cohort_effects_mut().birth_era = BirthEra::Crisis;

        let result = apply_context_effects(
            IndividualState::new(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(result.hexaco().neuroticism() > 0.0);
        assert!(result.needs().stress().delta() > 0.0);
    }

    #[test]
    fn cohort_affinity_scales_cultural_stress() {
        let mut matching = EcologicalContext::default();
        matching.macrosystem_mut().cultural_stress = 0.6;
        matching
            .chronosystem_mut()
            .historical_period_mut()
            .era_name = "Stability".to_string();
        matching
            .chronosystem_mut()
            .historical_period_mut()
            .stability_level = 0.7;
        matching
            .chronosystem_mut()
            .cohort_effects_mut()
            .birth_era = BirthEra::Stability;

        let mut mismatch = matching.clone();
        mismatch
            .chronosystem_mut()
            .cohort_effects_mut()
            .birth_era = BirthEra::Crisis;

        let aligned = apply_context_effects(
            IndividualState::new(),
            &matching,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );
        let mismatched = apply_context_effects(
            IndividualState::new(),
            &mismatch,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(mismatched.needs().stress().delta() < aligned.needs().stress().delta());
    }

    #[test]
    fn duration_scales_cumulative_effects() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.8;

        let state = IndividualState::new();
        let short =
            apply_context_effects(state.clone(), &context, 0.6, Duration::days(30), LifeStage::Adult, test_timestamp());
        let long =
            apply_context_effects(state, &context, 0.6, Duration::days(60), LifeStage::Adult, test_timestamp());

        assert!(long.needs().stress().delta() > short.needs().stress().delta());
    }

    #[test]
    fn proximal_process_gate_scales_microsystem_effects() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;
        let work_id = MicrosystemId::new("work").unwrap();
        let mut work = WorkContext::default();
        work.workload_stress = 0.9;
        work.interaction_profile.interaction_frequency = 0.2;
        work.interaction_profile.interaction_complexity = 0.2;
        context.add_microsystem(work_id, Microsystem::new_work(work));

        let state = IndividualState::new();
        let gated = apply_context_effects(
            state,
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        let mut open_context = EcologicalContext::default();
        open_context.macrosystem_mut().cultural_stress = 0.0;
        let work_id = MicrosystemId::new("work_open").unwrap();
        let mut work = WorkContext::default();
        work.workload_stress = 0.9;
        work.interaction_profile.interaction_frequency = 0.8;
        work.interaction_profile.interaction_complexity = 0.8;
        open_context.add_microsystem(work_id, Microsystem::new_work(work));

        let state = IndividualState::new();
        let open =
            apply_context_effects(state, &open_context, 0.6, Duration::days(30), LifeStage::Adult, test_timestamp());

        assert!(gated.needs().stress().delta() < open.needs().stress().delta());
    }

    #[test]
    fn high_frequency_low_complexity_reduces_developmental_impact() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;
        let work_id = MicrosystemId::new("work").unwrap();
        let mut work = WorkContext::default();
        work.workload_stress = 0.9;
        work.interaction_profile.interaction_frequency = 0.8;
        work.interaction_profile.interaction_complexity = 0.1;
        context.add_microsystem(work_id, Microsystem::new_work(work));

        let state = IndividualState::new();
        let result = apply_context_effects(
            state,
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(result.needs().stress().delta().abs() < 0.01);
    }

    #[test]
    fn effects_dont_apply_below_threshold() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;
        let work_id = MicrosystemId::new("work").unwrap();
        let mut work = WorkContext::default();
        work.workload_stress = 0.9;
        work.interaction_profile.interaction_frequency = 0.1;
        work.interaction_profile.interaction_complexity = 0.1;
        context.add_microsystem(work_id, Microsystem::new_work(work));

        let state = IndividualState::new();
        let result = apply_context_effects(
            state,
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(result.needs().stress().delta().abs() < 0.01);
    }

    #[test]
    fn parent_experiences_higher_family_stress_impact() {
        let mut context = EcologicalContext::default();
        let family_id = MicrosystemId::new("family").unwrap();
        let mut family = FamilyContext::default();
        family.caregiving_burden = 0.8;
        family.interaction_profile.interaction_frequency = 0.8;
        family.interaction_profile.interaction_complexity = 0.8;
        family.family_role = FamilyRole::Parent;
        context.add_microsystem(family_id, Microsystem::new_family(family));

        let parent_state = apply_context_effects(
            IndividualState::new(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        let mut context = EcologicalContext::default();
        let family_id = MicrosystemId::new("family_extended").unwrap();
        let mut family = FamilyContext::default();
        family.caregiving_burden = 0.8;
        family.interaction_profile.interaction_frequency = 0.8;
        family.interaction_profile.interaction_complexity = 0.8;
        family.family_role = FamilyRole::Extended;
        context.add_microsystem(family_id, Microsystem::new_family(family));

        let extended_state = apply_context_effects(
            IndividualState::new(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(
            parent_state.needs().stress().delta() > extended_state.needs().stress().delta()
        );
    }

    #[test]
    fn child_absorbs_parental_stress_at_higher_rate() {
        let mut context = EcologicalContext::default();
        let family_id = MicrosystemId::new("family").unwrap();
        let mut family = FamilyContext::default();
        family.caregiving_burden = 0.8;
        family.interaction_profile.interaction_frequency = 0.8;
        family.interaction_profile.interaction_complexity = 0.8;
        family.family_role = FamilyRole::Child;
        context.add_microsystem(family_id, Microsystem::new_family(family));

        let child_state = apply_context_effects(
            IndividualState::new(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        let mut context = EcologicalContext::default();
        let family_id = MicrosystemId::new("family_extended").unwrap();
        let mut family = FamilyContext::default();
        family.caregiving_burden = 0.8;
        family.interaction_profile.interaction_frequency = 0.8;
        family.interaction_profile.interaction_complexity = 0.8;
        family.family_role = FamilyRole::Extended;
        context.add_microsystem(family_id, Microsystem::new_family(family));

        let extended_state = apply_context_effects(
            IndividualState::new(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(child_state.needs().stress().delta() > extended_state.needs().stress().delta());
    }

    #[test]
    fn role_multipliers_affect_all_family_dimensions() {
        let mut context = EcologicalContext::default();
        let family_id = MicrosystemId::new("family").unwrap();
        let mut family = FamilyContext::default();
        family.family_satisfaction = 0.9;
        family.warmth = 0.9;
        family.caregiving_burden = 0.8;
        family.hostility = 0.6;
        family.interaction_profile.interaction_frequency = 0.8;
        family.interaction_profile.interaction_complexity = 0.8;
        family.family_role = FamilyRole::Parent;
        context.add_microsystem(family_id, Microsystem::new_family(family));

        let parent_state = apply_context_effects(
            IndividualState::new(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        let mut context = EcologicalContext::default();
        let family_id = MicrosystemId::new("family_extended").unwrap();
        let mut family = FamilyContext::default();
        family.family_satisfaction = 0.9;
        family.warmth = 0.9;
        family.caregiving_burden = 0.8;
        family.hostility = 0.6;
        family.interaction_profile.interaction_frequency = 0.8;
        family.interaction_profile.interaction_complexity = 0.8;
        family.family_role = FamilyRole::Extended;
        context.add_microsystem(family_id, Microsystem::new_family(family));

        let extended_state = apply_context_effects(
            IndividualState::new(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(
            parent_state.needs().stress().delta().abs()
                > extended_state.needs().stress().delta().abs()
        );
        assert!(
            parent_state.social_cognition().loneliness().delta().abs()
                > extended_state.social_cognition().loneliness().delta().abs()
        );
        assert!(
            parent_state.mood().valence_delta().abs()
                > extended_state.mood().valence_delta().abs()
        );
    }

    #[test]
    fn negative_duration_returns_unchanged_state() {
        let context = EcologicalContext::default();
        let state = IndividualState::new();
        let result = apply_context_effects(
            state.clone(),
            &context,
            0.5,
            Duration::days(0) - Duration::days(1),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert_eq!(result, state);
    }

    #[test]
    fn microsystem_multiplier_zero_skips_microsystem_effects() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;
        let work_id = MicrosystemId::new("work").unwrap();
        let mut work = WorkContext::default();
        work.workload_stress = 0.9;
        work.interaction_profile.interaction_frequency = 0.0;
        work.interaction_profile.interaction_complexity = 0.0;
        context.add_microsystem(work_id, Microsystem::new_work(work));

        let state = IndividualState::new();
        let result = apply_context_effects(
            state.clone(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(result.needs().stress().delta().abs() < 0.01);
    }

    #[test]
    fn microsystem_with_no_work_context_no_additional_work_stress() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;
        let social_id = MicrosystemId::new("social").unwrap();
        let mut social = SocialContext::default();
        social.hostility = 0.6;
        social.interaction_profile.interaction_frequency = 0.8;
        social.interaction_profile.interaction_complexity = 0.8;
        context.add_microsystem(social_id, Microsystem::new_social(social));

        let state = IndividualState::new();
        let result = apply_context_effects(
            state.clone(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        let no_work_context = result.clone();

        let mut context_with_work = EcologicalContext::default();
        context_with_work.macrosystem_mut().cultural_stress = 0.0;
        let social_id = MicrosystemId::new("social").unwrap();
        let mut social = SocialContext::default();
        social.hostility = 0.6;
        social.interaction_profile.interaction_frequency = 0.8;
        social.interaction_profile.interaction_complexity = 0.8;
        context_with_work.add_microsystem(social_id, Microsystem::new_social(social));

        let work_id = MicrosystemId::new("work").unwrap();
        let mut work = WorkContext::default();
        work.workload_stress = 0.8;
        work.interaction_profile.interaction_frequency = 0.8;
        work.interaction_profile.interaction_complexity = 0.8;
        context_with_work.add_microsystem(work_id, Microsystem::new_work(work));

        let with_work = apply_context_effects(
            IndividualState::new(),
            &context_with_work,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(with_work.needs().stress().delta() > no_work_context.needs().stress().delta());
    }

    #[test]
    fn microsystem_with_no_family_context() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;
        let work_id = MicrosystemId::new("work").unwrap();
        let mut work = WorkContext::default();
        work.workload_stress = 0.5;
        work.interaction_profile.interaction_frequency = 0.8;
        work.interaction_profile.interaction_complexity = 0.8;
        context.add_microsystem(work_id, Microsystem::new_work(work));

        let state = IndividualState::new();
        let result = apply_context_effects(
            state,
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(result.needs().stress().delta() >= 0.0);
    }

    #[test]
    fn mesosystem_spillover_zero_skips_direct_spillover() {
        let mut context = EcologicalContext::default();
        let work_id = MicrosystemId::new("work").unwrap();
        let family_id = MicrosystemId::new("family").unwrap();

        let mut work = WorkContext::default();
        work.workload_stress = 0.0;
        work.interaction_profile.interaction_frequency = 0.8;
        work.interaction_profile.interaction_complexity = 0.8;
        context.add_microsystem(work_id, Microsystem::new_work(work));

        let mut family = FamilyContext::default();
        family.caregiving_burden = 0.0;
        family.interaction_profile.interaction_frequency = 0.8;
        family.interaction_profile.interaction_complexity = 0.8;
        context.add_microsystem(family_id, Microsystem::new_family(family));

        let mut state = IndividualState::new();
        apply_mesosystem_spillover(&mut state, &context, 1.0, 1.0);

        assert!(state.needs().stress().delta() <= 0.01);
    }

    #[test]
    fn role_conflict_zero_skips_effect() {
        let mut context = EcologicalContext::default();
        let work_id = MicrosystemId::new("work").unwrap();
        let family_id = MicrosystemId::new("family").unwrap();

        let mut work = WorkContext::default();
        work.workload_stress = 0.5;
        work.interaction_profile.interaction_frequency = 0.0;
        context.add_microsystem(work_id, Microsystem::new_work(work));

        let mut family = FamilyContext::default();
        family.caregiving_burden = 0.3;
        family.interaction_profile.interaction_frequency = 0.0;
        context.add_microsystem(family_id, Microsystem::new_family(family));

        let mut state = IndividualState::new();
        apply_mesosystem_spillover(&mut state, &context, 1.0, 1.0);

        let initial_stress = state.needs().stress().delta();
        assert!(initial_stress.abs() < 0.01);
    }

    #[test]
    fn mesosystem_consistency_perfect_skips_effect() {
        let mut consistent = EcologicalContext::default();

        let mut work = WorkContext::default();
        work.role_clarity = 1.0;
        work.predictability = 1.0;
        work.warmth = 1.0;
        work.hostility = 0.0;
        work.interaction_profile.interaction_frequency = 0.0;
        consistent.add_microsystem(
            MicrosystemId::new("work").unwrap(),
            Microsystem::new_work(work),
        );

        let mut family = FamilyContext::default();
        family.role_clarity = 1.0;
        family.predictability = 1.0;
        family.warmth = 1.0;
        family.hostility = 0.0;
        family.interaction_profile.interaction_frequency = 0.0;
        consistent.add_microsystem(
            MicrosystemId::new("family").unwrap(),
            Microsystem::new_family(family),
        );

        let mut state = IndividualState::new();
        apply_mesosystem_spillover(&mut state, &consistent, 1.0, 1.0);

        assert_eq!(state.needs().stress().delta(), 0.0);
    }

    #[test]
    fn exosystem_deficit_zero_skips_coping_reduction() {
        let mut context = EcologicalContext::default();
        context.exosystem_mut().resource_availability = 0.5;
        context.macrosystem_mut().cultural_stress = 0.0;

        let state = IndividualState::new();
        let result = apply_context_effects(
            state.clone(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert_eq!(
            result
                .person_characteristics()
                .emotional_regulation_assets()
                .delta(),
            0.0
        );
    }

    #[test]
    fn institutional_support_at_threshold() {
        let mut context = EcologicalContext::default();
        context.exosystem_mut().institutional_support = 0.4;
        context.exosystem_mut().resource_availability = 0.6;
        context.macrosystem_mut().cultural_stress = 0.0;

        let state = IndividualState::new();
        let result = apply_context_effects(
            state,
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert_eq!(result.disposition().reactance().delta(), 0.0);
    }

    #[test]
    fn adolescent_with_parent_work_stress() {
        let mut context = EcologicalContext::default();
        context.exosystem_mut().resource_availability = 0.6;
        context.exosystem_mut().institutional_support = 0.6;
        context.exosystem_mut().parent_work_environment = Some(ParentWorkQuality {
            stress_level: 1.0,
            schedule_flexibility: 0.0,
            income_stability: 0.5,
        });

        let result = apply_context_effects(
            IndividualState::new(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adolescent,
            test_timestamp(),
        );

        assert!(result.social_cognition().loneliness().delta() > 0.0);
    }

    #[test]
    fn child_with_high_parent_capacity() {
        let mut context = EcologicalContext::default();
        context.exosystem_mut().resource_availability = 0.6;
        context.exosystem_mut().institutional_support = 0.6;
        context.exosystem_mut().parent_work_environment = Some(ParentWorkQuality {
            stress_level: 0.1,
            schedule_flexibility: 0.8,
            income_stability: 0.9,
        });

        let result = apply_context_effects(
            IndividualState::new(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Child,
            test_timestamp(),
        );

        assert!(result.social_cognition().loneliness().delta().abs() < f32::EPSILON);
    }

    #[test]
    fn hierarchy_penalty_zero_skips_effect() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;
        context.macrosystem_mut().cultural_orientation.power_distance = 0.0;

        let state = IndividualState::new();
        let result = apply_context_effects(
            state,
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert_eq!(result.needs().stress().delta(), 0.0);
    }

    #[test]
    fn uncertainty_avoidance_no_work_contexts() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;
        context.macrosystem_mut().cultural_orientation.uncertainty_avoidance = 0.8;

        let state = IndividualState::new();
        let result = apply_context_effects(
            state.clone(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert_eq!(result.needs().stress().delta(), state.needs().stress().delta());
    }

    #[test]
    fn plasticity_boost_zero_skips_effect() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;

        let state = IndividualState::new();
        let result = apply_context_effects(
            state.clone(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert_eq!(
            result
                .person_characteristics()
                .curiosity_tendency()
                .delta(),
            state.person_characteristics().curiosity_tendency().delta()
        );
    }

    #[test]
    fn cohort_weight_zero_skips_hexaco_modification() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;
        context
            .chronosystem_mut()
            .historical_period_mut()
            .era_name = "Unknown".to_string();

        let state = IndividualState::new();
        let result = apply_context_effects(
            state.clone(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert_eq!(result.hexaco().neuroticism(), state.hexaco().neuroticism());
        assert_eq!(result.hexaco().openness(), state.hexaco().openness());
    }

    #[test]
    fn birth_era_unknown_skips_cohort_effects() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;
        context
            .chronosystem_mut()
            .cohort_effects_mut()
            .birth_era = BirthEra::Unknown;
        // Ensure cohort_weight > 0.0 to enter the match statement
        context
            .chronosystem_mut()
            .historical_period_mut()
            .era_name = "Stability".to_string();

        let state = IndividualState::new();
        let result = apply_context_effects(
            state.clone(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        // Unknown era should not modify HEXACO traits
        assert_eq!(result.hexaco().neuroticism(), state.hexaco().neuroticism());
        assert_eq!(result.hexaco().openness(), state.hexaco().openness());
    }

    #[test]
    fn stability_cohort_increases_openness() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;
        context
            .chronosystem_mut()
            .historical_period_mut()
            .era_name = "Stability".to_string();
        context
            .chronosystem_mut()
            .cohort_effects_mut()
            .birth_era = BirthEra::Stability;

        let state = IndividualState::new();
        let initial_openness = state.hexaco().openness();
        let result = apply_context_effects(
            state,
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(result.hexaco().openness() > initial_openness);
    }

    #[test]
    fn scarcity_cohort_increases_conscientiousness() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;
        context
            .chronosystem_mut()
            .historical_period_mut()
            .era_name = "Scarcity".to_string();
        context
            .chronosystem_mut()
            .cohort_effects_mut()
            .birth_era = BirthEra::Scarcity;

        let state = IndividualState::new();
        let initial_cons = state.hexaco().conscientiousness();
        let result = apply_context_effects(
            state,
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(result.hexaco().conscientiousness() > initial_cons);
    }

    #[test]
    fn expansion_cohort_increases_openness_and_extraversion() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;
        context
            .chronosystem_mut()
            .historical_period_mut()
            .era_name = "Expansion".to_string();
        context
            .chronosystem_mut()
            .cohort_effects_mut()
            .birth_era = BirthEra::Expansion;

        let state = IndividualState::new();
        let initial_openness = state.hexaco().openness();
        let initial_extraversion = state.hexaco().extraversion();
        let result = apply_context_effects(
            state,
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(result.hexaco().openness() > initial_openness);
        assert!(result.hexaco().extraversion() > initial_extraversion);
    }

    #[test]
    fn cohort_affinity_matching_era() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.6;
        context
            .chronosystem_mut()
            .historical_period_mut()
            .era_name = "Crisis".to_string();
        context
            .chronosystem_mut()
            .cohort_effects_mut()
            .birth_era = BirthEra::Crisis;

        let result = apply_context_effects(
            IndividualState::new(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(result.needs().stress().delta() > 0.0);
    }

    #[test]
    fn birth_era_crisis_during_unstable_period_increases_stress() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;
        context
            .chronosystem_mut()
            .historical_period_mut()
            .era_name = "Crisis".to_string();
        context
            .chronosystem_mut()
            .historical_period_mut()
            .stability_level = 0.6;
        context
            .chronosystem_mut()
            .cohort_effects_mut()
            .birth_era = BirthEra::Crisis;

        let state = IndividualState::new();
        let result = apply_context_effects(
            state,
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(result.needs().stress().delta() > 0.0);
    }

    #[test]
    fn family_support_below_threshold() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;
        let family_id = MicrosystemId::new("family").unwrap();
        let mut family = FamilyContext::default();
        family.family_satisfaction = 0.5;
        family.warmth = 0.5;
        family.interaction_profile.interaction_frequency = 0.8;
        family.interaction_profile.interaction_complexity = 0.8;
        context.add_microsystem(family_id, Microsystem::new_family(family));

        let state = IndividualState::new();
        let result = apply_context_effects(
            state.clone(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(result.needs().stress().delta() <= state.needs().stress().delta() + 0.05);
    }

    #[test]
    fn low_workload_vs_high_workload() {
        let mut low_context = EcologicalContext::default();
        low_context.macrosystem_mut().cultural_stress = 0.0;
        let work_id = MicrosystemId::new("work").unwrap();
        let mut work = WorkContext::default();
        work.workload_stress = 0.5;
        work.interaction_profile.interaction_frequency = 0.8;
        work.interaction_profile.interaction_complexity = 0.8;
        low_context.add_microsystem(work_id, Microsystem::new_work(work));

        let low_result = apply_context_effects(
            IndividualState::new(),
            &low_context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        let mut high_context = EcologicalContext::default();
        high_context.macrosystem_mut().cultural_stress = 0.0;
        let work_id = MicrosystemId::new("work").unwrap();
        let mut work = WorkContext::default();
        work.workload_stress = 0.9;
        work.interaction_profile.interaction_frequency = 0.8;
        work.interaction_profile.interaction_complexity = 0.8;
        high_context.add_microsystem(work_id, Microsystem::new_work(work));

        let high_result = apply_context_effects(
            IndividualState::new(),
            &high_context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(high_result.needs().stress().delta() > low_result.needs().stress().delta());
    }

    #[test]
    fn caregiving_below_threshold() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;
        let family_id = MicrosystemId::new("family").unwrap();
        let mut family = FamilyContext::default();
        family.caregiving_burden = 0.3;
        family.interaction_profile.interaction_frequency = 0.8;
        family.interaction_profile.interaction_complexity = 0.8;
        context.add_microsystem(family_id, Microsystem::new_family(family));

        let state = IndividualState::new();
        let result = apply_context_effects(
            state.clone(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(result.needs().stress().delta() <= state.needs().stress().delta() + 0.05);
    }

    #[test]
    fn hostility_below_threshold() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;
        let family_id = MicrosystemId::new("family").unwrap();
        let mut family = FamilyContext::default();
        family.hostility = 0.4;
        family.interaction_profile.interaction_frequency = 0.8;
        family.interaction_profile.interaction_complexity = 0.8;
        context.add_microsystem(family_id, Microsystem::new_family(family));

        let state = IndividualState::new();
        let result = apply_context_effects(
            state.clone(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(result.needs().stress().delta() <= state.needs().stress().delta() + 0.05);
    }

    #[test]
    fn warmth_below_threshold() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;
        let family_id = MicrosystemId::new("family").unwrap();
        let mut family = FamilyContext::default();
        family.warmth = 0.4;
        family.interaction_profile.interaction_frequency = 0.8;
        family.interaction_profile.interaction_complexity = 0.8;
        context.add_microsystem(family_id, Microsystem::new_family(family));

        let state = IndividualState::new();
        let result = apply_context_effects(
            state.clone(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(result.mood().valence_delta() <= state.mood().valence_delta() + 0.01);
    }

    #[test]
    fn high_hierarchy_penalty_with_high_relationship_quality() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;
        context.macrosystem_mut().cultural_orientation.power_distance = 0.7;

        let state = IndividualState::new();
        let result = apply_context_effects(
            state,
            &context,
            0.9,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(result.needs().stress().delta() < 0.01);
    }

    #[test]
    fn avg_predictability_above_uncertainty_avoidance() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;
        context.macrosystem_mut().cultural_orientation.uncertainty_avoidance = 0.3;

        let work_id = MicrosystemId::new("work").unwrap();
        let mut work = WorkContext::default();
        work.predictability = 0.8;
        work.interaction_profile.interaction_frequency = 0.0;
        work.interaction_profile.interaction_complexity = 0.0;
        context.add_microsystem(work_id, Microsystem::new_work(work));

        let state = IndividualState::new();
        let result = apply_context_effects(
            state.clone(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert_eq!(result.needs().stress().delta(), state.needs().stress().delta());
    }

    #[test]
    fn off_time_stress_effects_applied() {
        let context = EcologicalContext::default();

        let state = IndividualState::new();
        let result = apply_context_effects(
            state.clone(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert_ne!(result.needs().stress().delta(), state.needs().stress().delta());
    }

    #[test]
    fn stability_at_threshold() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;
        context
            .chronosystem_mut()
            .historical_period_mut()
            .stability_level = 0.5;

        let state = IndividualState::new();
        let result = apply_context_effects(
            state,
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert_eq!(result.needs().stress().delta(), 0.0);
    }

    #[test]
    fn resource_scarcity_below_threshold() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;
        context
            .chronosystem_mut()
            .historical_period_mut()
            .resource_scarcity = 0.4;

        let state = IndividualState::new();
        let result = apply_context_effects(
            state,
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert_eq!(result.disposition().grievance().delta(), 0.0);
    }

    #[test]
    fn birth_era_matches_current_era_epsilon_check_skips() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;
        context
            .chronosystem_mut()
            .historical_period_mut()
            .era_name = "Stability".to_string();
        context
            .chronosystem_mut()
            .cohort_effects_mut()
            .birth_era = BirthEra::Stability;

        let state = IndividualState::new();
        let result = apply_context_effects(
            state.clone(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert_eq!(
            result.needs().stress().delta(),
            state.needs().stress().delta()
        );
    }

    #[test]
    fn multiple_work_family_conflicts_clamped_at_one() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.0;

        let work1_id = MicrosystemId::new("work1").unwrap();
        let mut work1 = WorkContext::default();
        work1.workload_stress = 0.8;
        work1.interaction_profile.interaction_frequency = 0.8;
        work1.interaction_profile.interaction_complexity = 0.8;
        context.add_microsystem(work1_id, Microsystem::new_work(work1));

        let family1_id = MicrosystemId::new("family1").unwrap();
        let mut family1 = FamilyContext::default();
        family1.caregiving_burden = 0.8;
        family1.interaction_profile.interaction_frequency = 0.8;
        family1.interaction_profile.interaction_complexity = 0.8;
        context.add_microsystem(family1_id, Microsystem::new_family(family1));

        let work2_id = MicrosystemId::new("work2").unwrap();
        let mut work2 = WorkContext::default();
        work2.workload_stress = 0.9;
        work2.interaction_profile.interaction_frequency = 0.8;
        work2.interaction_profile.interaction_complexity = 0.8;
        context.add_microsystem(work2_id, Microsystem::new_work(work2));

        let family2_id = MicrosystemId::new("family2").unwrap();
        let mut family2 = FamilyContext::default();
        family2.caregiving_burden = 0.9;
        family2.interaction_profile.interaction_frequency = 0.8;
        family2.interaction_profile.interaction_complexity = 0.8;
        context.add_microsystem(family2_id, Microsystem::new_family(family2));

        let result = apply_context_effects(
            IndividualState::new(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(result.needs().stress().delta() > 0.0);
    }

    #[test]
    fn cohort_affinity_epsilon_check_when_eras_mismatch() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.6;
        context
            .chronosystem_mut()
            .historical_period_mut()
            .era_name = "Crisis".to_string();
        context
            .chronosystem_mut()
            .cohort_effects_mut()
            .birth_era = BirthEra::Stability;

        let state = IndividualState::new();
        let result = apply_context_effects(
            state.clone(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        assert!(result.needs().stress().delta() != state.needs().stress().delta());
    }

    #[test]
    fn zero_duration_returns_unmodified_state() {
        let context = EcologicalContext::default();
        let state = IndividualState::new();
        let state_clone = state.clone();

        let result = apply_context_effects(
            state,
            &context,
            0.6,
            Duration::days(0),
            LifeStage::Adult,
            test_timestamp(),
        );

        // With zero duration, time_scale is 0, so state should be returned unchanged
        assert_eq!(result.mood().valence().delta(), state_clone.mood().valence().delta());
    }

    #[test]
    fn context_with_no_work_or_family_microsystems() {
        let mut context = EcologicalContext::default();
        // Add a microsystem that's neither work nor family (e.g., social context)
        let social_id = MicrosystemId::new("friends").unwrap();
        let social = SocialContext::default();
        context.add_microsystem(social_id, Microsystem::new_social(social));

        let result = apply_context_effects(
            IndividualState::new(),
            &context,
            0.6,
            Duration::days(30),
            LifeStage::Adult,
            test_timestamp(),
        );

        // Should complete without error, even though no work/family microsystems exist
        assert!(result.mood().valence().delta() >= -1.0 && result.mood().valence().delta() <= 1.0);
    }
}
