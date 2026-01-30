//! Event interpretation and application for entity state changes.
//!
//! This module provides functions for interpreting events based on
//! entity personality, applying state changes, and computing salience.

use crate::entity::Entity;
use crate::enums::{
    Attribution, AttributionStability, Direction, DispositionPath, EventCategory, EventPayload,
    EventType, LifeDomain, MentalHealthPath, MoodPath, NeedsPath, RealizationType,
    SocialCognitionPath, StatePath, SupportType,
};
use crate::event::{compute_arousal_modulated_salience, Event};
#[cfg(test)]
use crate::memory::MemoryTag;
use crate::relationship::{get_antecedent_for_event, Relationship, TrustAntecedent};
use crate::types::{EventId, Timestamp};

/// Interpretation of an event based on entity state and personality.
///
/// The interpretation determines how strongly the event affects different
/// psychological dimensions based on the entity's personality (HEXACO),
/// current state, and the event's properties.
#[derive(Debug, Clone)]
pub struct InterpretedEvent {
    /// Original event.
    pub event: Event,
    /// ID of the original event for reference.
    pub original_event: EventId,
    /// Computed attribution for this event.
    pub attribution: Attribution,
    /// Valence modifier based on event type and personality.
    pub valence_delta: f32,
    /// Arousal modifier based on event severity and emotionality.
    pub arousal_delta: f32,
    /// Dominance modifier for control-related events.
    pub dominance_delta: f32,
    /// Loneliness impact for belonging events.
    pub loneliness_delta: f32,
    /// Perceived reciprocal caring impact.
    pub prc_delta: f32,
    /// Perceived liability impact for burden events.
    pub perceived_liability_delta: f32,
    /// Self-hate impact.
    pub self_hate_delta: f32,
    /// Acquired capability impact for trauma events.
    pub acquired_capability_delta: f32,
    /// Interpersonal hopelessness impact.
    pub interpersonal_hopelessness_delta: f32,
    /// Computed salience for memory encoding.
    pub salience: f32,
    /// Perceived severity after personality modulation.
    pub perceived_severity: f64,
    /// Memory salience for encoding.
    pub memory_salience: f64,
    /// State changes to apply, as (path, delta) pairs.
    pub state_deltas: Vec<(StatePath, f64)>,
}

impl InterpretedEvent {
    /// Creates a new interpreted event with all deltas scaled by a factor.
    ///
    /// This is used by developmental processing to apply plasticity and
    /// sensitive period multipliers to event impact.
    ///
    /// # Arguments
    ///
    /// * `factor` - The scaling factor to apply to all deltas
    ///
    /// # Returns
    ///
    /// A new `InterpretedEvent` with scaled deltas.
    #[must_use]
    pub fn scaled_by(&self, factor: f64) -> Self {
        let factor_f32 = factor as f32;

        InterpretedEvent {
            event: self.event.clone(),
            original_event: self.original_event.clone(),
            attribution: self.attribution.clone(),
            valence_delta: self.valence_delta * factor_f32,
            arousal_delta: self.arousal_delta * factor_f32,
            dominance_delta: self.dominance_delta * factor_f32,
            loneliness_delta: self.loneliness_delta * factor_f32,
            prc_delta: self.prc_delta * factor_f32,
            perceived_liability_delta: self.perceived_liability_delta * factor_f32,
            self_hate_delta: self.self_hate_delta * factor_f32,
            acquired_capability_delta: self.acquired_capability_delta * factor_f32,
            interpersonal_hopelessness_delta: self.interpersonal_hopelessness_delta * factor_f32,
            salience: self.salience, // Salience is not scaled
            perceived_severity: self.perceived_severity * factor,
            memory_salience: self.memory_salience, // Memory salience is not scaled
            state_deltas: self
                .state_deltas
                .iter()
                .map(|(path, delta)| (*path, delta * factor))
                .collect(),
        }
    }
}

/// Default impact magnitudes for event processing.
pub mod impact {
    /// Base valence impact for negative events.
    pub const NEGATIVE_VALENCE: f32 = -0.3;
    /// Base valence impact for positive events.
    pub const POSITIVE_VALENCE: f32 = 0.3;
    /// Base arousal impact for high-intensity events.
    pub const HIGH_AROUSAL: f32 = 0.4;
    /// Base dominance impact for control events.
    pub const CONTROL_DOMINANCE: f32 = 0.3;
    /// Base loneliness impact for exclusion events.
    pub const EXCLUSION_LONELINESS: f32 = 0.2;
    /// Base loneliness reduction for inclusion events.
    pub const INCLUSION_LONELINESS: f32 = -0.2;
    /// Base PRC impact for support events.
    pub const SUPPORT_PRC: f32 = 0.15;
    /// Base PRC reduction for betrayal events.
    pub const BETRAYAL_PRC: f32 = -0.2;
    /// Base perceived liability impact.
    pub const BURDEN_LIABILITY: f32 = 0.25;
    /// Base self-hate impact for stable self-attributions.
    pub const SELF_HATE: f32 = 0.1;
    /// Base AC impact for trauma events.
    pub const TRAUMA_AC: f32 = 0.15;
    /// Base interpersonal hopelessness impact.
    pub const INTERPERSONAL_HOPELESSNESS: f32 = 0.1;
}

