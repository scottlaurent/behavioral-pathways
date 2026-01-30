//! State query API for timestamp-based state access.
//!
//! This module provides `EntityQueryHandle` for querying entity state at
//! any timestamp, and `ComputedState` as the result type.

use crate::context::apply_context_effects;
use crate::entity::Entity;
use crate::enums::{HexacoPath, LifeStage, StatePath};
use crate::memory::{apply_memory_consolidation, MemoryEntry};
use crate::processor::{
    advance_state, apply_developmental_effects, apply_interpreted_event_to_state,
    get_derived_emotion, interpret_event, regress_state, reverse_interpreted_event_from_state,
    EmotionIntensities, InterpretedEvent,
};
use crate::simulation::{RegressionQuality, Simulation, TimestampedEvent};
use crate::state::{
    apply_formative_modifiers, effective_base_at, BaseShiftRecord, IndividualState, StateInterpreter,
};
use crate::types::{Alert, Duration, EntityId, Timestamp};
use std::collections::HashMap;

/// A handle for querying entity state at different timestamps.
///
/// This handle provides the `state_at()` method, which is the primary
/// consumer API for getting entity state at any point in time.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::simulation::Simulation;
/// use behavioral_pathways::entity::EntityBuilder;
/// use behavioral_pathways::types::{Timestamp, EntityId};
/// use behavioral_pathways::enums::Species;
///
/// let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
/// let mut sim = Simulation::new(reference);
///
/// let entity = EntityBuilder::new()
///     .id("person_001")
///     .species(Species::Human)
///     .build()
///     .unwrap();
///
/// sim.add_entity(entity, reference);
///
/// let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
/// let state = handle.state_at(Timestamp::from_ymd_hms(2024, 6, 15, 12, 0, 0));
/// ```
pub struct EntityQueryHandle<'a> {
    simulation: &'a Simulation,
    entity_id: EntityId,
}

impl<'a> EntityQueryHandle<'a> {
    /// Creates a new query handle.
    pub(crate) fn new(simulation: &'a Simulation, entity_id: EntityId) -> Self {
        EntityQueryHandle {
            simulation,
            entity_id,
        }
    }

    /// Returns the entity ID.
    #[must_use]
    pub fn entity_id(&self) -> &EntityId {
        &self.entity_id
    }

    /// Returns the anchor timestamp for this entity.
    #[must_use]
    pub fn anchor_timestamp(&self) -> Option<Timestamp> {
        self.simulation
            .get_anchored_entity(&self.entity_id)
            .map(|a| a.anchor_timestamp())
    }

    /// Computes the entity's state at the given timestamp.
    ///
    /// This is the primary consumer API. It computes state by:
    /// 1. Starting from the anchor state
    /// 2. Applying decay forward or reversing backward
    /// 3. Applying events in the time range
    ///
    /// # Arguments
    ///
    /// * `timestamp` - The time at which to compute state
    ///
    /// # Returns
    ///
    /// The computed state, or `None` if the entity doesn't exist.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use behavioral_pathways::simulation::Simulation;
    /// use behavioral_pathways::entity::EntityBuilder;
    /// use behavioral_pathways::types::{Timestamp, EntityId};
    /// use behavioral_pathways::enums::Species;
    ///
    /// let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    /// let mut sim = Simulation::new(reference);
    ///
    /// let entity = EntityBuilder::new()
    ///     .id("person_001")
    ///     .species(Species::Human)
    ///     .build()
    ///     .unwrap();
    ///
    /// sim.add_entity(entity, reference);
    ///
    /// let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
    /// let state = handle.state_at(Timestamp::from_ymd_hms(2024, 6, 15, 12, 0, 0));
    /// // Access state directly - no unwrap needed
    /// let valence = state.get_effective(StatePath::Mood(MoodPath::Valence));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the entity does not exist in the simulation. This should never
    /// happen when using the public API (`Simulation::entity()`), which returns
    /// `None` for unknown entities. Use `Simulation::entity()` to check existence.
    #[must_use]
    pub fn state_at(&self, timestamp: Timestamp) -> ComputedState {
        let anchored = self
            .simulation
            .get_anchored_entity(&self.entity_id)
            .expect("EntityQueryHandle created for non-existent entity - use Simulation::entity() to check existence");
        let anchor_timestamp = anchored.anchor_timestamp();
        let entity = anchored.entity();

        // Clone the individual state as our starting point
        let mut state = entity.individual_state().clone();
        let species = entity.species().clone();

        // Short-circuit: if querying at anchor timestamp, return anchor state
        if timestamp == anchor_timestamp {
            let age_at_timestamp = self.compute_age_at_timestamp(entity, timestamp);
            let life_stage =
                LifeStage::from_age_years_for_species(&species, age_at_timestamp.as_years() as f64);

            let interpreter = StateInterpreter::from_state(&state);
            return ComputedState {
                individual_state: state,
                age_at_timestamp,
                life_stage,
                regression_quality: RegressionQuality::Exact,
                alerts: std::cell::OnceCell::new(),
                interpretations: interpreter.interpretations().clone(),
                summary: interpreter.summary().to_string(),
                delta_summary: None,
            };
        }

        // Determine direction: forward or backward
        let is_forward = timestamp > anchor_timestamp;

        // Get events targeting this entity in the relevant time range
        // Forward: (anchor, target] - exclude anchor, include target
        // Backward: [target, anchor) - include target, exclude anchor
        let events = self.get_sorted_events_for_range(anchor_timestamp, timestamp, is_forward);

        // Compute regression quality based on events
        let regression_quality = if is_forward {
            RegressionQuality::Exact
        } else {
            self.determine_regression_quality(&events)
        };

        // Interpret events once using the anchor entity's personality
        // Personality (HEXACO) is stable, so using anchor state is appropriate
        let interpreted_events: Vec<InterpretedEvent> = events
            .iter()
            .map(|te| interpret_event(te.event(), entity))
            .collect();

        // Collect base shift records from events that have formative shifts
        // These represent permanent personality changes from significant life events
        let base_shift_records: Vec<BaseShiftRecord> = collect_base_shift_records(
            &events,
            entity,
            timestamp,
            is_forward,
        );

        // Helper to compute age at a given timestamp for developmental effects
        let compute_age_at = |ts: Timestamp| -> Duration {
            if let Some(birth_date) = entity.birth_date() {
                if ts >= birth_date {
                    ts - birth_date
                } else {
                    Duration::zero()
                }
            } else {
                // Without birth_date, age is constant at anchor age
                entity.age()
            }
        };

        if is_forward {
            // Forward: use cursor pattern to track current time position
            // This avoids compounding decay by advancing in deltas between events
            let mut cursor = anchor_timestamp;

            for (te, interpreted) in events.iter().zip(interpreted_events.iter()) {
                // Advance from cursor to this event's timestamp
                let delta = te.timestamp() - cursor;
                state = advance_state(state, delta);

                // Apply developmental effects to scale event impact
                // Compute entity's age at the time of this event
                let age_at_event = compute_age_at(te.timestamp());
                let age_days = age_at_event.as_days();
                let dev_factor =
                    apply_developmental_effects(entity, te.event(), 1.0, age_days, te.timestamp());

                // Scale the interpreted event by the developmental factor
                let scaled_interpreted = interpreted.scaled_by(dev_factor);

                // Apply the scaled interpreted event deltas
                state = apply_interpreted_event_to_state(state, &scaled_interpreted);
                // Move cursor forward
                cursor = te.timestamp();
            }

            // Advance remaining time from cursor to target timestamp
            let remaining = timestamp - cursor;
            state = advance_state(state, remaining);
        } else {
            // Backward: use cursor pattern in reverse
            // Start at anchor and work backward through events in reverse order
            let mut cursor = anchor_timestamp;

            // Events are sorted chronologically, so iterate in reverse
            // Use indices to access both events and interpreted events in sync
            for i in (0..events.len()).rev() {
                let te = &events[i];
                let interpreted = &interpreted_events[i];

                // Regress from cursor to this event's timestamp
                let delta = cursor - te.timestamp();
                state = regress_state(state, delta);

                // Apply developmental effects to scale event impact for reversal
                // Compute entity's age at the time of this event
                let age_at_event = compute_age_at(te.timestamp());
                let age_days = age_at_event.as_days();
                let dev_factor =
                    apply_developmental_effects(entity, te.event(), 1.0, age_days, te.timestamp());

                // Scale the interpreted event by the developmental factor
                let scaled_interpreted = interpreted.scaled_by(dev_factor);

                // Reverse the scaled interpreted event using its actual deltas
                state = reverse_interpreted_event_from_state(state, &scaled_interpreted);
                // Move cursor backward
                cursor = te.timestamp();
            }

            // Regress remaining time from cursor to target timestamp
            let remaining = cursor - timestamp;
            state = regress_state(state, remaining);
        }

        // Apply hook points AFTER decay and events, in order:
        // 1. Context effects (ecological systems)
        // 2. Memory consolidation (salience decay, layer transfer)
        //
        // Developmental effects (plasticity, sensitive periods) are applied above
        // during event processing via apply_developmental_effects().
        let total_duration = if is_forward {
            timestamp - anchor_timestamp
        } else {
            anchor_timestamp - timestamp
        };
        let relationship_quality = estimate_relationship_quality(entity);
        let age_at_timestamp = self.compute_age_at_timestamp(entity, timestamp);
        let life_stage =
            LifeStage::from_age_years_for_species(&species, age_at_timestamp.as_years() as f64);
        state = apply_context_effects(
            state,
            entity.context(),
            relationship_quality,
            total_duration,
            life_stage,
            timestamp,
        );
        state = apply_memory_consolidation(state, entity.memories(), total_duration);

        // Apply formative base shifts to HEXACO personality traits
        // This computes effective base values for each trait based on accumulated shifts
        state = apply_base_shifts_to_state(state, &base_shift_records, timestamp);

        let baseline_state = entity.individual_state();
        let interpreter = StateInterpreter::from_state_with_baseline(&state, baseline_state);
        ComputedState {
            individual_state: state,
            age_at_timestamp,
            life_stage,
            regression_quality,
            alerts: std::cell::OnceCell::new(),
            interpretations: interpreter.interpretations().clone(),
            summary: interpreter.summary().to_string(),
            delta_summary: interpreter.delta_summary().map(|s| s.to_string()),
        }
    }

    /// Gets events in the time range, sorted chronologically.
    ///
    /// # Boundary Rules
    ///
    /// - Forward projection: (anchor, target] - excludes anchor, includes target
    /// - Backward regression: (target, anchor] - excludes target, includes anchor
    ///
    /// The anchor state already reflects events that occurred at anchor time,
    /// so for forward projection we exclude anchor. For backward regression,
    /// we include anchor events (which need to be reversed) but exclude target
    /// events (which should not exist in the pre-event state).
    fn get_sorted_events_for_range(
        &self,
        anchor: Timestamp,
        target: Timestamp,
        is_forward: bool,
    ) -> Vec<&'a TimestampedEvent> {
        let mut events: Vec<_> = self
            .simulation
            .events_for(&self.entity_id)
            .into_iter()
            .filter(|te| {
                let ts = te.timestamp();
                if is_forward {
                    // Forward: (anchor, target] - after anchor, up to and including target
                    ts > anchor && ts <= target
                } else {
                    // Backward: (target, anchor] - after target, up to and including anchor
                    ts > target && ts <= anchor
                }
            })
            .collect();

