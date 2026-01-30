//! Ecological context system for Bronfenbrenner's bioecological model.
//!
//! This module implements the five nested ecological layers that influence
//! human development:
//!
//! - **Microsystem**: Immediate environments with face-to-face interactions
//! - **Mesosystem**: Computed linkages between microsystems (spillover, role conflict)
//! - **Exosystem**: Settings that affect the individual indirectly
//! - **Macrosystem**: Overarching cultural, societal, and ideological patterns
//! - **Chronosystem**: Temporal dimension including life transitions and historical events
//!
//! # Design
//!
//! Entity holds a single `context: EcologicalContext` field that composes all 5 layers.
//! Microsystem values are static f64 (not StateValue) - they change via explicit events,
//! not gradual decay. Mesosystem values are always computed from microsystem data.

mod chronosystem;
mod effects;
mod exosystem;
mod macrosystem;
mod mesosystem;
mod microsystem;

pub use chronosystem::{
    ChronosystemContext, CohortEffects, CriticalPeriod, HistoricalPeriod, NonNormativeEvent,
    NormativeTransition, TurningPoint, TurningPointDomain,
};
pub(crate) use effects::apply_context_effects;
pub use exosystem::{ExosystemContext, ParentWorkQuality};
pub use macrosystem::{
    CulturalOrientation, InstitutionalStructure, MacrosystemConstraintSet, MacrosystemContext,
};
pub use mesosystem::{
    check_proximal_process_gate, MesosystemCache, MesosystemLinkage, MesosystemState,
    ProximalProcessGateError, INTERACTION_COMPLEXITY_THRESHOLD, INTERACTION_FREQUENCY_THRESHOLD,
};
pub use microsystem::{
    EducationContext, FamilyContext, FamilyRole, HealthcareContext, InteractionProfile,
    Microsystem, MicrosystemType, NeighborhoodContext, ReligiousContext, SocialContext,
    WorkContext,
};

use crate::enums::ContextPath;
use crate::types::MicrosystemId;
use std::collections::HashMap;

/// Aggregate container for all ecological context layers.
///
/// This struct composes all five Bronfenbrenner layers and provides
/// unified access to context dimensions.
///
/// # Microsystem Storage
///
/// Microsystems are stored in a HashMap keyed by MicrosystemId. This allows:
/// - Multiple microsystems of the same type (e.g., two jobs)
/// - O(1) lookup by ID
/// - Efficient iteration for mesosystem computation
///
/// # Mesosystem Computation
///
/// Mesosystem values are computed from microsystem data and stored as a
/// derived snapshot for query. A cache is maintained for linkage computations,
/// invalidated per-simulation-step.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::context::{EcologicalContext, Microsystem, MicrosystemType, WorkContext};
/// use behavioral_pathways::types::MicrosystemId;
///
/// let mut context = EcologicalContext::default();
///
/// // Add a work microsystem
/// let work_id = MicrosystemId::new("work_acme").unwrap();
/// let work = Microsystem::new_work(WorkContext::default());
/// context.add_microsystem(work_id.clone(), work);
///
/// // Query microsystem count
/// assert_eq!(context.microsystem_count(), 1);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct EcologicalContext {
    /// Microsystem instances keyed by ID.
    microsystems: HashMap<MicrosystemId, Microsystem>,

    /// Exosystem context (indirect influences).
    exosystem: ExosystemContext,

    /// Macrosystem context (cultural patterns).
    macrosystem: MacrosystemContext,

    /// Chronosystem context (temporal dimension).
    chronosystem: ChronosystemContext,

    /// Cached mesosystem linkages (computed from microsystems).
    mesosystem_cache: MesosystemCache,

    /// Stored mesosystem state computed from microsystems.
    mesosystem_state: MesosystemState,
}

impl EcologicalContext {
    /// Creates a new EcologicalContext with default values.
    #[must_use]
    pub fn new() -> Self {
        EcologicalContext {
            microsystems: HashMap::new(),
            exosystem: ExosystemContext::default(),
            macrosystem: MacrosystemContext::default(),
            chronosystem: ChronosystemContext::default(),
            mesosystem_cache: MesosystemCache::new(),
            mesosystem_state: MesosystemState::default(),
        }
    }

    // --- Microsystem Management ---

    /// Adds a microsystem with the given ID.
    ///
    /// If a microsystem with the same ID already exists, it is replaced.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for this microsystem instance
    /// * `microsystem` - The microsystem to add
    pub fn add_microsystem(&mut self, id: MicrosystemId, microsystem: Microsystem) {
        self.microsystems.insert(id, microsystem);
        self.mesosystem_cache.invalidate();
    }

    /// Gets a reference to a microsystem by ID.
    #[must_use]
    pub fn get_microsystem(&self, id: &MicrosystemId) -> Option<&Microsystem> {
        self.microsystems.get(id)
    }

    /// Gets a mutable reference to a microsystem by ID.
    pub fn get_microsystem_mut(&mut self, id: &MicrosystemId) -> Option<&mut Microsystem> {
        self.microsystems.get_mut(id)
    }

    /// Removes a microsystem by ID.
    ///
    /// Returns the removed microsystem if it existed.
    pub fn remove_microsystem(&mut self, id: &MicrosystemId) -> Option<Microsystem> {
        let result = self.microsystems.remove(id);
        if result.is_some() {
            self.mesosystem_cache.invalidate();
        }
        result
    }

