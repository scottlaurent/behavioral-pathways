//! State evolution functions for temporal state computation.
//!
//! This module provides pure functions for advancing and regressing
//! IndividualState over time, and for applying/reversing events.
//!
//! These functions are the core building blocks for the `state_at()` API.

#[cfg(test)]
use crate::event::Event;
use crate::enums::SocialCognitionPath;
use crate::state::{IndividualState, SocialCognition};
use crate::types::Duration;

/// Advances state forward in time by applying decay.
///
/// This is a pure function that returns a new state with decay applied.
/// The original state is not modified.
///
/// # Arguments
///
/// * `state` - The starting state
/// * `duration` - The time to advance
///
/// # Returns
///
/// A new `IndividualState` with decay applied.
///
/// # Examples
///
/// ```ignore
/// use behavioral_pathways::processor::advance_state;
/// use behavioral_pathways::state::IndividualState;
/// use behavioral_pathways::types::Duration;
///
/// let mut initial = IndividualState::new();
/// initial.mood_mut().add_valence_delta(0.5);
///
/// let advanced = advance_state(initial, Duration::hours(6));
///
/// // Delta should have decayed (6-hour half-life for mood)
/// assert!(advanced.mood().valence_delta() < 0.5);
/// ```
#[must_use]
pub(crate) fn advance_state(state: IndividualState, duration: Duration) -> IndividualState {
    let mut new_state = state;
    new_state.apply_decay(duration);
    new_state
}

/// Regresses state backward in time by reversing decay.
///
/// This is a pure function that returns a new state with decay reversed.
/// The original state is not modified.
///
/// Note: Some dimensions (like Acquired Capability) cannot be regressed
/// because they have no decay. These dimensions remain unchanged.
///
/// # Arguments
///
/// * `state` - The current state
/// * `duration` - The time to regress
///
/// # Returns
///
/// A new `IndividualState` with decay reversed where possible.
///
/// # Examples
///
/// ```ignore
/// use behavioral_pathways::processor::regress_state;
/// use behavioral_pathways::state::IndividualState;
/// use behavioral_pathways::types::Duration;
///
/// let initial = IndividualState::new();
/// let regressed = regress_state(initial, Duration::hours(6));
/// ```
#[must_use]
pub(crate) fn regress_state(state: IndividualState, duration: Duration) -> IndividualState {
    let mut new_state = state;

    // Reverse decay on all decayable dimensions
    // Note: This is an approximation - we reverse by applying the inverse of decay
    reverse_decay_on_state(&mut new_state, duration);

    new_state
}

fn apply_social_cognition_delta(
    social: &mut SocialCognition,
    path: SocialCognitionPath,
    amount: f32,
    chronic: bool,
) {
    match path {
        SocialCognitionPath::Loneliness => {
            if chronic {
                social.loneliness_mut().add_chronic_delta(amount);
            } else {
                social.add_loneliness_delta(amount);
            }
        }
        SocialCognitionPath::PerceivedReciprocalCaring => {
            if chronic {
                social
                    .perceived_reciprocal_caring_mut()
                    .add_chronic_delta(amount);
            } else {
                social.add_perceived_reciprocal_caring_delta(amount);
            }
        }
        SocialCognitionPath::PerceivedLiability => {
            if chronic {
                social.perceived_liability_mut().add_chronic_delta(amount);
            } else {
                social.add_perceived_liability_delta(amount);
            }
        }
        SocialCognitionPath::SelfHate => {
            if chronic {
                social.self_hate_mut().add_chronic_delta(amount);
            } else {
                social.add_self_hate_delta(amount);
            }
        }
        SocialCognitionPath::PerceivedCompetence => {
            if chronic {
                social
                    .perceived_competence_mut()
                    .add_chronic_delta(amount);
            } else {
                social.add_perceived_competence_delta(amount);
            }
        }
    }
}

/// Applies decay reversal to all reversible dimensions of a state.
fn reverse_decay_on_state(state: &mut IndividualState, duration: Duration) {
    // Mood dimensions have 6-hour half-life
    reverse_dimension_decay(state.mood_mut().valence_mut(), duration, Duration::hours(6));
    reverse_dimension_decay(state.mood_mut().arousal_mut(), duration, Duration::hours(6));
    reverse_dimension_decay(
        state.mood_mut().dominance_mut(),
        duration,
        Duration::hours(6),
    );

    // Social cognition dimensions have various half-lives
    reverse_dimension_decay(
        state.social_cognition_mut().loneliness_mut(),
        duration,
        Duration::days(1),
    );
    reverse_dimension_decay(
        state.social_cognition_mut()
            .perceived_reciprocal_caring_mut(),
        duration,
        Duration::days(2),
    );
    reverse_dimension_decay(
        state.social_cognition_mut().perceived_liability_mut(),
        duration,
        Duration::days(3),
    );
    reverse_dimension_decay(
        state.social_cognition_mut().self_hate_mut(),
        duration,
        Duration::days(3),
    );
    reverse_dimension_decay(
        state.needs_mut().stress_mut(),
        duration,
        Duration::hours(12),
    );
    reverse_dimension_decay(
        state.needs_mut().fatigue_mut(),
        duration,
        Duration::hours(8),
    );
    reverse_dimension_decay(state.needs_mut().purpose_mut(), duration, Duration::days(3));

    // Mental health dimensions (except AC which has no decay)
    reverse_dimension_decay(
        state.mental_health_mut().depression_mut(),
        duration,
        Duration::weeks(2),
    );
    reverse_dimension_decay(
        state.mental_health_mut().interpersonal_hopelessness_mut(),
        duration,
        Duration::weeks(2),
    );
    // Acquired Capability has no decay - cannot be reversed

    // Disposition dimensions
    reverse_dimension_decay(
        state.disposition_mut().empathy_mut(),
        duration,
        Duration::weeks(4),
    );
    reverse_dimension_decay(
        state.disposition_mut().aggression_mut(),
        duration,
        Duration::weeks(1),
    );
    reverse_dimension_decay(
        state.disposition_mut().grievance_mut(),
        duration,
        Duration::weeks(1),
    );

    // Person characteristics - social capital
    reverse_dimension_decay(
        state.person_characteristics_mut().social_capital_mut(),
        duration,
        Duration::weeks(4),
    );
}

/// Reverses decay on a single state value.
fn reverse_dimension_decay(
    state_value: &mut crate::state::StateValue,
    duration: Duration,
    half_life: Duration,
) {
    let current_delta = state_value.delta();

    // Skip if delta is effectively zero
    if current_delta.abs() < f32::EPSILON {
        return;
    }

    // Compute reversal factor: exp(ln(2) * t / half_life)
    let elapsed_ms = duration.as_millis() as f64;
    let half_life_ms = half_life.as_millis() as f64;

    if half_life_ms <= 0.0 {
        return;
    }

    let ln2 = std::f64::consts::LN_2;
    let exponent = ln2 * elapsed_ms / half_life_ms;

    // Guard against overflow
    if exponent > 700.0 {
        return;
    }

    let reversal_factor = exponent.exp();
    let original_delta = (current_delta as f64) * reversal_factor;

    // Clamp to reasonable range to avoid numerical issues
    let clamped = original_delta.clamp(-100.0, 100.0) as f32;
    state_value.set_delta(clamped);
}

/// Applies an event's effects to state, returning a new state.
///
/// This is a pure function that interprets the event and applies
/// the resulting deltas to the state.
///
/// # Arguments
///
/// * `state` - The current state
/// * `event` - The event to apply
///
/// # Returns
///
/// A new `IndividualState` with event effects applied.
///
/// # Examples
///
/// ```ignore
/// use behavioral_pathways::processor::apply_event_to_state;
/// use behavioral_pathways::state::IndividualState;
/// use behavioral_pathways::event::EventBuilder;
/// use behavioral_pathways::enums::EventType;
///
/// let state = IndividualState::new();
/// let event = EventBuilder::new(EventType::SocialExclusion)
///     .severity(0.7)
///     .build()
///     .unwrap();
///
/// let new_state = apply_event_to_state(state, &event);
/// ```
#[cfg(test)]
#[must_use]
pub(crate) fn apply_event_to_state(state: IndividualState, event: &Event) -> IndividualState {
    use crate::enums::{EventCategory, EventTag, EventType};

    let mut new_state = state;
    let severity = event.severity() as f32;
    let category = event.category();
    let event_type = event.event_type();
    let chronic = event.has_tag(EventTag::ChronicPattern);

    // Apply base impacts based on event type and category
    match category {
        EventCategory::SocialBelonging => {
            // SocialBelonging only contains SocialExclusion and SocialInclusion
            if matches!(event_type, EventType::SocialExclusion) {
                new_state.mood_mut().add_valence_delta(-0.3 * severity);
                apply_social_cognition_delta(
                    new_state.social_cognition_mut(),
                    SocialCognitionPath::Loneliness,
                    0.2 * severity,
                    chronic,
                );
                apply_social_cognition_delta(
                    new_state.social_cognition_mut(),
                    SocialCognitionPath::PerceivedReciprocalCaring,
                    -0.1 * severity,
                    chronic,
                );
            } else {
                // SocialInclusion
                new_state.mood_mut().add_valence_delta(0.3 * severity);
                apply_social_cognition_delta(
                    new_state.social_cognition_mut(),
                    SocialCognitionPath::Loneliness,
                    -0.2 * severity,
                    chronic,
                );
                apply_social_cognition_delta(
                    new_state.social_cognition_mut(),
                    SocialCognitionPath::PerceivedReciprocalCaring,
                    0.1 * severity,
                    chronic,
                );
            }
        }
        EventCategory::BurdenPerception => {
            new_state.mood_mut().add_valence_delta(-0.3 * severity);
            apply_social_cognition_delta(
                new_state.social_cognition_mut(),
                SocialCognitionPath::PerceivedLiability,
                0.25 * severity,
                chronic,
            );
            apply_social_cognition_delta(
                new_state.social_cognition_mut(),
                SocialCognitionPath::SelfHate,
                0.05 * severity,
                chronic,
            );
        }
        EventCategory::Trauma => {
            new_state.mood_mut().add_valence_delta(-0.3 * severity);
            new_state.mood_mut().add_arousal_delta(0.4 * severity);
            new_state
                .mental_health_mut()
                .add_acquired_capability_delta(0.15 * severity);
        }
        EventCategory::Control => {
            // Control only contains Humiliation and Empowerment
            if matches!(event_type, EventType::Humiliation) {
                new_state.mood_mut().add_valence_delta(-0.3 * severity);
                new_state.mood_mut().add_dominance_delta(-0.3 * severity);
            } else {
                // Empowerment
                new_state.mood_mut().add_valence_delta(0.3 * severity);
                new_state.mood_mut().add_dominance_delta(0.3 * severity);
            }
        }
        EventCategory::Achievement => {
            // Achievement only contains Achievement and Failure
            if matches!(event_type, EventType::Achievement) {
                new_state.mood_mut().add_valence_delta(0.3 * severity);
                new_state.mood_mut().add_dominance_delta(0.1 * severity);
            } else {
                // Failure
                new_state.mood_mut().add_valence_delta(-0.3 * severity);
                new_state.mood_mut().add_dominance_delta(-0.1 * severity);
            }
        }
        EventCategory::Social | EventCategory::Contextual => {
            // Minimal direct state impact
            new_state.mood_mut().add_arousal_delta(0.1 * severity);
        }
    }

    apply_protective_factors(&mut new_state, event, severity);

    if event.has_tag(EventTag::MoralViolation) {
        new_state.set_recent_moral_violation_flag(1.0);
    }

    new_state
}