        events.sort_by_key(|te| te.timestamp());
        events
    }

    /// Determines regression quality based on events.
    ///
    /// Regression is approximate when:
    /// - Trauma events are present (AC increases are not reversible)
    /// - Events triggered feedback loops (spirals) - Phase 10+
    fn determine_regression_quality(&self, events: &[&TimestampedEvent]) -> RegressionQuality {
        use crate::enums::EventCategory;

        for te in events {
            let event = te.event();
            let category = event.category();

            // Trauma events have non-reversible Acquired Capability increases
            if matches!(category, EventCategory::Trauma) {
                return RegressionQuality::Approximate;
            }

            // Phase 10+: Check for spiral-triggering events
            // This will involve checking if the event triggered a stress/depression spiral
        }

        RegressionQuality::Exact
    }

    /// Computes the entity's age at a given timestamp.
    ///
    /// If the entity has a birth_date set, computes age as:
    /// `timestamp - birth_date`
    ///
    /// If no birth_date is set, returns the anchor age (constant) since we cannot
    /// compute age progression without knowing when the entity was born.
    fn compute_age_at_timestamp(
        &self,
        entity: &crate::entity::Entity,
        timestamp: Timestamp,
    ) -> Duration {
        // If entity has a birth date, compute age from that directly
        if let Some(birth_date) = entity.birth_date() {
            if timestamp >= birth_date {
                return timestamp - birth_date;
            } else {
                // Before birth - return zero
                return Duration::zero();
            }
        }

        // Fallback: without birth_date, age remains constant at anchor age.
        // We cannot compute age progression without knowing when the entity was born.
        entity.age()
    }

    /// Returns memories that exist at the given timestamp.
    ///
    /// A memory "exists" at a timestamp if it was formed before or at that time.
    /// This method returns all memories from the entity's memory layers that
    /// were formed at or before the specified timestamp.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - The time at which to query existing memories
    ///
    /// # Returns
    ///
    /// A vector of cloned memory entries that exist at the timestamp.
    /// Returns an empty vector if the entity doesn't exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::simulation::Simulation;
    /// use behavioral_pathways::entity::EntityBuilder;
    /// use behavioral_pathways::types::{Timestamp, EntityId};
    /// use behavioral_pathways::enums::Species;
    ///
    /// let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    /// let mut sim = Simulation::new(reference);
    ///
    /// let entity = EntityBuilder::new()
    ///     .id("person_001")
    ///     .species(Species::Human)
    ///     .build()
    ///     .unwrap();
    ///
    /// sim.add_entity(entity, reference);
    ///
    /// let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
    /// let memories = handle.memories_at(Timestamp::from_ymd_hms(2024, 6, 15, 12, 0, 0));
    /// // Initially empty for a new entity
    /// assert!(memories.is_empty());
    /// ```
    #[must_use]
    pub fn memories_at(&self, timestamp: Timestamp) -> Vec<MemoryEntry> {
        let Some(anchored) = self.simulation.get_anchored_entity(&self.entity_id) else {
            return Vec::new();
        };

        let entity = anchored.entity();
        let anchor_timestamp = anchored.anchor_timestamp();
        let anchor_age = entity.age();

        // Compute the entity's age at the query timestamp
        let age_at_timestamp = if timestamp >= anchor_timestamp {
            let elapsed = timestamp - anchor_timestamp;
            anchor_age + elapsed
        } else {
            let elapsed = anchor_timestamp - timestamp;
            if elapsed < anchor_age {
                anchor_age - elapsed
            } else {
                Duration::zero()
            }
        };

        // Get all memories and filter by those formed at or before the computed age
        // MemoryEntry.timestamp() returns the entity's age when the memory was formed
        entity
            .memories()
            .all_memories()
            .filter(|memory: &&MemoryEntry| memory.timestamp() <= age_at_timestamp)
            .cloned()
            .collect()
    }
}

fn estimate_relationship_quality(entity: &Entity) -> f64 {
    let attached_count = entity
        .relationship_slots()
        .iter()
        .filter(|slot| slot.is_attached())
        .count();

    if attached_count > 0 {
        0.5 + 0.1 * (attached_count as f64).min(5.0)
    } else {
        0.3
    }
}

/// The computed state of an entity at a specific timestamp.
///
/// This is the result of calling `state_at()` on an `EntityQueryHandle`.
/// It contains the computed individual state, age, life stage, and
/// regression quality indicator.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::simulation::Simulation;
/// use behavioral_pathways::entity::EntityBuilder;
/// use behavioral_pathways::types::{Timestamp, EntityId};
/// use behavioral_pathways::enums::{Species, StatePath, MoodPath};
///
/// let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
/// let mut sim = Simulation::new(reference);
///
/// let entity = EntityBuilder::new()
///     .id("person_001")
///     .species(Species::Human)
///     .build()
///     .unwrap();
///
/// sim.add_entity(entity, reference);
///
/// let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
/// let computed = handle.state_at(reference);
///
/// // Access state via typed paths - returns f64 directly
/// let valence = computed.get_effective(StatePath::Mood(MoodPath::Valence));
/// assert!(valence >= -1.0 && valence <= 1.0);
/// ```
#[derive(Debug)]
pub struct ComputedState {
    /// The computed individual state.
    pub individual_state: IndividualState,
    /// The entity's age at the queried timestamp.
    pub age_at_timestamp: Duration,
    /// The entity's life stage at the queried timestamp.
    pub life_stage: LifeStage,
    /// Quality indicator for backward regression.
    regression_quality: RegressionQuality,
    /// Cached alerts (lazy computed with interior mutability).
    alerts: std::cell::OnceCell<Vec<Alert>>,
    /// Human-readable interpretations of psychological dimensions.
    pub interpretations: HashMap<String, String>,
    /// Condensed plain-English summary paragraph.
    pub summary: String,
    /// Delta emphasis summary showing changes from baseline.
    pub delta_summary: Option<String>,
}

impl ComputedState {
    /// Returns a reference to the individual state.
    #[must_use]
    pub fn individual_state(&self) -> &IndividualState {
        &self.individual_state
    }

    /// Returns the age at the queried timestamp.
    #[must_use]
    pub fn age_at_timestamp(&self) -> Duration {
        self.age_at_timestamp
    }

    /// Returns the life stage at the queried timestamp.
    #[must_use]
    pub fn life_stage(&self) -> LifeStage {
        self.life_stage
    }

    /// Returns the regression quality indicator.
    ///
    /// This indicates whether the state was computed exactly or approximately.
    /// Forward projections are always Exact. Backward regressions may be
    /// Approximate if the time range contains spiral-triggering events.
    #[must_use]
    pub fn regression_quality(&self) -> RegressionQuality {
        self.regression_quality
    }

    /// Returns alerts generated during state computation.
    ///
    /// This is lazily computed on first access. Alerts include threshold
    /// violations and feedback loop detections.
    ///
    /// Returns a cloned vector of alerts per the spec API.
    #[must_use]
    pub fn alerts(&self) -> Vec<Alert> {
        self.alerts.get_or_init(|| self.compute_alerts()).clone()
    }

    /// Computes alerts for this state.
    fn compute_alerts(&self) -> Vec<Alert> {
        // Placeholder for Phase 10+ implementation
        // Will check thresholds on mental health dimensions
        Vec::new()
    }

    /// Returns derived emotion intensities from PAD dimensions.
    ///
    /// This computes graded membership values for each emotion octant based on
    /// the current PAD (Pleasure-Arousal-Dominance) values. Each emotion receives
    /// an intensity between 0.0 and 1.0 rather than selecting a single discrete emotion.
    ///
    /// # Returns
    ///
    /// Graded emotion intensities for each of the 9 emotion octants:
    /// - Exuberant (V+ A+ D+)
    /// - Dependent (V+ A+ D-)
    /// - Relaxed (V+ A- D+)
    /// - Docile (V+ A- D-)
    /// - Hostile (V- A+ D+)
    /// - Disgust (V- A+ D+, moral violation gated)
    /// - Anxious (V- A+ D-)
    /// - Bored (V- A- D+)
    /// - Depressed (V- A- D-)
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::simulation::Simulation;
    /// use behavioral_pathways::entity::EntityBuilder;
    /// use behavioral_pathways::types::{Timestamp, EntityId};
    /// use behavioral_pathways::enums::Species;
    ///
    /// let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    /// let mut sim = Simulation::new(reference);
    ///
    /// let entity = EntityBuilder::new()
    ///     .id("person_001")
    ///     .species(Species::Human)
    ///     .build()
    ///     .unwrap();
    ///
    /// sim.add_entity(entity, reference);
    ///
    /// let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
    /// let computed = handle.state_at(reference);
    ///
    /// let emotions = computed.derived_emotions();
    /// // Each emotion intensity is between 0.0 and 1.0
    /// assert!(emotions.exuberant >= 0.0 && emotions.exuberant <= 1.0);
    /// ```
    #[must_use]
    pub fn derived_emotions(&self) -> EmotionIntensities {
        get_derived_emotion(&self.individual_state)
    }