    /// Lists all microsystem IDs of a given type.
    #[must_use]
    pub fn list_microsystems(&self, microsystem_type: MicrosystemType) -> Vec<MicrosystemId> {
        self.microsystems
            .iter()
            .filter(|(_, m)| m.microsystem_type() == microsystem_type)
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Returns the total number of microsystems.
    #[must_use]
    pub fn microsystem_count(&self) -> usize {
        self.microsystems.len()
    }

    /// Returns true if no microsystems are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.microsystems.is_empty()
    }

    /// Returns an iterator over all microsystem entries.
    pub fn microsystems_iter(&self) -> impl Iterator<Item = (&MicrosystemId, &Microsystem)> {
        self.microsystems.iter()
    }

    // --- Layer Accessors ---

    /// Returns a reference to the exosystem context.
    #[must_use]
    pub fn exosystem(&self) -> &ExosystemContext {
        &self.exosystem
    }

    /// Returns a mutable reference to the exosystem context.
    pub fn exosystem_mut(&mut self) -> &mut ExosystemContext {
        &mut self.exosystem
    }

    /// Returns a reference to the macrosystem context.
    #[must_use]
    pub fn macrosystem(&self) -> &MacrosystemContext {
        &self.macrosystem
    }

    /// Returns a mutable reference to the macrosystem context.
    pub fn macrosystem_mut(&mut self) -> &mut MacrosystemContext {
        &mut self.macrosystem
    }

    /// Returns a reference to the chronosystem context.
    #[must_use]
    pub fn chronosystem(&self) -> &ChronosystemContext {
        &self.chronosystem
    }

    /// Returns a mutable reference to the chronosystem context.
    pub fn chronosystem_mut(&mut self) -> &mut ChronosystemContext {
        &mut self.chronosystem
    }

    // --- Mesosystem Access ---

    /// Returns a reference to the mesosystem cache.
    #[must_use]
    pub fn mesosystem_cache(&self) -> &MesosystemCache {
        &self.mesosystem_cache
    }

    /// Computes and returns the mesosystem state snapshot.
    ///
    /// This recomputes and stores mesosystem values derived from current
    /// microsystems for later query.
    #[must_use]
    pub fn mesosystem_state(&mut self) -> &MesosystemState {
        self.mesosystem_state = MesosystemState::compute(&self.microsystems);
        &self.mesosystem_state
    }

    /// Returns the last computed mesosystem state snapshot.
    #[must_use]
    pub fn mesosystem_state_cached(&self) -> &MesosystemState {
        &self.mesosystem_state
    }

    /// Invalidates the mesosystem cache.
    ///
    /// This is called internally during state computation to ensure
    /// mesosystem values are recomputed for the next query.
    pub fn invalidate_mesosystem_cache(&mut self) {
        self.mesosystem_cache.invalidate();
    }

    /// Computes spillover from one microsystem to another.
    ///
    /// Spillover represents how stress or satisfaction in one domain
    /// affects another. The direction matters: work->home differs from home->work.
    ///
    /// # Arguments
    ///
    /// * `from` - Source microsystem ID
    /// * `to` - Target microsystem ID
    ///
    /// # Returns
    ///
    /// Spillover coefficient in range [0.0, 1.0], or 0.0 if either microsystem
    /// does not exist.
    #[must_use]
    pub fn get_spillover(&self, from: &MicrosystemId, to: &MicrosystemId) -> f64 {
        self.mesosystem_cache
            .get_spillover(from, to, &self.microsystems)
    }

    /// Computes role conflict between two microsystems.
    ///
    /// Role conflict is symmetric - the conflict between work and family
    /// is the same regardless of direction.
    ///
    /// # Arguments
    ///
    /// * `context_a` - First microsystem ID
    /// * `context_b` - Second microsystem ID
    ///
    /// # Returns
    ///
    /// Role conflict score in range [0.0, 1.0], or 0.0 if either microsystem
    /// does not exist.
    #[must_use]
    pub fn get_role_conflict(&self, context_a: &MicrosystemId, context_b: &MicrosystemId) -> f64 {
        self.mesosystem_cache
            .get_role_conflict(context_a, context_b, &self.microsystems)
    }

    /// Lists all active mesosystem linkages.
    ///
    /// Returns pairs of microsystem IDs that have interactions.
    #[must_use]
    pub fn list_linkages(&self) -> Vec<(MicrosystemId, MicrosystemId)> {
        self.mesosystem_cache.list_linkages(&self.microsystems)
    }

    // --- Bidirectional Context Processing ---

    /// Computes the aggregate social warmth across all microsystems.
    ///
    /// This is used in person-to-context shaping: high extraversion entities
    /// tend to increase social warmth in their environments.
    ///
    /// Returns the average warmth across all social microsystems,
    /// or 0.5 if no social microsystems exist.
    #[must_use]
    pub fn aggregate_social_warmth(&self) -> f64 {
        let social_contexts: Vec<_> = self
            .microsystems
            .values()
            .filter_map(|m| m.social())
            .collect();

        if social_contexts.is_empty() {
            0.5
        } else {
            let sum: f64 = social_contexts.iter().map(|s| s.warmth).sum();
            sum / social_contexts.len() as f64
        }
    }

    /// Computes the aggregate stress level across all microsystems.
    ///
    /// This is used for context-to-person effects: high aggregate stress
    /// can increase the person's stress levels.
    #[must_use]
    pub fn aggregate_stress(&self) -> f64 {
        if self.microsystems.is_empty() {
            return 0.0;
        }

        let total_stress: f64 = self.microsystems.values().map(|m| m.stress_level()).sum();
        total_stress / self.microsystems.len() as f64
    }