#[cfg(test)]
fn apply_protective_factors(state: &mut IndividualState, event: &Event, severity: f32) {
    use crate::enums::{EventPayload, EventTag, EventType, LifeDomain, RealizationType, SupportType};

    let event_type = event.event_type();
    let chronic = event.has_tag(EventTag::ChronicPattern);

    if event_type == EventType::Achievement {
        match event.payload() {
            EventPayload::Achievement { domain, magnitude } => {
                let magnitude = *magnitude as f32;
                let productivity = severity * magnitude;

                if matches!(
                    domain,
                    LifeDomain::Work | LifeDomain::Academic | LifeDomain::Financial
                ) {
                    apply_social_cognition_delta(
                        state.social_cognition_mut(),
                        SocialCognitionPath::PerceivedLiability,
                        -0.12 * productivity,
                        chronic,
                    );
                    apply_social_cognition_delta(
                        state.social_cognition_mut(),
                        SocialCognitionPath::SelfHate,
                        -0.08 * productivity,
                        chronic,
                    );
                    state
                        .mental_health_mut()
                        .add_self_worth_delta(0.05 * productivity);
                }

                if matches!(domain, LifeDomain::Work | LifeDomain::Academic | LifeDomain::Creative) {
                    state.needs_mut().add_purpose_delta(0.08 * productivity);
                }
            }
            _ if event.has_tag(EventTag::Work) => {
                let productivity = severity * 0.6;
                apply_social_cognition_delta(
                    state.social_cognition_mut(),
                    SocialCognitionPath::PerceivedLiability,
                    -0.12 * productivity,
                    chronic,
                );
                apply_social_cognition_delta(
                    state.social_cognition_mut(),
                    SocialCognitionPath::SelfHate,
                    -0.08 * productivity,
                    chronic,
                );
                state
                    .mental_health_mut()
                    .add_self_worth_delta(0.05 * productivity);
                state.needs_mut().add_purpose_delta(0.08 * productivity);
            }
            _ => {}
        }
    }

    if event_type == EventType::SocialInclusion {
        if let EventPayload::SocialInclusion { group_id: Some(_) } = event.payload() {
            apply_social_cognition_delta(
                state.social_cognition_mut(),
                SocialCognitionPath::Loneliness,
                -0.08 * severity,
                chronic,
            );
            apply_social_cognition_delta(
                state.social_cognition_mut(),
                SocialCognitionPath::PerceivedReciprocalCaring,
                0.05 * severity,
                chronic,
            );
        }
    }

    if event_type == EventType::Support {
        // Only Emotional and Companionship support types have protective effects
        // Instrumental and Informational support types have no direct protective effect here
        match event.payload() {
            EventPayload::Support {
                support_type: SupportType::Emotional,
                effectiveness,
            }
            | EventPayload::Support {
                support_type: SupportType::Companionship,
                effectiveness,
            } => {
                let eff = *effectiveness as f32;
                apply_social_cognition_delta(
                    state.social_cognition_mut(),
                    SocialCognitionPath::PerceivedLiability,
                    -0.1 * severity * eff,
                    chronic,
                );
                apply_social_cognition_delta(
                    state.social_cognition_mut(),
                    SocialCognitionPath::SelfHate,
                    -0.08 * severity * eff,
                    chronic,
                );
                state
                    .mental_health_mut()
                    .add_self_worth_delta(0.05 * severity * eff);
            }
            _ => {
                // Other support types or non-Support payloads
            }
        }
    }

    if event_type == EventType::Realization {
        if let EventPayload::Realization {
            realization_type: RealizationType::ExistentialInsight,
        } = event.payload()
        {
            state.needs_mut().add_purpose_delta(0.15 * severity);
            apply_social_cognition_delta(
                state.social_cognition_mut(),
                SocialCognitionPath::PerceivedLiability,
                -0.05 * severity,
                chronic,
            );
            apply_social_cognition_delta(
                state.social_cognition_mut(),
                SocialCognitionPath::SelfHate,
                -0.05 * severity,
                chronic,
            );
            state
                .mental_health_mut()
                .add_self_worth_delta(0.04 * severity);
        }
    }
}

/// Applies an interpreted event's effects to state using the actual deltas.
///
/// This function applies the deltas from an `InterpretedEvent` to an `IndividualState`.
/// It is similar to `apply_interpreted_event` in the event module but works on
/// `IndividualState` directly rather than `Entity`.
///
/// # Arguments
///
/// * `state` - The current state
/// * `interpreted` - The interpreted event with computed deltas
///
/// # Returns
///
/// A new `IndividualState` with event effects applied.
#[must_use]
pub(crate) fn apply_interpreted_event_to_state(
    state: IndividualState,
    interpreted: &crate::processor::InterpretedEvent,
) -> IndividualState {
    use crate::enums::{EventTag, MentalHealthPath, MoodPath, NeedsPath, StatePath};

    let mut new_state = state;
    let chronic = interpreted.event.has_tag(EventTag::ChronicPattern);

    // Apply each delta from the interpreted event
    for (path, delta) in &interpreted.state_deltas {
        let delta_f32 = *delta as f32;

        match path {
            StatePath::Mood(MoodPath::Valence) => {
                new_state.mood_mut().add_valence_delta(delta_f32);
            }
            StatePath::Mood(MoodPath::Arousal) => {
                new_state.mood_mut().add_arousal_delta(delta_f32);
            }
            StatePath::Mood(MoodPath::Dominance) => {
                new_state.mood_mut().add_dominance_delta(delta_f32);
            }
            StatePath::SocialCognition(path) => {
                apply_social_cognition_delta(
                    new_state.social_cognition_mut(),
                    *path,
                    delta_f32,
                    chronic,
                );
            }
            StatePath::Needs(NeedsPath::Purpose) => {
                new_state.needs_mut().add_purpose_delta(delta_f32);
            }
            StatePath::MentalHealth(MentalHealthPath::AcquiredCapability) => {
                new_state
                    .mental_health_mut()
                    .add_acquired_capability_delta(delta_f32);
            }
            StatePath::MentalHealth(MentalHealthPath::InterpersonalHopelessness) => {
                new_state
                    .mental_health_mut()
                    .add_interpersonal_hopelessness_delta(delta_f32);
            }
            // Other paths are not typically in interpreted events
            _ => {}
        }
    }

    if interpreted.event.has_tag(EventTag::MoralViolation) {
        new_state.set_recent_moral_violation_flag(1.0);
    }

    new_state
}

/// Reverses an interpreted event's effects from state using the actual deltas.
///
/// This function uses the deltas from an `InterpretedEvent` to precisely reverse
/// the event's effects. This is more accurate than `reverse_event_from_state`
/// because it uses the personality-modulated deltas that were actually applied.
///
/// Note: Acquired Capability increases are NOT reversed (permanent per ITS theory).
///
/// # Arguments
///
/// * `state` - The current state
/// * `interpreted` - The interpreted event with computed deltas
///
/// # Returns
///
/// A new `IndividualState` with event effects reversed.
#[must_use]
pub(crate) fn reverse_interpreted_event_from_state(
    state: IndividualState,
    interpreted: &crate::processor::InterpretedEvent,
) -> IndividualState {
    use crate::enums::{EventTag, MentalHealthPath, MoodPath, NeedsPath, StatePath};

    let mut new_state = state;
    let chronic = interpreted.event.has_tag(EventTag::ChronicPattern);

    // Reverse each delta from the interpreted event
    for (path, delta) in &interpreted.state_deltas {
        let neg_delta = -(*delta as f32);

        match path {
            StatePath::Mood(MoodPath::Valence) => {
                new_state.mood_mut().add_valence_delta(neg_delta);
            }
            StatePath::Mood(MoodPath::Arousal) => {
                new_state.mood_mut().add_arousal_delta(neg_delta);
            }
            StatePath::Mood(MoodPath::Dominance) => {
                new_state.mood_mut().add_dominance_delta(neg_delta);
            }
            StatePath::SocialCognition(path) => {
                apply_social_cognition_delta(
                    new_state.social_cognition_mut(),
                    *path,
                    neg_delta,
                    chronic,
                );
            }
            StatePath::Needs(NeedsPath::Purpose) => {
                new_state.needs_mut().add_purpose_delta(neg_delta);
            }
            StatePath::MentalHealth(MentalHealthPath::AcquiredCapability) => {
                // AC is NOT reversed - it's permanent per ITS theory
                // Skip this delta intentionally
            }
            StatePath::MentalHealth(MentalHealthPath::InterpersonalHopelessness) => {
                new_state
                    .mental_health_mut()
                    .add_interpersonal_hopelessness_delta(neg_delta);
            }
            // Other paths are not typically in interpreted events
            _ => {}
        }
    }

    new_state
}