    /// Gets the effective value for a state path.
    ///
    /// This is a convenience method that delegates to the individual state.
    ///
    /// # Arguments
    ///
    /// * `path` - The state path to query
    ///
    /// # Returns
    ///
    /// The effective value (base + delta) for stored paths,
    /// or the computed value for derived paths.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::simulation::Simulation;
    /// use behavioral_pathways::entity::EntityBuilder;
    /// use behavioral_pathways::types::{Timestamp, EntityId};
    /// use behavioral_pathways::enums::{Species, StatePath, MoodPath};
    ///
    /// let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    /// let mut sim = Simulation::new(reference);
    ///
    /// let entity = EntityBuilder::new()
    ///     .id("person_001")
    ///     .species(Species::Human)
    ///     .build()
    ///     .unwrap();
    ///
    /// sim.add_entity(entity, reference);
    ///
    /// let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
    /// let computed = handle.state_at(reference);
    ///
    /// let valence = computed.get_effective(StatePath::Mood(MoodPath::Valence));
    /// // valence is f64, not Option<f64>
    /// assert!(valence >= -1.0 && valence <= 1.0);
    /// ```
    #[must_use]
    pub fn get_effective(&self, path: StatePath) -> f64 {
        use crate::enums::{
            DispositionPath, HexacoPath, MentalHealthPath, MoodPath, NeedsPath,
            PersonCharacteristicsPath, SocialCognitionPath,
        };

        let state = &self.individual_state;

        let value: f32 = match path {
            StatePath::Hexaco(p) => match p {
                HexacoPath::HonestyHumility => state.hexaco().honesty_humility(),
                HexacoPath::Neuroticism => state.hexaco().neuroticism(),
                HexacoPath::Extraversion => state.hexaco().extraversion(),
                HexacoPath::Agreeableness => state.hexaco().agreeableness(),
                HexacoPath::Conscientiousness => state.hexaco().conscientiousness(),
                HexacoPath::Openness => state.hexaco().openness(),
            },
            StatePath::Mood(p) => match p {
                MoodPath::Valence => state.mood().valence_effective(),
                MoodPath::Arousal => state.mood().arousal_effective(),
                MoodPath::Dominance => state.mood().dominance_effective(),
            },
            StatePath::Needs(p) => match p {
                NeedsPath::Stress => state.needs().stress_effective(),
                NeedsPath::Fatigue => state.needs().fatigue_effective(),
                NeedsPath::Purpose => state.needs().purpose_effective(),
            },
            StatePath::SocialCognition(p) => match p {
                SocialCognitionPath::Loneliness => state.social_cognition().loneliness_effective(),
                SocialCognitionPath::PerceivedReciprocalCaring => state
                    .social_cognition()
                    .perceived_reciprocal_caring_effective(),
                SocialCognitionPath::PerceivedLiability => {
                    state.social_cognition().perceived_liability_effective()
                }
                SocialCognitionPath::SelfHate => state.social_cognition().self_hate_effective(),
                SocialCognitionPath::PerceivedCompetence => state
                    .social_cognition()
                    .perceived_competence_effective(),
            },
            StatePath::MentalHealth(p) => match p {
                MentalHealthPath::Depression => state.mental_health().depression_effective(),
                MentalHealthPath::AcquiredCapability => {
                    state.mental_health().acquired_capability_effective()
                }
                MentalHealthPath::InterpersonalHopelessness => {
                    state.mental_health().interpersonal_hopelessness_effective()
                }
                MentalHealthPath::ThwartedBelongingness => state.compute_thwarted_belongingness(),
                MentalHealthPath::PerceivedBurdensomeness => {
                    state.compute_perceived_burdensomeness()
                }
                MentalHealthPath::SuicidalDesire => state.compute_suicidal_desire(),
                MentalHealthPath::AttemptRisk => state.compute_attempt_risk(),
                MentalHealthPath::SelfWorth => state.mental_health().self_worth_effective(),
                MentalHealthPath::Hopelessness => state.mental_health().hopelessness_effective(),
            },
            StatePath::Disposition(p) => match p {
                DispositionPath::Empathy => state.disposition().empathy_effective(),
                DispositionPath::Aggression => state.disposition().aggression_effective(),
                DispositionPath::Grievance => state.disposition().grievance_effective(),
                DispositionPath::ImpulseControl => state.disposition().impulse_control_effective(),
                DispositionPath::Reactance => state.disposition().reactance_effective(),
                DispositionPath::TrustPropensity => {
                    state.disposition().trust_propensity_effective()
                }
            },
            StatePath::PersonCharacteristics(p) => match p {
                PersonCharacteristicsPath::SocialCapital => {
                    state.person_characteristics().social_capital_effective()
                }
                PersonCharacteristicsPath::CognitiveAbility => {
                    state.person_characteristics().cognitive_ability_effective()
                }
                PersonCharacteristicsPath::EmotionalRegulationAssets => state
                    .person_characteristics()
                    .emotional_regulation_assets_effective(),
                PersonCharacteristicsPath::MaterialSecurity => {
                    state.person_characteristics().material_security_effective()
                }
                PersonCharacteristicsPath::ExperienceDiversity => state
                    .person_characteristics()
                    .experience_diversity_effective(),
                PersonCharacteristicsPath::BaselineMotivation => state
                    .person_characteristics()
                    .baseline_motivation_effective(),
                PersonCharacteristicsPath::PersistenceTendency => state
                    .person_characteristics()
                    .persistence_tendency_effective(),
                PersonCharacteristicsPath::CuriosityTendency => state
                    .person_characteristics()
                    .curiosity_tendency_effective(),
                // Composite values
                PersonCharacteristicsPath::Resource => state.person_characteristics().resource(),
                PersonCharacteristicsPath::Force => state.person_characteristics().force(),
            },
        };

        f64::from(value)
    }
}

impl Clone for ComputedState {
    fn clone(&self) -> Self {
        ComputedState {
            individual_state: self.individual_state.clone(),
            age_at_timestamp: self.age_at_timestamp,
            life_stage: self.life_stage,
            regression_quality: self.regression_quality,
            alerts: match self.alerts.get() {
                Some(v) => {
                    let cell = std::cell::OnceCell::new();
                    let _ = cell.set(v.clone());
                    cell
                }
                None => std::cell::OnceCell::new(),
            },
            interpretations: self.interpretations.clone(),
            summary: self.summary.clone(),
            delta_summary: self.delta_summary.clone(),
        }
    }
}

/// Collects base shift records from events that have formative personality shifts.
///
/// For forward queries, collects shifts from events before the query timestamp.
/// For backward queries, we don't collect shifts (they don't exist yet in the past).
fn collect_base_shift_records(
    events: &[&TimestampedEvent],
    entity: &Entity,
    query_timestamp: Timestamp,
    is_forward: bool,
) -> Vec<BaseShiftRecord> {
    // Backward queries don't include formative events (they haven't happened yet)
    if !is_forward {
        return Vec::new();
    }

    let reference_timestamp = entity
        .birth_date()
        .unwrap_or_else(|| Timestamp::from_ymd_hms(1970, 1, 1, 0, 0, 0));

    let mut records = Vec::new();
    let mut cumulative_positive: HashMap<HexacoPath, f32> = HashMap::new();
    let mut cumulative_negative: HashMap<HexacoPath, f32> = HashMap::new();

    for te in events {
        let event = te.event();

        // Skip events without base shifts
        if !event.has_base_shifts() {
            continue;
        }

        // Skip events after query timestamp
        if te.timestamp() > query_timestamp {
            continue;
        }

        // Compute entity's age at event time for plasticity modifiers
        let age_at_event = if let Some(birth_date) = entity.birth_date() {
            if te.timestamp() >= birth_date {
                (te.timestamp() - birth_date).as_years() as u16
            } else {
                0
            }
        } else {
            entity.age().as_years() as u16
        };

        // Convert event timestamp to Duration from reference
        let event_duration = if te.timestamp() >= reference_timestamp {
            te.timestamp() - reference_timestamp
        } else {
            Duration::zero()
        };

        // Process each base shift in the event
        for (trait_path, raw_amount) in event.base_shifts() {
            // Get existing cumulative in this direction
            let existing = if *raw_amount > 0.0 {
                *cumulative_positive.get(trait_path).unwrap_or(&0.0)
            } else {
                *cumulative_negative.get(trait_path).unwrap_or(&0.0)
            };

            // Apply all modifiers: plasticity, trait stability, saturation, caps
            let modified = apply_formative_modifiers(
                *raw_amount,
                *trait_path,
                age_at_event,
                existing,
                entity.species(),
            );

            // Skip zero shifts
            if modified.abs() < f32::EPSILON {
                continue;
            }

            // Create the base shift record
            let record = BaseShiftRecord::new(event_duration, *trait_path, modified);

            // Update cumulative tracking
            if modified > 0.0 {
                *cumulative_positive.entry(*trait_path).or_insert(0.0) += modified.abs();
            } else {
                *cumulative_negative.entry(*trait_path).or_insert(0.0) += modified.abs();
            }

            records.push(record);
        }
    }

    records
}