/// Interprets an event based on entity state and personality.
///
/// This function computes how an event should modify the entity's state
/// based on their personality traits (HEXACO), current emotional state,
/// and the event's properties.
///
/// # HEXACO Integration
///
/// - Emotionality modulates arousal response
/// - Agreeableness affects social event interpretation
/// - Honesty-Humility affects attribution patterns
///
/// # Arguments
///
/// * `event` - The event to interpret
/// * `entity` - The entity interpreting the event
///
/// # Returns
///
/// An interpreted event with computed deltas for each state dimension
///
/// # Examples
///
/// ```ignore
/// use behavioral_pathways::processor::interpret_event;
/// use behavioral_pathways::event::EventBuilder;
/// use behavioral_pathways::enums::EventType;
/// use behavioral_pathways::entity::EntityBuilder;
/// use behavioral_pathways::enums::Species;
///
/// let entity = EntityBuilder::new()
///     .species(Species::Human)
///     .build()
///     .unwrap();
///
/// let event = EventBuilder::new(EventType::SocialExclusion)
///     .severity(0.7)
///     .build()
///     .unwrap();
///
/// let interpreted = interpret_event(&event, &entity);
/// assert!(interpreted.valence_delta < 0.0); // Exclusion is negative
/// assert!(interpreted.loneliness_delta > 0.0); // Increases loneliness
/// ```
#[must_use]
pub(crate) fn interpret_event(event: &Event, entity: &Entity) -> InterpretedEvent {
    // Get personality for modulation
    let hexaco = entity.individual_state().hexaco();
    let emotionality = hexaco.emotionality(); // HEXACO Emotionality
    let agreeableness = hexaco.agreeableness();
    let honesty_humility = hexaco.honesty_humility();

    // Get current arousal for salience computation
    let current_arousal = entity
        .get_effective(StatePath::Mood(MoodPath::Arousal))
        .unwrap_or(0.0) as f32;

    let severity = event.severity() as f32;
    let category = event.category();
    let event_type = event.event_type();

    // Compute base impacts based on event type and category
    let mut valence_delta = 0.0;
    let mut arousal_delta = 0.0;
    let mut dominance_delta = 0.0;
    let mut loneliness_delta = 0.0;
    let mut prc_delta = 0.0;
    let mut perceived_liability_delta = 0.0;
    let mut self_hate_delta = 0.0;
    let mut acquired_capability_delta = 0.0;
    let mut interpersonal_hopelessness_delta = 0.0;
    let mut purpose_delta = 0.0;
    let mut self_worth_delta = 0.0;

    // Apply base impacts by category
    match category {
        EventCategory::SocialBelonging => {
            // TB pathway
            match event_type {
                EventType::SocialExclusion => {
                    valence_delta = impact::NEGATIVE_VALENCE * severity;
                    loneliness_delta = impact::EXCLUSION_LONELINESS * severity;
                    prc_delta = -0.1 * severity;
                }
                EventType::SocialInclusion => {
                    valence_delta = impact::POSITIVE_VALENCE * severity;
                    loneliness_delta = impact::INCLUSION_LONELINESS * severity;
                    prc_delta = 0.1 * severity;
                }
                _ => {}
            }
        }
        EventCategory::BurdenPerception => {
            // PB pathway
            valence_delta = impact::NEGATIVE_VALENCE * severity;
            perceived_liability_delta = impact::BURDEN_LIABILITY * severity;
            self_hate_delta = impact::SELF_HATE * severity * 0.5;
        }
        EventCategory::Trauma => {
            // AC pathway - NEVER decays
            valence_delta = impact::NEGATIVE_VALENCE * severity;
            arousal_delta = impact::HIGH_AROUSAL * severity;
            acquired_capability_delta = impact::TRAUMA_AC * severity;
        }
        EventCategory::Control => match event_type {
            EventType::Humiliation => {
                valence_delta = impact::NEGATIVE_VALENCE * severity;
                dominance_delta = -impact::CONTROL_DOMINANCE * severity;
            }
            EventType::Empowerment => {
                valence_delta = impact::POSITIVE_VALENCE * severity;
                dominance_delta = impact::CONTROL_DOMINANCE * severity;
            }
            _ => {}
        },
        EventCategory::Achievement => match event_type {
            EventType::Achievement => {
                valence_delta = impact::POSITIVE_VALENCE * severity;
                dominance_delta = 0.1 * severity;
            }
            EventType::Failure => {
                valence_delta = impact::NEGATIVE_VALENCE * severity;
                dominance_delta = -0.1 * severity;
            }
            EventType::Loss => {
                // Loss events (job loss, significant loss, death, etc.)
                // Spec: spec/subsystems/event-system.md Loss entry
                valence_delta = -0.15 * severity;
                dominance_delta = -0.10 * severity;
                arousal_delta = 0.10 * severity;
            }
            _ => {}
        },
        EventCategory::Social => {
            // Conflict has specific blueprint effects
            if event_type == EventType::Conflict {
                // Spec: spec/subsystems/event-system.md Conflict entry
                valence_delta = -0.10 * severity;
                arousal_delta = 0.12 * severity;
                dominance_delta = -0.12 * severity;
                loneliness_delta = 0.08 * severity;
                perceived_liability_delta = 0.04 * severity;
                // self_worth and grievance are added later in state_deltas section
            } else if event_type == EventType::Support && matches!(event.payload(), EventPayload::Empty) {
                // Support without payload uses blueprint
                // Spec: spec/subsystems/event-system.md Support entry
                valence_delta = 0.08 * severity;
                loneliness_delta = -0.20 * severity;
                perceived_liability_delta = -0.10 * severity;
            } else {
                // General social events - process payload
                process_social_event_payload(
                    event,
                    &mut valence_delta,
                    &mut arousal_delta,
                    &mut prc_delta,
                    &mut loneliness_delta,
                );
            }
        }
        EventCategory::Contextual => {
            // Environmental events - minimal direct state impact
            arousal_delta = 0.1 * severity;
        }
    }

    // Protective factors lower TB/PB without touching AC.
    if event_type == EventType::Achievement {
        if let EventPayload::Achievement { domain, magnitude } = event.payload() {
            let productivity = severity * (*magnitude as f32);
            if matches!(domain, LifeDomain::Work | LifeDomain::Academic | LifeDomain::Financial) {
                perceived_liability_delta -= 0.12 * productivity;
                self_hate_delta -= 0.08 * productivity;
                self_worth_delta += 0.05 * productivity;
            }
            if matches!(domain, LifeDomain::Work | LifeDomain::Academic | LifeDomain::Creative) {
                purpose_delta += 0.08 * productivity;
            }
        }
    }

    if event_type == EventType::SocialInclusion {
        if let EventPayload::SocialInclusion { group_id: Some(_) } = event.payload() {
            loneliness_delta -= 0.08 * severity;
            prc_delta += 0.05 * severity;
        }
    }

    if event_type == EventType::Support {
        if let EventPayload::Support {
            support_type,
            effectiveness,
        } = event.payload()
        {
            if matches!(
                support_type,
                SupportType::Emotional | SupportType::Companionship
            ) {
                let eff = *effectiveness as f32;
                perceived_liability_delta -= 0.1 * severity * eff;
                self_hate_delta -= 0.08 * severity * eff;
                self_worth_delta += 0.05 * severity * eff;
            }
        }
    }

    if event_type == EventType::Realization {
        if let EventPayload::Realization {
            realization_type: RealizationType::ExistentialInsight,
        } = event.payload()
        {
            purpose_delta += 0.15 * severity;
            perceived_liability_delta -= 0.05 * severity;
            self_hate_delta -= 0.05 * severity;
            self_worth_delta += 0.04 * severity;
        }
    }

    // Modulate by Emotionality (higher = stronger emotional response)
    let emotionality_factor = 1.0 + (emotionality * 0.3);
    valence_delta *= emotionality_factor;
    arousal_delta *= emotionality_factor;

    // Modulate social events by Agreeableness
    if matches!(
        category,
        EventCategory::Social | EventCategory::SocialBelonging
    ) {
        let agree_factor = 1.0 + (agreeableness * 0.2);
        loneliness_delta *= agree_factor;
        prc_delta *= agree_factor;
    }

    // Compute attribution (simplified model)
    let attribution = compute_attribution(event, honesty_humility);

    // If stable self-attribution for negative event, increase self-hate and hopelessness
    if attribution.is_self_caused() && attribution.is_stable() && valence_delta < 0.0 {
        self_hate_delta += impact::SELF_HATE * severity;
        interpersonal_hopelessness_delta += impact::INTERPERSONAL_HOPELESSNESS * severity;
    }

    // Compute salience with arousal modulation
    let valence_for_salience = valence_delta;
    let base_salience = compute_base_salience(event);
    let salience = compute_arousal_modulated_salience(
        base_salience,
        current_arousal + arousal_delta,
        valence_for_salience,
        category,
        entity.species(),
    );

    // Compute perceived severity (modulated by emotionality)
    let emotionality_factor = 1.0 + (emotionality * 0.3);
    let perceived_severity = (severity * emotionality_factor) as f64;

    // Build state_deltas vector
    let mut state_deltas: Vec<(StatePath, f64)> = Vec::new();

    // Mood deltas
    if valence_delta.abs() > f32::EPSILON {
        state_deltas.push((StatePath::Mood(MoodPath::Valence), valence_delta as f64));
    }
    if arousal_delta.abs() > f32::EPSILON {
        state_deltas.push((StatePath::Mood(MoodPath::Arousal), arousal_delta as f64));
    }
    if dominance_delta.abs() > f32::EPSILON {
        state_deltas.push((StatePath::Mood(MoodPath::Dominance), dominance_delta as f64));
    }

    // Needs deltas
    if loneliness_delta.abs() > f32::EPSILON {
        state_deltas.push((
            StatePath::SocialCognition(SocialCognitionPath::Loneliness),
            loneliness_delta as f64,
        ));
    }
    if prc_delta.abs() > f32::EPSILON {
        state_deltas.push((
            StatePath::SocialCognition(SocialCognitionPath::PerceivedReciprocalCaring),
            prc_delta as f64,
        ));
    }
    if perceived_liability_delta.abs() > f32::EPSILON {
        state_deltas.push((
            StatePath::SocialCognition(SocialCognitionPath::PerceivedLiability),
            perceived_liability_delta as f64,
        ));
    }
    if self_hate_delta.abs() > f32::EPSILON {
        state_deltas.push((
            StatePath::SocialCognition(SocialCognitionPath::SelfHate),
            self_hate_delta as f64,
        ));
    }

    // Mental health deltas
    if acquired_capability_delta.abs() > f32::EPSILON {
        state_deltas.push((
            StatePath::MentalHealth(MentalHealthPath::AcquiredCapability),
            acquired_capability_delta as f64,
        ));
    }
    if interpersonal_hopelessness_delta.abs() > f32::EPSILON {
        state_deltas.push((
            StatePath::MentalHealth(MentalHealthPath::InterpersonalHopelessness),
            interpersonal_hopelessness_delta as f64,
        ));
    }
    if purpose_delta.abs() > f32::EPSILON {
        state_deltas.push((
            StatePath::Needs(NeedsPath::Purpose),
            purpose_delta as f64,
        ));
    }
    if self_worth_delta.abs() > f32::EPSILON {
        state_deltas.push((
            StatePath::MentalHealth(MentalHealthPath::SelfWorth),
            self_worth_delta as f64,
        ));
    }

    // Loss-specific state changes (self_worth and grievance)
    // Spec: spec/subsystems/event-system.md Loss entry
    if event_type == EventType::Loss {
        let self_worth_delta = -0.15 * severity * emotionality_factor;
        let grievance_delta = 0.05 * severity * emotionality_factor;

        state_deltas.push((
            StatePath::MentalHealth(MentalHealthPath::SelfWorth),
            self_worth_delta as f64,
        ));
        state_deltas.push((
            StatePath::Disposition(DispositionPath::Grievance),
            grievance_delta as f64,
        ));
    }

    // Conflict-specific state changes (self_worth and grievance)
    // Spec: spec/subsystems/event-system.md Conflict entry
    if event_type == EventType::Conflict {
        let self_worth_delta = -0.06 * severity * emotionality_factor;
        let grievance_delta = 0.04 * severity * emotionality_factor;

        state_deltas.push((
            StatePath::MentalHealth(MentalHealthPath::SelfWorth),
            self_worth_delta as f64,
        ));
        state_deltas.push((
            StatePath::Disposition(DispositionPath::Grievance),
            grievance_delta as f64,
        ));
    }

    InterpretedEvent {
        event: event.clone(),
        original_event: event.id().clone(),
        attribution,
        valence_delta,
        arousal_delta,
        dominance_delta,
        loneliness_delta,
        prc_delta,
        perceived_liability_delta,
        self_hate_delta,
        acquired_capability_delta,
        interpersonal_hopelessness_delta,
        salience,
        perceived_severity,
        memory_salience: salience as f64,
        state_deltas,
    }
}

/// Computes base salience from event properties.
fn compute_base_salience(event: &Event) -> f32 {
    let severity = event.severity() as f32;
    let category_boost = match event.category() {
        EventCategory::Trauma => 0.2,
        EventCategory::SocialBelonging | EventCategory::BurdenPerception => 0.1,
        _ => 0.0,
    };

    (0.3 + severity * 0.5 + category_boost).clamp(0.0, 1.0)
}