    /// Computes the aggregate hostility across all microsystems.
    ///
    /// Returns the average hostility across all microsystems, or 0.0 if none exist.
    #[must_use]
    pub fn aggregate_hostility(&self) -> f64 {
        if self.microsystems.is_empty() {
            return 0.0;
        }

        let total_hostility: f64 = self.microsystems.values().map(|m| m.hostility()).sum();
        total_hostility / self.microsystems.len() as f64
    }

    /// Applies person-to-context shaping effects.
    ///
    /// High extraversion increases warmth in social microsystems.
    /// This is called SECOND in the bidirectional processing order.
    ///
    /// # Arguments
    ///
    /// * `extraversion` - The entity's extraversion level (-1.0 to 1.0)
    /// * `conscientiousness` - The entity's conscientiousness level (-1.0 to 1.0)
    /// * `agreeableness` - The entity's agreeableness level (-1.0 to 1.0)
    /// * `neuroticism` - The entity's neuroticism level (-1.0 to 1.0)
    /// * `grievance` - The entity's grievance level (0.0 to 1.0)
    pub fn apply_person_to_context_shaping(
        &mut self,
        extraversion: f32,
        conscientiousness: f32,
        agreeableness: f32,
        neuroticism: f32,
        grievance: f32,
    ) {
        // High extraversion (> 0.3) increases social warmth
        if extraversion > 0.3 {
            let boost = f64::from(extraversion - 0.3) * 0.1; // Max ~0.07 boost

            for microsystem in self.microsystems.values_mut() {
                if let Some(social) = microsystem.social_mut() {
                    social.warmth = (social.warmth + boost).min(1.0);
                }
            }
        }

        // High conscientiousness (> 0.3) increases work structure
        if conscientiousness > 0.3 {
            let boost = f64::from(conscientiousness - 0.3) * 0.1;

            for microsystem in self.microsystems.values_mut() {
                if let Some(work) = microsystem.work_mut() {
                    work.role_clarity = (work.role_clarity + boost).min(1.0);
                    work.predictability = (work.predictability + boost * 0.5).min(1.0);
                }
            }
        }

        // High agreeableness (> 0.3) increases family warmth
        if agreeableness > 0.3 {
            let boost = f64::from(agreeableness - 0.3) * 0.1;

            for microsystem in self.microsystems.values_mut() {
                if let Some(family) = microsystem.family_mut() {
                    family.warmth = (family.warmth + boost).min(1.0);
                }
            }
        }

        // High neuroticism (> 0.3) reduces tolerance for instability
        if neuroticism > 0.3 {
            let penalty = f64::from(neuroticism - 0.3) * 0.1;

            for microsystem in self.microsystems.values_mut() {
                if let Some(work) = microsystem.work_mut() {
                    work.stability = (work.stability - penalty).max(0.0);
                    work.predictability = (work.predictability - penalty).max(0.0);
                }
                if let Some(family) = microsystem.family_mut() {
                    family.stability = (family.stability - penalty).max(0.0);
                    family.predictability = (family.predictability - penalty).max(0.0);
                }
            }
        }

        // High grievance (> 0.3) increases perceived hostility
        if grievance > 0.3 {
            let boost = f64::from(grievance - 0.3) * 0.1;

            for microsystem in self.microsystems.values_mut() {
                if let Some(work) = microsystem.work_mut() {
                    work.hostility = (work.hostility + boost).min(1.0);
                }
                if let Some(family) = microsystem.family_mut() {
                    family.hostility = (family.hostility + boost).min(1.0);
                }
                if let Some(social) = microsystem.social_mut() {
                    social.hostility = (social.hostility + boost).min(1.0);
                }
            }
        }
    }

    /// Computes context effects on person state.
    ///
    /// Returns adjustments to stress and loneliness based on context.
    /// This is called FIRST in the bidirectional processing order.
    ///
    /// # Arguments
    ///
    /// * `relationship_quality` - Average relationship quality (0.0 to 1.0)
    ///
    /// # Returns
    ///
    /// Tuple of (stress_adjustment, loneliness_adjustment) to be applied to state.
    #[must_use]
    pub fn compute_context_to_person_effects(&self, relationship_quality: f64) -> (f32, f32) {
        let aggregate_stress = self.aggregate_stress();
        let social_warmth = self.aggregate_social_warmth();
        let aggregate_hostility = self.aggregate_hostility();

        // Context stress increases person stress
        // Scaled by 0.1 to avoid overwhelming individual state
        let mut stress_adjustment = (aggregate_stress * 0.1) as f32;

        // Low social warmth + low relationship quality increases loneliness
        // Scaled modestly to avoid overwhelming individual state
        let loneliness_factor = (1.0 - social_warmth) * (1.0 - relationship_quality);
        let loneliness_adjustment = (loneliness_factor * 0.1) as f32;

        // Low relationship quality increases hostility perception
        let hostility_factor = aggregate_hostility * (1.0 - relationship_quality);
        stress_adjustment += (hostility_factor * 0.05) as f32;

        (stress_adjustment, loneliness_adjustment)
    }

    // --- Context Value Access ---

    /// Gets a context value by path.
    ///
    /// # Arguments
    ///
    /// * `path` - The context path to query
    ///
    /// # Returns
    ///
    /// The value at the path, or None if the path does not exist
    /// (e.g., microsystem ID not found).
    #[must_use]
    pub fn get(&self, path: &ContextPath) -> Option<f64> {
        match path {
            ContextPath::Microsystem(id, mpath) => {
                self.microsystems.get(id).map(|m| m.get_value(mpath))
            }
            ContextPath::Exosystem(epath) => Some(self.exosystem.get_value(epath)),
            ContextPath::Macrosystem(mpath) => Some(self.macrosystem.get_value(mpath)),
            ContextPath::Chronosystem(cpath) => Some(self.chronosystem.get_value(cpath)),
        }
    }