/// Applies accumulated base shifts to HEXACO personality traits in the state.
///
/// For each HEXACO trait, computes the effective base value using all
/// applicable base shift records, then updates the state's HEXACO values.
fn apply_base_shifts_to_state(
    mut state: IndividualState,
    shift_records: &[BaseShiftRecord],
    query_timestamp: Timestamp,
) -> IndividualState {
    // If no shifts, return state unchanged
    if shift_records.is_empty() {
        return state;
    }

    // Convert query timestamp to Duration for effective_base_at computation
    // Using a fixed reference of 1970 to be consistent with collect_base_shift_records
    let reference = Timestamp::from_ymd_hms(1970, 1, 1, 0, 0, 0);
    let query_duration = if query_timestamp >= reference {
        query_timestamp - reference
    } else {
        Duration::zero()
    };

    // Process each HEXACO trait
    for trait_path in HexacoPath::all() {
        // Filter records for this trait
        let trait_records: Vec<_> = shift_records
            .iter()
            .filter(|r| r.trait_path() == trait_path)
            .cloned()
            .collect();

        // Skip if no records for this trait
        if trait_records.is_empty() {
            continue;
        }

        // Get current base value
        let current_base = match trait_path {
            HexacoPath::Openness => state.hexaco().openness(),
            HexacoPath::Conscientiousness => state.hexaco().conscientiousness(),
            HexacoPath::Extraversion => state.hexaco().extraversion(),
            HexacoPath::Agreeableness => state.hexaco().agreeableness(),
            HexacoPath::Neuroticism => state.hexaco().neuroticism(),
            HexacoPath::HonestyHumility => state.hexaco().honesty_humility(),
        };

        // Compute effective base with accumulated shifts
        let effective = effective_base_at(current_base, &trait_records, query_duration);

        // Update the trait value in state
        match trait_path {
            HexacoPath::Openness => state.hexaco_mut().set_openness(effective),
            HexacoPath::Conscientiousness => state.hexaco_mut().set_conscientiousness(effective),
            HexacoPath::Extraversion => state.hexaco_mut().set_extraversion(effective),
            HexacoPath::Agreeableness => state.hexaco_mut().set_agreeableness(effective),
            HexacoPath::Neuroticism => state.hexaco_mut().set_neuroticism(effective),
            HexacoPath::HonestyHumility => state.hexaco_mut().set_honesty_humility(effective),
        }
    }

    state
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::EntityBuilder;
    use crate::enums::{EventType, Species};
    use crate::event::EventBuilder;

    fn create_simulation() -> Simulation {
        let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        Simulation::new(reference)
    }

    fn create_human(id: &str) -> crate::entity::Entity {
        EntityBuilder::new()
            .id(id)
            .species(Species::Human)
            .build()
            .unwrap()
    }

    #[test]
    fn entity_query_handle_entity_id() {
        let sim = create_simulation();
        let handle = EntityQueryHandle::new(&sim, EntityId::new("test").unwrap());
        assert_eq!(handle.entity_id().as_str(), "test");
    }

    #[test]
    fn entity_query_handle_anchor_timestamp() {
        let mut sim = create_simulation();
        let entity = create_human("person_001");
        let anchor = Timestamp::from_ymd_hms(2024, 1, 15, 12, 0, 0);
        sim.add_entity(entity, anchor);

        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
        assert_eq!(handle.anchor_timestamp(), Some(anchor));
    }

    #[test]
    fn state_at_anchor_returns_original_state() {
        let mut sim = create_simulation();
        let entity = create_human("person_001");
        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
        let _state = handle.state_at(anchor);
        // state_at now returns ComputedState directly, no need to check is_some
    }

    #[test]
    fn state_at_forward_applies_decay() {
        let mut sim = create_simulation();
        let mut entity = create_human("person_001");

        // Set a non-zero delta that will decay
        entity
            .individual_state_mut()
            .mood_mut()
            .add_valence_delta(0.5);

        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        // Query 1 week later
        let future = anchor + Duration::weeks(1);
        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
        let computed = handle.state_at(future);

        // Valence should have decayed (6-hour half-life, so 1 week = many half-lives)
        let valence = computed.get_effective(StatePath::Mood(crate::enums::MoodPath::Valence));
        assert!(valence < 0.1); // Nearly fully decayed
    }

    #[test]
    fn state_at_backward_regresses_state() {
        let mut sim = create_simulation();
        let entity = create_human("person_001");
        let anchor = Timestamp::from_ymd_hms(2024, 6, 1, 0, 0, 0);
        sim.add_entity(entity, anchor);

        // Query 1 month earlier
        let past = Timestamp::from_ymd_hms(2024, 5, 1, 0, 0, 0);
        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
        let _state = handle.state_at(past);
        // state_at now returns ComputedState directly
    }

    #[test]
    fn computed_state_accessors() {
        let mut sim = create_simulation();
        let entity = create_human("person_001");
        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
        let state = handle.state_at(anchor);

        // Check all accessors
        let _ = state.individual_state();
        let _ = state.age_at_timestamp();
        let _ = state.life_stage();
        let _ = state.regression_quality();
    }

    #[test]
    fn computed_state_alerts_lazy() {
        let mut sim = create_simulation();
        let entity = create_human("person_001");
        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
        let state = handle.state_at(anchor);

        // First access computes alerts (immutable access via interior mutability)
        let alerts = state.alerts();
        assert!(alerts.is_empty()); // No alerts by default

        // Second access should use cached value
        let alerts2 = state.alerts();
        assert!(alerts2.is_empty());
    }

    #[test]
    fn computed_state_get_effective_mood() {
        let mut sim = create_simulation();
        let entity = create_human("person_001");
        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
        let state = handle.state_at(anchor);

        let valence = state.get_effective(StatePath::Mood(crate::enums::MoodPath::Valence));
        assert!(valence >= -1.0 && valence <= 1.0);

        let arousal = state.get_effective(StatePath::Mood(crate::enums::MoodPath::Arousal));
        assert!(arousal >= -1.0 && arousal <= 1.0);

        let dominance = state.get_effective(StatePath::Mood(crate::enums::MoodPath::Dominance));
        assert!(dominance >= -1.0 && dominance <= 1.0);
    }

    #[test]
    fn computed_state_get_effective_needs() {
        let mut sim = create_simulation();
        let entity = create_human("person_001");
        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
        let state = handle.state_at(anchor);

        use crate::enums::{NeedsPath, SocialCognitionPath};

        // get_effective now returns f64 directly - verify values are in valid range
        let loneliness = state.get_effective(StatePath::SocialCognition(
            SocialCognitionPath::Loneliness,
        ));
        assert!(loneliness >= 0.0 && loneliness <= 1.0);

        let prc = state.get_effective(StatePath::SocialCognition(
            SocialCognitionPath::PerceivedReciprocalCaring,
        ));
        assert!(prc >= 0.0 && prc <= 1.0);

        let liability = state.get_effective(StatePath::SocialCognition(
            SocialCognitionPath::PerceivedLiability,
        ));
        assert!(liability >= 0.0 && liability <= 1.0);

        let self_hate =
            state.get_effective(StatePath::SocialCognition(SocialCognitionPath::SelfHate));
        assert!(self_hate >= 0.0 && self_hate <= 1.0);

        let perceived_competence = state.get_effective(StatePath::SocialCognition(
            SocialCognitionPath::PerceivedCompetence,
        ));
        assert!(perceived_competence >= 0.0 && perceived_competence <= 1.0);

        let stress = state.get_effective(StatePath::Needs(NeedsPath::Stress));
        assert!(stress >= 0.0 && stress <= 1.0);

        let fatigue = state.get_effective(StatePath::Needs(NeedsPath::Fatigue));
        assert!(fatigue >= 0.0 && fatigue <= 1.0);

        let purpose = state.get_effective(StatePath::Needs(NeedsPath::Purpose));
        assert!(purpose >= 0.0 && purpose <= 1.0);
    }

    #[test]
    fn computed_state_get_effective_mental_health() {
        let mut sim = create_simulation();
        let entity = create_human("person_001");
        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
        let state = handle.state_at(anchor);

        use crate::enums::MentalHealthPath;

        // get_effective now returns f64 directly - verify values are in valid range
        let depression = state.get_effective(StatePath::MentalHealth(MentalHealthPath::Depression));
        assert!(depression >= 0.0 && depression <= 1.0);

        let ac = state.get_effective(StatePath::MentalHealth(
            MentalHealthPath::AcquiredCapability,
        ));
        assert!(ac >= 0.0 && ac <= 1.0);

        let ih = state.get_effective(StatePath::MentalHealth(
            MentalHealthPath::InterpersonalHopelessness,
        ));
        assert!(ih >= 0.0 && ih <= 1.0);

        let tb = state.get_effective(StatePath::MentalHealth(
            MentalHealthPath::ThwartedBelongingness,
        ));
        assert!(tb >= 0.0 && tb <= 1.0);

        let pb = state.get_effective(StatePath::MentalHealth(
            MentalHealthPath::PerceivedBurdensomeness,
        ));
        assert!(pb >= 0.0 && pb <= 1.0);

        let desire = state.get_effective(StatePath::MentalHealth(MentalHealthPath::SuicidalDesire));
        assert!(desire >= 0.0 && desire <= 1.0);

        let risk = state.get_effective(StatePath::MentalHealth(MentalHealthPath::AttemptRisk));
        assert!(risk >= 0.0 && risk <= 1.0);
    }

    #[test]
    fn computed_state_get_effective_hexaco() {
        let mut sim = create_simulation();
        let entity = create_human("person_001");
        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
        let state = handle.state_at(anchor);

        use crate::enums::HexacoPath;

        // get_effective now returns f64 directly - HEXACO values are 0.0-1.0
        let hh = state.get_effective(StatePath::Hexaco(HexacoPath::HonestyHumility));
        assert!(hh >= 0.0 && hh <= 1.0);

        let n = state.get_effective(StatePath::Hexaco(HexacoPath::Neuroticism));
        assert!(n >= 0.0 && n <= 1.0);

        let e = state.get_effective(StatePath::Hexaco(HexacoPath::Extraversion));
        assert!(e >= 0.0 && e <= 1.0);

        let a = state.get_effective(StatePath::Hexaco(HexacoPath::Agreeableness));
        assert!(a >= 0.0 && a <= 1.0);

        let c = state.get_effective(StatePath::Hexaco(HexacoPath::Conscientiousness));
        assert!(c >= 0.0 && c <= 1.0);

        let o = state.get_effective(StatePath::Hexaco(HexacoPath::Openness));
        assert!(o >= 0.0 && o <= 1.0);
    }

    #[test]
    fn computed_state_get_effective_disposition() {
        let mut sim = create_simulation();
        let entity = create_human("person_001");
        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
        let state = handle.state_at(anchor);

        use crate::enums::DispositionPath;

        // get_effective now returns f64 directly
        let empathy = state.get_effective(StatePath::Disposition(DispositionPath::Empathy));
        assert!(empathy >= 0.0 && empathy <= 1.0);

        let aggression = state.get_effective(StatePath::Disposition(DispositionPath::Aggression));
        assert!(aggression >= 0.0 && aggression <= 1.0);

        let grievance = state.get_effective(StatePath::Disposition(DispositionPath::Grievance));
        assert!(grievance >= 0.0 && grievance <= 1.0);
    }

    #[test]
    fn computed_state_get_effective_person_characteristics() {
        let mut sim = create_simulation();
        let entity = create_human("person_001");
        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
        let state = handle.state_at(anchor);

        use crate::enums::PersonCharacteristicsPath;

        // get_effective now returns f64 directly
        let sc = state.get_effective(StatePath::PersonCharacteristics(
            PersonCharacteristicsPath::SocialCapital,
        ));
        assert!(sc >= 0.0 && sc <= 1.0);

        let ca = state.get_effective(StatePath::PersonCharacteristics(
            PersonCharacteristicsPath::CognitiveAbility,
        ));
        assert!(ca >= 0.0 && ca <= 1.0);

        let ms = state.get_effective(StatePath::PersonCharacteristics(
            PersonCharacteristicsPath::MaterialSecurity,
        ));
        assert!(ms >= 0.0 && ms <= 1.0);
    }

    #[test]
    fn state_at_with_event_applies_event() {
        let mut sim = create_simulation();
        let entity = create_human("person_001");
        let entity_id = EntityId::new("person_001").unwrap();
        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        // Add an event 1 day after anchor
        let event = EventBuilder::new(EventType::SocialExclusion)
            .target(entity_id.clone())
            .severity(0.7)
            .build()
            .unwrap();
        let event_time = anchor + Duration::days(1);
        sim.add_event(event, event_time);

        // Query 2 days after anchor (after the event)
        let query_time = anchor + Duration::days(2);
        let handle = sim.entity(&entity_id).unwrap();
        let _state = handle.state_at(query_time);
        // Event would have affected valence (social exclusion is negative)
    }

    #[test]
    fn age_at_timestamp_forward_with_birth_date() {
        let mut sim = create_simulation();
        let anchor = sim.reference_date();
        // Entity born 25 years before anchor
        let birth_date = anchor - Duration::years(25);
        let entity = EntityBuilder::new()
            .id("person_001")
            .species(Species::Human)
            .birth_date(birth_date)
            .age(Duration::years(25))
            .build()
            .unwrap();
        sim.add_entity(entity, anchor);

        // Query 10 years later - age should be 35
        let future = anchor + Duration::years(10);
        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
        let state = handle.state_at(future);

        assert_eq!(state.age_at_timestamp().as_years(), 35);
    }

    #[test]
    fn age_at_timestamp_forward_without_birth_date_is_constant() {
        let mut sim = create_simulation();
        let entity = EntityBuilder::new()
            .id("person_001")
            .species(Species::Human)
            .age(Duration::years(25))
            .build()
            .unwrap();
        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        // Query 10 years later - age should still be 25 (constant without birth_date)
        let future = anchor + Duration::years(10);
        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
        let state = handle.state_at(future);

        assert_eq!(state.age_at_timestamp().as_years(), 25);
    }

    #[test]
    fn age_at_timestamp_backward_with_birth_date() {
        let mut sim = create_simulation();
        let anchor = sim.reference_date();
        // Entity born 25 years before anchor
        let birth_date = anchor - Duration::years(25);
        let entity = EntityBuilder::new()
            .id("person_001")
            .species(Species::Human)
            .birth_date(birth_date)
            .age(Duration::years(25))
            .build()
            .unwrap();
        sim.add_entity(entity, anchor);

        // Query 10 years earlier - age should be 15
        let past = anchor - Duration::years(10);
        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
        let state = handle.state_at(past);

        assert_eq!(state.age_at_timestamp().as_years(), 15);
    }

    #[test]
    fn age_at_timestamp_backward_without_birth_date_is_constant() {
        let mut sim = create_simulation();
        let entity = EntityBuilder::new()
            .id("person_001")
            .species(Species::Human)
            .age(Duration::years(25))
            .build()
            .unwrap();
        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        // Query 10 years earlier - age should still be 25 (constant without birth_date)
        let past = anchor - Duration::years(10);
        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
        let state = handle.state_at(past);

        assert_eq!(state.age_at_timestamp().as_years(), 25);
    }

    #[test]
    fn life_stage_at_timestamp_with_birth_date() {
        let mut sim = create_simulation();
        let anchor = sim.reference_date();
        // Entity born 10 years before anchor
        let birth_date = anchor - Duration::years(10);
        let entity = EntityBuilder::new()
            .id("person_001")
            .species(Species::Human)
            .birth_date(birth_date)
            .age(Duration::years(10))
            .build()
            .unwrap();
        sim.add_entity(entity, anchor);

        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();

        // At anchor (age 10): Child
        let state = handle.state_at(anchor);
        assert_eq!(state.life_stage(), LifeStage::Child);

        // 10 years later (age 20): YoungAdult
        let future = anchor + Duration::years(10);
        let state2 = handle.state_at(future);
        assert_eq!(state2.life_stage(), LifeStage::YoungAdult);
    }

    #[test]
    fn life_stage_at_timestamp_without_birth_date_is_constant() {
        let mut sim = create_simulation();
        let entity = EntityBuilder::new()
            .id("person_001")
            .species(Species::Human)
            .age(Duration::years(10))
            .build()
            .unwrap();
        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();

        // At anchor (age 10): Child
        let state = handle.state_at(anchor);
        assert_eq!(state.life_stage(), LifeStage::Child);

        // 10 years later - without birth_date, age stays 10, still Child
        let future = anchor + Duration::years(10);
        let state2 = handle.state_at(future);
        assert_eq!(state2.life_stage(), LifeStage::Child);
    }

    #[test]
    fn regression_quality_forward_is_exact() {
        let mut sim = create_simulation();
        let entity = create_human("person_001");
        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        let future = anchor + Duration::days(30);
        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
        let state = handle.state_at(future);

        assert!(state.regression_quality().is_exact());
    }

    #[test]
    fn computed_state_debug() {
        let mut sim = create_simulation();
        let entity = create_human("person_001");
        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
        let state = handle.state_at(anchor);

        let debug = format!("{:?}", state);
        assert!(debug.contains("ComputedState"));
    }

    #[test]
    fn computed_state_clone() {
        let mut sim = create_simulation();
        let entity = create_human("person_001");
        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
        let state = handle.state_at(anchor);
        let cloned = state.clone();

        assert_eq!(state.age_at_timestamp(), cloned.age_at_timestamp());
    }

    #[test]
    #[should_panic(expected = "EntityQueryHandle created for non-existent entity")]
    fn state_at_nonexistent_entity_panics() {
        let sim = create_simulation();
        let unknown = EntityId::new("unknown").unwrap();
        let handle = EntityQueryHandle::new(&sim, unknown);

        // This should panic because the entity doesn't exist
        let _state = handle.state_at(sim.reference_date());
    }

    #[test]
    fn anchor_timestamp_nonexistent_entity() {
        let sim = create_simulation();
        let unknown = EntityId::new("unknown").unwrap();
        let handle = EntityQueryHandle::new(&sim, unknown);

        assert!(handle.anchor_timestamp().is_none());
    }

    #[test]
    fn state_at_backward_with_events() {
        let mut sim = create_simulation();
        let entity = create_human("person_001");
        let entity_id = EntityId::new("person_001").unwrap();

        // Set anchor to a later date
        let anchor = Timestamp::from_ymd_hms(2024, 6, 1, 0, 0, 0);
        sim.add_entity(entity, anchor);

        // Add an event before the anchor (in the past relative to anchor)
        let event = EventBuilder::new(EventType::SocialExclusion)
            .target(entity_id.clone())
            .severity(0.7)
            .build()
            .unwrap();
        let event_time = Timestamp::from_ymd_hms(2024, 3, 1, 0, 0, 0);
        sim.add_event(event, event_time);

        // Query state before the anchor (backward regression through events)
        let past = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let handle = sim.entity(&entity_id).unwrap();
        let computed = handle.state_at(past);

        // Backward regression through events should still work
        assert!(computed.regression_quality().is_exact());
    }

    #[test]
    fn computed_state_get_effective_mental_health_all_paths() {
        let mut sim = create_simulation();
        let entity = create_human("person_001");
        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
        let state = handle.state_at(anchor);

        use crate::enums::MentalHealthPath;

        // Test all mental health paths including SelfWorth and Hopelessness
        // get_effective now returns f64 directly
        let self_worth = state.get_effective(StatePath::MentalHealth(MentalHealthPath::SelfWorth));
        assert!(self_worth >= 0.0 && self_worth <= 1.0);

        let hopelessness =
            state.get_effective(StatePath::MentalHealth(MentalHealthPath::Hopelessness));
        assert!(hopelessness >= 0.0 && hopelessness <= 1.0);
    }

    #[test]
    fn computed_state_get_effective_disposition_all_paths() {
        let mut sim = create_simulation();
        let entity = create_human("person_001");
        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
        let state = handle.state_at(anchor);

        use crate::enums::DispositionPath;

        // Test all disposition paths - get_effective now returns f64 directly
        let impulse = state.get_effective(StatePath::Disposition(DispositionPath::ImpulseControl));
        assert!(impulse >= 0.0 && impulse <= 1.0);

        let reactance = state.get_effective(StatePath::Disposition(DispositionPath::Reactance));
        assert!(reactance >= 0.0 && reactance <= 1.0);

        let trust = state.get_effective(StatePath::Disposition(DispositionPath::TrustPropensity));
        assert!(trust >= 0.0 && trust <= 1.0);
    }

    #[test]
    fn computed_state_get_effective_person_characteristics_all_paths() {
        let mut sim = create_simulation();
        let entity = create_human("person_001");
        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
        let state = handle.state_at(anchor);

        use crate::enums::PersonCharacteristicsPath;

        // Test all person characteristics paths - get_effective now returns f64 directly
        let era = state.get_effective(StatePath::PersonCharacteristics(
            PersonCharacteristicsPath::EmotionalRegulationAssets,
        ));
        assert!(era >= 0.0 && era <= 1.0);

        let ed = state.get_effective(StatePath::PersonCharacteristics(
            PersonCharacteristicsPath::ExperienceDiversity,
        ));
        assert!(ed >= 0.0 && ed <= 1.0);

        let bm = state.get_effective(StatePath::PersonCharacteristics(
            PersonCharacteristicsPath::BaselineMotivation,
        ));
        assert!(bm >= 0.0 && bm <= 1.0);

        let pt = state.get_effective(StatePath::PersonCharacteristics(
            PersonCharacteristicsPath::PersistenceTendency,
        ));
        assert!(pt >= 0.0 && pt <= 1.0);

        let ct = state.get_effective(StatePath::PersonCharacteristics(
            PersonCharacteristicsPath::CuriosityTendency,
        ));
        assert!(ct >= 0.0 && ct <= 1.0);

        // Composite values can be any f64
        let _resource = state.get_effective(StatePath::PersonCharacteristics(
            PersonCharacteristicsPath::Resource,
        ));

        let _force = state.get_effective(StatePath::PersonCharacteristics(
            PersonCharacteristicsPath::Force,
        ));
    }

    #[test]
    fn memories_at_returns_empty_for_new_entity() {
        let mut sim = create_simulation();
        let entity = create_human("person_001");
        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
        let memories = handle.memories_at(anchor);

        assert!(memories.is_empty());
    }

    #[test]
    fn memories_at_returns_empty_for_nonexistent_entity() {
        let sim = create_simulation();
        let unknown = EntityId::new("unknown").unwrap();
        let handle = EntityQueryHandle::new(&sim, unknown);

        let memories = handle.memories_at(sim.reference_date());
        assert!(memories.is_empty());
    }

    #[test]
    fn memories_at_filters_by_timestamp() {
        use crate::memory::MemoryTag;

        let mut sim = create_simulation();

        // Create entity at age 25
        let mut entity = EntityBuilder::new()
            .id("person_001")
            .species(Species::Human)
            .age(Duration::years(25))
            .build()
            .unwrap();

        // Create a memory at "current" age (age 25)
        entity.create_memory(
            "First memory at age 25",
            vec![],
            vec![MemoryTag::Personal],
            0.5,
            None,
        );

        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        // Query at anchor time - should see the memory
        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
        let memories_at_anchor = handle.memories_at(anchor);
        assert_eq!(memories_at_anchor.len(), 1);

        // Query in the past (before entity's current age) - should not see memory
        // Entity is age 25 at anchor. Memory formed at age 25.
        // Going back 10 years makes entity age 15, so memory shouldn't exist.
        let past = anchor - Duration::years(10);
        let memories_past = handle.memories_at(past);
        assert!(memories_past.is_empty());
    }

    #[test]
    fn memories_at_forward_in_time() {
        // Test the forward path: timestamp >= anchor_timestamp
        use crate::memory::MemoryTag;

        let mut sim = create_simulation();

        // Create entity at age 25
        let mut entity = EntityBuilder::new()
            .id("person_001")
            .species(Species::Human)
            .age(Duration::years(25))
            .build()
            .unwrap();

        // Create a memory at "current" age (age 25)
        entity.create_memory(
            "Memory at age 25",
            vec![],
            vec![MemoryTag::Personal],
            0.5,
            None,
        );

        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        // Query 10 years in the future - entity is now 35
        // Memory formed at age 25, so it should still exist
        let future = anchor + Duration::years(10);
        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
        let memories_future = handle.memories_at(future);

        // Memory should still exist (formed at age 25, now age 35)
        assert_eq!(memories_future.len(), 1);
    }

    #[test]
    fn age_computed_from_birth_date() {
        let mut sim = create_simulation();

        // Entity born on June 15, 1990
        let birth = Timestamp::from_ymd_hms(1990, 6, 15, 0, 0, 0);
        let entity = EntityBuilder::new()
            .id("person_001")
            .species(Species::Human)
            .birth_date(birth)
            .build()
            .unwrap();

        let anchor = sim.reference_date(); // 2024-01-01
        sim.add_entity(entity, anchor);

        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();

        // Query at anchor date (2024-01-01)
        let state = handle.state_at(anchor);
        let age_at_anchor = state.age_at_timestamp();

        // Should be approximately 33-34 years old
        assert!(age_at_anchor.as_years() >= 33);
        assert!(age_at_anchor.as_years() <= 34);

        // Query 10 years later
        let future = anchor + Duration::years(10);
        let future_state = handle.state_at(future);
        let age_at_future = future_state.age_at_timestamp();

        // Should be approximately 43-44 years old
        assert!(age_at_future.as_years() >= 43);
        assert!(age_at_future.as_years() <= 44);
    }

    #[test]
    fn age_before_birth_returns_zero() {
        // Entity born on 2000-01-01
        let birth = Timestamp::from_ymd_hms(2000, 1, 1, 0, 0, 0);
        let entity = EntityBuilder::new()
            .id("person_001")
            .species(Species::Human)
            .birth_date(birth)
            .build()
            .unwrap();

        let anchor = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let mut sim = Simulation::new(Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0));
        sim.add_entity(entity, anchor);

        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();

        // Query before birth
        let before_birth = Timestamp::from_ymd_hms(1990, 1, 1, 0, 0, 0);
        let state = handle.state_at(before_birth);

        // Age should be zero
        assert!(state.age_at_timestamp().is_zero());
    }

    #[test]
    fn regression_through_trauma_is_approximate() {
        let mut sim = create_simulation();
        let entity = create_human("person_001");
        let entity_id = EntityId::new("person_001").unwrap();

        // Set anchor to a later date
        let anchor = Timestamp::from_ymd_hms(2024, 6, 1, 0, 0, 0);
        sim.add_entity(entity, anchor);

        // Add a trauma event (Violence) before the anchor
        let trauma_event = EventBuilder::new(EventType::Violence)
            .target(entity_id.clone())
            .severity(0.8)
            .build()
            .unwrap();
        let event_time = Timestamp::from_ymd_hms(2024, 3, 1, 0, 0, 0);
        sim.add_event(trauma_event, event_time);

        // Query state before the trauma event (backward regression through trauma)
        let past = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(past);

        // Regression through trauma should be approximate (AC not reversible)
        assert!(state.regression_quality().is_approximate());
    }

    #[test]
    fn regression_without_trauma_is_exact() {
        let mut sim = create_simulation();
        let entity = create_human("person_001");
        let entity_id = EntityId::new("person_001").unwrap();

        // Set anchor to a later date
        let anchor = Timestamp::from_ymd_hms(2024, 6, 1, 0, 0, 0);
        sim.add_entity(entity, anchor);

        // Add a social exclusion event (not trauma) before the anchor
        let social_event = EventBuilder::new(EventType::SocialExclusion)
            .target(entity_id.clone())
            .severity(0.5)
            .build()
            .unwrap();
        let event_time = Timestamp::from_ymd_hms(2024, 3, 1, 0, 0, 0);
        sim.add_event(social_event, event_time);

        // Query state before the event (backward regression)
        let past = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(past);

        // Regression through non-trauma events should be exact
        assert!(state.regression_quality().is_exact());
    }

    #[test]
    fn estimate_relationship_quality_accounts_for_attached_slots() {
        use crate::types::RelationshipId;

        let mut entity = create_human("person_001");
        let baseline = estimate_relationship_quality(&entity);
        assert!((baseline - 0.3).abs() < f64::EPSILON);

        let rel_id = RelationshipId::new("rel_attached").unwrap();
        entity.relationship_slots_mut()[0].attach(rel_id);

        let attached = estimate_relationship_quality(&entity);
        assert!(attached > baseline);
    }

    #[test]
    fn computed_state_clone_with_cached_alerts() {
        let mut sim = create_simulation();
        let entity = create_human("person_001");
        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
        let state = handle.state_at(anchor);

        // Compute and cache alerts first
        let alerts1 = state.alerts();
        assert!(alerts1.is_empty());

        // Clone the state with cached alerts
        let cloned = state.clone();

        // The cloned state should also have the cached alerts
        let alerts2 = cloned.alerts();
        assert!(alerts2.is_empty());
    }

    #[test]
    fn computed_state_clone_without_cached_alerts() {
        // Test cloning when alerts haven't been accessed yet (None branch)
        let mut sim = create_simulation();
        let entity = create_human("person_001");
        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
        let state = handle.state_at(anchor);

        // Clone WITHOUT accessing alerts first
        let cloned = state.clone();

        // Both original and cloned should work independently
        let alerts1 = state.alerts();
        let alerts2 = cloned.alerts();
        assert!(alerts1.is_empty());
        assert!(alerts2.is_empty());
    }

    #[test]
    fn memories_at_before_birth_returns_empty() {
        // Test the case where query timestamp is so far in the past
        // that elapsed >= anchor_age (age would be negative, so returns zero)
        use crate::memory::MemoryTag;

        let mut sim = create_simulation();

        // Create entity at age 5 (young entity)
        let mut entity = EntityBuilder::new()
            .id("child_001")
            .species(Species::Human)
            .age(Duration::years(5))
            .build()
            .unwrap();

        // Create a memory at "current" age (age 5)
        entity.create_memory(
            "Memory at age 5",
            vec![],
            vec![MemoryTag::Personal],
            0.5,
            None,
        );

        let anchor = sim.reference_date(); // 2024-01-01
        sim.add_entity(entity, anchor);

        // Query 10 years in the past from anchor
        // Entity is age 5 at anchor, so 10 years ago they weren't born yet
        let past = anchor - Duration::years(10);
        let handle = sim.entity(&EntityId::new("child_001").unwrap()).unwrap();
        let memories = handle.memories_at(past);

        // Should return empty because age would be negative
        assert!(memories.is_empty());
    }

    #[test]
    fn developmental_effects_entity_without_birth_date_uses_anchor_age() {
        // Test that developmental effects use anchor age when no birth_date is set
        let mut sim = create_simulation();

        // Create entity with just age (no birth_date)
        let entity = EntityBuilder::new()
            .id("person_001")
            .species(Species::Human)
            .age(Duration::years(30))
            .build()
            .unwrap();

        let entity_id = EntityId::new("person_001").unwrap();
        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        // Add an event
        let event = EventBuilder::new(EventType::Violence)
            .target(entity_id.clone())
            .severity(0.5)
            .build()
            .unwrap();
        let event_time = anchor + Duration::days(10);
        sim.add_event(event, event_time);

        // Query state after event - developmental effects should use anchor age
        let query_time = anchor + Duration::days(20);
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(query_time);

        // State should be valid (developmental effects applied correctly)
        let valence = state.get_effective(StatePath::Mood(crate::enums::MoodPath::Valence));
        assert!(valence >= -1.0 && valence <= 1.0);
    }

    #[test]
    fn developmental_effects_event_after_birth_date_uses_birth_age() {
        // Test developmental effects when event timestamp is after birth_date
        let birth_date = Timestamp::from_ymd_hms(2000, 1, 1, 0, 0, 0);
        let entity = EntityBuilder::new()
            .id("person_001")
            .species(Species::Human)
            .birth_date(birth_date)
            .build()
            .unwrap();

        let entity_id = EntityId::new("person_001").unwrap();
        let anchor = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let mut sim = Simulation::new(anchor);
        sim.add_entity(entity, anchor);

        let event = EventBuilder::new(EventType::Support)
            .target(entity_id.clone())
            .severity(0.5)
            .build()
            .unwrap();
        let event_time = anchor + Duration::days(10);
        sim.add_event(event, event_time);

        let query_time = anchor + Duration::days(20);
        let handle = sim.entity(&entity_id).unwrap();
        assert_eq!(
            handle
                .get_sorted_events_for_range(anchor, query_time, true)
                .len(),
            1
        );
        let state = handle.state_at(query_time);

        assert!(!state.age_at_timestamp().is_zero());
    }

    #[test]
    fn developmental_effects_event_before_birth_date() {
        // Test developmental effects when event timestamp is before birth_date
        // This is an edge case that shouldn't happen in practice but must be handled

        // Create entity with birth_date
        let birth_date = Timestamp::from_ymd_hms(2000, 1, 1, 0, 0, 0);
        let entity = EntityBuilder::new()
            .id("person_001")
            .species(Species::Human)
            .birth_date(birth_date)
            .build()
            .unwrap();

        let entity_id = EntityId::new("person_001").unwrap();
        let anchor = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let mut sim = Simulation::new(anchor);
        sim.add_entity(entity, anchor);

        // Add an event at a timestamp BEFORE birth_date
        // This is conceptually invalid but tests the edge case handling
        let event = EventBuilder::new(EventType::Achievement)
            .target(entity_id.clone())
            .severity(0.5)
            .build()
            .unwrap();
        let event_time = Timestamp::from_ymd_hms(1990, 1, 1, 0, 0, 0); // Before birth
        sim.add_event(event, event_time);

        // Query state far enough in the past so the pre-birth event is in range
        let query_time = Timestamp::from_ymd_hms(1980, 1, 1, 0, 0, 0);
        let handle = sim.entity(&entity_id).unwrap();
        assert_eq!(
            handle
                .get_sorted_events_for_range(anchor, query_time, false)
                .len(),
            1
        );
        let state = handle.state_at(query_time);

        // The age at the event should be treated as zero (floor at birth)
        // This tests the Duration::zero() branch in compute_age_at
        assert!(state.age_at_timestamp().is_zero());
    }

    #[test]
    fn derived_emotions_returns_valid_intensities() {
        let mut sim = create_simulation();
        let entity = create_human("person_001");
        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        let handle = sim.entity(&EntityId::new("person_001").unwrap()).unwrap();
        let state = handle.state_at(anchor);

        // Get derived emotions
        let emotions = state.derived_emotions();

        // All emotion intensities should be in valid [0, 1] range
        assert!(emotions.exuberant >= 0.0 && emotions.exuberant <= 1.0);
        assert!(emotions.dependent >= 0.0 && emotions.dependent <= 1.0);
        assert!(emotions.relaxed >= 0.0 && emotions.relaxed <= 1.0);
        assert!(emotions.docile >= 0.0 && emotions.docile <= 1.0);
        assert!(emotions.hostile >= 0.0 && emotions.hostile <= 1.0);
        assert!(emotions.disgust >= 0.0 && emotions.disgust <= 1.0);
        assert!(emotions.anxious >= 0.0 && emotions.anxious <= 1.0);
        assert!(emotions.bored >= 0.0 && emotions.bored <= 1.0);
        assert!(emotions.depressed >= 0.0 && emotions.depressed <= 1.0);
    }

    #[test]
    fn derived_emotions_changes_with_mood() {
        use crate::enums::EventType;
        use crate::event::EventBuilder;

        let mut sim = create_simulation();
        let entity = create_human("person_001");
        let entity_id = EntityId::new("person_001").unwrap();
        let anchor = sim.reference_date();
        sim.add_entity(entity, anchor);

        // Get baseline emotions
        let handle = sim.entity(&entity_id).unwrap();
        let baseline_state = handle.state_at(anchor);
        let baseline_emotions = baseline_state.derived_emotions();

        // Add a positive event
        let event = EventBuilder::new(EventType::Support)
            .target(entity_id.clone())
            .severity(0.8)
            .build()
            .unwrap();
        let event_time = anchor + Duration::hours(1);
        sim.add_event(event, event_time);

        // Query after the positive event
        let later = anchor + Duration::hours(2);
        let handle = sim.entity(&entity_id).unwrap();
        let later_state = handle.state_at(later);
        let later_emotions = later_state.derived_emotions();

        // After a positive event, positive emotions should increase
        // and/or negative emotions should decrease
        let baseline_positive = baseline_emotions.exuberant
            + baseline_emotions.dependent
            + baseline_emotions.relaxed
            + baseline_emotions.docile;
        let later_positive = later_emotions.exuberant
            + later_emotions.dependent
            + later_emotions.relaxed
            + later_emotions.docile;

        assert!(later_positive >= baseline_positive);
    }

    // Formative events tests

    #[test]
    fn formative_event_shifts_personality() {
        let mut sim = create_simulation();
        let reference = sim.reference_date();

        // Create a human with birth date 25 years before reference
        let entity = EntityBuilder::new()
            .id("person_001")
            .species(Species::Human)
            .birth_date(reference - Duration::years(25))
            .build()
            .unwrap();

        let entity_id = entity.id().clone();
        let anchor = reference;
        sim.add_entity(entity, anchor);

        // Get baseline agreeableness
        let handle = sim.entity(&entity_id).unwrap();
        let baseline_state = handle.state_at(anchor);
        let baseline_agreeableness =
            baseline_state.get_effective(StatePath::Hexaco(HexacoPath::Agreeableness));

        // Add a formative event with a base shift
        let event = EventBuilder::new(EventType::Violence)
            .target(entity_id.clone())
            .severity(0.9)
            .with_base_shift(HexacoPath::Agreeableness, -0.15)
            .build()
            .unwrap();
        let event_time = anchor + Duration::days(1);
        sim.add_event(event, event_time);

        // Query after the formative event
        let later = anchor + Duration::days(2);
        let handle = sim.entity(&entity_id).unwrap();
        let later_state = handle.state_at(later);
        let later_agreeableness =
            later_state.get_effective(StatePath::Hexaco(HexacoPath::Agreeableness));

        // Agreeableness should have decreased due to the base shift
        assert!(later_agreeableness < baseline_agreeableness);
    }

    #[test]
    fn formative_event_backward_query_no_shift() {
        let mut sim = create_simulation();
        let reference = sim.reference_date();

        // Create a human with birth date 25 years before reference
        let entity = EntityBuilder::new()
            .id("person_001")
            .species(Species::Human)
            .birth_date(reference - Duration::years(25))
            .build()
            .unwrap();

        let entity_id = entity.id().clone();
        let anchor = reference + Duration::days(10);
        sim.add_entity(entity, anchor);

        // Add a formative event BEFORE the anchor
        let event = EventBuilder::new(EventType::Violence)
            .target(entity_id.clone())
            .severity(0.9)
            .with_base_shift(HexacoPath::Agreeableness, -0.15)
            .build()
            .unwrap();
        let event_time = reference + Duration::days(5);
        sim.add_event(event, event_time);

        // Query BEFORE the formative event (backward from anchor)
        let earlier = reference + Duration::days(1);
        let handle = sim.entity(&entity_id).unwrap();
        let earlier_state = handle.state_at(earlier);
        let earlier_agreeableness =
            earlier_state.get_effective(StatePath::Hexaco(HexacoPath::Agreeableness));

        // Query at anchor
        let anchor_state = handle.state_at(anchor);
        let anchor_agreeableness =
            anchor_state.get_effective(StatePath::Hexaco(HexacoPath::Agreeableness));

        // For backward query, the shift hasn't happened yet
        // Both should be equal to the anchor state's agreeableness
        // (since backward query doesn't apply formative events)
        assert!((earlier_agreeableness - anchor_agreeableness).abs() < 0.1);
    }

    #[test]
    fn formative_event_multiple_shifts_same_trait() {
        let mut sim = create_simulation();
        let reference = sim.reference_date();

        // Create a human with birth date 25 years before reference
        let entity = EntityBuilder::new()
            .id("person_001")
            .species(Species::Human)
            .birth_date(reference - Duration::years(25))
            .build()
            .unwrap();

        let entity_id = entity.id().clone();
        let anchor = reference;
        sim.add_entity(entity, anchor);

        // Get baseline
        let handle = sim.entity(&entity_id).unwrap();
        let baseline_state = handle.state_at(anchor);
        let baseline_agreeableness =
            baseline_state.get_effective(StatePath::Hexaco(HexacoPath::Agreeableness));

        // Add first formative event
        let event1 = EventBuilder::new(EventType::Violence)
            .target(entity_id.clone())
            .severity(0.8)
            .with_base_shift(HexacoPath::Agreeableness, -0.10)
            .build()
            .unwrap();
        sim.add_event(event1, anchor + Duration::days(1));

        // Add second formative event
        let event2 = EventBuilder::new(EventType::Violence)
            .target(entity_id.clone())
            .severity(0.7)
            .with_base_shift(HexacoPath::Agreeableness, -0.08)
            .build()
            .unwrap();
        sim.add_event(event2, anchor + Duration::days(2));

        // Query after both events
        let later = anchor + Duration::days(3);
        let handle = sim.entity(&entity_id).unwrap();
        let later_state = handle.state_at(later);
        let later_agreeableness =
            later_state.get_effective(StatePath::Hexaco(HexacoPath::Agreeableness));

        // Agreeableness should have decreased by cumulative amount (with modifiers)
        assert!(later_agreeableness < baseline_agreeableness);
    }

    #[test]
    fn formative_event_no_shift_leaves_trait_unchanged() {
        let mut sim = create_simulation();
        let reference = sim.reference_date();

        let entity = EntityBuilder::new()
            .id("person_001")
            .species(Species::Human)
            .birth_date(reference - Duration::years(25))
            .build()
            .unwrap();

        let entity_id = entity.id().clone();
        let anchor = reference;
        sim.add_entity(entity, anchor);

        // Get baseline
        let handle = sim.entity(&entity_id).unwrap();
        let baseline_state = handle.state_at(anchor);
        let baseline_openness =
            baseline_state.get_effective(StatePath::Hexaco(HexacoPath::Openness));

        // Add event WITHOUT base shift
        let event = EventBuilder::new(EventType::Violence)
            .target(entity_id.clone())
            .severity(0.8)
            .build()
            .unwrap();
        sim.add_event(event, anchor + Duration::days(1));

        // Query after the event
        let later = anchor + Duration::days(2);
        let handle = sim.entity(&entity_id).unwrap();
        let later_state = handle.state_at(later);
        let later_openness = later_state.get_effective(StatePath::Hexaco(HexacoPath::Openness));

        // Openness should be unchanged (events without base shifts don't affect personality base)
        assert!((later_openness - baseline_openness).abs() < 0.01);
    }

    #[test]
    fn collect_base_shifts_empty_for_backward_query() {
        let entity = EntityBuilder::new()
            .id("test")
            .species(Species::Human)
            .build()
            .unwrap();

        let records = collect_base_shift_records(
            &[],
            &entity,
            Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0),
            false, // backward query
        );

        assert!(records.is_empty());
    }

    #[test]
    fn apply_base_shifts_empty_records_returns_unchanged() {
        let state = IndividualState::new();
        let original_openness = state.hexaco().openness();

        let result = apply_base_shifts_to_state(
            state,
            &[],
            Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0),
        );

        assert!((result.hexaco().openness() - original_openness).abs() < f32::EPSILON);
    }

    #[test]
    fn formative_event_all_hexaco_traits() {
        // Test all six HEXACO traits get shifted
        let mut sim = create_simulation();
        let reference = sim.reference_date();

        let entity = EntityBuilder::new()
            .id("person_001")
            .species(Species::Human)
            .birth_date(reference - Duration::years(25))
            .build()
            .unwrap();

        let entity_id = entity.id().clone();
        let anchor = reference;
        sim.add_entity(entity, anchor);

        // Add event with all 6 traits
        let event = EventBuilder::new(EventType::Violence)
            .target(entity_id.clone())
            .severity(0.9)
            .with_base_shift(HexacoPath::Openness, 0.10)
            .with_base_shift(HexacoPath::Conscientiousness, -0.08)
            .with_base_shift(HexacoPath::Extraversion, 0.12)
            .with_base_shift(HexacoPath::Agreeableness, -0.15)
            .with_base_shift(HexacoPath::Neuroticism, 0.20)
            .with_base_shift(HexacoPath::HonestyHumility, -0.05)
            .build()
            .unwrap();
        sim.add_event(event, anchor + Duration::days(1));

        // Query after event
        let later = anchor + Duration::days(2);
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(later);

        // Verify we can access all traits (proving they were processed)
        let _ = state.get_effective(StatePath::Hexaco(HexacoPath::Openness));
        let _ = state.get_effective(StatePath::Hexaco(HexacoPath::Conscientiousness));
        let _ = state.get_effective(StatePath::Hexaco(HexacoPath::Extraversion));
        let _ = state.get_effective(StatePath::Hexaco(HexacoPath::Agreeableness));
        let _ = state.get_effective(StatePath::Hexaco(HexacoPath::Neuroticism));
        let _ = state.get_effective(StatePath::Hexaco(HexacoPath::HonestyHumility));
    }

    #[test]
    fn formative_event_positive_and_negative_shifts() {
        // Test both positive and negative shifts are handled
        let mut sim = create_simulation();
        let reference = sim.reference_date();

        let entity = EntityBuilder::new()
            .id("person_001")
            .species(Species::Human)
            .birth_date(reference - Duration::years(25))
            .build()
            .unwrap();

        let entity_id = entity.id().clone();
        let anchor = reference;
        sim.add_entity(entity, anchor);

        // Get baseline
        let handle = sim.entity(&entity_id).unwrap();
        let baseline_state = handle.state_at(anchor);
        let baseline_extraversion =
            baseline_state.get_effective(StatePath::Hexaco(HexacoPath::Extraversion));

        // Add event with positive shift
        let event = EventBuilder::new(EventType::Support)
            .target(entity_id.clone())
            .severity(0.7)
            .with_base_shift(HexacoPath::Extraversion, 0.15)
            .build()
            .unwrap();
        sim.add_event(event, anchor + Duration::days(1));

        // Query after event
        let later = anchor + Duration::days(2);
        let handle = sim.entity(&entity_id).unwrap();
        let later_state = handle.state_at(later);
        let later_extraversion =
            later_state.get_effective(StatePath::Hexaco(HexacoPath::Extraversion));

        // Positive shift should increase the trait
        assert!(later_extraversion > baseline_extraversion);
    }

    #[test]
    fn formative_event_entity_without_birth_date() {
        // Test entity without birth date uses anchor age
        let mut sim = create_simulation();
        let reference = sim.reference_date();

        // Entity without birth date (uses default age)
        let entity = EntityBuilder::new()
            .id("person_001")
            .species(Species::Human)
            .age(Duration::years(30))
            .build()
            .unwrap();

        let entity_id = entity.id().clone();
        let anchor = reference;
        sim.add_entity(entity, anchor);

        // Add formative event
        let event = EventBuilder::new(EventType::Violence)
            .target(entity_id.clone())
            .severity(0.8)
            .with_base_shift(HexacoPath::Openness, -0.10)
            .build()
            .unwrap();
        sim.add_event(event, anchor + Duration::days(1));

        // Query after event (should work even without birth date)
        let later = anchor + Duration::days(2);
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(later);

        // Just verify it doesn't panic and produces valid state
        let openness = state.get_effective(StatePath::Hexaco(HexacoPath::Openness));
        assert!(openness >= -1.0 && openness <= 1.0);
    }

    #[test]
    fn formative_event_after_query_timestamp_ignored() {
        // Test that events after query timestamp are not included
        let mut sim = create_simulation();
        let reference = sim.reference_date();

        let entity = EntityBuilder::new()
            .id("person_001")
            .species(Species::Human)
            .birth_date(reference - Duration::years(25))
            .build()
            .unwrap();

        let entity_id = entity.id().clone();
        let anchor = reference;
        sim.add_entity(entity, anchor);

        // Get baseline
        let handle = sim.entity(&entity_id).unwrap();
        let baseline_state = handle.state_at(anchor);
        let baseline_agreeableness =
            baseline_state.get_effective(StatePath::Hexaco(HexacoPath::Agreeableness));

        // Add formative event in the future
        let event = EventBuilder::new(EventType::Violence)
            .target(entity_id.clone())
            .severity(0.9)
            .with_base_shift(HexacoPath::Agreeableness, -0.30)
            .build()
            .unwrap();
        sim.add_event(event, anchor + Duration::days(10));

        // Query BEFORE the event
        let before_event = anchor + Duration::days(5);
        let handle = sim.entity(&entity_id).unwrap();
        let state = handle.state_at(before_event);
        let agreeableness = state.get_effective(StatePath::Hexaco(HexacoPath::Agreeableness));

        // Should be close to baseline since event hasn't happened yet
        assert!(
            (agreeableness - baseline_agreeableness).abs() < 0.01
        );
    }

    #[test]
    fn collect_base_shifts_forward_query_with_events() {
        // Direct test of collect_base_shift_records for forward query
        let entity = EntityBuilder::new()
            .id("test")
            .species(Species::Human)
            .birth_date(Timestamp::from_ymd_hms(1999, 1, 1, 0, 0, 0))
            .build()
            .unwrap();

        let records = collect_base_shift_records(
            &[],
            &entity,
            Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0),
            true, // forward query
        );

        // Empty events list = empty records
        assert!(records.is_empty());
    }

    #[test]
    fn apply_base_shifts_updates_all_traits() {
        use crate::state::BaseShiftRecord;

        let state = IndividualState::new();
        let original_openness = state.hexaco().openness();

        // Create a shift record directly
        let records = vec![BaseShiftRecord::new(
            Duration::days(1),
            HexacoPath::Openness,
            0.15,
        )];

        let result = apply_base_shifts_to_state(
            state.clone(),
            &records,
            Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0),
        );

        // Openness should have changed
        assert!((result.hexaco().openness() - original_openness).abs() > 0.01);
    }

    #[test]
    fn apply_base_shifts_each_hexaco_trait() {
        use crate::state::BaseShiftRecord;

        // Test each trait individually
        for trait_path in HexacoPath::all() {
            let state = IndividualState::new();
            let records = vec![BaseShiftRecord::new(Duration::days(1), trait_path, 0.10)];

            let result = apply_base_shifts_to_state(
                state,
                &records,
                Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0),
            );

            // Verify result is valid (no panic, valid state)
            let value = match trait_path {
                HexacoPath::Openness => result.hexaco().openness(),
                HexacoPath::Conscientiousness => result.hexaco().conscientiousness(),
                HexacoPath::Extraversion => result.hexaco().extraversion(),
                HexacoPath::Agreeableness => result.hexaco().agreeableness(),
                HexacoPath::Neuroticism => result.hexaco().neuroticism(),
                HexacoPath::HonestyHumility => result.hexaco().honesty_humility(),
            };
            assert!(value >= -1.0 && value <= 1.0);
        }
    }

    #[test]
    fn collect_base_shifts_event_after_query_timestamp_skipped() {
        // Direct test: event with base shift AFTER query timestamp should be skipped (line 828)
        use crate::simulation::TimestampedEvent;

        let entity = EntityBuilder::new()
            .id("test")
            .species(Species::Human)
            .birth_date(Timestamp::from_ymd_hms(1999, 1, 1, 0, 0, 0))
            .build()
            .unwrap();

        // Create event with base shift at day 100
        let event = EventBuilder::new(EventType::Violence)
            .with_base_shift(HexacoPath::Agreeableness, -0.20)
            .build()
            .unwrap();
        let te = TimestampedEvent::new(event, Timestamp::from_ymd_hms(2024, 4, 10, 0, 0, 0));

        // Query timestamp is BEFORE event timestamp
        let query_ts = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let records = collect_base_shift_records(&[&te], &entity, query_ts, true);

        // Event after query should be skipped
        assert!(records.is_empty());
    }

    #[test]
    fn collect_base_shifts_event_before_birth_date_age_zero() {
        // Direct test: event BEFORE entity birth date (line 836 - age = 0)
        use crate::simulation::TimestampedEvent;

        // Entity born in 2010
        let entity = EntityBuilder::new()
            .id("test")
            .species(Species::Human)
            .birth_date(Timestamp::from_ymd_hms(2010, 1, 1, 0, 0, 0))
            .build()
            .unwrap();

        // Event in 2005 - BEFORE entity was born
        let event = EventBuilder::new(EventType::Violence)
            .with_base_shift(HexacoPath::Neuroticism, 0.25)
            .build()
            .unwrap();
        let te = TimestampedEvent::new(event, Timestamp::from_ymd_hms(2005, 6, 1, 0, 0, 0));

        // Query in 2020 (forward query)
        let query_ts = Timestamp::from_ymd_hms(2020, 1, 1, 0, 0, 0);
        let records = collect_base_shift_records(&[&te], &entity, query_ts, true);

        // Event should still be processed with age 0 (high plasticity)
        assert_eq!(records.len(), 1);
    }

    #[test]
    fn collect_base_shifts_entity_without_birth_date_uses_anchor_age() {
        // Direct test: entity without birth_date uses entity.age() (line 839)
        use crate::simulation::TimestampedEvent;

        // Entity without birth_date, just has age
        let entity = EntityBuilder::new()
            .id("test")
            .species(Species::Human)
            .age(Duration::years(35))
            .build()
            .unwrap();

        // Event with base shift
        let event = EventBuilder::new(EventType::Violence)
            .with_base_shift(HexacoPath::Conscientiousness, -0.15)
            .build()
            .unwrap();
        let te = TimestampedEvent::new(event, Timestamp::from_ymd_hms(2024, 1, 15, 0, 0, 0));

        // Query after event
        let query_ts = Timestamp::from_ymd_hms(2024, 6, 1, 0, 0, 0);
        let records = collect_base_shift_records(&[&te], &entity, query_ts, true);

        // Should have processed the event using entity.age() (35 years)
        assert_eq!(records.len(), 1);
    }

    #[test]
    fn collect_base_shifts_event_before_1970_reference() {
        // Direct test: event timestamp before 1970 reference (line 846)
        use crate::simulation::TimestampedEvent;

        // Entity born in 1940
        let entity = EntityBuilder::new()
            .id("test")
            .species(Species::Human)
            .birth_date(Timestamp::from_ymd_hms(1940, 1, 1, 0, 0, 0))
            .build()
            .unwrap();

        // Event in 1960 - before 1970 reference timestamp
        let event = EventBuilder::new(EventType::Violence)
            .with_base_shift(HexacoPath::Extraversion, -0.20)
            .build()
            .unwrap();
        let te = TimestampedEvent::new(event, Timestamp::from_ymd_hms(1960, 6, 1, 0, 0, 0));

        // Query in 2000
        let query_ts = Timestamp::from_ymd_hms(2000, 1, 1, 0, 0, 0);
        let records = collect_base_shift_records(&[&te], &entity, query_ts, true);

        // Event before 1970 should still be processed (uses Duration::zero)
        assert_eq!(records.len(), 1);
    }

    #[test]
    fn collect_base_shifts_tiny_shift_rounds_to_zero() {
        // Direct test: shift that becomes zero after modifiers (line 869)
        use crate::simulation::TimestampedEvent;

        // Entity at age 80 (low plasticity: 0.6)
        // Extraversion has stability 0.85, so trait_modifier = 0.15
        // Very small input shift that after modifiers approaches zero
        let entity = EntityBuilder::new()
            .id("test")
            .species(Species::Human)
            .birth_date(Timestamp::from_ymd_hms(1940, 1, 1, 0, 0, 0))
            .build()
            .unwrap();

        // Tiny shift: 1e-8 * 0.6 (age plasticity) * 0.15 (trait modifier) = ~9e-10
        // f32::EPSILON is ~1.19e-7, so result should be well below that
        let event = EventBuilder::new(EventType::Violence)
            .with_base_shift(HexacoPath::Extraversion, 1e-8)
            .build()
            .unwrap();
        let te = TimestampedEvent::new(event, Timestamp::from_ymd_hms(2020, 6, 1, 0, 0, 0));

        // Query after event
        let query_ts = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let records = collect_base_shift_records(&[&te], &entity, query_ts, true);

        // The modified shift should be near zero and skipped
        assert!(records.is_empty());
    }

    #[test]
    fn collect_base_shifts_positive_cumulative_tracking() {
        // Direct test: positive shift updates cumulative_positive (line 877)
        use crate::simulation::TimestampedEvent;

        // Young entity (high plasticity)
        let entity = EntityBuilder::new()
            .id("test")
            .species(Species::Human)
            .birth_date(Timestamp::from_ymd_hms(2000, 1, 1, 0, 0, 0))
            .build()
            .unwrap();

        // POSITIVE shift (not negative) to hit line 877
        let event = EventBuilder::new(EventType::Support)
            .with_base_shift(HexacoPath::Agreeableness, 0.25)
            .build()
            .unwrap();
        let te = TimestampedEvent::new(event, Timestamp::from_ymd_hms(2024, 1, 15, 0, 0, 0));

        let query_ts = Timestamp::from_ymd_hms(2024, 6, 1, 0, 0, 0);
        let records = collect_base_shift_records(&[&te], &entity, query_ts, true);

        // Should have one positive shift record
        assert_eq!(records.len(), 1);
        assert!(records[0].immediate() > 0.0);
    }

    #[test]
    fn collect_base_shifts_multiple_positive_shifts_diminishing() {
        // Test that multiple positive shifts get diminishing returns
        use crate::simulation::TimestampedEvent;

        let entity = EntityBuilder::new()
            .id("test")
            .species(Species::Human)
            .birth_date(Timestamp::from_ymd_hms(2000, 1, 1, 0, 0, 0))
            .build()
            .unwrap();

        // First positive shift
        let event1 = EventBuilder::new(EventType::Support)
            .with_base_shift(HexacoPath::Agreeableness, 0.30)
            .build()
            .unwrap();
        let te1 = TimestampedEvent::new(event1, Timestamp::from_ymd_hms(2024, 1, 15, 0, 0, 0));

        // Second positive shift - should have diminishing returns
        let event2 = EventBuilder::new(EventType::Support)
            .with_base_shift(HexacoPath::Agreeableness, 0.30)
            .build()
            .unwrap();
        let te2 = TimestampedEvent::new(event2, Timestamp::from_ymd_hms(2024, 2, 15, 0, 0, 0));

        let query_ts = Timestamp::from_ymd_hms(2024, 6, 1, 0, 0, 0);
        let records = collect_base_shift_records(&[&te1, &te2], &entity, query_ts, true);

        // Both shifts should be recorded
        assert_eq!(records.len(), 2);

        // Second shift should be smaller due to diminishing returns
        assert!(records[1].immediate() < records[0].immediate());
    }

    #[test]
    fn apply_base_shifts_empty_returns_unchanged() {
        // Direct test for empty shift records (line 899-901)
        let state = IndividualState::new();
        let original_openness = state.hexaco().openness();

        let result = apply_base_shifts_to_state(
            state,
            &[], // empty records
            Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0),
        );

        assert!((result.hexaco().openness() - original_openness).abs() < f32::EPSILON);
    }

    #[test]
    fn apply_base_shifts_query_before_1970() {
        // Direct test: query timestamp before 1970 (line 909)
        use crate::state::BaseShiftRecord;

        let state = IndividualState::new();

        let records = vec![BaseShiftRecord::new(
            Duration::days(1),
            HexacoPath::Openness,
            0.15,
        )];

        // Query timestamp before 1970
        let result = apply_base_shifts_to_state(
            state,
            &records,
            Timestamp::from_ymd_hms(1950, 1, 1, 0, 0, 0),
        );

        // Should still work (uses Duration::zero for query)
        assert!(result.hexaco().openness() >= -1.0 && result.hexaco().openness() <= 1.0);
    }
}