/// Computes attribution based on event and personality.
fn compute_attribution(event: &Event, honesty_humility: f32) -> Attribution {
    // Simple model: higher honesty-humility = more internal attribution
    // Event source affects attribution
    if let Some(source) = event.source() {
        // There's a clear external cause
        let stability = if event.severity() > 0.7 {
            AttributionStability::Stable
        } else {
            AttributionStability::Unstable
        };
        return Attribution::Other(source.clone(), stability);
    }

    // No clear source - attribution based on personality
    let stability = if event.severity() > 0.7 {
        AttributionStability::Stable
    } else {
        AttributionStability::Unstable
    };

    // Higher honesty-humility = more likely to self-attribute
    if honesty_humility > 0.3 {
        Attribution::SelfCaused(stability)
    } else if honesty_humility < -0.3 {
        Attribution::Situational(stability)
    } else {
        Attribution::Unknown
    }
}

/// Processes payload for social events.
fn process_social_event_payload(
    event: &Event,
    valence_delta: &mut f32,
    arousal_delta: &mut f32,
    prc_delta: &mut f32,
    loneliness_delta: &mut f32,
) {
    let severity = event.severity() as f32;

    match event.payload() {
        EventPayload::Support { effectiveness, .. } => {
            let eff = *effectiveness as f32;
            *valence_delta = impact::POSITIVE_VALENCE * severity * eff;
            *prc_delta = impact::SUPPORT_PRC * severity * eff;
            *loneliness_delta = -0.1 * severity * eff;
        }
        EventPayload::Betrayal {
            confidence_violated,
        } => {
            let conf = *confidence_violated as f32;
            *valence_delta = impact::NEGATIVE_VALENCE * severity * conf;
            *prc_delta = impact::BETRAYAL_PRC * severity * conf;
            *arousal_delta = 0.2 * severity * conf;
        }
        EventPayload::Conflict {
            physical, verbal, ..
        } => {
            *valence_delta = impact::NEGATIVE_VALENCE * severity;
            if *physical {
                *arousal_delta = impact::HIGH_AROUSAL * severity;
            } else if *verbal {
                *arousal_delta = 0.2 * severity;
            }
        }
        EventPayload::Interaction {
            duration_minutes, ..
        } => {
            // Longer positive interactions reduce loneliness
            let duration_factor = (*duration_minutes as f32 / 60.0).min(1.0);
            *loneliness_delta = -0.05 * duration_factor;
        }
        _ => {}
    }
}

/// Applies an interpreted event to an entity, modifying their state.
///
/// This function iterates through the state_deltas and applies each
/// change to the entity's state dimensions.
///
/// # Arguments
///
/// * `interpreted` - The interpreted event with computed deltas
/// * `entity` - The entity to modify
#[cfg(test)]
pub(crate) fn apply_interpreted_event(interpreted: &InterpretedEvent, entity: &mut Entity) {
    use crate::enums::EventTag;

    let chronic = interpreted.event.has_tag(EventTag::ChronicPattern);

    // Apply state changes by iterating state_deltas
    for (path, delta) in &interpreted.state_deltas {
        let delta_f32 = *delta as f32;
        match path {
            StatePath::Mood(MoodPath::Valence) => {
                entity
                    .individual_state_mut()
                    .mood_mut()
                    .add_valence_delta(delta_f32);
            }
            StatePath::Mood(MoodPath::Arousal) => {
                entity
                    .individual_state_mut()
                    .mood_mut()
                    .add_arousal_delta(delta_f32);
            }
            StatePath::Mood(MoodPath::Dominance) => {
                entity
                    .individual_state_mut()
                    .mood_mut()
                    .add_dominance_delta(delta_f32);
            }
            StatePath::SocialCognition(SocialCognitionPath::Loneliness) => {
                let social = entity.individual_state_mut().social_cognition_mut();
                if chronic {
                    social.loneliness_mut().add_chronic_delta(delta_f32);
                } else {
                    social.add_loneliness_delta(delta_f32);
                }
            }
            StatePath::SocialCognition(SocialCognitionPath::PerceivedReciprocalCaring) => {
                let social = entity.individual_state_mut().social_cognition_mut();
                if chronic {
                    social
                        .perceived_reciprocal_caring_mut()
                        .add_chronic_delta(delta_f32);
                } else {
                    social.add_perceived_reciprocal_caring_delta(delta_f32);
                }
            }
            StatePath::SocialCognition(SocialCognitionPath::PerceivedLiability) => {
                let social = entity.individual_state_mut().social_cognition_mut();
                if chronic {
                    social
                        .perceived_liability_mut()
                        .add_chronic_delta(delta_f32);
                } else {
                    social.add_perceived_liability_delta(delta_f32);
                }
            }
            StatePath::SocialCognition(SocialCognitionPath::SelfHate) => {
                let social = entity.individual_state_mut().social_cognition_mut();
                if chronic {
                    social.self_hate_mut().add_chronic_delta(delta_f32);
                } else {
                    social.add_self_hate_delta(delta_f32);
                }
            }
            StatePath::Needs(NeedsPath::Purpose) => {
                entity
                    .individual_state_mut()
                    .needs_mut()
                    .add_purpose_delta(delta_f32);
            }
            StatePath::MentalHealth(MentalHealthPath::AcquiredCapability) => {
                entity
                    .individual_state_mut()
                    .mental_health_mut()
                    .add_acquired_capability_delta(delta_f32);
            }
            StatePath::MentalHealth(MentalHealthPath::InterpersonalHopelessness) => {
                entity
                    .individual_state_mut()
                    .mental_health_mut()
                    .add_interpersonal_hopelessness_delta(delta_f32);
            }
            StatePath::MentalHealth(MentalHealthPath::SelfWorth) => {
                entity
                    .individual_state_mut()
                    .mental_health_mut()
                    .add_self_worth_delta(delta_f32);
            }
            StatePath::Disposition(DispositionPath::Grievance) => {
                entity
                    .individual_state_mut()
                    .disposition_mut()
                    .add_grievance_delta(delta_f32);
            }
            // Other paths are not processed by event interpretation
            _ => {}
        }
    }

    // Create and store memory
    store_event_memory(interpreted, entity);
}

/// Processes an event into trust antecedents for related relationships.
///
/// For events with a source and target, this updates the target's
/// trustworthiness perceptions of the source.
pub(crate) fn process_event_to_relationships(
    event: &Event,
    timestamp: Timestamp,
    relationships: &mut [Relationship],
) {
    let (Some(source), Some(target)) = (event.source(), event.target()) else {
        return;
    };

    let mappings = get_antecedent_for_event(event);
    if mappings.is_empty() {
        return;
    }

    let severity = event.severity() as f32;

    for relationship in relationships.iter_mut() {
        let Some(direction) = direction_for_relationship(relationship, target, source) else {
            continue;
        };

        let consistency = relationship.pattern().consistency.clamp(0.0, 1.0);
        let consistency_weight = 0.5 + (consistency * 0.5);

        for mapping in &mappings {
            let raw_magnitude = (mapping.base_magnitude * severity).clamp(0.0, 1.0);
            let magnitude = raw_magnitude * consistency_weight;
            if magnitude <= 0.0 {
                continue;
            }
            let antecedent = TrustAntecedent::new(
                timestamp,
                mapping.antecedent_type,
                mapping.direction,
                magnitude,
                mapping.context,
            );
            relationship.append_antecedent(direction, antecedent);
        }

        let history = relationship.antecedent_history(direction).to_vec();
        relationship
            .trustworthiness_mut(direction)
            .recompute_from_antecedents(&history);
    }
}

fn direction_for_relationship(
    relationship: &Relationship,
    trustor: &crate::types::EntityId,
    trustee: &crate::types::EntityId,
) -> Option<Direction> {
    if relationship.entity_a() == trustor && relationship.entity_b() == trustee {
        Some(Direction::AToB)
    } else if relationship.entity_b() == trustor && relationship.entity_a() == trustee {
        Some(Direction::BToA)
    } else {
        None
    }
}

/// Stores a memory of the event.
#[cfg(test)]
fn store_event_memory(interpreted: &InterpretedEvent, entity: &mut Entity) {
    let event = &interpreted.event;

    // Determine tags from event
    let mut tags = Vec::new();
    match event.category() {
        EventCategory::Trauma => tags.push(MemoryTag::Violence),
        EventCategory::SocialBelonging => tags.push(MemoryTag::Personal),
        EventCategory::Achievement => tags.push(MemoryTag::Achievement),
        _ => tags.push(MemoryTag::Personal),
    }

    // Get participants from event source
    let mut participants = Vec::new();
    if let Some(source) = event.source() {
        participants.push(source.clone());
    }

    // Get microsystem context
    let microsystem = event.microsystem_context().cloned();

    // Use the entity's create_memory method which handles everything
    let _ = entity.create_memory(
        event.event_type().name(),
        participants,
        tags,
        interpreted.salience,
        microsystem,
    );
}