    /// Sets a context value by path.
    ///
    /// # Arguments
    ///
    /// * `path` - The context path to modify
    /// * `value` - The new value
    ///
    /// # Returns
    ///
    /// `true` if the value was set, `false` if the path does not exist
    /// (e.g., microsystem ID not found).
    pub fn set(&mut self, path: &ContextPath, value: f64) -> bool {
        match path {
            ContextPath::Microsystem(id, mpath) => {
                if let Some(m) = self.microsystems.get_mut(id) {
                    m.set_value(mpath, value);
                    // Note: We don't invalidate cache here - that happens at end of advance()
                    true
                } else {
                    false
                }
            }
            ContextPath::Exosystem(epath) => {
                self.exosystem.set_value(epath, value);
                true
            }
            ContextPath::Macrosystem(mpath) => {
                self.macrosystem.set_value(mpath, value);
                true
            }
            ContextPath::Chronosystem(cpath) => {
                self.chronosystem.set_value(cpath, value);
                true
            }
        }
    }
}

impl Default for EcologicalContext {
    fn default() -> Self {
        EcologicalContext::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::{ChronosystemPath, ExosystemPath, MacrosystemPath, MicrosystemPath};

    #[test]
    fn ecological_context_creation_default() {
        let context = EcologicalContext::default();
        assert!(context.is_empty());
        assert_eq!(context.microsystem_count(), 0);
    }

    #[test]
    fn ecological_context_new() {
        let context = EcologicalContext::new();
        assert!(context.is_empty());
    }

    #[test]
    fn add_microsystem() {
        let mut context = EcologicalContext::default();
        let work_id = MicrosystemId::new("work_acme").unwrap();
        let work = Microsystem::new_work(WorkContext::default());

        context.add_microsystem(work_id.clone(), work);

        assert_eq!(context.microsystem_count(), 1);
        assert!(context.get_microsystem(&work_id).is_some());
    }

    #[test]
    fn add_multiple_microsystems() {
        let mut context = EcologicalContext::default();

        let work_id = MicrosystemId::new("work_acme").unwrap();
        let family_id = MicrosystemId::new("family_primary").unwrap();

        context.add_microsystem(
            work_id.clone(),
            Microsystem::new_work(WorkContext::default()),
        );
        context.add_microsystem(
            family_id.clone(),
            Microsystem::new_family(FamilyContext::default()),
        );

        assert_eq!(context.microsystem_count(), 2);
        assert!(context.get_microsystem(&work_id).is_some());
        assert!(context.get_microsystem(&family_id).is_some());
    }

    #[test]
    fn remove_microsystem() {
        let mut context = EcologicalContext::default();
        let work_id = MicrosystemId::new("work_acme").unwrap();

        context.add_microsystem(
            work_id.clone(),
            Microsystem::new_work(WorkContext::default()),
        );
        assert_eq!(context.microsystem_count(), 1);

        let removed = context.remove_microsystem(&work_id);
        assert!(removed.is_some());
        assert_eq!(context.microsystem_count(), 0);
    }

    #[test]
    fn remove_nonexistent_microsystem() {
        let mut context = EcologicalContext::default();
        let work_id = MicrosystemId::new("work_acme").unwrap();

        let removed = context.remove_microsystem(&work_id);
        assert!(removed.is_none());
    }

    #[test]
    fn list_microsystems_by_type() {
        let mut context = EcologicalContext::default();

        let work1 = MicrosystemId::new("work_acme").unwrap();
        let work2 = MicrosystemId::new("work_other").unwrap();
        let family_id = MicrosystemId::new("family_primary").unwrap();

        context.add_microsystem(work1.clone(), Microsystem::new_work(WorkContext::default()));
        context.add_microsystem(work2.clone(), Microsystem::new_work(WorkContext::default()));
        context.add_microsystem(
            family_id.clone(),
            Microsystem::new_family(FamilyContext::default()),
        );

        let work_ids = context.list_microsystems(MicrosystemType::Work);
        assert_eq!(work_ids.len(), 2);

        let family_ids = context.list_microsystems(MicrosystemType::Family);
        assert_eq!(family_ids.len(), 1);
    }

    #[test]
    fn get_microsystem_mut() {
        let mut context = EcologicalContext::default();
        let work_id = MicrosystemId::new("work_acme").unwrap();
        let mut work = WorkContext::default();
        work.workload_stress = 0.3;

        context.add_microsystem(work_id.clone(), Microsystem::new_work(work));

        let microsystem = context.get_microsystem_mut(&work_id).unwrap();
        let work_ref = microsystem.work_mut().unwrap();
        work_ref.workload_stress = 0.8;

        let retrieved = context.get_microsystem(&work_id).unwrap();
        assert!((retrieved.work().unwrap().workload_stress - 0.8).abs() < f64::EPSILON);
    }

    #[test]
    fn microsystems_iter() {
        let mut context = EcologicalContext::default();

        let work_id = MicrosystemId::new("work_acme").unwrap();
        let family_id = MicrosystemId::new("family_primary").unwrap();

        context.add_microsystem(work_id, Microsystem::new_work(WorkContext::default()));
        context.add_microsystem(family_id, Microsystem::new_family(FamilyContext::default()));

        let count = context.microsystems_iter().count();
        assert_eq!(count, 2);
    }

    #[test]
    fn exosystem_accessor() {
        let context = EcologicalContext::default();
        let exo = context.exosystem();
        assert!(exo.resource_availability >= 0.0 && exo.resource_availability <= 1.0);
    }

    #[test]
    fn exosystem_mut_accessor() {
        let mut context = EcologicalContext::default();
        context.exosystem_mut().resource_availability = 0.9;
        assert!((context.exosystem().resource_availability - 0.9).abs() < f64::EPSILON);
    }

    #[test]
    fn macrosystem_accessor() {
        let context = EcologicalContext::default();
        let macro_ctx = context.macrosystem();
        let _ = macro_ctx.cultural_orientation;
    }

    #[test]
    fn macrosystem_mut_accessor() {
        let mut context = EcologicalContext::default();
        context.macrosystem_mut().cultural_stress = 0.4;
        assert!((context.macrosystem().cultural_stress - 0.4).abs() < f64::EPSILON);
    }

    #[test]
    fn chronosystem_accessor() {
        let context = EcologicalContext::default();
        let chrono = context.chronosystem();
        let _ = chrono.historical_period();
    }

    #[test]
    fn chronosystem_mut_accessor() {
        let mut context = EcologicalContext::default();
        context
            .chronosystem_mut()
            .historical_period_mut()
            .stability_level = 0.3;
        assert!(
            (context.chronosystem().historical_period().stability_level - 0.3).abs() < f64::EPSILON
        );
    }

    #[test]
    fn invalidate_mesosystem_cache() {
        let mut context = EcologicalContext::default();
        // Just verify it doesn't panic
        context.invalidate_mesosystem_cache();
    }

    #[test]
    fn context_path_macrosystem_query() {
        let context = EcologicalContext::default();
        let path = ContextPath::Macrosystem(MacrosystemPath::PowerDistance);
        let value = context.get(&path);
        assert!(value.is_some());
        assert!(value.unwrap() >= 0.0 && value.unwrap() <= 1.0);
    }

    #[test]
    fn context_path_exosystem_query() {
        let context = EcologicalContext::default();
        let path = ContextPath::Exosystem(ExosystemPath::ResourceAvailability);
        let value = context.get(&path);
        assert!(value.is_some());
    }

    #[test]
    fn context_path_chronosystem_query() {
        let context = EcologicalContext::default();
        let path = ContextPath::Chronosystem(ChronosystemPath::StabilityLevel);
        let value = context.get(&path);
        assert!(value.is_some());
    }

    #[test]
    fn context_path_microsystem_query() {
        let mut context = EcologicalContext::default();
        let work_id = MicrosystemId::new("work_acme").unwrap();
        let mut work = WorkContext::default();
        work.workload_stress = 0.7;
        context.add_microsystem(work_id.clone(), Microsystem::new_work(work));

        let path = ContextPath::Microsystem(
            work_id,
            MicrosystemPath::Work(crate::enums::WorkPath::WorkloadStress),
        );
        let value = context.get(&path);
        assert!(value.is_some());
        assert!((value.unwrap() - 0.7).abs() < f64::EPSILON);
    }

    #[test]
    fn context_path_microsystem_query_nonexistent() {
        let context = EcologicalContext::default();
        let work_id = MicrosystemId::new("nonexistent").unwrap();
        let path = ContextPath::Microsystem(
            work_id,
            MicrosystemPath::Work(crate::enums::WorkPath::WorkloadStress),
        );
        let value = context.get(&path);
        assert!(value.is_none());
    }

    #[test]
    fn context_set_macrosystem() {
        let mut context = EcologicalContext::default();
        let path = ContextPath::Macrosystem(MacrosystemPath::CulturalStress);

        let result = context.set(&path, 0.6);
        assert!(result);

        let value = context.get(&path).unwrap();
        assert!((value - 0.6).abs() < f64::EPSILON);
    }

    #[test]
    fn context_set_exosystem() {
        let mut context = EcologicalContext::default();
        let path = ContextPath::Exosystem(ExosystemPath::InstitutionalSupport);

        let result = context.set(&path, 0.8);
        assert!(result);

        let value = context.get(&path).unwrap();
        assert!((value - 0.8).abs() < f64::EPSILON);
    }

    #[test]
    fn context_set_chronosystem() {
        let mut context = EcologicalContext::default();
        let path = ContextPath::Chronosystem(ChronosystemPath::ResourceScarcity);

        let result = context.set(&path, 0.4);
        assert!(result);

        let value = context.get(&path).unwrap();
        assert!((value - 0.4).abs() < f64::EPSILON);
    }

    #[test]
    fn context_set_microsystem() {
        let mut context = EcologicalContext::default();
        let work_id = MicrosystemId::new("work_acme").unwrap();
        context.add_microsystem(
            work_id.clone(),
            Microsystem::new_work(WorkContext::default()),
        );

        let path = ContextPath::Microsystem(
            work_id.clone(),
            MicrosystemPath::Work(crate::enums::WorkPath::WorkloadStress),
        );

        let result = context.set(&path, 0.9);
        assert!(result);

        let value = context.get(&path).unwrap();
        assert!((value - 0.9).abs() < f64::EPSILON);
    }

    #[test]
    fn context_set_microsystem_nonexistent() {
        let mut context = EcologicalContext::default();
        let work_id = MicrosystemId::new("nonexistent").unwrap();
        let path = ContextPath::Microsystem(
            work_id,
            MicrosystemPath::Work(crate::enums::WorkPath::WorkloadStress),
        );

        let result = context.set(&path, 0.9);
        assert!(!result);
    }

    #[test]
    fn clone_and_equality() {
        let context1 = EcologicalContext::default();
        let context2 = context1.clone();
        assert_eq!(context1, context2);
    }

    #[test]
    fn debug_format() {
        let context = EcologicalContext::default();
        let debug = format!("{:?}", context);
        assert!(debug.contains("EcologicalContext"));
    }

    #[test]
    fn microsystem_multiple_contexts_aggregate() {
        let mut context = EcologicalContext::default();

        // Add multiple microsystems of different types
        let work_id = MicrosystemId::new("work_primary").unwrap();
        let family_id = MicrosystemId::new("family_primary").unwrap();
        let social_id = MicrosystemId::new("social_friends").unwrap();

        context.add_microsystem(work_id, Microsystem::new_work(WorkContext::default()));
        context.add_microsystem(family_id, Microsystem::new_family(FamilyContext::default()));
        context.add_microsystem(social_id, Microsystem::new_social(SocialContext::default()));

        assert_eq!(context.microsystem_count(), 3);
    }

    #[test]
    fn microsystem_id_identifies_instance_path_identifies_property() {
        let mut context = EcologicalContext::default();

        // Create two work contexts with different stress levels
        let work1_id = MicrosystemId::new("work_job1").unwrap();
        let work2_id = MicrosystemId::new("work_job2").unwrap();

        let mut work1 = WorkContext::default();
        work1.workload_stress = 0.3;
        let mut work2 = WorkContext::default();
        work2.workload_stress = 0.8;

        context.add_microsystem(work1_id.clone(), Microsystem::new_work(work1));
        context.add_microsystem(work2_id.clone(), Microsystem::new_work(work2));

        // MicrosystemId selects WHICH microsystem
        // MicrosystemPath selects WHICH dimension
        let path1 = ContextPath::Microsystem(
            work1_id,
            MicrosystemPath::Work(crate::enums::WorkPath::WorkloadStress),
        );
        let path2 = ContextPath::Microsystem(
            work2_id,
            MicrosystemPath::Work(crate::enums::WorkPath::WorkloadStress),
        );

        let value1 = context.get(&path1).unwrap();
        let value2 = context.get(&path2).unwrap();

        assert!((value1 - 0.3).abs() < f64::EPSILON);
        assert!((value2 - 0.8).abs() < f64::EPSILON);
    }

    // --- Additional coverage tests for mesosystem methods ---

    #[test]
    fn mesosystem_cache_accessor() {
        let context = EcologicalContext::default();
        let cache = context.mesosystem_cache();
        assert!(!cache.is_valid());
    }

    #[test]
    fn mesosystem_state_accessor_recomputes() {
        let mut context = EcologicalContext::default();
        assert_eq!(context.mesosystem_state_cached(), &MesosystemState::default());

        let mut work = WorkContext::default();
        work.role_clarity = 0.2;
        work.predictability = 0.3;
        work.warmth = 0.2;
        work.hostility = 0.1;
        work.workload_stress = 0.8;
        work.interaction_profile.interaction_frequency = 0.8;
        context.add_microsystem(MicrosystemId::new("work").unwrap(), Microsystem::new_work(work));

        let mut social = SocialContext::default();
        social.warmth = 0.8;
        social.predictability = 0.9;
        social.hostility = 0.1;
        social.interaction_profile.interaction_frequency = 0.6;
        context.add_microsystem(
            MicrosystemId::new("social").unwrap(),
            Microsystem::new_social(social),
        );

        let state = context.mesosystem_state();
        assert!((state.work_social_conflict - 0.3).abs() < f64::EPSILON);
        assert!((context.mesosystem_state_cached().work_social_conflict - 0.3).abs() < f64::EPSILON);
    }

    #[test]
    fn ecological_context_get_spillover() {
        let mut context = EcologicalContext::default();
        let work_id = MicrosystemId::new("work").unwrap();
        let family_id = MicrosystemId::new("family").unwrap();

        let mut work = WorkContext::default();
        work.workload_stress = 0.8;
        work.interaction_profile.interaction_frequency = 0.7;
        context.add_microsystem(work_id.clone(), Microsystem::new_work(work));

        let mut family = FamilyContext::default();
        family.predictability = 0.3;
        family.stability = 0.3;
        context.add_microsystem(family_id.clone(), Microsystem::new_family(family));

        let spillover = context.get_spillover(&work_id, &family_id);
        assert!(spillover >= 0.0 && spillover <= 1.0);
    }

    #[test]
    fn ecological_context_get_role_conflict() {
        let mut context = EcologicalContext::default();
        let work_id = MicrosystemId::new("work").unwrap();
        let family_id = MicrosystemId::new("family").unwrap();

        let mut work = WorkContext::default();
        work.workload_stress = 0.8;
        work.interaction_profile.interaction_frequency = 0.8;
        context.add_microsystem(work_id.clone(), Microsystem::new_work(work));

        let mut family = FamilyContext::default();
        family.caregiving_burden = 0.8;
        family.interaction_profile.interaction_frequency = 0.8;
        context.add_microsystem(family_id.clone(), Microsystem::new_family(family));

        let conflict = context.get_role_conflict(&work_id, &family_id);
        assert!(conflict >= 0.0 && conflict <= 1.0);
    }

    #[test]
    fn ecological_context_list_linkages() {
        let mut context = EcologicalContext::default();
        let work_id = MicrosystemId::new("work").unwrap();
        let family_id = MicrosystemId::new("family").unwrap();

        context.add_microsystem(
            work_id.clone(),
            Microsystem::new_work(WorkContext::default()),
        );
        context.add_microsystem(
            family_id.clone(),
            Microsystem::new_family(FamilyContext::default()),
        );

        let linkages = context.list_linkages();
        // Should have 1 linkage between work and family
        assert_eq!(linkages.len(), 1);
    }

    // --- Bidirectional processing tests ---

    #[test]
    fn aggregate_social_warmth_no_social_microsystems() {
        let mut context = EcologicalContext::default();
        let work_id = MicrosystemId::new("work").unwrap();
        context.add_microsystem(work_id, Microsystem::new_work(WorkContext::default()));

        // No social microsystems -> returns default 0.5
        let warmth = context.aggregate_social_warmth();
        assert!((warmth - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn aggregate_social_warmth_with_social_microsystems() {
        let mut context = EcologicalContext::default();

        let social1_id = MicrosystemId::new("social1").unwrap();
        let social2_id = MicrosystemId::new("social2").unwrap();

        let mut social1 = SocialContext::default();
        social1.warmth = 0.8;
        let mut social2 = SocialContext::default();
        social2.warmth = 0.4;

        context.add_microsystem(social1_id, Microsystem::new_social(social1));
        context.add_microsystem(social2_id, Microsystem::new_social(social2));

        let warmth = context.aggregate_social_warmth();
        // (0.8 + 0.4) / 2 = 0.6
        assert!((warmth - 0.6).abs() < f64::EPSILON);
    }

    #[test]
    fn aggregate_stress_empty_context() {
        let context = EcologicalContext::default();
        let stress = context.aggregate_stress();
        assert!((stress - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn aggregate_stress_with_microsystems() {
        let mut context = EcologicalContext::default();

        let work_id = MicrosystemId::new("work").unwrap();
        let mut work = WorkContext::default();
        work.workload_stress = 0.8;
        context.add_microsystem(work_id, Microsystem::new_work(work));

        let stress = context.aggregate_stress();
        // Work stress = 0.8
        assert!(stress > 0.0 && stress <= 1.0);
    }

    #[test]
    fn aggregate_hostility_empty_context() {
        let context = EcologicalContext::default();
        let hostility = context.aggregate_hostility();
        assert!((hostility - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn aggregate_hostility_with_microsystems() {
        let mut context = EcologicalContext::default();
        let work_id = MicrosystemId::new("work").unwrap();
        let mut work = WorkContext::default();
        work.hostility = 0.7;
        context.add_microsystem(work_id, Microsystem::new_work(work));

        let hostility = context.aggregate_hostility();
        assert!((hostility - 0.7).abs() < f64::EPSILON);
    }

    #[test]
    fn apply_person_to_context_shaping_high_extraversion() {
        let mut context = EcologicalContext::default();

        let social_id = MicrosystemId::new("social").unwrap();
        let mut social = SocialContext::default();
        social.warmth = 0.5;
        context.add_microsystem(social_id.clone(), Microsystem::new_social(social));

        // High extraversion should boost warmth
        context.apply_person_to_context_shaping(0.7, 0.0, 0.0, 0.0, 0.0);

        let updated = context
            .get_microsystem(&social_id)
            .unwrap()
            .social()
            .unwrap();
        assert!(updated.warmth > 0.5);
    }

    #[test]
    fn apply_person_to_context_shaping_low_extraversion() {
        let mut context = EcologicalContext::default();

        let social_id = MicrosystemId::new("social").unwrap();
        let mut social = SocialContext::default();
        social.warmth = 0.5;
        context.add_microsystem(social_id.clone(), Microsystem::new_social(social));

        // Low extraversion should not change warmth
        context.apply_person_to_context_shaping(0.2, 0.0, 0.0, 0.0, 0.0);

        let updated = context
            .get_microsystem(&social_id)
            .unwrap()
            .social()
            .unwrap();
        assert!((updated.warmth - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn high_conscientiousness_increases_work_clarity() {
        let mut context = EcologicalContext::default();
        let work_id = MicrosystemId::new("work").unwrap();
        let mut work = WorkContext::default();
        work.role_clarity = 0.4;
        context.add_microsystem(work_id.clone(), Microsystem::new_work(work));

        context.apply_person_to_context_shaping(0.0, 0.7, 0.0, 0.0, 0.0);

        let updated = context
            .get_microsystem(&work_id)
            .unwrap()
            .work()
            .unwrap();
        assert!(updated.role_clarity > 0.4);
    }

    #[test]
    fn conscientiousness_ignores_non_work_microsystems() {
        let mut context = EcologicalContext::default();
        let social_id = MicrosystemId::new("social").unwrap();
        let mut social = SocialContext::default();
        social.warmth = 0.6;
        context.add_microsystem(social_id.clone(), Microsystem::new_social(social));

        context.apply_person_to_context_shaping(0.0, 0.7, 0.0, 0.0, 0.0);

        let updated = context
            .get_microsystem(&social_id)
            .unwrap()
            .social()
            .unwrap();
        assert!((updated.warmth - 0.6).abs() < f64::EPSILON);
    }

    #[test]
    fn high_neuroticism_reduces_stability_tolerance() {
        let mut context = EcologicalContext::default();
        let work_id = MicrosystemId::new("work").unwrap();
        let mut work = WorkContext::default();
        work.stability = 0.8;
        work.predictability = 0.8;
        context.add_microsystem(work_id.clone(), Microsystem::new_work(work));

        context.apply_person_to_context_shaping(0.0, 0.0, 0.0, 0.7, 0.0);

        let updated = context
            .get_microsystem(&work_id)
            .unwrap()
            .work()
            .unwrap();
        assert!(updated.stability < 0.8);
        assert!(updated.predictability < 0.8);
    }

    #[test]
    fn neuroticism_and_grievance_affect_family_context() {
        let mut context = EcologicalContext::default();
        let family_id = MicrosystemId::new("family").unwrap();
        let mut family = FamilyContext::default();
        family.stability = 0.8;
        family.predictability = 0.8;
        family.hostility = 0.2;
        context.add_microsystem(family_id.clone(), Microsystem::new_family(family));

        context.apply_person_to_context_shaping(0.0, 0.0, 0.0, 0.7, 0.8);

        let updated = context
            .get_microsystem(&family_id)
            .unwrap()
            .family()
            .unwrap();
        assert!(updated.stability < 0.8);
        assert!(updated.predictability < 0.8);
        assert!(updated.hostility > 0.2);
    }

    #[test]
    fn high_agreeableness_increases_family_warmth() {
        let mut context = EcologicalContext::default();
        let family_id = MicrosystemId::new("family").unwrap();
        let mut family = FamilyContext::default();
        family.warmth = 0.4;
        context.add_microsystem(family_id.clone(), Microsystem::new_family(family));

        context.apply_person_to_context_shaping(0.0, 0.0, 0.7, 0.0, 0.0);

        let updated = context
            .get_microsystem(&family_id)
            .unwrap()
            .family()
            .unwrap();
        assert!(updated.warmth > 0.4);
    }

    #[test]
    fn agreeableness_ignores_non_family_microsystems() {
        let mut context = EcologicalContext::default();
        let work_id = MicrosystemId::new("work").unwrap();
        let mut work = WorkContext::default();
        work.role_clarity = 0.4;
        context.add_microsystem(work_id.clone(), Microsystem::new_work(work));

        context.apply_person_to_context_shaping(0.0, 0.0, 0.7, 0.0, 0.0);

        let updated = context
            .get_microsystem(&work_id)
            .unwrap()
            .work()
            .unwrap();
        assert!((updated.role_clarity - 0.4).abs() < f64::EPSILON);
    }

    #[test]
    fn high_grievance_increases_perceived_hostility() {
        let mut context = EcologicalContext::default();
        let work_id = MicrosystemId::new("work").unwrap();
        let social_id = MicrosystemId::new("social").unwrap();

        let mut work = WorkContext::default();
        work.hostility = 0.2;
        let mut social = SocialContext::default();
        social.hostility = 0.2;

        context.add_microsystem(work_id.clone(), Microsystem::new_work(work));
        context.add_microsystem(social_id.clone(), Microsystem::new_social(social));

        context.apply_person_to_context_shaping(0.0, 0.0, 0.0, 0.0, 0.8);

        let work_updated = context
            .get_microsystem(&work_id)
            .unwrap()
            .work()
            .unwrap();
        let social_updated = context
            .get_microsystem(&social_id)
            .unwrap()
            .social()
            .unwrap();

        assert!(work_updated.hostility > 0.2);
        assert!(social_updated.hostility > 0.2);
    }

    #[test]
    fn compute_context_to_person_effects_high_stress() {
        let mut context = EcologicalContext::default();

        let work_id = MicrosystemId::new("work").unwrap();
        let mut work = WorkContext::default();
        work.workload_stress = 0.9;
        context.add_microsystem(work_id, Microsystem::new_work(work));

        let (stress_adj, loneliness_adj) = context.compute_context_to_person_effects(0.5);

        // High context stress should produce positive stress adjustment
        assert!(stress_adj > 0.0);
        // Loneliness adjustment depends on social warmth and relationship quality
        assert!(loneliness_adj >= 0.0);
    }

    #[test]
    fn low_relationship_quality_increases_context_hostility() {
        let mut context = EcologicalContext::default();
        let work_id = MicrosystemId::new("work").unwrap();
        let mut work = WorkContext::default();
        work.hostility = 0.8;
        context.add_microsystem(work_id, Microsystem::new_work(work));

        let (high_quality_stress, _) = context.compute_context_to_person_effects(0.9);
        let (low_quality_stress, _) = context.compute_context_to_person_effects(0.2);

        assert!(low_quality_stress > high_quality_stress);
    }

    #[test]
    fn compute_context_to_person_effects_empty_context() {
        let context = EcologicalContext::default();

        let (stress_adj, loneliness_adj) = context.compute_context_to_person_effects(0.5);

        // Empty context should produce minimal adjustments
        assert!((stress_adj - 0.0).abs() < 0.01);
        // With 0.5 relationship quality and 0.5 default social warmth
        // loneliness_factor = 0.5 * 0.5 = 0.25, adjustment = 0.025
        assert!(loneliness_adj < 0.05);
    }

    // --- Required Phase 7 tests ---

    #[test]
    fn ecological_context_entity_integration() {
        // Entity.context field holds EcologicalContext
        use crate::entity::EntityBuilder;
        use crate::enums::Species;

        let entity = EntityBuilder::new()
            .species(Species::Human)
            .build()
            .unwrap();

        // Verify entity has context field accessible
        let context = entity.context();

        // Context is EcologicalContext type - verify it's functional
        assert!(context.is_empty()); // No microsystems initially
        assert_eq!(context.microsystem_count(), 0);

        // Verify all layers are accessible
        let _ = context.exosystem();
        let _ = context.macrosystem();
        let _ = context.chronosystem();
        let _ = context.mesosystem_cache();

        // Verify mutable access works
        let mut entity = entity;
        let work_id = MicrosystemId::new("work_test").unwrap();
        entity.context_mut().add_microsystem(
            work_id.clone(),
            Microsystem::new_work(WorkContext::default()),
        );

        assert_eq!(entity.context().microsystem_count(), 1);
        assert!(entity.context().get_microsystem(&work_id).is_some());
    }
}