/// Reverses an event's effects from state, returning a new state.
///
/// This is the inverse of `apply_event_to_state`. It subtracts the
/// deltas that would have been added by the event.
///
/// Note: Some event effects cannot be perfectly reversed (e.g., Acquired
/// Capability increases are permanent). In those cases, the reversal
/// is approximate.
///
/// # Deprecated
///
/// This function uses hardcoded delta values that do not account for
/// personality modulation. For accurate event reversal, use
/// [`reverse_interpreted_event_from_state`] with an `InterpretedEvent`
/// that contains the actual deltas computed during event interpretation.
///
/// # Arguments
///
/// * `state` - The current state
/// * `event` - The event to reverse
///
/// # Returns
///
/// A new `IndividualState` with event effects reversed (approximately).
#[cfg(test)]
fn reverse_event_from_state(state: IndividualState, event: &Event) -> IndividualState {
    use crate::enums::{EventCategory, EventTag, EventType};

    let mut new_state = state;
    let severity = event.severity() as f32;
    let category = event.category();
    let event_type = event.event_type();
    let chronic = event.has_tag(EventTag::ChronicPattern);

    // Reverse impacts by subtracting what was added
    match category {
        EventCategory::SocialBelonging => {
            // SocialBelonging only contains SocialExclusion and SocialInclusion
            if matches!(event_type, EventType::SocialExclusion) {
                new_state.mood_mut().add_valence_delta(0.3 * severity);
                apply_social_cognition_delta(
                    new_state.social_cognition_mut(),
                    SocialCognitionPath::Loneliness,
                    -0.2 * severity,
                    chronic,
                );
                apply_social_cognition_delta(
                    new_state.social_cognition_mut(),
                    SocialCognitionPath::PerceivedReciprocalCaring,
                    0.1 * severity,
                    chronic,
                );
            } else {
                // SocialInclusion
                new_state.mood_mut().add_valence_delta(-0.3 * severity);
                apply_social_cognition_delta(
                    new_state.social_cognition_mut(),
                    SocialCognitionPath::Loneliness,
                    0.2 * severity,
                    chronic,
                );
                apply_social_cognition_delta(
                    new_state.social_cognition_mut(),
                    SocialCognitionPath::PerceivedReciprocalCaring,
                    -0.1 * severity,
                    chronic,
                );
            }
        }
        EventCategory::BurdenPerception => {
            new_state.mood_mut().add_valence_delta(0.3 * severity);
            apply_social_cognition_delta(
                new_state.social_cognition_mut(),
                SocialCognitionPath::PerceivedLiability,
                -0.25 * severity,
                chronic,
            );
            apply_social_cognition_delta(
                new_state.social_cognition_mut(),
                SocialCognitionPath::SelfHate,
                -0.05 * severity,
                chronic,
            );
        }
        EventCategory::Trauma => {
            new_state.mood_mut().add_valence_delta(0.3 * severity);
            new_state.mood_mut().add_arousal_delta(-0.4 * severity);
            // AC cannot be reversed - it's permanent
            // We don't subtract the AC delta
        }
        EventCategory::Control => {
            // Control only contains Humiliation and Empowerment
            if matches!(event_type, EventType::Humiliation) {
                new_state.mood_mut().add_valence_delta(0.3 * severity);
                new_state.mood_mut().add_dominance_delta(0.3 * severity);
            } else {
                // Empowerment
                new_state.mood_mut().add_valence_delta(-0.3 * severity);
                new_state.mood_mut().add_dominance_delta(-0.3 * severity);
            }
        }
        EventCategory::Achievement => {
            // Achievement only contains Achievement and Failure
            if matches!(event_type, EventType::Achievement) {
                new_state.mood_mut().add_valence_delta(-0.3 * severity);
                new_state.mood_mut().add_dominance_delta(-0.1 * severity);
            } else {
                // Failure
                new_state.mood_mut().add_valence_delta(0.3 * severity);
                new_state.mood_mut().add_dominance_delta(0.1 * severity);
            }
        }
        EventCategory::Social | EventCategory::Contextual => {
            new_state.mood_mut().add_arousal_delta(-0.1 * severity);
        }
    }

    apply_protective_factors(&mut new_state, event, -severity);

    new_state
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::{EventPayload, EventTag, EventType, LifeDomain, RealizationType, SupportType};
    use crate::event::EventBuilder;
    use crate::types::GroupId;

    #[test]
    fn advance_state_applies_decay() {
        let mut state = IndividualState::new();
        state.mood_mut().add_valence_delta(0.8);

        let advanced = advance_state(state, Duration::weeks(1));

        // After 1 week with 6-hour half-life, delta should be nearly zero
        assert!(advanced.mood().valence_delta() < 0.01);
    }

    #[test]
    fn advance_state_zero_duration_unchanged() {
        let mut state = IndividualState::new();
        state.mood_mut().add_valence_delta(0.5);

        let advanced = advance_state(state, Duration::zero());

        assert!((advanced.mood().valence_delta() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn regress_state_reverses_decay() {
        let mut state = IndividualState::new();
        // Set a small delta that would result from decay
        state.mood_mut().add_valence_delta(0.25); // Half of 0.5 after one half-life

        let regressed = regress_state(state, Duration::hours(6));

        // After reversing 6 hours (one half-life), delta should approximately double
        assert!(regressed.mood().valence_delta() > 0.4);
    }

    #[test]
    fn regress_state_zero_duration_unchanged() {
        let mut state = IndividualState::new();
        state.mood_mut().add_valence_delta(0.5);

        let regressed = regress_state(state, Duration::zero());

        assert!((regressed.mood().valence_delta() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn apply_event_to_state_social_exclusion() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::SocialExclusion)
            .severity(0.7)
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        // Social exclusion should decrease valence
        assert!(new_state.mood().valence_delta() < 0.0);
        // And increase loneliness
        assert!(new_state.social_cognition().loneliness().delta() > 0.0);
    }

    #[test]
    fn apply_event_to_state_social_inclusion() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::SocialInclusion)
            .severity(0.7)
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        // Social inclusion should increase valence
        assert!(new_state.mood().valence_delta() > 0.0);
        // And decrease loneliness
        assert!(new_state.social_cognition().loneliness().delta() < 0.0);
    }

    #[test]
    fn chronic_pattern_decay_is_slower_for_tb_pb() {
        let acute_event = EventBuilder::new(EventType::SocialExclusion)
            .severity(1.0)
            .build()
            .unwrap();
        let chronic_event = EventBuilder::new(EventType::SocialExclusion)
            .severity(1.0)
            .tag(EventTag::ChronicPattern)
            .build()
            .unwrap();

        let mut acute_state = apply_event_to_state(IndividualState::new(), &acute_event);
        let mut chronic_state = apply_event_to_state(IndividualState::new(), &chronic_event);

        acute_state.apply_decay(Duration::days(1));
        chronic_state.apply_decay(Duration::days(1));

        let acute_delta = acute_state.social_cognition().loneliness().delta();
        let chronic_delta = chronic_state.social_cognition().loneliness().delta();

        assert!(chronic_delta > acute_delta);
    }

    #[test]
    fn apply_event_to_state_employment_productivity_reduces_pb() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::Achievement)
            .severity(0.8)
            .payload(EventPayload::Achievement {
                domain: LifeDomain::Work,
                magnitude: 0.9,
            })
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        assert!(new_state
            .social_cognition()
            .perceived_liability()
            .delta()
            < 0.0);
        assert!(new_state.social_cognition().self_hate().delta() < 0.0);
    }

    #[test]
    fn apply_event_to_state_group_participation_reduces_tb() {
        let state = IndividualState::new();
        let group_id = GroupId::new("group_alpha").unwrap();
        let event = EventBuilder::new(EventType::SocialInclusion)
            .severity(1.0)
            .payload(EventPayload::SocialInclusion {
                group_id: Some(group_id),
            })
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        assert!(new_state.social_cognition().loneliness().delta() < -0.2);
        assert!(new_state
            .social_cognition()
            .perceived_reciprocal_caring()
            .delta()
            > 0.1);
    }

    #[test]
    fn apply_event_to_state_recognition_reduces_pb() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::Support)
            .severity(0.8)
            .payload(EventPayload::Support {
                support_type: SupportType::Emotional,
                effectiveness: 0.9,
            })
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        assert!(new_state
            .social_cognition()
            .perceived_liability()
            .delta()
            < 0.0);
        assert!(new_state.social_cognition().self_hate().delta() < 0.0);
    }

    #[test]
    fn apply_event_to_state_trauma_increases_ac() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::Violence)
            .severity(0.9)
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        // Trauma should increase acquired capability
        assert!(new_state.mental_health().acquired_capability().delta() > 0.0);
    }

    #[test]
    fn apply_event_to_state_humiliation() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::Humiliation)
            .severity(0.8)
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        // Humiliation should decrease dominance
        assert!(new_state.mood().dominance_delta() < 0.0);
    }

    #[test]
    fn apply_event_to_state_empowerment() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::Empowerment)
            .severity(0.8)
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        // Empowerment should increase dominance
        assert!(new_state.mood().dominance_delta() > 0.0);
    }

    #[test]
    fn apply_event_to_state_achievement() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::Achievement)
            .severity(0.7)
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        // Achievement should increase valence
        assert!(new_state.mood().valence_delta() > 0.0);
    }

    #[test]
    fn apply_event_to_state_failure() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::Failure)
            .severity(0.6)
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        // Failure should decrease valence
        assert!(new_state.mood().valence_delta() < 0.0);
    }

    #[test]
    fn apply_event_to_state_burden_feedback() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::BurdenFeedback)
            .severity(0.8)
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        // Burden feedback should increase perceived liability
        assert!(
            new_state
                .social_cognition()
                .perceived_liability()
                .delta()
                > 0.0
        );
    }

    #[test]
    fn apply_event_to_state_employment_reduces_burdensomeness() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::Achievement)
            .severity(0.8)
            .payload(EventPayload::Achievement {
                domain: LifeDomain::Work,
                magnitude: 0.7,
            })
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        assert!(new_state.social_cognition().perceived_liability().delta() < 0.0);
        assert!(new_state.social_cognition().self_hate().delta() < 0.0);
        assert!(new_state.mental_health().self_worth().delta() > 0.0);
    }

    #[test]
    fn apply_event_to_state_group_participation_reduces_loneliness() {
        let state = IndividualState::new();
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

        let grouped_state = apply_event_to_state(state.clone(), &grouped);
        let solo_state = apply_event_to_state(state, &solo);

        assert!(grouped_state.social_cognition().loneliness().delta()
            < solo_state.social_cognition().loneliness().delta());
        assert!(grouped_state.social_cognition().perceived_reciprocal_caring().delta()
            > solo_state.social_cognition().perceived_reciprocal_caring().delta());
    }

    #[test]
    fn apply_event_to_state_recognition_reduces_burdensomeness() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::Support)
            .severity(0.6)
            .payload(EventPayload::Support {
                support_type: SupportType::Emotional,
                effectiveness: 0.8,
            })
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        assert!(new_state.social_cognition().perceived_liability().delta() < 0.0);
        assert!(new_state.social_cognition().self_hate().delta() < 0.0);
        assert!(new_state.mental_health().self_worth().delta() > 0.0);
    }

    #[test]
    fn apply_event_to_state_purpose_development_increases_purpose() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::Realization)
            .severity(0.7)
            .payload(EventPayload::Realization {
                realization_type: RealizationType::ExistentialInsight,
            })
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        assert!(new_state.needs().purpose().delta() > 0.0);
        assert!(new_state.social_cognition().perceived_liability().delta() < 0.0);
        assert!(new_state.social_cognition().self_hate().delta() < 0.0);
    }

    #[test]
    fn apply_event_to_state_moral_violation_sets_flag() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::Conflict)
            .severity(0.6)
            .tag(EventTag::MoralViolation)
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        assert!((new_state.recent_moral_violation_flag() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    #[allow(deprecated)]
    fn reverse_event_from_state_social_exclusion() {
        let mut state = IndividualState::new();
        // Simulate state after exclusion
        state.mood_mut().add_valence_delta(-0.21); // 0.7 * -0.3
        state
            .social_cognition_mut()
            .add_loneliness_delta(0.14); // 0.7 * 0.2

        let event = EventBuilder::new(EventType::SocialExclusion)
            .severity(0.7)
            .build()
            .unwrap();

        let reversed = reverse_event_from_state(state, &event);

        // Valence should be back to near zero
        assert!(reversed.mood().valence_delta().abs() < 0.01);
        // Loneliness should be back to near zero
        assert!(reversed.social_cognition().loneliness().delta().abs() < 0.01);
    }

    #[test]
    #[allow(deprecated)]
    fn reverse_event_from_state_trauma_keeps_ac() {
        let mut state = IndividualState::new();
        // Simulate state after trauma
        state
            .mental_health_mut()
            .add_acquired_capability_delta(0.135); // 0.9 * 0.15

        let event = EventBuilder::new(EventType::Violence)
            .severity(0.9)
            .build()
            .unwrap();

        let reversed = reverse_event_from_state(state, &event);

        // AC should NOT be reversed (it's permanent)
        assert!(reversed.mental_health().acquired_capability().delta() > 0.13);
    }

    #[test]
    #[allow(deprecated)]
    fn apply_then_reverse_restores_state() {
        let mut state = IndividualState::new();
        state.mood_mut().add_valence_delta(0.2);
        let original_valence = state.mood().valence_delta();

        let event = EventBuilder::new(EventType::Achievement)
            .severity(0.5)
            .build()
            .unwrap();

        let applied = apply_event_to_state(state, &event);
        let reversed = reverse_event_from_state(applied, &event);

        assert!((reversed.mood().valence_delta() - original_valence).abs() < 0.01);
    }

    #[test]
    fn advance_state_is_pure() {
        let mut state = IndividualState::new();
        state.mood_mut().add_valence_delta(0.5);

        let original_delta = state.mood().valence_delta();
        let _ = advance_state(state.clone(), Duration::hours(6));

        // Original should be unchanged (it was cloned)
        assert!((state.mood().valence_delta() - original_delta).abs() < f32::EPSILON);
    }

    #[test]
    fn regress_state_is_pure() {
        let mut state = IndividualState::new();
        state.mood_mut().add_valence_delta(0.5);

        let original_delta = state.mood().valence_delta();
        let _ = regress_state(state.clone(), Duration::hours(6));

        // Original should be unchanged (it was cloned)
        assert!((state.mood().valence_delta() - original_delta).abs() < f32::EPSILON);
    }

    #[test]
    fn apply_event_is_pure() {
        let mut state = IndividualState::new();
        state.mood_mut().add_valence_delta(0.0);

        let event = EventBuilder::new(EventType::Achievement)
            .severity(0.5)
            .build()
            .unwrap();

        let original_delta = state.mood().valence_delta();
        let _ = apply_event_to_state(state.clone(), &event);

        // Original should be unchanged (it was cloned)
        assert!((state.mood().valence_delta() - original_delta).abs() < f32::EPSILON);
    }

    #[test]
    #[allow(deprecated)]
    fn reverse_event_is_pure() {
        let mut state = IndividualState::new();
        state.mood_mut().add_valence_delta(0.2);

        let event = EventBuilder::new(EventType::Achievement)
            .severity(0.5)
            .build()
            .unwrap();

        let original_delta = state.mood().valence_delta();
        let _ = reverse_event_from_state(state.clone(), &event);

        // Original should be unchanged (it was cloned)
        assert!((state.mood().valence_delta() - original_delta).abs() < f32::EPSILON);
    }

    #[test]
    fn regress_state_handles_zero_delta() {
        let state = IndividualState::new();
        // All deltas are zero

        let regressed = regress_state(state, Duration::hours(6));

        // Should still be zero
        assert!(regressed.mood().valence_delta().abs() < f32::EPSILON);
    }

    #[test]
    fn apply_event_contextual_event() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::PolicyChange)
            .severity(0.5)
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        // Contextual events should increase arousal minimally
        assert!(new_state.mood().arousal_delta() > 0.0);
    }

    #[test]
    fn apply_event_social_event() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::Interaction)
            .severity(0.5)
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        // Social events should increase arousal minimally
        assert!(new_state.mood().arousal_delta() > 0.0);
    }

    #[test]
    #[allow(deprecated)]
    fn reverse_event_social_inclusion() {
        let mut state = IndividualState::new();
        // Simulate state after inclusion
        state.mood_mut().add_valence_delta(0.21); // 0.7 * 0.3
        state
            .social_cognition_mut()
            .add_loneliness_delta(-0.14); // 0.7 * -0.2

        let event = EventBuilder::new(EventType::SocialInclusion)
            .severity(0.7)
            .build()
            .unwrap();

        let reversed = reverse_event_from_state(state, &event);

        // Valence should be back to near zero
        assert!(reversed.mood().valence_delta().abs() < 0.01);
        // Loneliness should be back to near zero
        assert!(reversed.social_cognition().loneliness().delta().abs() < 0.01);
    }

    #[test]
    #[allow(deprecated)]
    fn reverse_event_humiliation() {
        let mut state = IndividualState::new();
        state.mood_mut().add_valence_delta(-0.24); // 0.8 * -0.3
        state.mood_mut().add_dominance_delta(-0.24); // 0.8 * -0.3

        let event = EventBuilder::new(EventType::Humiliation)
            .severity(0.8)
            .build()
            .unwrap();

        let reversed = reverse_event_from_state(state, &event);

        assert!(reversed.mood().valence_delta().abs() < 0.01);
        assert!(reversed.mood().dominance_delta().abs() < 0.01);
    }

    #[test]
    #[allow(deprecated)]
    fn reverse_event_empowerment() {
        let mut state = IndividualState::new();
        state.mood_mut().add_valence_delta(0.24); // 0.8 * 0.3
        state.mood_mut().add_dominance_delta(0.24); // 0.8 * 0.3

        let event = EventBuilder::new(EventType::Empowerment)
            .severity(0.8)
            .build()
            .unwrap();

        let reversed = reverse_event_from_state(state, &event);

        assert!(reversed.mood().valence_delta().abs() < 0.01);
        assert!(reversed.mood().dominance_delta().abs() < 0.01);
    }

    #[test]
    #[allow(deprecated)]
    fn reverse_event_failure() {
        let mut state = IndividualState::new();
        state.mood_mut().add_valence_delta(-0.18); // 0.6 * -0.3
        state.mood_mut().add_dominance_delta(-0.06); // 0.6 * -0.1

        let event = EventBuilder::new(EventType::Failure)
            .severity(0.6)
            .build()
            .unwrap();

        let reversed = reverse_event_from_state(state, &event);

        assert!(reversed.mood().valence_delta().abs() < 0.01);
        assert!(reversed.mood().dominance_delta().abs() < 0.01);
    }

    #[test]
    #[allow(deprecated)]
    fn reverse_event_burden_feedback() {
        let mut state = IndividualState::new();
        state.mood_mut().add_valence_delta(-0.24); // 0.8 * -0.3

        let event = EventBuilder::new(EventType::BurdenFeedback)
            .severity(0.8)
            .build()
            .unwrap();

        let reversed = reverse_event_from_state(state, &event);

        assert!(reversed.mood().valence_delta().abs() < 0.01);
    }

    #[test]
    #[allow(deprecated)]
    fn reverse_event_contextual() {
        let mut state = IndividualState::new();
        state.mood_mut().add_arousal_delta(0.05); // 0.5 * 0.1

        let event = EventBuilder::new(EventType::PolicyChange)
            .severity(0.5)
            .build()
            .unwrap();

        let reversed = reverse_event_from_state(state, &event);

        assert!(reversed.mood().arousal_delta().abs() < 0.01);
    }

    // Helper to create a test InterpretedEvent with specified state_deltas
    fn make_test_interpreted_event(
        event: crate::event::Event,
        state_deltas: Vec<(crate::enums::StatePath, f64)>,
    ) -> crate::processor::InterpretedEvent {
        crate::processor::InterpretedEvent {
            event,
            original_event: crate::types::EventId::new("test_event").unwrap(),
            attribution: crate::enums::Attribution::Unknown,
            valence_delta: 0.0,
            arousal_delta: 0.0,
            dominance_delta: 0.0,
            loneliness_delta: 0.0,
            prc_delta: 0.0,
            perceived_liability_delta: 0.0,
            self_hate_delta: 0.0,
            acquired_capability_delta: 0.0,
            interpersonal_hopelessness_delta: 0.0,
            salience: 0.5,
            perceived_severity: 0.5,
            memory_salience: 0.5,
            state_deltas,
        }
    }

    #[test]
    fn apply_interpreted_event_to_state_applies_deltas() {
        use crate::enums::{MoodPath, StatePath};

        let state = IndividualState::new();

        // Create a mock interpreted event with known deltas
        let event = EventBuilder::new(EventType::SocialExclusion)
            .severity(0.8)
            .build()
            .unwrap();

        let interpreted = make_test_interpreted_event(
            event,
            vec![
                (StatePath::Mood(MoodPath::Valence), -0.24),
                (StatePath::Mood(MoodPath::Arousal), 0.1),
            ],
        );

        let new_state = apply_interpreted_event_to_state(state, &interpreted);

        assert!((new_state.mood().valence_delta() - (-0.24)).abs() < 0.001);
        assert!((new_state.mood().arousal_delta() - 0.1).abs() < 0.001);
    }

    #[test]
    fn chronic_event_slows_tb_pb_decay() {
        use crate::enums::{EventTag, SocialCognitionPath, StatePath};

        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::SocialExclusion)
            .severity(0.8)
            .tag(EventTag::ChronicPattern)
            .build()
            .unwrap();

        let interpreted = make_test_interpreted_event(
            event,
            vec![(
                StatePath::SocialCognition(SocialCognitionPath::Loneliness),
                0.2,
            )],
        );

        let mut new_state = apply_interpreted_event_to_state(state, &interpreted);
        let original_delta = new_state.social_cognition().loneliness().delta();

        new_state.apply_decay(SocialCognition::LONELINESS_DECAY_HALF_LIFE);

        let decayed_delta = new_state.social_cognition().loneliness().delta();
        assert!(decayed_delta > original_delta * 0.5);
    }

    #[test]
    fn reverse_interpreted_event_from_state_reverses_deltas() {
        use crate::enums::{MoodPath, StatePath};

        let mut state = IndividualState::new();
        // Simulate state after event was applied
        state.mood_mut().add_valence_delta(-0.24);
        state.mood_mut().add_arousal_delta(0.1);

        let event = EventBuilder::new(EventType::SocialExclusion)
            .severity(0.8)
            .build()
            .unwrap();

        let interpreted = make_test_interpreted_event(
            event,
            vec![
                (StatePath::Mood(MoodPath::Valence), -0.24),
                (StatePath::Mood(MoodPath::Arousal), 0.1),
            ],
        );

        let reversed = reverse_interpreted_event_from_state(state, &interpreted);

        // Deltas should be reversed (back to near zero)
        assert!(reversed.mood().valence_delta().abs() < 0.001);
        assert!(reversed.mood().arousal_delta().abs() < 0.001);
    }

    #[test]
    fn reverse_interpreted_event_does_not_reverse_ac() {
        use crate::enums::{MentalHealthPath, StatePath};

        let mut state = IndividualState::new();
        // Simulate state after trauma with AC increase
        state
            .mental_health_mut()
            .add_acquired_capability_delta(0.15);

        let event = EventBuilder::new(EventType::Violence)
            .severity(0.9)
            .build()
            .unwrap();

        let interpreted = make_test_interpreted_event(
            event,
            vec![(
                StatePath::MentalHealth(MentalHealthPath::AcquiredCapability),
                0.15,
            )],
        );

        let reversed = reverse_interpreted_event_from_state(state, &interpreted);

        // AC should NOT be reversed (permanent per ITS theory)
        assert!(reversed.mental_health().acquired_capability().delta() > 0.14);
    }

    #[test]
    fn apply_then_reverse_interpreted_restores_state() {
        use crate::enums::{MoodPath, SocialCognitionPath, StatePath};

        let mut state = IndividualState::new();
        state.mood_mut().add_valence_delta(0.2);
        let original_valence = state.mood().valence_delta();

        let event = EventBuilder::new(EventType::SocialInclusion)
            .severity(0.6)
            .build()
            .unwrap();

        let interpreted = make_test_interpreted_event(
            event,
            vec![
                (StatePath::Mood(MoodPath::Valence), 0.18),
                (
                    StatePath::SocialCognition(SocialCognitionPath::Loneliness),
                    -0.12,
                ),
            ],
        );

        let applied = apply_interpreted_event_to_state(state, &interpreted);
        let reversed = reverse_interpreted_event_from_state(applied, &interpreted);

        // Original valence should be restored
        assert!((reversed.mood().valence_delta() - original_valence).abs() < 0.001);
        // Loneliness should be back to zero
        assert!(reversed.social_cognition().loneliness().delta().abs() < 0.001);
    }

    #[test]
    fn apply_interpreted_event_to_state_handles_all_paths() {
        use crate::enums::{MentalHealthPath, MoodPath, SocialCognitionPath, StatePath};

        let state = IndividualState::new();

        let event = EventBuilder::new(EventType::SocialExclusion)
            .severity(0.8)
            .build()
            .unwrap();

        let interpreted = make_test_interpreted_event(
            event,
            vec![
                (StatePath::Mood(MoodPath::Valence), -0.2),
                (StatePath::Mood(MoodPath::Arousal), 0.15),
                (StatePath::Mood(MoodPath::Dominance), -0.1),
                (
                    StatePath::SocialCognition(SocialCognitionPath::Loneliness),
                    0.25,
                ),
                (
                    StatePath::SocialCognition(SocialCognitionPath::PerceivedReciprocalCaring),
                    -0.1,
                ),
                (
                    StatePath::SocialCognition(SocialCognitionPath::PerceivedLiability),
                    0.05,
                ),
                (StatePath::SocialCognition(SocialCognitionPath::SelfHate), 0.05),
                (
                    StatePath::MentalHealth(MentalHealthPath::AcquiredCapability),
                    0.1,
                ),
                (
                    StatePath::MentalHealth(MentalHealthPath::InterpersonalHopelessness),
                    0.08,
                ),
            ],
        );

        let new_state = apply_interpreted_event_to_state(state, &interpreted);

        // All paths should have their deltas applied
        assert!((new_state.mood().valence_delta() - (-0.2)).abs() < 0.001);
        assert!((new_state.mood().arousal_delta() - 0.15).abs() < 0.001);
        assert!((new_state.mood().dominance_delta() - (-0.1)).abs() < 0.001);
        assert!(
            (new_state.social_cognition().loneliness().delta() - 0.25).abs() < 0.001
        );
        assert!(
            (new_state
                .social_cognition()
                .perceived_reciprocal_caring()
                .delta()
                - (-0.1))
                .abs()
                < 0.001
        );
        assert!(
            (new_state.social_cognition().perceived_liability().delta() - 0.05).abs() < 0.001
        );
        assert!((new_state.social_cognition().self_hate().delta() - 0.05).abs() < 0.001);
        assert!((new_state.mental_health().acquired_capability().delta() - 0.1).abs() < 0.001);
        assert!(
            (new_state
                .mental_health()
                .interpersonal_hopelessness()
                .delta()
                - 0.08)
                .abs()
                < 0.001
        );
    }

    #[test]
    fn reverse_interpreted_event_handles_all_paths() {
        use crate::enums::{MentalHealthPath, MoodPath, SocialCognitionPath, StatePath};

        let mut state = IndividualState::new();
        // Simulate state after event was applied with all paths
        state.mood_mut().add_valence_delta(-0.2);
        state.mood_mut().add_arousal_delta(0.15);
        state.mood_mut().add_dominance_delta(-0.1);
        state
            .social_cognition_mut()
            .add_loneliness_delta(0.25);
        state
            .social_cognition_mut()
            .add_perceived_reciprocal_caring_delta(-0.1);
        state
            .social_cognition_mut()
            .add_perceived_liability_delta(0.05);
        state
            .social_cognition_mut()
            .add_self_hate_delta(0.05);
        state.mental_health_mut().add_acquired_capability_delta(0.1);
        state
            .mental_health_mut()
            .add_interpersonal_hopelessness_delta(0.08);

        let event = EventBuilder::new(EventType::SocialExclusion)
            .severity(0.8)
            .build()
            .unwrap();

        let interpreted = make_test_interpreted_event(
            event,
            vec![
                (StatePath::Mood(MoodPath::Valence), -0.2),
                (StatePath::Mood(MoodPath::Arousal), 0.15),
                (StatePath::Mood(MoodPath::Dominance), -0.1),
                (
                    StatePath::SocialCognition(SocialCognitionPath::Loneliness),
                    0.25,
                ),
                (
                    StatePath::SocialCognition(SocialCognitionPath::PerceivedReciprocalCaring),
                    -0.1,
                ),
                (
                    StatePath::SocialCognition(SocialCognitionPath::PerceivedLiability),
                    0.05,
                ),
                (StatePath::SocialCognition(SocialCognitionPath::SelfHate), 0.05),
                (
                    StatePath::MentalHealth(MentalHealthPath::AcquiredCapability),
                    0.1,
                ),
                (
                    StatePath::MentalHealth(MentalHealthPath::InterpersonalHopelessness),
                    0.08,
                ),
            ],
        );

        let reversed = reverse_interpreted_event_from_state(state, &interpreted);

        // All paths except AC should be reversed to near zero
        assert!(reversed.mood().valence_delta().abs() < 0.001);
        assert!(reversed.mood().arousal_delta().abs() < 0.001);
        assert!(reversed.mood().dominance_delta().abs() < 0.001);
        assert!(
            reversed
                .social_cognition()
                .loneliness()
                .delta()
                .abs()
                < 0.001
        );
        assert!(
            reversed
                .social_cognition()
                .perceived_reciprocal_caring()
                .delta()
                .abs()
                < 0.001
        );
        assert!(
            reversed
                .social_cognition()
                .perceived_liability()
                .delta()
                .abs()
                < 0.001
        );
        assert!(
            reversed
                .social_cognition()
                .self_hate()
                .delta()
                .abs()
                < 0.001
        );
        // AC should NOT be reversed
        assert!(reversed.mental_health().acquired_capability().delta() > 0.09);
        // Hopelessness should be reversed
        assert!(
            reversed
                .mental_health()
                .interpersonal_hopelessness()
                .delta()
                .abs()
                < 0.001
        );
    }

    #[test]
    fn apply_interpreted_event_ignores_unhandled_paths() {
        use crate::enums::{DispositionPath, StatePath};

        let state = IndividualState::new();

        let event = EventBuilder::new(EventType::SocialExclusion)
            .severity(0.5)
            .build()
            .unwrap();

        // Create interpreted event with a path that falls into the _ => {} branch
        let interpreted = make_test_interpreted_event(
            event,
            vec![
                // Disposition paths are not explicitly handled, should hit _ branch
                (StatePath::Disposition(DispositionPath::ImpulseControl), 0.3),
            ],
        );

        // This should not panic and should leave state unchanged
        let new_state = apply_interpreted_event_to_state(state.clone(), &interpreted);

        // State should be unchanged since Disposition paths are not handled
        assert!(
            (new_state.disposition().impulse_control().delta()
                - state.disposition().impulse_control().delta())
            .abs()
                < 0.001
        );
    }

    #[test]
    fn reverse_interpreted_event_ignores_unhandled_paths() {
        use crate::enums::{DispositionPath, StatePath};

        let state = IndividualState::new();

        let event = EventBuilder::new(EventType::SocialExclusion)
            .severity(0.5)
            .build()
            .unwrap();

        // Create interpreted event with a path that falls into the _ => {} branch
        let interpreted = make_test_interpreted_event(
            event,
            vec![
                // Disposition paths are not explicitly handled, should hit _ branch
                (StatePath::Disposition(DispositionPath::ImpulseControl), 0.3),
            ],
        );

        // This should not panic and should leave state unchanged
        let reversed = reverse_interpreted_event_from_state(state.clone(), &interpreted);

        // State should be unchanged since Disposition paths are not handled
        assert!(
            (reversed.disposition().impulse_control().delta()
                - state.disposition().impulse_control().delta())
            .abs()
                < 0.001
        );
    }

    #[test]
    fn reverse_dimension_decay_handles_zero_half_life() {
        // Test the half_life <= 0 early return (line 198)
        // When half_life is zero, the function should return early without modification.
        use crate::state::StateValue;

        let mut state_value = StateValue::new(0.5).with_delta(0.3);
        let original_delta = state_value.delta();
        let duration = Duration::days(1);
        let half_life = Duration::zero(); // Zero half-life triggers early return

        reverse_dimension_decay(&mut state_value, duration, half_life);

        // Delta should be unchanged because function returned early
        assert!((state_value.delta() - original_delta).abs() < 0.001);
    }

    #[test]
    fn reverse_dimension_decay_handles_extremely_long_duration() {
        // Test the exponent > 700 early return (line 206)
        // This guards against exp() overflow with very long durations.
        // exponent = ln(2) * elapsed_ms / half_life_ms
        // For exponent > 700, we need elapsed_ms / half_life_ms > 1010
        // With 1 day half-life, 1010+ days elapsed triggers this.
        use crate::state::StateValue;

        let mut state_value = StateValue::new(0.5).with_delta(0.3);
        let original_delta = state_value.delta();
        let duration = Duration::years(10); // Very long duration
        let half_life = Duration::days(1); // 1 day half-life

        // exponent = ln(2) * (10*365*24*60*60*1000) / (24*60*60*1000)
        // exponent = ln(2) * 3650 = 2530 which is > 700
        reverse_dimension_decay(&mut state_value, duration, half_life);

        // Delta should be unchanged because function returned early due to overflow guard
        assert!((state_value.delta() - original_delta).abs() < 0.001);
    }

    #[test]
    fn apply_social_cognition_delta_loneliness_acute() {
        let mut state = IndividualState::new();
        apply_social_cognition_delta(
            state.social_cognition_mut(),
            SocialCognitionPath::Loneliness,
            0.5,
            false,
        );
        assert!((state.social_cognition().loneliness().delta() - 0.5).abs() < 0.001);
    }

    #[test]
    fn apply_social_cognition_delta_loneliness_chronic() {
        let mut state = IndividualState::new();
        apply_social_cognition_delta(
            state.social_cognition_mut(),
            SocialCognitionPath::Loneliness,
            0.5,
            true,
        );
        // Chronic delta uses slower decay - just verify it was applied
        assert!(state.social_cognition().loneliness().delta() > 0.0);
    }

    #[test]
    fn apply_social_cognition_delta_perceived_reciprocal_caring_acute() {
        let mut state = IndividualState::new();
        apply_social_cognition_delta(
            state.social_cognition_mut(),
            SocialCognitionPath::PerceivedReciprocalCaring,
            0.3,
            false,
        );
        assert!(
            (state
                .social_cognition()
                .perceived_reciprocal_caring()
                .delta()
                - 0.3)
                .abs()
                < 0.001
        );
    }

    #[test]
    fn apply_social_cognition_delta_perceived_reciprocal_caring_chronic() {
        let mut state = IndividualState::new();
        apply_social_cognition_delta(
            state.social_cognition_mut(),
            SocialCognitionPath::PerceivedReciprocalCaring,
            0.3,
            true,
        );
        assert!(state.social_cognition().perceived_reciprocal_caring().delta() > 0.0);
    }

    #[test]
    fn apply_social_cognition_delta_perceived_liability_acute() {
        let mut state = IndividualState::new();
        apply_social_cognition_delta(
            state.social_cognition_mut(),
            SocialCognitionPath::PerceivedLiability,
            0.4,
            false,
        );
        assert!(
            (state
                .social_cognition()
                .perceived_liability()
                .delta()
                - 0.4)
                .abs()
                < 0.001
        );
    }

    #[test]
    fn apply_social_cognition_delta_perceived_liability_chronic() {
        let mut state = IndividualState::new();
        apply_social_cognition_delta(
            state.social_cognition_mut(),
            SocialCognitionPath::PerceivedLiability,
            0.4,
            true,
        );
        assert!(state.social_cognition().perceived_liability().delta() > 0.0);
    }

    #[test]
    fn apply_social_cognition_delta_self_hate_acute() {
        let mut state = IndividualState::new();
        apply_social_cognition_delta(
            state.social_cognition_mut(),
            SocialCognitionPath::SelfHate,
            0.2,
            false,
        );
        assert!(
            (state.social_cognition().self_hate().delta() - 0.2).abs() < 0.001
        );
    }

    #[test]
    fn apply_social_cognition_delta_self_hate_chronic() {
        let mut state = IndividualState::new();
        apply_social_cognition_delta(
            state.social_cognition_mut(),
            SocialCognitionPath::SelfHate,
            0.2,
            true,
        );
        assert!(state.social_cognition().self_hate().delta() > 0.0);
    }

    #[test]
    fn apply_social_cognition_delta_perceived_competence_acute() {
        let mut state = IndividualState::new();
        apply_social_cognition_delta(
            state.social_cognition_mut(),
            SocialCognitionPath::PerceivedCompetence,
            0.35,
            false,
        );
        assert!(
            (state
                .social_cognition()
                .perceived_competence()
                .delta()
                - 0.35)
                .abs()
                < 0.001
        );
    }

    #[test]
    fn apply_social_cognition_delta_perceived_competence_chronic() {
        let mut state = IndividualState::new();
        apply_social_cognition_delta(
            state.social_cognition_mut(),
            SocialCognitionPath::PerceivedCompetence,
            0.35,
            true,
        );
        assert!(state.social_cognition().perceived_competence().delta() > 0.0);
    }

    #[test]
    fn apply_event_to_state_with_tag_moral_violation() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::Conflict)
            .severity(0.5)
            .tag(EventTag::MoralViolation)
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        assert!((new_state.recent_moral_violation_flag() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn apply_event_achievement_academic_domain_affects_purpose() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::Achievement)
            .severity(0.6)
            .payload(EventPayload::Achievement {
                domain: LifeDomain::Academic,
                magnitude: 0.8,
            })
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        // Academic achievement should increase purpose
        assert!(new_state.needs().purpose().delta() > 0.0);
    }

    #[test]
    fn apply_event_achievement_financial_domain_reduces_pb() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::Achievement)
            .severity(0.7)
            .payload(EventPayload::Achievement {
                domain: LifeDomain::Financial,
                magnitude: 0.85,
            })
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        // Financial achievement should reduce perceived liability
        assert!(new_state.social_cognition().perceived_liability().delta() < 0.0);
    }

    #[test]
    fn apply_event_achievement_creative_domain_increases_purpose() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::Achievement)
            .severity(0.5)
            .payload(EventPayload::Achievement {
                domain: LifeDomain::Creative,
                magnitude: 0.75,
            })
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        // Creative achievement should increase purpose
        assert!(new_state.needs().purpose().delta() > 0.0);
    }

    #[test]
    fn apply_event_achievement_other_domain_no_special_effects() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::Achievement)
            .severity(0.5)
            .payload(EventPayload::Achievement {
                domain: LifeDomain::Social,
                magnitude: 0.7,
            })
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        // Social achievement increases valence but no special purpose/liability effects
        assert!(new_state.mood().valence_delta() > 0.0);
    }

    #[test]
    fn apply_event_with_work_tag_affects_purpose() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::Achievement)
            .severity(0.6)
            .tag(EventTag::Work)
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        // Work tag should increase purpose
        assert!(new_state.needs().purpose().delta() > 0.0);
    }

    #[test]
    fn apply_event_social_inclusion_without_group_no_extra_effect() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::SocialInclusion)
            .severity(0.6)
            .payload(EventPayload::SocialInclusion { group_id: None })
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        // Without group_id, should have only base effects, not extra loneliness reduction
        assert!(new_state.social_cognition().loneliness().delta() < 0.0);
    }

    #[test]
    fn apply_event_support_companionship_reduces_burdensomeness() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::Support)
            .severity(0.7)
            .payload(EventPayload::Support {
                support_type: SupportType::Companionship,
                effectiveness: 0.8,
            })
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        // Companionship support should reduce perceived liability
        assert!(new_state.social_cognition().perceived_liability().delta() < 0.0);
    }

    #[test]
    fn apply_event_support_instrumental_no_extra_effect() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::Support)
            .severity(0.7)
            .payload(EventPayload::Support {
                support_type: SupportType::Instrumental,
                effectiveness: 0.8,
            })
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        // Instrumental support has only base arousal effects (Social category), not extra liability reduction
        assert!(new_state.mood().arousal_delta() > 0.0);
    }

    #[test]
    fn apply_event_support_informational_no_extra_effect() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::Support)
            .severity(0.7)
            .payload(EventPayload::Support {
                support_type: SupportType::Informational,
                effectiveness: 0.8,
            })
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        // Informational support has only base arousal effects (Social category), not extra liability reduction
        assert!(new_state.mood().arousal_delta() > 0.0);
    }

    #[test]
    fn apply_event_realization_non_existential_no_extra_effect() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::Realization)
            .severity(0.6)
            .payload(EventPayload::Realization {
                realization_type: RealizationType::SelfInsight,
            })
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        // Non-existential realization should have only base arousal effect
        assert!(new_state.mood().arousal_delta() > 0.0);
    }

    #[test]
    fn apply_event_existential_insight_increases_purpose() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::Realization)
            .severity(0.8)
            .payload(EventPayload::Realization {
                realization_type: RealizationType::ExistentialInsight,
            })
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        // Existential insight should increase purpose and reduce liability
        assert!(new_state.needs().purpose().delta() > 0.0);
        assert!(new_state.social_cognition().perceived_liability().delta() < 0.0);
    }

    #[test]
    fn reverse_event_failure_restores_state() {
        let mut state = IndividualState::new();
        state.mood_mut().add_valence_delta(-0.18); // 0.6 * -0.3
        state.mood_mut().add_dominance_delta(-0.06); // 0.6 * -0.1

        let event = EventBuilder::new(EventType::Failure)
            .severity(0.6)
            .build()
            .unwrap();

        let reversed = reverse_event_from_state(state, &event);

        // Should restore to near zero
        assert!(reversed.mood().valence_delta().abs() < 0.01);
        assert!(reversed.mood().dominance_delta().abs() < 0.01);
    }

    #[test]
    fn reverse_event_social_event_arousal_reversal() {
        let mut state = IndividualState::new();
        state.mood_mut().add_arousal_delta(0.06); // 0.6 * 0.1

        let event = EventBuilder::new(EventType::Interaction)
            .severity(0.6)
            .build()
            .unwrap();

        let reversed = reverse_event_from_state(state, &event);

        assert!(reversed.mood().arousal_delta().abs() < 0.01);
    }

    #[test]
    fn apply_interpreted_event_needs_purpose_path() {
        use crate::enums::{NeedsPath, StatePath};

        let state = IndividualState::new();

        let event = EventBuilder::new(EventType::Achievement)
            .severity(0.5)
            .build()
            .unwrap();

        let interpreted = make_test_interpreted_event(
            event,
            vec![(StatePath::Needs(NeedsPath::Purpose), 0.15)],
        );

        let new_state = apply_interpreted_event_to_state(state, &interpreted);

        assert!((new_state.needs().purpose().delta() - 0.15).abs() < 0.001);
    }

    #[test]
    fn reverse_interpreted_event_needs_purpose_path() {
        use crate::enums::{NeedsPath, StatePath};

        let mut state = IndividualState::new();
        state.needs_mut().add_purpose_delta(0.15);

        let event = EventBuilder::new(EventType::Realization)
            .severity(0.5)
            .build()
            .unwrap();

        let interpreted = make_test_interpreted_event(
            event,
            vec![(StatePath::Needs(NeedsPath::Purpose), 0.15)],
        );

        let reversed = reverse_interpreted_event_from_state(state, &interpreted);

        // Should be reversed to near zero
        assert!(reversed.needs().purpose().delta().abs() < 0.001);
    }

    #[test]
    fn reverse_dimension_decay_with_negative_delta() {
        use crate::state::StateValue;

        let mut state_value = StateValue::new(0.5).with_delta(-0.3);
        let original_delta = state_value.delta();
        let duration = Duration::hours(6);
        let half_life = Duration::hours(6);

        reverse_dimension_decay(&mut state_value, duration, half_life);

        // Negative delta should double in magnitude (reversal)
        assert!(state_value.delta() < original_delta);
        assert!(state_value.delta().abs() > original_delta.abs() * 0.9);
    }

    #[test]
    fn reverse_dimension_decay_with_extremely_small_delta() {
        use crate::state::StateValue;

        let mut state_value = StateValue::new(0.5).with_delta(1e-10);
        let original_delta = state_value.delta();
        let duration = Duration::hours(6);
        let half_life = Duration::hours(6);

        reverse_dimension_decay(&mut state_value, duration, half_life);

        // Should skip due to epsilon check
        assert!((state_value.delta() - original_delta).abs() < 1e-15);
    }

    #[test]
    fn apply_event_chronic_pattern_affects_pb_slower_decay() {
        let state = IndividualState::new();
        let chronic_event = EventBuilder::new(EventType::BurdenFeedback)
            .severity(0.8)
            .tag(EventTag::ChronicPattern)
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &chronic_event);

        // Chronic pattern should apply burden increase
        assert!(new_state.social_cognition().perceived_liability().delta() > 0.0);
    }

    #[test]
    fn apply_event_trauma_multiple_effects() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::Violence)
            .severity(1.0)
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        // Trauma should decrease valence, increase arousal, and increase AC
        assert!(new_state.mood().valence_delta() < 0.0);
        assert!(new_state.mood().arousal_delta() > 0.0);
        assert!(new_state.mental_health().acquired_capability().delta() > 0.0);
    }

    #[test]
    fn apply_event_burden_perception_reduces_valence() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::BurdenFeedback)
            .severity(0.75)
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        assert!(new_state.mood().valence_delta() < 0.0);
        assert!(new_state.social_cognition().perceived_liability().delta() > 0.0);
    }

    #[test]
    fn apply_event_to_state_zero_severity() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::Achievement)
            .severity(0.0)
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        // With zero severity, deltas should be zero (multiplied by 0)
        assert!(new_state.mood().valence_delta().abs() < 0.001);
    }

    #[test]
    fn apply_interpreted_event_moral_violation_flag() {
        let state = IndividualState::new();

        let event = EventBuilder::new(EventType::Conflict)
            .severity(0.5)
            .tag(EventTag::MoralViolation)
            .build()
            .unwrap();

        let interpreted = make_test_interpreted_event(event, vec![]);

        let new_state = apply_interpreted_event_to_state(state, &interpreted);

        assert!((new_state.recent_moral_violation_flag() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn apply_interpreted_event_empty_deltas() {
        let state = IndividualState::new();

        let event = EventBuilder::new(EventType::SocialInclusion)
            .severity(0.5)
            .build()
            .unwrap();

        let interpreted = make_test_interpreted_event(event, vec![]);

        let new_state = apply_interpreted_event_to_state(state, &interpreted);

        // State should remain unchanged
        assert!(new_state.mood().valence_delta().abs() < f32::EPSILON);
    }

    #[test]
    fn reverse_event_achievement_to_failure() {
        let mut state = IndividualState::new();
        // Simulate state after achievement
        state.mood_mut().add_valence_delta(0.21); // 0.7 * 0.3
        state.mood_mut().add_dominance_delta(0.07); // 0.7 * 0.1

        let event = EventBuilder::new(EventType::Achievement)
            .severity(0.7)
            .build()
            .unwrap();

        let reversed = reverse_event_from_state(state, &event);

        // Should restore to near zero
        assert!(reversed.mood().valence_delta().abs() < 0.01);
        assert!(reversed.mood().dominance_delta().abs() < 0.01);
    }

    #[test]
    fn apply_event_to_state_very_high_severity() {
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::Violence)
            .severity(1.0)
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        // Very high severity should produce larger deltas
        assert!(new_state.mood().valence_delta() < -0.2);
        assert!(new_state.mood().arousal_delta() > 0.3);
    }

    #[test]
    fn apply_protective_factors_achievement_without_payload() {
        // When achievement event doesn't have Achievement payload, it falls into
        // the catch-all pattern that checks for Work tag
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::Achievement)
            .severity(0.6)
            .tag(EventTag::Work)
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        // Work tag protective factor should apply
        assert!(new_state.needs().purpose().delta() > 0.0);
    }

    #[test]
    fn apply_event_social_inclusion_with_valid_group() {
        let state = IndividualState::new();
        let group_id = GroupId::new("test_group").unwrap();
        let event = EventBuilder::new(EventType::SocialInclusion)
            .severity(0.9)
            .payload(EventPayload::SocialInclusion {
                group_id: Some(group_id),
            })
            .build()
            .unwrap();

        let new_state = apply_event_to_state(state, &event);

        // Group inclusion should have extra protective factors
        let loneliness_delta = new_state.social_cognition().loneliness().delta();
        // Base effect: -0.2 * 0.9 = -0.18, plus extra: -0.08 * 0.9 = -0.072
        assert!(loneliness_delta < -0.2);
    }

    #[test]
    fn reverse_dimension_decay_clamping_large_reversal() {
        use crate::state::StateValue;

        let mut state_value = StateValue::new(0.5).with_delta(10.0);
        let duration = Duration::years(1);
        let half_life = Duration::hours(6);

        reverse_dimension_decay(&mut state_value, duration, half_life);

        // Should be clamped to 100.0 due to clamp(-100.0, 100.0)
        assert!(state_value.delta() <= 100.0);
        assert!(state_value.delta() >= -100.0);
    }

    #[test]
    fn apply_interpreted_event_moral_violation_flag_set() {
        let state = IndividualState::new();

        let event = EventBuilder::new(EventType::Humiliation)
            .severity(0.5)
            .tag(EventTag::MoralViolation)
            .build()
            .unwrap();

        let interpreted = make_test_interpreted_event(
            event,
            vec![(crate::enums::StatePath::Mood(crate::enums::MoodPath::Valence), -0.15)],
        );

        let new_state = apply_interpreted_event_to_state(state, &interpreted);

        assert!((new_state.recent_moral_violation_flag() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn apply_event_support_effectiveness_multiplier() {
        let state = IndividualState::new();
        let high_eff = EventBuilder::new(EventType::Support)
            .severity(0.5)
            .payload(EventPayload::Support {
                support_type: SupportType::Emotional,
                effectiveness: 1.0,
            })
            .build()
            .unwrap();

        let low_eff = EventBuilder::new(EventType::Support)
            .severity(0.5)
            .payload(EventPayload::Support {
                support_type: SupportType::Emotional,
                effectiveness: 0.3,
            })
            .build()
            .unwrap();

        let high_state = apply_event_to_state(state.clone(), &high_eff);
        let low_state = apply_event_to_state(state, &low_eff);

        // Higher effectiveness should have greater protective effect
        assert!(
            high_state.mental_health().self_worth().delta()
                > low_state.mental_health().self_worth().delta()
        );
    }

    #[test]
    fn reverse_event_with_protective_factors() {
        let mut state = IndividualState::new();
        // Simulate state after achievement with protective effects
        state.mood_mut().add_valence_delta(0.3 * 0.8); // Base effect
        state
            .social_cognition_mut()
            .add_perceived_liability_delta(-0.12 * 0.8 * 0.6); // Protective work effect
        state
            .social_cognition_mut()
            .add_self_hate_delta(-0.08 * 0.8 * 0.6);

        let event = EventBuilder::new(EventType::Achievement)
            .severity(0.8)
            .tag(EventTag::Work)
            .build()
            .unwrap();

        let reversed = reverse_event_from_state(state, &event);

        // After reversing protective factors, protective effect should be reversed too
        // which means perceived_liability should increase (become less negative)
        assert!(reversed.social_cognition().perceived_liability().delta() > -0.01);
    }

    #[test]
    fn reverse_dimension_decay_exponent_overflow_guard() {
        use crate::state::StateValue;

        let mut state_value = StateValue::new(0.5).with_delta(10.0);
        let duration = Duration::years(100);
        let half_life = Duration::hours(1);

        reverse_dimension_decay(&mut state_value, duration, half_life);

        assert!(state_value.delta().is_finite());
    }

    // ========================================================================
    // Coverage tests for state_evolution.rs missed regions
    // ========================================================================

    #[test]
    fn reverse_dimension_decay_very_long_half_life() {
        use crate::state::StateValue;

        // Test edge case: very long half_life with short duration should barely change delta
        let mut state_value = StateValue::new(0.5).with_delta(0.3);
        let duration = Duration::seconds(1);
        let half_life = Duration::days(1000); // Very long half-life relative to duration

        // This test primarily verifies the function handles extreme values gracefully
        reverse_dimension_decay(&mut state_value, duration, half_life);
        assert!(state_value.delta().is_finite());
        // Delta should be nearly the same since only 1 second elapsed vs 1000 day half-life
        assert!(state_value.delta() > 0.29);
    }

    #[test]
    fn reverse_dimension_decay_very_small_delta() {
        use crate::state::StateValue;

        // Test that very small deltas are handled correctly
        let mut state_value = StateValue::new(0.5).with_delta(1e-10);
        let duration = Duration::hours(1);
        let half_life = Duration::hours(6);

        reverse_dimension_decay(&mut state_value, duration, half_life);

        // Should not cause underflow or NaN
        assert!(state_value.delta().is_finite());
    }

    #[test]
    fn apply_decay_chronically_affected_dimension() {
        use crate::state::IndividualState;

        let mut state = IndividualState::new();
        let social = state.social_cognition_mut();

        // Add both normal and chronic delta to test decay of both
        social.loneliness_mut().add_delta(0.2);
        social.loneliness_mut().add_chronic_delta(0.1);

        let initial_delta = state.social_cognition().loneliness().delta();
        assert!(initial_delta > 0.0);

        let duration = Duration::days(30); // Long duration to ensure decay happens
        state.apply_decay(duration);

        // Delta should decay over time (chronic decays slower than normal)
        let final_delta = state.social_cognition().loneliness().delta();
        // With 30 days elapsed, delta should have decayed somewhat
        assert!(final_delta < initial_delta);
    }

    #[test]
    fn reverse_dimension_decay_overflow_guard() {
        use crate::state::StateValue;

        // Test that extremely large duration doesn't cause overflow
        // This tests the guard at line 245: if exponent > 700.0
        let mut state_value = StateValue::new(0.5).with_delta(0.1);
        let duration = Duration::days(365 * 1000); // 1000 years - extreme case
        let half_life = Duration::hours(6);

        reverse_dimension_decay(&mut state_value, duration, half_life);

        // Should not cause overflow - delta should remain unchanged due to guard
        assert!((state_value.delta() - 0.1).abs() < f32::EPSILON);
    }

    #[test]
    fn support_event_with_instrumental_type_does_not_affect_social_cognition() {
        // This test covers the case where Support event has a non-Emotional/non-Companionship
        // support type, exercising the else branch of the if matches! block
        let state = IndividualState::new();
        let event = EventBuilder::new(EventType::Support)
            .severity(0.5)
            .payload(EventPayload::Support {
                support_type: SupportType::Instrumental,
                effectiveness: 1.0,
            })
            .build()
            .unwrap();

        let initial_liability = state.social_cognition().perceived_liability().delta();
        let initial_self_hate = state.social_cognition().self_hate().delta();
        let initial_self_worth = state.mental_health().self_worth().delta();

        let new_state = apply_event_to_state(state, &event);

        // Instrumental support does NOT trigger the Emotional/Companionship-specific effects
        // The social cognition deltas should remain unchanged
        assert!(
            (new_state.social_cognition().perceived_liability().delta() - initial_liability).abs()
                < f32::EPSILON
        );
        assert!(
            (new_state.social_cognition().self_hate().delta() - initial_self_hate).abs()
                < f32::EPSILON
        );
        assert!(
            (new_state.mental_health().self_worth().delta() - initial_self_worth).abs()
                < f32::EPSILON
        );
    }
}