/// Processes an event completely: interprets and applies it.
///
/// This is a convenience function that combines interpretation and
/// application in a single call.
///
/// # Arguments
///
/// * `event` - The event to process
/// * `entity` - The entity to modify
///
/// # Returns
///
/// The interpreted event for logging/debugging
///
/// # Examples
///
/// ```ignore
/// use behavioral_pathways::processor::process_event;
/// use behavioral_pathways::event::EventBuilder;
/// use behavioral_pathways::enums::{EventType, MoodPath, StatePath};
/// use behavioral_pathways::entity::EntityBuilder;
/// use behavioral_pathways::enums::Species;
///
/// let mut entity = EntityBuilder::new()
///     .species(Species::Human)
///     .build()
///     .unwrap();
///
/// let initial_valence = entity.get_effective(StatePath::Mood(MoodPath::Valence));
///
/// let event = EventBuilder::new(EventType::SocialExclusion)
///     .severity(0.7)
///     .build()
///     .unwrap();
///
/// process_event(&event, &mut entity);
///
/// let final_valence = entity.get_effective(StatePath::Mood(MoodPath::Valence));
/// assert!(final_valence < initial_valence); // Exclusion reduced valence
/// ```
#[cfg(test)]
pub(crate) fn process_event(event: &Event, entity: &mut Entity) -> InterpretedEvent {
    let interpreted = interpret_event(event, entity);
    apply_interpreted_event(&interpreted, entity);
    interpreted
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::EntityBuilder;
    use crate::enums::{
        Direction, EventTag, LifeDomain, PersonalityProfile, RealizationType, Species,
        SupportType, WeaponType,
    };
    use crate::event::EventBuilder;
    use crate::memory::MemoryTag;
    use crate::state::Hexaco;
    use crate::types::{EntityId, GroupId};

    fn create_human() -> Entity {
        EntityBuilder::new()
            .species(Species::Human)
            .build()
            .unwrap()
    }

    #[test]
    fn interpret_social_exclusion_negative_valence() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::SocialExclusion)
            .severity(0.7)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        assert!(interpreted.valence_delta < 0.0);
        assert!(interpreted.loneliness_delta > 0.0);
    }

    #[test]
    fn interpret_social_inclusion_positive_valence() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::SocialInclusion)
            .severity(0.7)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        assert!(interpreted.valence_delta > 0.0);
        assert!(interpreted.loneliness_delta < 0.0);
    }

    #[test]
    fn interpret_event_ignores_unhandled_event_type_for_category() {
        let entity = create_human();
        let mut event = EventBuilder::new(EventType::Interaction)
            .severity(0.4)
            .build()
            .unwrap();

        event.set_category_for_test(EventCategory::SocialBelonging);
        let interpreted = interpret_event(&event, &entity);
        assert!(interpreted.loneliness_delta.abs() < f32::EPSILON);

        event.set_category_for_test(EventCategory::Control);
        let interpreted = interpret_event(&event, &entity);
        assert!(interpreted.dominance_delta.abs() < f32::EPSILON);

        event.set_category_for_test(EventCategory::Achievement);
        let interpreted = interpret_event(&event, &entity);
        assert!(interpreted.dominance_delta.abs() < f32::EPSILON);
    }

    #[test]
    fn interpret_burden_feedback_increases_liability() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::BurdenFeedback)
            .severity(0.8)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        assert!(interpreted.perceived_liability_delta > 0.0);
        assert!(interpreted.valence_delta < 0.0);
    }

    #[test]
    fn interpret_violence_increases_ac() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Violence)
            .severity(0.9)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        assert!(interpreted.acquired_capability_delta > 0.0);
        assert!(interpreted.arousal_delta > 0.0);
    }

    #[test]
    fn interpret_humiliation_decreases_dominance() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Humiliation)
            .severity(0.6)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        assert!(interpreted.dominance_delta < 0.0);
    }

    #[test]
    fn interpret_empowerment_increases_dominance() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Empowerment)
            .severity(0.6)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        assert!(interpreted.dominance_delta > 0.0);
        assert!(interpreted.valence_delta > 0.0);
    }

    #[test]
    fn interpret_support_with_payload() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Support)
            .severity(0.7)
            .payload(EventPayload::Support {
                support_type: SupportType::Emotional,
                effectiveness: 0.9,
            })
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        assert!(interpreted.valence_delta > 0.0);
        assert!(interpreted.prc_delta > 0.0);
    }

    #[test]
    fn interpret_employment_reduces_burdensomeness() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Achievement)
            .severity(0.8)
            .payload(EventPayload::Achievement {
                domain: LifeDomain::Work,
                magnitude: 0.7,
            })
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        assert!(interpreted.perceived_liability_delta < 0.0);
        assert!(interpreted.self_hate_delta < 0.0);
        assert!(interpreted
            .state_deltas
            .iter()
            .any(|(path, _)| *path == StatePath::MentalHealth(MentalHealthPath::SelfWorth)));
    }

    #[test]
    fn interpret_financial_achievement_impacts_self_worth_only() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Achievement)
            .severity(0.7)
            .payload(EventPayload::Achievement {
                domain: LifeDomain::Financial,
                magnitude: 0.6,
            })
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        let has_self_worth = interpreted
            .state_deltas
            .iter()
            .any(|(path, _)| *path == StatePath::MentalHealth(MentalHealthPath::SelfWorth));
        let has_purpose = interpreted
            .state_deltas
            .iter()
            .any(|(path, _)| *path == StatePath::Needs(NeedsPath::Purpose));

        assert!(has_self_worth);
        assert!(!has_purpose);
    }

    #[test]
    fn interpret_creative_achievement_boosts_purpose_only() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Achievement)
            .severity(0.7)
            .payload(EventPayload::Achievement {
                domain: LifeDomain::Creative,
                magnitude: 0.6,
            })
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        let has_self_worth = interpreted
            .state_deltas
            .iter()
            .any(|(path, _)| *path == StatePath::MentalHealth(MentalHealthPath::SelfWorth));
        let has_purpose = interpreted
            .state_deltas
            .iter()
            .any(|(path, _)| *path == StatePath::Needs(NeedsPath::Purpose));

        assert!(!has_self_worth);
        assert!(has_purpose);
    }

    #[test]
    fn interpret_group_participation_strengthens_belonging() {
        let entity = create_human();
        let grouped = EventBuilder::new(EventType::SocialInclusion)
            .severity(0.5)
            .payload(EventPayload::SocialInclusion {
                group_id: Some(GroupId::new("group_1").unwrap()),
            })
            .build()
            .unwrap();
        let solo = EventBuilder::new(EventType::SocialInclusion)
            .severity(0.5)
            .payload(EventPayload::SocialInclusion { group_id: None })
            .build()
            .unwrap();

        let grouped_interpreted = interpret_event(&grouped, &entity);
        let solo_interpreted = interpret_event(&solo, &entity);

        assert!(grouped_interpreted.loneliness_delta < solo_interpreted.loneliness_delta);
        assert!(grouped_interpreted.prc_delta > solo_interpreted.prc_delta);
    }

    #[test]
    fn interpret_recognition_reduces_burdensomeness() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Support)
            .severity(0.6)
            .payload(EventPayload::Support {
                support_type: SupportType::Emotional,
                effectiveness: 0.8,
            })
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        assert!(interpreted.perceived_liability_delta < 0.0);
        assert!(interpreted.self_hate_delta < 0.0);
    }

    #[test]
    fn interpret_purpose_development_adds_purpose_delta() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Realization)
            .severity(0.7)
            .payload(EventPayload::Realization {
                realization_type: RealizationType::ExistentialInsight,
            })
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        assert!(interpreted.perceived_liability_delta < 0.0);
        assert!(interpreted.self_hate_delta < 0.0);
        assert!(interpreted
            .state_deltas
            .iter()
            .any(|(path, _)| *path == StatePath::Needs(NeedsPath::Purpose)));
    }

    #[test]
    fn apply_interpreted_event_modifies_state() {
        let mut entity = create_human();
        let initial_valence = entity.get_effective(StatePath::Mood(MoodPath::Valence));

        let event = EventBuilder::new(EventType::SocialExclusion)
            .severity(0.7)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        apply_interpreted_event(&interpreted, &mut entity);

        let final_valence = entity.get_effective(StatePath::Mood(MoodPath::Valence));
        assert!(final_valence < initial_valence);
    }

    #[test]
    fn apply_interpreted_event_creates_memory() {
        let mut entity = create_human();

        let event = EventBuilder::new(EventType::Violence)
            .severity(0.9)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        apply_interpreted_event(&interpreted, &mut entity);

        assert!(entity.memories().total_count() > 0);
    }

    #[test]
    fn process_event_combines_interpret_and_apply() {
        let mut entity = create_human();
        let initial_valence = entity.get_effective(StatePath::Mood(MoodPath::Valence));

        let event = EventBuilder::new(EventType::SocialExclusion)
            .severity(0.7)
            .build()
            .unwrap();

        let interpreted = process_event(&event, &mut entity);

        let final_valence = entity.get_effective(StatePath::Mood(MoodPath::Valence));
        assert!(final_valence < initial_valence);
        assert!(interpreted.valence_delta < 0.0);
    }

    #[test]
    fn salience_computed_with_arousal() {
        let entity = create_human();

        let event = EventBuilder::new(EventType::Violence)
            .severity(0.9)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        // Trauma events should have high salience
        assert!(interpreted.salience >= 0.5);
    }

    #[test]
    fn attribution_with_source_is_other() {
        let entity = create_human();
        let source = crate::types::EntityId::new("attacker").unwrap();

        let event = EventBuilder::new(EventType::Violence)
            .source(source.clone())
            .severity(0.9)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        assert!(interpreted.attribution.is_other());
        assert_eq!(interpreted.attribution.other_entity(), Some(&source));
    }

    #[test]
    fn attribution_high_severity_is_stable() {
        let entity = create_human();
        let source = crate::types::EntityId::new("attacker").unwrap();

        let event = EventBuilder::new(EventType::Violence)
            .source(source)
            .severity(0.9)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        assert!(interpreted.attribution.is_stable());
    }

    #[test]
    fn emotionality_modulates_response() {
        // Create entity with high emotionality
        let high_emot_entity = EntityBuilder::new()
            .species(Species::Human)
            .personality(PersonalityProfile::Anxious)
            .build()
            .unwrap();

        // Create entity with low emotionality
        let low_emot_entity = EntityBuilder::new()
            .species(Species::Human)
            .personality(PersonalityProfile::Leader)
            .build()
            .unwrap();

        let event = EventBuilder::new(EventType::SocialExclusion)
            .severity(0.7)
            .build()
            .unwrap();

        let high_interpreted = interpret_event(&event, &high_emot_entity);
        let low_interpreted = interpret_event(&event, &low_emot_entity);

        // High emotionality should have stronger response
        assert!(high_interpreted.valence_delta.abs() > low_interpreted.valence_delta.abs());
    }

    #[test]
    fn achievement_positive_valence() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Achievement)
            .severity(0.8)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        assert!(interpreted.valence_delta > 0.0);
        assert!(interpreted.dominance_delta > 0.0);
    }

    #[test]
    fn failure_negative_valence() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Failure)
            .severity(0.6)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        assert!(interpreted.valence_delta < 0.0);
        assert!(interpreted.dominance_delta < 0.0);
    }

    #[test]
    fn traumatic_exposure_increases_ac() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::TraumaticExposure)
            .severity(0.8)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        assert!(interpreted.acquired_capability_delta > 0.0);
    }

    #[test]
    fn betrayal_reduces_prc() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Betrayal)
            .severity(0.7)
            .payload(EventPayload::Betrayal {
                confidence_violated: 0.8,
            })
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        assert!(interpreted.prc_delta < 0.0);
    }

    #[test]
    fn conflict_with_physical_increases_arousal() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Conflict)
            .severity(0.7)
            .payload(EventPayload::Conflict {
                verbal: true,
                physical: true,
                resolved: false,
            })
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        // Conflict always increases arousal per STATE_EFFECT_BLUEPRINT
        // Base delta: 0.12 * severity * emotionality_factor
        // With severity 0.7: 0.12 * 0.7 = 0.084, then emotionality modulation
        assert!(interpreted.arousal_delta > 0.05);
    }

    #[test]
    fn memory_layer_based_on_salience() {
        let mut entity = create_human();

        // High salience event (trauma)
        let high_salience_event = EventBuilder::new(EventType::Violence)
            .severity(0.95)
            .payload(EventPayload::Violence {
                weapon: Some(WeaponType::Firearm),
                injury_severity: 0.8,
            })
            .build()
            .unwrap();

        process_event(&high_salience_event, &mut entity);

        // Should have memory in long-term or short-term due to high salience
        assert!(entity.memories().total_count() > 0);
    }

    #[test]
    fn interpreted_event_debug() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Violence)
            .severity(0.7)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        let debug = format!("{:?}", interpreted);

        assert!(debug.contains("InterpretedEvent"));
    }

    #[test]
    fn interpreted_event_clone() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Violence)
            .severity(0.7)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        let cloned = interpreted.clone();

        assert!((interpreted.valence_delta - cloned.valence_delta).abs() < f32::EPSILON);
    }

    #[test]
    fn contextual_event_increases_arousal() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::PolicyChange)
            .severity(0.7)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        // Contextual events should increase arousal minimally
        assert!(interpreted.arousal_delta > 0.0);
    }

    #[test]
    fn interaction_event_reduces_loneliness() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Interaction)
            .severity(0.5)
            .payload(EventPayload::Interaction {
                topic: Some(crate::enums::InteractionTopic::DeepConversation),
                duration_minutes: 60,
            })
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        // Long interactions should reduce loneliness
        assert!(interpreted.loneliness_delta < 0.0);
    }

    #[test]
    fn verbal_conflict_moderate_arousal() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Conflict)
            .severity(0.5)
            .payload(EventPayload::Conflict {
                verbal: true,
                physical: false,
                resolved: false,
            })
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        // Verbal conflict should have moderate arousal
        assert!(interpreted.arousal_delta > 0.0);
        assert!(interpreted.arousal_delta < 0.3);
    }

    #[test]
    fn conflict_without_arousal_flags_still_increases_arousal() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Conflict)
            .severity(0.5)
            .payload(EventPayload::Conflict {
                verbal: false,
                physical: false,
                resolved: false,
            })
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        // Per STATE_EFFECT_BLUEPRINT spec, all conflict increases arousal
        // Even non-verbal, non-physical conflict activates nervous system
        // Base: 0.12 * 0.5 severity = 0.06, then emotionality modulation
        assert!(interpreted.arousal_delta > 0.03);
    }

    #[test]
    fn attribution_with_source_is_other_and_unstable() {
        let entity = create_human();
        let source = EntityId::new("source").unwrap();
        let event = EventBuilder::new(EventType::Interaction)
            .severity(0.4)
            .source(source)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        assert!(interpreted.attribution.is_other());
        assert!(!interpreted.attribution.is_stable());
    }

    #[test]
    fn self_caused_stable_attribution_adds_hopelessness_delta() {
        let hexaco = Hexaco::new().with_honesty_humility(0.8);
        let mut entity = EntityBuilder::new()
            .species(Species::Human)
            .hexaco(hexaco)
            .build()
            .unwrap();

        let event = EventBuilder::new(EventType::BurdenFeedback)
            .severity(0.9)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        assert!(interpreted.attribution.is_self_caused());
        assert!(interpreted.attribution.is_stable());
        assert!(interpreted.interpersonal_hopelessness_delta > 0.0);
        assert!(interpreted.state_deltas.iter().any(|(path, _)| {
            matches!(
                path,
                StatePath::MentalHealth(MentalHealthPath::InterpersonalHopelessness)
            )
        }));

        let before = entity
            .get_effective(StatePath::MentalHealth(
                MentalHealthPath::InterpersonalHopelessness,
            ))
            .unwrap_or(0.0);
        apply_interpreted_event(&interpreted, &mut entity);
        let after = entity
            .get_effective(StatePath::MentalHealth(
                MentalHealthPath::InterpersonalHopelessness,
            ))
            .unwrap_or(0.0);
        assert!(after > before);
    }

    #[test]
    fn low_honesty_humility_situational_attribution() {
        // Create entity with low honesty-humility (Rebel personality)
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .personality(PersonalityProfile::Rebel)
            .build()
            .unwrap();

        let event = EventBuilder::new(EventType::Failure)
            .severity(0.5)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        // Low honesty-humility should lead to situational attribution
        assert!(interpreted.attribution.is_situational());
    }

    #[test]
    fn unknown_attribution_for_middle_honesty_humility() {
        // Create entity with neutral personality (middle honesty-humility)
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .personality(PersonalityProfile::Balanced)
            .build()
            .unwrap();

        let event = EventBuilder::new(EventType::Failure)
            .severity(0.3) // Low severity = unstable attribution
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        // Middle honesty-humility with no source should lead to unknown attribution
        assert!(interpreted.attribution.is_unknown());
    }

    #[test]
    fn loss_event_uses_personal_tag() {
        let mut entity = create_human();

        let event = EventBuilder::new(EventType::Loss)
            .severity(0.6)
            .build()
            .unwrap();

        process_event(&event, &mut entity);

        // Loss event should create a memory (via Social category -> Personal tag)
        assert!(entity.memories().total_count() > 0);
    }

    #[test]
    fn achievement_event_uses_achievement_tag() {
        let mut entity = create_human();

        let event = EventBuilder::new(EventType::Achievement)
            .severity(0.8)
            .build()
            .unwrap();

        process_event(&event, &mut entity);

        let tagged = entity.memories().retrieve_by_tag(MemoryTag::Achievement);
        assert!(!tagged.is_empty());
    }

    #[test]
    fn source_event_adds_participant_to_memory() {
        let mut entity = create_human();
        let source = EntityId::new("source_participant").unwrap();

        let event = EventBuilder::new(EventType::Interaction)
            .severity(0.6)
            .source(source.clone())
            .build()
            .unwrap();

        process_event(&event, &mut entity);

        let has_participant = entity
            .memories()
            .all_memories()
            .any(|entry| entry.involves_participant(&source));
        assert!(has_participant);
    }

    #[test]
    fn context_transition_event() {
        let entity = create_human();
        let from = crate::types::MicrosystemId::new("home").unwrap();
        let to = crate::types::MicrosystemId::new("work").unwrap();

        let event = EventBuilder::new(EventType::ContextTransition)
            .severity(0.3)
            .payload(EventPayload::ContextTransition { from, to })
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        // Context transitions are Contextual category
        assert!(interpreted.arousal_delta > 0.0);
    }

    #[test]
    fn realization_event() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Realization)
            .severity(0.5)
            .payload(EventPayload::Realization {
                realization_type: crate::enums::RealizationType::SelfInsight,
            })
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        // Realization is Contextual category
        assert!(interpreted.arousal_delta >= 0.0);
    }

    #[test]
    fn historical_event() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::HistoricalEvent)
            .severity(0.8)
            .payload(EventPayload::HistoricalEvent {
                event_type: crate::enums::HistoricalEventType::Pandemic,
                scope: crate::enums::HistoricalScope::Global,
            })
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        // Historical events are Contextual category
        assert!(interpreted.arousal_delta > 0.0);
    }

    #[test]
    fn interpreted_event_has_original_event_id() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Violence)
            .severity(0.7)
            .build()
            .unwrap();
        let expected_id = event.id().clone();

        let interpreted = interpret_event(&event, &entity);

        assert_eq!(interpreted.original_event, expected_id);
    }

    #[test]
    fn interpreted_event_has_perceived_severity() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Violence)
            .severity(0.5)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        // Perceived severity should be close to actual severity for neutral personality
        assert!(interpreted.perceived_severity > 0.0);
    }

    #[test]
    fn interpreted_event_has_memory_salience() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Violence)
            .severity(0.9)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        // Memory salience should match computed salience
        assert!((interpreted.memory_salience - interpreted.salience as f64).abs() < f64::EPSILON);
    }

    #[test]
    fn interpreted_event_has_state_deltas() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Violence)
            .severity(0.9)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        // Violence should have valence and arousal deltas at minimum
        assert!(!interpreted.state_deltas.is_empty());

        // Should have valence delta
        let has_valence = interpreted
            .state_deltas
            .iter()
            .any(|(path, _)| matches!(path, StatePath::Mood(MoodPath::Valence)));
        assert!(has_valence);
    }

    #[test]
    fn apply_via_state_deltas_same_as_direct() {
        // Create two identical entities
        let mut entity1 = create_human();
        let mut entity2 = create_human();

        let event = EventBuilder::new(EventType::SocialExclusion)
            .severity(0.7)
            .build()
            .unwrap();

        // Apply to both
        let interpreted = interpret_event(&event, &entity1);
        apply_interpreted_event(&interpreted, &mut entity1);
        apply_interpreted_event(&interpreted, &mut entity2);

        // Both should have same valence
        let v1 = entity1.get_effective(StatePath::Mood(MoodPath::Valence));
        let v2 = entity2.get_effective(StatePath::Mood(MoodPath::Valence));
        assert!((v1.unwrap_or(0.0) - v2.unwrap_or(0.0)).abs() < 0.001);
    }

    #[test]
    fn state_deltas_include_all_nonzero_changes() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::BurdenFeedback)
            .severity(0.9)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        // Burden feedback should affect perceived_liability
        let has_liability = interpreted
            .state_deltas
            .iter()
            .any(|(path, _)| matches!(path, StatePath::SocialCognition(SocialCognitionPath::PerceivedLiability)));
        assert!(has_liability);
    }

    #[test]
    fn apply_interpreted_event_applies_all_state_deltas() {
        let mut entity = create_human();
        let event = EventBuilder::new(EventType::Violence)
            .severity(0.9)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        // Record initial values
        let initial_valence = entity
            .get_effective(StatePath::Mood(MoodPath::Valence))
            .unwrap();

        apply_interpreted_event(&interpreted, &mut entity);

        // Valence should have changed
        let new_valence = entity
            .get_effective(StatePath::Mood(MoodPath::Valence))
            .unwrap();
        assert!((new_valence - initial_valence).abs() > 0.01);
    }

    #[test]
    fn apply_interpreted_event_handles_dominance_path() {
        let mut entity = create_human();
        let event = EventBuilder::new(EventType::Humiliation)
            .severity(0.8)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        apply_interpreted_event(&interpreted, &mut entity);

        // Humiliation should decrease dominance
        let dominance = entity
            .get_effective(StatePath::Mood(MoodPath::Dominance))
            .unwrap();
        assert!(dominance < 0.0);
    }

    #[test]
    fn apply_interpreted_event_handles_mental_health_paths() {
        let mut entity = create_human();
        let event = EventBuilder::new(EventType::TraumaticExposure)
            .severity(0.9)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        // Should have AC delta
        let has_ac = interpreted.state_deltas.iter().any(|(path, _)| {
            matches!(
                path,
                StatePath::MentalHealth(MentalHealthPath::AcquiredCapability)
            )
        });
        assert!(has_ac);

        apply_interpreted_event(&interpreted, &mut entity);
    }

    #[test]
    fn apply_interpreted_event_handles_needs_paths() {
        let mut entity = create_human();
        let event = EventBuilder::new(EventType::SocialExclusion)
            .severity(0.9)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        // Should have loneliness delta
        let has_loneliness = interpreted
            .state_deltas
            .iter()
            .any(|(path, _)| matches!(path, StatePath::SocialCognition(SocialCognitionPath::Loneliness)));
        assert!(has_loneliness);

        apply_interpreted_event(&interpreted, &mut entity);

        // Check loneliness increased
        let loneliness = entity
            .get_effective(StatePath::SocialCognition(SocialCognitionPath::Loneliness))
            .unwrap();
        assert!(loneliness > 0.0);
    }

    #[test]
    fn apply_interpreted_event_handles_self_hate_delta() {
        let mut entity = create_human();
        let event = EventBuilder::new(EventType::BurdenFeedback)
            .severity(0.95)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        // Should have self_hate delta
        let has_self_hate = interpreted
            .state_deltas
            .iter()
            .any(|(path, _)| matches!(path, StatePath::SocialCognition(SocialCognitionPath::SelfHate)));
        assert!(has_self_hate);

        apply_interpreted_event(&interpreted, &mut entity);
    }

    #[test]
    fn apply_interpreted_event_handles_prc_delta() {
        let mut entity = create_human();
        let event = EventBuilder::new(EventType::Support)
            .severity(0.8)
            .payload(EventPayload::Support {
                support_type: crate::enums::SupportType::Emotional,
                effectiveness: 0.9,
            })
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        // Should have PRC delta
        let has_prc = interpreted.state_deltas.iter().any(|(path, _)| {
            matches!(path, StatePath::SocialCognition(SocialCognitionPath::PerceivedReciprocalCaring))
        });
        assert!(has_prc);

        apply_interpreted_event(&interpreted, &mut entity);
    }

    #[test]
    fn apply_interpreted_event_ignores_other_paths() {
        let mut entity = create_human();

        // Manually create an InterpretedEvent with a path that's not handled
        let event = EventBuilder::new(EventType::Interaction)
            .severity(0.5)
            .build()
            .unwrap();

        let mut interpreted = interpret_event(&event, &entity);

        // Add a path that's not handled (e.g., HEXACO)
        interpreted
            .state_deltas
            .push((StatePath::Hexaco(crate::enums::HexacoPath::Openness), 0.1));

        // This should not panic
        apply_interpreted_event(&interpreted, &mut entity);
    }

    // === InterpretedEvent::scaled_by tests ===

    #[test]
    fn scaled_by_scales_all_deltas() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::SocialExclusion)
            .severity(0.8)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        let original_valence = interpreted.valence_delta;

        let scaled = interpreted.scaled_by(2.0);

        // All individual deltas should be scaled
        assert!((scaled.valence_delta - original_valence * 2.0).abs() < 0.001);
        assert!((scaled.arousal_delta - interpreted.arousal_delta * 2.0).abs() < 0.001);
        assert!((scaled.dominance_delta - interpreted.dominance_delta * 2.0).abs() < 0.001);
        assert!((scaled.loneliness_delta - interpreted.loneliness_delta * 2.0).abs() < 0.001);
        assert!((scaled.prc_delta - interpreted.prc_delta * 2.0).abs() < 0.001);
        assert!(
            (scaled.perceived_liability_delta - interpreted.perceived_liability_delta * 2.0).abs()
                < 0.001
        );
        assert!((scaled.self_hate_delta - interpreted.self_hate_delta * 2.0).abs() < 0.001);
        assert!(
            (scaled.acquired_capability_delta - interpreted.acquired_capability_delta * 2.0).abs()
                < 0.001
        );
        assert!(
            (scaled.interpersonal_hopelessness_delta
                - interpreted.interpersonal_hopelessness_delta * 2.0)
                .abs()
                < 0.001
        );
    }

    #[test]
    fn scaled_by_scales_state_deltas() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::SocialExclusion)
            .severity(0.8)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        let scaled = interpreted.scaled_by(1.5);

        // state_deltas should be scaled by the same factor
        for (i, (path, delta)) in scaled.state_deltas.iter().enumerate() {
            let (orig_path, orig_delta) = &interpreted.state_deltas[i];
            assert_eq!(path, orig_path);
            assert!((*delta - orig_delta * 1.5).abs() < 0.0001);
        }
    }

    #[test]
    fn scaled_by_preserves_salience() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Violence)
            .severity(0.9)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        let scaled = interpreted.scaled_by(2.0);

        // Salience should not be scaled
        assert!((scaled.salience - interpreted.salience).abs() < 0.001);
        assert!((scaled.memory_salience - interpreted.memory_salience).abs() < 0.001);
    }

    #[test]
    fn scaled_by_scales_perceived_severity() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Violence)
            .severity(0.7)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        let scaled = interpreted.scaled_by(1.5);

        // Perceived severity should be scaled
        assert!((scaled.perceived_severity - interpreted.perceived_severity * 1.5).abs() < 0.001);
    }

    #[test]
    fn scaled_by_with_factor_one_preserves_values() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Achievement)
            .severity(0.6)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        let scaled = interpreted.scaled_by(1.0);

        // All values should be preserved when scaling by 1.0
        assert!((scaled.valence_delta - interpreted.valence_delta).abs() < 0.001);
        assert!((scaled.perceived_severity - interpreted.perceived_severity).abs() < 0.001);
    }

    #[test]
    fn scaled_by_with_small_factor_reduces_impact() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Failure)
            .severity(0.8)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        let scaled = interpreted.scaled_by(0.5);

        // Impact should be reduced
        assert!(scaled.valence_delta.abs() < interpreted.valence_delta.abs());
    }

    #[test]
    fn scaled_by_preserves_event_and_attribution() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Betrayal)
            .severity(0.7)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        let scaled = interpreted.scaled_by(2.0);

        // Event and attribution should be preserved
        assert_eq!(scaled.event.event_type(), interpreted.event.event_type());
        assert_eq!(scaled.original_event, interpreted.original_event);
        // Attribution type should match (note: Attribution derives Clone)
    }

    #[test]
    fn antecedent_tracks_trust_building_events() {
        use crate::relationship::{AntecedentDirection, AntecedentType, Relationship};
        use crate::types::Timestamp;

        let alice = EntityId::new("alice").unwrap();
        let bob = EntityId::new("bob").unwrap();
        let event = EventBuilder::new(EventType::Support)
            .source(alice.clone())
            .target(bob.clone())
            .severity(1.0)
            .build()
            .unwrap();
        let timestamp = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);

        let mut relationships = vec![Relationship::try_between(alice, bob).unwrap()];
        process_event_to_relationships(&event, timestamp, &mut relationships);

        let history = relationships[0].antecedent_history(Direction::BToA);
        assert!(!history.is_empty());
        assert!(history.iter().any(|entry| {
            entry.antecedent_type() == AntecedentType::Benevolence
                && entry.direction() == AntecedentDirection::Positive
        }));
        assert!(relationships[0]
            .trustworthiness(Direction::BToA)
            .benevolence()
            .delta()
            > 0.0);
    }

    #[test]
    fn antecedent_magnitude_scales_with_relationship_consistency() {
        use crate::relationship::{AntecedentType, Relationship};
        use crate::types::Timestamp;

        let alice = EntityId::new("alice").unwrap();
        let bob = EntityId::new("bob").unwrap();
        let event = EventBuilder::new(EventType::Support)
            .source(alice.clone())
            .target(bob.clone())
            .severity(1.0)
            .build()
            .unwrap();
        let timestamp = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);

        let low_consistency = Relationship::try_between(alice.clone(), bob.clone()).unwrap();
        let low_consistency_value = low_consistency.pattern().consistency.clamp(0.0, 1.0);
        let mut high_consistency = Relationship::try_between(alice, bob).unwrap();
        high_consistency.pattern_mut().consistency = 1.0;
        let high_consistency_value = high_consistency.pattern().consistency.clamp(0.0, 1.0);

        let base_magnitude = get_antecedent_for_event(&event)
            .iter()
            .find(|mapping| mapping.antecedent_type == AntecedentType::Benevolence)
            .map(|mapping| mapping.base_magnitude)
            .unwrap_or(0.0);
        let raw_magnitude = (base_magnitude * event.severity() as f32).clamp(0.0, 1.0);
        let low_expected = raw_magnitude * (0.5 + low_consistency_value * 0.5);
        let high_expected = raw_magnitude * (0.5 + high_consistency_value * 0.5);

        let mut relationships = vec![low_consistency, high_consistency];
        process_event_to_relationships(&event, timestamp, &mut relationships);

        let low_mag = relationships[0]
            .antecedent_history(Direction::BToA)
            .iter()
            .find(|entry| entry.antecedent_type() == AntecedentType::Benevolence)
            .unwrap()
            .magnitude();
        let high_mag = relationships[1]
            .antecedent_history(Direction::BToA)
            .iter()
            .find(|entry| entry.antecedent_type() == AntecedentType::Benevolence)
            .unwrap()
            .magnitude();

        assert!(high_mag > low_mag);
        assert!((low_mag - low_expected).abs() < 0.01);
        assert!((high_mag - high_expected).abs() < 0.01);
    }

    #[test]
    fn antecedent_history_provides_trust_narrative() {
        use crate::relationship::Relationship;
        use crate::types::Timestamp;

        let alice = EntityId::new("alice").unwrap();
        let bob = EntityId::new("bob").unwrap();
        let event = EventBuilder::new(EventType::Support)
            .source(alice.clone())
            .target(bob.clone())
            .severity(0.8)
            .build()
            .unwrap();
        let timestamp = Timestamp::from_ymd_hms(2024, 1, 2, 0, 0, 0);

        let mut relationships = vec![Relationship::try_between(alice, bob).unwrap()];
        process_event_to_relationships(&event, timestamp, &mut relationships);

        let history = relationships[0].antecedent_history(Direction::BToA);
        assert!(history
            .iter()
            .any(|entry| entry.context().contains("support")));
    }

    #[test]
    fn process_event_to_relationships_applies_pattern_consistency_weight() {
        use crate::relationship::{get_antecedent_for_event, Relationship};
        use crate::types::Timestamp;

        let alice = EntityId::new("alice").unwrap();
        let bob = EntityId::new("bob").unwrap();
        let event = EventBuilder::new(EventType::Support)
            .source(alice.clone())
            .target(bob.clone())
            .severity(0.8)
            .build()
            .unwrap();
        let timestamp = Timestamp::from_ymd_hms(2024, 1, 3, 0, 0, 0);

        let mappings = get_antecedent_for_event(&event);
        assert!(!mappings.is_empty());
        let raw_magnitude = (mappings[0].base_magnitude * event.severity() as f32).clamp(0.0, 1.0);

        let mut low_consistency = Relationship::try_between(alice.clone(), bob.clone()).unwrap();
        low_consistency.pattern_mut().consistency = 0.0;

        let mut high_consistency = Relationship::try_between(alice, bob).unwrap();
        high_consistency.pattern_mut().consistency = 1.0;

        let mut relationships = vec![low_consistency, high_consistency];
        process_event_to_relationships(&event, timestamp, &mut relationships);

        let low_mag = relationships[0].antecedent_history(Direction::BToA)[0].magnitude();
        let high_mag = relationships[1].antecedent_history(Direction::BToA)[0].magnitude();

        let expected_low = raw_magnitude * 0.5;
        let expected_high = raw_magnitude;

        assert!((low_mag - expected_low).abs() < 0.001);
        assert!((high_mag - expected_high).abs() < 0.001);
        assert!(high_mag > low_mag);
    }

    #[test]
    fn process_event_to_relationships_tracks_a_to_b_direction() {
        use crate::relationship::Relationship;
        use crate::types::Timestamp;

        let alice = EntityId::new("alice").unwrap();
        let bob = EntityId::new("bob").unwrap();
        let event = EventBuilder::new(EventType::Support)
            .source(bob.clone())
            .target(alice.clone())
            .severity(1.0)
            .build()
            .unwrap();
        let timestamp = Timestamp::from_ymd_hms(2024, 1, 4, 0, 0, 0);

        let mut relationships = vec![Relationship::try_between(alice, bob).unwrap()];
        process_event_to_relationships(&event, timestamp, &mut relationships);

        assert!(!relationships[0]
            .antecedent_history(Direction::AToB)
            .is_empty());
    }

    #[test]
    fn process_event_to_relationships_requires_source_and_target() {
        use crate::relationship::Relationship;
        use crate::types::Timestamp;

        let alice = EntityId::new("alice").unwrap();
        let bob = EntityId::new("bob").unwrap();
        let event = EventBuilder::new(EventType::Support)
            .severity(0.8)
            .build()
            .unwrap();
        let timestamp = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);

        let mut relationships = vec![Relationship::try_between(alice, bob).unwrap()];
        process_event_to_relationships(&event, timestamp, &mut relationships);

        assert!(relationships[0]
            .antecedent_history(Direction::AToB)
            .is_empty());
        assert!(relationships[0]
            .antecedent_history(Direction::BToA)
            .is_empty());
    }

    #[test]
    fn process_event_to_relationships_skips_unmapped_events() {
        use crate::relationship::Relationship;
        use crate::types::Timestamp;

        let alice = EntityId::new("alice").unwrap();
        let bob = EntityId::new("bob").unwrap();
        let event = EventBuilder::new(EventType::PolicyChange)
            .source(alice.clone())
            .target(bob.clone())
            .severity(0.8)
            .build()
            .unwrap();
        let timestamp = Timestamp::from_ymd_hms(2024, 1, 2, 0, 0, 0);

        let mut relationships = vec![Relationship::try_between(alice, bob).unwrap()];
        process_event_to_relationships(&event, timestamp, &mut relationships);

        assert!(relationships[0]
            .antecedent_history(Direction::AToB)
            .is_empty());
        assert!(relationships[0]
            .antecedent_history(Direction::BToA)
            .is_empty());
    }

    #[test]
    fn process_event_to_relationships_skips_unrelated_relationships() {
        use crate::relationship::Relationship;
        use crate::types::Timestamp;

        let alice = EntityId::new("alice").unwrap();
        let bob = EntityId::new("bob").unwrap();
        let carol = EntityId::new("carol").unwrap();
        let dave = EntityId::new("dave").unwrap();
        let event = EventBuilder::new(EventType::Support)
            .source(carol)
            .target(dave)
            .severity(0.8)
            .build()
            .unwrap();
        let timestamp = Timestamp::from_ymd_hms(2024, 1, 3, 0, 0, 0);

        let mut relationships = vec![Relationship::try_between(alice, bob).unwrap()];
        process_event_to_relationships(&event, timestamp, &mut relationships);

        assert!(relationships[0]
            .antecedent_history(Direction::AToB)
            .is_empty());
        assert!(relationships[0]
            .antecedent_history(Direction::BToA)
            .is_empty());
    }

    #[test]
    fn interaction_with_physical_conflict_payload_increases_arousal() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Interaction)
            .severity(0.7)
            .payload(EventPayload::Conflict {
                verbal: false,
                physical: true,
                resolved: false,
            })
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        assert!(interpreted.valence_delta < 0.0);
        assert!(interpreted.arousal_delta > 0.0);
        assert!(interpreted.arousal_delta > 0.2);
    }

    #[test]
    fn interaction_with_verbal_conflict_payload_moderate_arousal() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Interaction)
            .severity(0.7)
            .payload(EventPayload::Conflict {
                verbal: true,
                physical: false,
                resolved: false,
            })
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        assert!(interpreted.valence_delta < 0.0);
        assert!(interpreted.arousal_delta > 0.0);
        assert!(interpreted.arousal_delta < 0.25);
    }

    #[test]
    fn interaction_with_conflict_payload_sets_negative_valence() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Interaction)
            .severity(0.6)
            .payload(EventPayload::Conflict {
                verbal: false,
                physical: false,
                resolved: false,
            })
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        assert!(interpreted.valence_delta < 0.0);
    }

    #[test]
    fn support_emotional_type_reduces_perceived_liability() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Support)
            .severity(0.7)
            .payload(EventPayload::Support {
                support_type: SupportType::Emotional,
                effectiveness: 0.9,
            })
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        // Emotional support should reduce liability, self-hate, and increase self-worth
        assert!(interpreted.perceived_liability_delta < 0.0);
        assert!(interpreted.self_hate_delta < 0.0);
    }

    #[test]
    fn support_companionship_reduces_liability_and_self_hate() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Support)
            .severity(0.6)
            .payload(EventPayload::Support {
                support_type: SupportType::Companionship,
                effectiveness: 0.85,
            })
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        // Companionship matches the Emotional|Companionship pattern
        assert!(interpreted.perceived_liability_delta < 0.0);
        assert!(interpreted.self_hate_delta < 0.0);
    }

    #[test]
    fn support_instrumental_type_does_not_trigger_special_treatment() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Support)
            .severity(0.7)
            .payload(EventPayload::Support {
                support_type: SupportType::Instrumental,
                effectiveness: 0.9,
            })
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        // Instrumental support doesn't get the special Emotional/Companionship treatment
        // So self_hate_delta should be 0
        assert!(interpreted.self_hate_delta.abs() < f32::EPSILON);
    }

    #[test]
    fn relationship_skips_when_no_matching_direction() {
        use crate::relationship::Relationship;
        use crate::types::Timestamp;

        let alice = EntityId::new("alice").unwrap();
        let bob = EntityId::new("bob").unwrap();
        let charlie = EntityId::new("charlie").unwrap();
        let diana = EntityId::new("diana").unwrap();

        // Event between charlie and diana
        let event = EventBuilder::new(EventType::Support)
            .source(charlie)
            .target(diana)
            .severity(0.5)
            .build()
            .unwrap();

        let timestamp = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);

        // Relationship between alice and bob (unrelated to event)
        let mut relationships = vec![Relationship::try_between(alice, bob).unwrap()];
        process_event_to_relationships(&event, timestamp, &mut relationships);

        // No antecedents should be added since relationship doesn't match event participants
        assert!(relationships[0]
            .antecedent_history(Direction::AToB)
            .is_empty());
        assert!(relationships[0]
            .antecedent_history(Direction::BToA)
            .is_empty());
    }

    #[test]
    fn relationship_skips_zero_magnitude_antecedents() {
        use crate::relationship::Relationship;
        use crate::types::Timestamp;

        let alice = EntityId::new("alice").unwrap();
        let bob = EntityId::new("bob").unwrap();

        // Event with zero severity produces zero raw_magnitude
        let event = EventBuilder::new(EventType::Support)
            .source(alice.clone())
            .target(bob.clone())
            .severity(0.0)
            .build()
            .unwrap();

        let timestamp = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);

        let mut relationships = vec![Relationship::try_between(alice, bob).unwrap()];
        process_event_to_relationships(&event, timestamp, &mut relationships);

        // With severity 0.0, magnitude is 0.0, so no antecedents added
        assert!(relationships[0]
            .antecedent_history(Direction::BToA)
            .is_empty());
    }

    // ========================================================================
    // Coverage for chronic delta applications and remaining gaps
    // ========================================================================

    #[test]
    fn chronic_loneliness_delta_uses_add_chronic_delta() {
        let mut entity = create_human();
        let event = EventBuilder::new(EventType::SocialExclusion)
            .severity(0.8)
            .tag(EventTag::ChronicPattern)
            .build()
            .unwrap();

        let mut interpreted = interpret_event(&event, &entity);

        // Manually add loneliness delta to state_deltas
        interpreted.state_deltas.push((
            StatePath::SocialCognition(SocialCognitionPath::Loneliness),
            0.15,
        ));

        apply_interpreted_event(&interpreted, &mut entity);

        // Chronic loneliness delta should be applied
        let loneliness = entity
            .get_effective(StatePath::SocialCognition(SocialCognitionPath::Loneliness))
            .unwrap_or(0.0);
        assert!(loneliness > 0.0);
    }

    #[test]
    fn chronic_prc_delta_uses_add_chronic_delta() {
        let mut entity = create_human();
        let event = EventBuilder::new(EventType::Betrayal)
            .severity(0.7)
            .tag(EventTag::ChronicPattern)
            .build()
            .unwrap();

        let mut interpreted = interpret_event(&event, &entity);

        // Manually add PRC delta to state_deltas (positive for easier assertion)
        interpreted.state_deltas.push((
            StatePath::SocialCognition(SocialCognitionPath::PerceivedReciprocalCaring),
            0.12,
        ));

        apply_interpreted_event(&interpreted, &mut entity);

        // Chronic PRC delta should be applied
        let prc = entity
            .get_effective(StatePath::SocialCognition(
                SocialCognitionPath::PerceivedReciprocalCaring,
            ))
            .unwrap_or(0.0);
        // Verify chronic delta was applied
        assert!(prc > 0.0);
    }

    #[test]
    fn chronic_perceived_liability_delta_uses_add_chronic_delta() {
        let mut entity = create_human();
        let event = EventBuilder::new(EventType::BurdenFeedback)
            .severity(0.6)
            .tag(EventTag::ChronicPattern)
            .build()
            .unwrap();

        let mut interpreted = interpret_event(&event, &entity);

        // Manually add liability delta to state_deltas
        interpreted.state_deltas.push((
            StatePath::SocialCognition(SocialCognitionPath::PerceivedLiability),
            0.14,
        ));

        apply_interpreted_event(&interpreted, &mut entity);

        // Chronic liability delta should be applied
        let liability = entity
            .get_effective(StatePath::SocialCognition(
                SocialCognitionPath::PerceivedLiability,
            ))
            .unwrap_or(0.0);
        assert!(liability > 0.0);
    }

    #[test]
    fn chronic_self_hate_delta_uses_add_chronic_delta() {
        let mut entity = create_human();
        let event = EventBuilder::new(EventType::BurdenFeedback)
            .severity(0.5)
            .tag(EventTag::ChronicPattern)
            .build()
            .unwrap();

        let mut interpreted = interpret_event(&event, &entity);

        // Manually add self-hate delta to state_deltas
        interpreted.state_deltas.push((
            StatePath::SocialCognition(SocialCognitionPath::SelfHate),
            0.13,
        ));

        apply_interpreted_event(&interpreted, &mut entity);

        // Chronic self-hate delta should be applied
        let self_hate = entity
            .get_effective(StatePath::SocialCognition(SocialCognitionPath::SelfHate))
            .unwrap_or(0.0);
        assert!(self_hate > 0.0);
    }

    #[test]
    fn purpose_delta_applies_to_needs() {
        let mut entity = create_human();
        let event = EventBuilder::new(EventType::Realization)
            .severity(0.7)
            .build()
            .unwrap();

        let mut interpreted = interpret_event(&event, &entity);

        // Manually add purpose delta to state_deltas
        interpreted.state_deltas.push((
            StatePath::Needs(NeedsPath::Purpose),
            0.18,
        ));

        apply_interpreted_event(&interpreted, &mut entity);

        // Purpose delta should be applied
        let purpose = entity
            .get_effective(StatePath::Needs(NeedsPath::Purpose))
            .unwrap_or(0.0);
        assert!(purpose > 0.0);
    }

    #[test]
    fn achievement_event_with_empty_payload_skips_domain_logic() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Achievement)
            .severity(0.6)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        // Should have base achievement effects but not domain-specific effects
        assert!(interpreted.valence_delta > 0.0);
    }

    #[test]
    fn social_inclusion_with_no_group_skips_group_logic() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::SocialInclusion)
            .severity(0.5)
            .payload(EventPayload::SocialInclusion { group_id: None })
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        // Should have base effects but group-specific logic skipped
        assert!(interpreted.valence_delta > 0.0);
    }

    #[test]
    fn support_event_with_empty_payload_uses_blueprint() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Support)
            .severity(0.5)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        // Should have blueprint effects, not payload-specific effects
        assert!(interpreted.valence_delta > 0.0);
    }

    #[test]
    fn realization_without_existential_insight_skips_insight_logic() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::Realization)
            .severity(0.7)
            .payload(EventPayload::Realization {
                realization_type: RealizationType::SelfInsight,
            })
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        // Should have base effects but not ExistentialInsight-specific effects
        assert!(!interpreted.state_deltas.is_empty());
    }
}
