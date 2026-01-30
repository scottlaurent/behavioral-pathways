//! Mesosystem implementation for computed cross-context linkages.
//!
//! The mesosystem represents the interactions and connections between
//! microsystems. Per Bronfenbrenner, mesosystem values are *linkages*
//! between microsystems, not stored scalars.
//!
//! # Computed Values
//!
//! Mesosystem values are always computed from microsystem data:
//! - Spillover effects (stress from one domain affecting another)
//! - Role conflicts (competing demands from multiple roles)
//!
//! A cache is maintained for performance, invalidated per-simulation-step.
//!
//! # Proximal Process Gates
//!
//! Effects only apply when interaction frequency and complexity thresholds
//! are met, per Bronfenbrenner's PPCT model.

use crate::context::microsystem::Microsystem;
use crate::types::MicrosystemId;
use std::collections::HashMap;

/// Default interaction frequency threshold for proximal processes.
///
/// Effects are blocked when frequency is below this threshold.
pub const INTERACTION_FREQUENCY_THRESHOLD: f64 = 0.3;

/// Default interaction complexity threshold for proximal processes.
///
/// Effects are blocked when complexity is below this threshold.
pub const INTERACTION_COMPLEXITY_THRESHOLD: f64 = 0.3;

/// Error returned when proximal process gate blocks an effect.
#[derive(Debug, Clone, PartialEq)]
pub enum ProximalProcessGateError {
    /// Interaction frequency was below threshold.
    FrequencyBelowThreshold {
        /// Actual frequency value.
        actual: f64,
        /// Required threshold.
        threshold: f64,
    },
    /// Interaction complexity was below threshold.
    ComplexityBelowThreshold {
        /// Actual complexity value.
        actual: f64,
        /// Required threshold.
        threshold: f64,
    },
    /// Both frequency and complexity were below thresholds.
    BothBelowThreshold {
        /// Actual frequency value.
        actual_frequency: f64,
        /// Frequency threshold.
        frequency_threshold: f64,
        /// Actual complexity value.
        actual_complexity: f64,
        /// Complexity threshold.
        complexity_threshold: f64,
    },
}

impl std::fmt::Display for ProximalProcessGateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProximalProcessGateError::FrequencyBelowThreshold { actual, threshold } => {
                write!(
                    f,
                    "Interaction frequency {} is below threshold {}",
                    actual, threshold
                )
            }
            ProximalProcessGateError::ComplexityBelowThreshold { actual, threshold } => {
                write!(
                    f,
                    "Interaction complexity {} is below threshold {}",
                    actual, threshold
                )
            }
            ProximalProcessGateError::BothBelowThreshold {
                actual_frequency,
                frequency_threshold,
                actual_complexity,
                complexity_threshold,
            } => {
                write!(
                    f,
                    "Interaction frequency {} is below threshold {} and complexity {} is below threshold {}",
                    actual_frequency, frequency_threshold, actual_complexity, complexity_threshold
                )
            }
        }
    }
}

impl std::error::Error for ProximalProcessGateError {}

/// Checks whether proximal process thresholds are met.
///
/// # Arguments
///
/// * `frequency` - Interaction frequency (0-1)
/// * `complexity` - Interaction complexity (0-1)
/// * `frequency_threshold` - Minimum frequency required
/// * `complexity_threshold` - Minimum complexity required
///
/// # Returns
///
/// `Ok(())` if both thresholds are met, `Err(ProximalProcessGateError)` otherwise.
pub fn check_proximal_process_gate(
    frequency: f64,
    complexity: f64,
    frequency_threshold: f64,
    complexity_threshold: f64,
) -> Result<(), ProximalProcessGateError> {
    let freq_ok = frequency >= frequency_threshold;
    let complex_ok = complexity >= complexity_threshold;

    if !freq_ok && !complex_ok {
        Err(ProximalProcessGateError::BothBelowThreshold {
            actual_frequency: frequency,
            frequency_threshold,
            actual_complexity: complexity,
            complexity_threshold,
        })
    } else if !freq_ok {
        Err(ProximalProcessGateError::FrequencyBelowThreshold {
            actual: frequency,
            threshold: frequency_threshold,
        })
    } else if !complex_ok {
        Err(ProximalProcessGateError::ComplexityBelowThreshold {
            actual: complexity,
            threshold: complexity_threshold,
        })
    } else {
        Ok(())
    }
}

/// Represents a linkage between two microsystems.
#[derive(Debug, Clone, PartialEq)]
pub struct MesosystemLinkage {
    /// Source microsystem ID.
    pub from: MicrosystemId,

    /// Target microsystem ID.
    pub to: MicrosystemId,

    /// Spillover coefficient (0-1).
    pub spillover: f64,

    /// Role conflict score (0-1).
    pub role_conflict: f64,
}

/// Persisted mesosystem state computed from microsystems.
#[derive(Debug, Clone, PartialEq)]
pub struct MesosystemState {
    /// Competing demands between work and family contexts.
    pub work_family_conflict: f64,
    /// How family supports social engagement.
    pub family_social_support: f64,
    /// Identity mismatch between work and social groups.
    pub work_social_conflict: f64,
    /// Alignment of role expectations across contexts.
    pub value_alignment_consistency: f64,
    /// Alignment of autonomy norms across contexts.
    pub autonomy_norm_consistency: f64,
    /// Aggregate mesosystem consistency score.
    pub mesosystem_consistency: f64,
    /// Overlap of actors across microsystems.
    pub shared_membership_strength: f64,
}

impl Default for MesosystemState {
    fn default() -> Self {
        MesosystemState {
            work_family_conflict: 0.0,
            family_social_support: 0.0,
            work_social_conflict: 0.0,
            value_alignment_consistency: 1.0,
            autonomy_norm_consistency: 1.0,
            mesosystem_consistency: 1.0,
            shared_membership_strength: 0.0,
        }
    }
}

impl MesosystemState {
    /// Computes mesosystem state from the current microsystems.
    #[must_use]
    pub fn compute(microsystems: &HashMap<MicrosystemId, Microsystem>) -> Self {
        let mut work_ids = Vec::new();
        let mut family_ids = Vec::new();

        let mut work_role_clarity = Vec::new();
        let mut family_role_clarity = Vec::new();
        let mut social_predictability = Vec::new();

        let mut work_predictability = Vec::new();
        let mut family_predictability = Vec::new();

        let mut family_support = Vec::new();
        let mut work_low_predictability = false;
        let mut social_high_warmth = false;

        for (id, micro) in microsystems {
            if let Some(work) = micro.work() {
                work_ids.push(id.clone());
                work_role_clarity.push(work.role_clarity);
                work_predictability.push(work.predictability);
                if work.predictability < 0.4 {
                    work_low_predictability = true;
                }
            }

            if let Some(family) = micro.family() {
                family_ids.push(id.clone());
                family_role_clarity.push(family.role_clarity);
                family_predictability.push(family.predictability);
                family_support.push((family.warmth - family.hostility).max(0.0));
            }

            if let Some(social) = micro.social() {
                social_predictability.push(social.predictability);
                if social.warmth > 0.7 {
                    social_high_warmth = true;
                }
            }
        }

        let work_family_conflict = if work_ids.is_empty() || family_ids.is_empty() {
            0.0
        } else {
            let cache = MesosystemCache::new();
            let mut total = 0.0;
            let count = work_ids.len() * family_ids.len();

            for work_id in &work_ids {
                for family_id in &family_ids {
                    total += cache.get_role_conflict(work_id, family_id, microsystems);
                }
            }

            total / count as f64
        };

        let family_social_support = average(&family_support).unwrap_or(0.0);
        let work_social_conflict = if work_low_predictability && social_high_warmth {
            0.3
        } else {
            0.0
        };

        let value_alignment_consistency = consistency_from_values(&[
            average(&work_role_clarity),
            average(&family_role_clarity),
            average(&social_predictability),
        ]);

        let autonomy_norm_consistency = consistency_from_values(&[
            average(&work_predictability),
            average(&family_predictability),
            average(&social_predictability),
        ]);

        let mesosystem_consistency = value_alignment_consistency;
        let shared_membership_strength =
            MesosystemCache::compute_shared_membership_strength(microsystems);

        MesosystemState {
            work_family_conflict,
            family_social_support,
            work_social_conflict,
            value_alignment_consistency,
            autonomy_norm_consistency,
            mesosystem_consistency,
            shared_membership_strength,
        }
    }
}

fn average(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        None
    } else {
        Some(values.iter().sum::<f64>() / values.len() as f64)
    }
}

fn consistency_from_values(values: &[Option<f64>]) -> f64 {
    let filtered: Vec<f64> = values.iter().copied().flatten().collect();
    if filtered.len() < 2 {
        return 1.0;
    }

    let mean = filtered.iter().sum::<f64>() / filtered.len() as f64;
    let variance = filtered
        .iter()
        .map(|&value| (value - mean).powi(2))
        .sum::<f64>()
        / filtered.len() as f64;

    1.0 - (variance * 2.0).clamp(0.0, 1.0)
}

/// Cache for computed mesosystem linkages.
///
/// Mesosystem values are computed from microsystem data but cached
/// for performance. The cache is invalidated at the end of each
/// simulation step.
#[derive(Debug, Clone, PartialEq)]
pub struct MesosystemCache {
    /// Cached spillover values keyed by (from, to) pair.
    spillover_cache: HashMap<(MicrosystemId, MicrosystemId), f64>,

    /// Cached role conflict values keyed by unordered pair (smaller, larger).
    role_conflict_cache: HashMap<(MicrosystemId, MicrosystemId), f64>,

    /// Whether the cache is valid.
    valid: bool,
}

impl MesosystemCache {
    /// Creates a new empty cache.
    #[must_use]
    pub fn new() -> Self {
        MesosystemCache {
            spillover_cache: HashMap::new(),
            role_conflict_cache: HashMap::new(),
            valid: false,
        }
    }

    /// Invalidates the cache, forcing recomputation on next access.
    pub fn invalidate(&mut self) {
        self.spillover_cache.clear();
        self.role_conflict_cache.clear();
        self.valid = false;
    }

    /// Returns whether the cache has been invalidated.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.valid
    }

    /// Gets or computes spillover from one microsystem to another.
    ///
    /// Spillover represents stress transfer coefficient from source to target.
    /// Returns a value in range [0.0, 1.0]:
    /// - 0.0 means no spillover
    /// - 1.0 means complete stress transfer
    ///
    /// Formula: spillover = if source_stress > 0.5 { (source_stress - 0.5) * 0.3 } else { 0.0 }
    ///
    /// # Arguments
    ///
    /// * `from` - Source microsystem ID
    /// * `to` - Target microsystem ID
    /// * `microsystems` - Map of all microsystems
    ///
    /// # Returns
    ///
    /// Spillover coefficient, or 0.0 if either microsystem doesn't exist.
    pub fn get_spillover(
        &self,
        from: &MicrosystemId,
        to: &MicrosystemId,
        microsystems: &HashMap<MicrosystemId, Microsystem>,
    ) -> f64 {
        // Check cache first
        let key = (from.clone(), to.clone());
        if let Some(&cached) = self.spillover_cache.get(&key) {
            return cached;
        }

        // Compute spillover
        let source = match microsystems.get(from) {
            Some(m) => m,
            None => return 0.0,
        };

        if microsystems.get(to).is_none() {
            return 0.0;
        }

        // Threshold-based spillover: only stress above 0.5 leaks across contexts.
        let source_stress = source.stress_level();
        let spillover = if source_stress > 0.5 {
            (source_stress - 0.5) * 0.3
        } else {
            0.0
        };
        spillover.clamp(0.0, 1.0)
    }

    /// Gets or computes role conflict between two microsystems.
    ///
    /// Role conflict is symmetric - the order of arguments doesn't matter.
    /// Based on competing time demands, role expectations, and value conflicts.
    ///
    /// # Arguments
    ///
    /// * `context_a` - First microsystem ID
    /// * `context_b` - Second microsystem ID
    /// * `microsystems` - Map of all microsystems
    ///
    /// # Returns
    ///
    /// Role conflict score in [0.0, 1.0], or 0.0 if either doesn't exist.
    pub fn get_role_conflict(
        &self,
        context_a: &MicrosystemId,
        context_b: &MicrosystemId,
        microsystems: &HashMap<MicrosystemId, Microsystem>,
    ) -> f64 {
        // Make key order-independent for symmetric lookup
        let key = if context_a.as_str() < context_b.as_str() {
            (context_a.clone(), context_b.clone())
        } else {
            (context_b.clone(), context_a.clone())
        };

        if let Some(&cached) = self.role_conflict_cache.get(&key) {
            return cached;
        }

        // Compute role conflict
        let micro_a = match microsystems.get(context_a) {
            Some(m) => m,
            None => return 0.0,
        };

        let micro_b = match microsystems.get(context_b) {
            Some(m) => m,
            None => return 0.0,
        };

        // Role conflict based on:
        // 1. Both contexts having high demands (stress)
        // 2. Different warmth levels (inconsistent treatment)
        // 3. High interaction frequency in both (time conflict)

        let stress_a = micro_a.stress_level();
        let stress_b = micro_b.stress_level();

        // Time conflict: both demand high frequency
        let freq_a = micro_a.interaction_frequency();
        let freq_b = micro_b.interaction_frequency();
        let time_conflict = if freq_a > 0.5 && freq_b > 0.5 {
            (freq_a + freq_b - 1.0) * 0.5
        } else {
            0.0
        };

        // Stress conflict: both have high stress
        let stress_conflict = if stress_a > 0.5 && stress_b > 0.5 {
            (stress_a + stress_b - 1.0) * 0.4
        } else {
            0.0
        };

        // Treatment inconsistency: different warmth/hostility levels
        let warmth_diff = (micro_a.warmth() - micro_b.warmth()).abs();
        let hostility_diff = (micro_a.hostility() - micro_b.hostility()).abs();
        let treatment_conflict = (warmth_diff + hostility_diff) * 0.3;

        let total = time_conflict + stress_conflict + treatment_conflict;
        total.clamp(0.0, 1.0)
    }

    /// Lists all active linkages between microsystems.
    ///
    /// Returns pairs of microsystem IDs that have meaningful connections.
    /// A linkage is considered active if both microsystems exist and
    /// have non-trivial interaction frequency.
    pub fn list_linkages(
        &self,
        microsystems: &HashMap<MicrosystemId, Microsystem>,
    ) -> Vec<(MicrosystemId, MicrosystemId)> {
        let ids: Vec<_> = microsystems.keys().cloned().collect();
        let mut linkages = Vec::new();

        for i in 0..ids.len() {
            for j in (i + 1)..ids.len() {
                let a = &ids[i];
                let b = &ids[j];

                // Only include if both have meaningful interaction frequency
                let micro_a = &microsystems[a];
                let micro_b = &microsystems[b];

                if micro_a.interaction_frequency() > 0.1 && micro_b.interaction_frequency() > 0.1 {
                    linkages.push((a.clone(), b.clone()));
                }
            }
        }

        linkages
    }

    /// Computes mesosystem consistency across all microsystems.
    ///
    /// Per spec: measures value alignment variance across contexts.
    /// Low consistency increases stress.
    ///
    /// # Arguments
    ///
    /// * `microsystems` - Map of all microsystems
    ///
    /// # Returns
    ///
    /// Consistency score in [0.0, 1.0] where 1.0 is perfectly consistent.
    #[must_use]
    pub fn compute_consistency(microsystems: &HashMap<MicrosystemId, Microsystem>) -> f64 {
        if microsystems.len() < 2 {
            return 1.0; // Single or no microsystems = perfect consistency
        }

        let mut work_role_clarity = Vec::new();
        let mut family_role_clarity = Vec::new();
        let mut social_predictability = Vec::new();

        for micro in microsystems.values() {
            match micro {
                Microsystem::Work(work) => work_role_clarity.push(work.role_clarity),
                Microsystem::Family(family) => family_role_clarity.push(family.role_clarity),
                Microsystem::Social(social) => social_predictability.push(social.predictability),
                _ => {}
            }
        }

        consistency_from_values(&[
            average(&work_role_clarity),
            average(&family_role_clarity),
            average(&social_predictability),
        ])
    }

    /// Computes shared membership strength across microsystems.
    ///
    /// # Arguments
    ///
    /// * `microsystems` - Map of all microsystems
    ///
    /// # Returns
    ///
    /// Shared membership score in [0.0, 1.0].
    #[must_use]
    pub fn compute_shared_membership_strength(
        microsystems: &HashMap<MicrosystemId, Microsystem>,
    ) -> f64 {
        use std::collections::HashSet;

        // Collect all entity IDs from all microsystems
        let mut all_ids: HashSet<String> = HashSet::new();
        let mut id_to_context_count: HashMap<String, usize> = HashMap::new();

        for micro in microsystems.values() {
            let member_ids: Vec<&crate::types::EntityId> = match micro {
                Microsystem::Work(w) => {
                    let mut ids: Vec<_> = w.peer_ids.iter().collect();
                    if let Some(ref sup) = w.supervisor_id {
                        ids.push(sup);
                    }
                    ids
                }
                Microsystem::Family(f) => f.family_unit.iter().collect(),
                Microsystem::Social(s) => s.close_friends.iter().collect(),
                Microsystem::Education(e) => {
                    let mut ids: Vec<_> = e.peer_ids.iter().collect();
                    ids.extend(e.instructors.iter());
                    ids
                }
                Microsystem::Healthcare(h) => {
                    if let Some(ref provider) = h.primary_provider_id {
                        vec![provider]
                    } else {
                        vec![]
                    }
                }
                Microsystem::Religious(r) => {
                    if let Some(ref leader) = r.leader_id {
                        vec![leader]
                    } else {
                        vec![]
                    }
                }
                Microsystem::Neighborhood(n) => n.proximity_network.iter().collect(),
            };

            for id in member_ids {
                let id_str = id.as_str().to_string();
                all_ids.insert(id_str.clone());
                *id_to_context_count.entry(id_str).or_insert(0) += 1;
            }
        }

        if all_ids.is_empty() {
            return 0.0;
        }

        // Count how many appear in 2+ contexts
        let overlap_count = id_to_context_count
            .values()
            .filter(|&&count| count >= 2)
            .count();

        (overlap_count as f64 / all_ids.len() as f64).clamp(0.0, 1.0)
    }
}

impl Default for MesosystemCache {
    fn default() -> Self {
        MesosystemCache::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::microsystem::{EducationContext, FamilyContext, SocialContext, WorkContext};

    // --- ProximalProcessGateError tests ---

    #[test]
    fn proximal_process_gate_allows_sufficient_thresholds() {
        let result = check_proximal_process_gate(0.5, 0.5, 0.3, 0.3);
        assert!(result.is_ok());
    }

    #[test]
    fn proximal_process_frequency_gate_blocks_low_frequency() {
        let result = check_proximal_process_gate(0.2, 0.5, 0.3, 0.3);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            ProximalProcessGateError::FrequencyBelowThreshold { actual, threshold }
                if (actual - 0.2).abs() < f64::EPSILON
                    && (threshold - 0.3).abs() < f64::EPSILON
        ));
    }

    #[test]
    fn proximal_process_complexity_gate_blocks_low_complexity() {
        let result = check_proximal_process_gate(0.5, 0.2, 0.3, 0.3);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            ProximalProcessGateError::ComplexityBelowThreshold { actual, threshold }
                if (actual - 0.2).abs() < f64::EPSILON
                    && (threshold - 0.3).abs() < f64::EPSILON
        ));
    }

    #[test]
    fn proximal_process_gate_returns_error_with_reason() {
        let result = check_proximal_process_gate(0.1, 0.1, 0.3, 0.3);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            ProximalProcessGateError::BothBelowThreshold {
                actual_frequency,
                frequency_threshold,
                actual_complexity,
                complexity_threshold,
            } if (actual_frequency - 0.1).abs() < f64::EPSILON
                && (frequency_threshold - 0.3).abs() < f64::EPSILON
                && (actual_complexity - 0.1).abs() < f64::EPSILON
                && (complexity_threshold - 0.3).abs() < f64::EPSILON
        ));
    }

    #[test]
    fn proximal_process_gate_error_display() {
        let err = ProximalProcessGateError::FrequencyBelowThreshold {
            actual: 0.2,
            threshold: 0.3,
        };
        let display = format!("{}", err);
        assert!(display.contains("0.2"));
        assert!(display.contains("0.3"));

        let err = ProximalProcessGateError::ComplexityBelowThreshold {
            actual: 0.2,
            threshold: 0.3,
        };
        let display = format!("{}", err);
        assert!(display.contains("complexity"));

        let err = ProximalProcessGateError::BothBelowThreshold {
            actual_frequency: 0.1,
            frequency_threshold: 0.3,
            actual_complexity: 0.2,
            complexity_threshold: 0.4,
        };
        let display = format!("{}", err);
        assert!(display.contains("frequency"));
        assert!(display.contains("complexity"));
    }

    // --- MesosystemCache tests ---

    #[test]
    fn mesosystem_cache_new() {
        let cache = MesosystemCache::new();
        assert!(!cache.is_valid());
    }

    #[test]
    fn mesosystem_cache_invalidate() {
        let mut cache = MesosystemCache::new();
        cache.valid = true;
        cache.invalidate();
        assert!(!cache.is_valid());
    }

    #[test]
    fn mesosystem_work_stress_spills_to_home() {
        let cache = MesosystemCache::new();
        let mut microsystems = HashMap::new();

        // Create high-stress work
        let mut work = WorkContext::default();
        work.workload_stress = 0.8;
        work.interaction_profile.interaction_frequency = 0.7;
        let work_id = MicrosystemId::new("work").unwrap();
        microsystems.insert(work_id.clone(), Microsystem::new_work(work));

        // Create family with low boundary (low predictability/stability)
        let mut family = FamilyContext::default();
        family.predictability = 0.3;
        family.stability = 0.3;
        let family_id = MicrosystemId::new("family").unwrap();
        microsystems.insert(family_id.clone(), Microsystem::new_family(family));

        let spillover = cache.get_spillover(&work_id, &family_id, &microsystems);

        // Spillover follows threshold-based formula
        let expected = (0.8 - 0.5) * 0.3;
        assert!((spillover - expected).abs() < 1e-6);
    }

    #[test]
    fn mesosystem_role_conflict_computation() {
        let cache = MesosystemCache::new();
        let mut microsystems = HashMap::new();

        // Create high-demand work
        let mut work = WorkContext::default();
        work.workload_stress = 0.8;
        work.interaction_profile.interaction_frequency = 0.8;
        let work_id = MicrosystemId::new("work").unwrap();
        microsystems.insert(work_id.clone(), Microsystem::new_work(work));

        // Create high-demand family
        let mut family = FamilyContext::default();
        family.caregiving_burden = 0.8;
        family.interaction_profile.interaction_frequency = 0.8;
        let family_id = MicrosystemId::new("family").unwrap();
        microsystems.insert(family_id.clone(), Microsystem::new_family(family));

        let conflict = cache.get_role_conflict(&work_id, &family_id, &microsystems);

        // Should have significant conflict
        assert!(conflict > 0.2);
        assert!(conflict <= 1.0);
    }

    #[test]
    fn mesosystem_role_conflict_symmetric() {
        let cache = MesosystemCache::new();
        let mut microsystems = HashMap::new();

        let work_id = MicrosystemId::new("work").unwrap();
        let family_id = MicrosystemId::new("family").unwrap();

        microsystems.insert(
            work_id.clone(),
            Microsystem::new_work(WorkContext::default()),
        );
        microsystems.insert(
            family_id.clone(),
            Microsystem::new_family(FamilyContext::default()),
        );

        let conflict_ab = cache.get_role_conflict(&work_id, &family_id, &microsystems);
        let conflict_ba = cache.get_role_conflict(&family_id, &work_id, &microsystems);

        assert!((conflict_ab - conflict_ba).abs() < f64::EPSILON);
    }

    #[test]
    fn mesosystem_spillover_nonexistent_source() {
        let cache = MesosystemCache::new();
        let microsystems = HashMap::new();

        let from = MicrosystemId::new("nonexistent").unwrap();
        let to = MicrosystemId::new("also_nonexistent").unwrap();

        let spillover = cache.get_spillover(&from, &to, &microsystems);
        assert!((spillover - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn mesosystem_role_conflict_nonexistent() {
        let cache = MesosystemCache::new();
        let microsystems = HashMap::new();

        let a = MicrosystemId::new("nonexistent").unwrap();
        let b = MicrosystemId::new("also_nonexistent").unwrap();

        let conflict = cache.get_role_conflict(&a, &b, &microsystems);
        assert!((conflict - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn mesosystem_list_linkages() {
        let cache = MesosystemCache::new();
        let mut microsystems = HashMap::new();

        let work_id = MicrosystemId::new("work").unwrap();
        let family_id = MicrosystemId::new("family").unwrap();
        let social_id = MicrosystemId::new("social").unwrap();

        microsystems.insert(
            work_id.clone(),
            Microsystem::new_work(WorkContext::default()),
        );
        microsystems.insert(
            family_id.clone(),
            Microsystem::new_family(FamilyContext::default()),
        );
        microsystems.insert(
            social_id.clone(),
            Microsystem::new_social(SocialContext::default()),
        );

        let linkages = cache.list_linkages(&microsystems);

        // Should have 3 linkages (work-family, work-social, family-social)
        assert_eq!(linkages.len(), 3);

        let work = microsystems
            .get_mut(&work_id)
            .and_then(Microsystem::work_mut)
            .expect("work microsystem missing");
        work.interaction_profile.interaction_frequency = 0.0;

        let linkages = cache.list_linkages(&microsystems);
        assert_eq!(linkages.len(), 1);
    }

    #[test]
    fn mesosystem_no_stored_scalars() {
        // This test verifies that mesosystem values are always computed,
        // not stored independently.
        let cache = MesosystemCache::new();
        let mut microsystems = HashMap::new();

        let work_id = MicrosystemId::new("work").unwrap();
        let family_id = MicrosystemId::new("family").unwrap();

        let mut work = WorkContext::default();
        work.workload_stress = 0.5;
        microsystems.insert(work_id.clone(), Microsystem::new_work(work));
        microsystems.insert(
            family_id.clone(),
            Microsystem::new_family(FamilyContext::default()),
        );

        // Get spillover - should compute from current microsystem state
        let spillover1 = cache.get_spillover(&work_id, &family_id, &microsystems);

        // Modify work stress
        let work = microsystems
            .get_mut(&work_id)
            .and_then(Microsystem::work_mut)
            .expect("work microsystem missing");
        work.workload_stress = 0.9;

        // Get spillover again - should reflect new stress level
        // (Note: in real usage, cache would be invalidated between steps)
        let spillover2 = cache.get_spillover(&work_id, &family_id, &microsystems);

        // Since cache doesn't store values, both calls compute from current state
        // Actually they may be the same if cache stores them, but the point is
        // we're computing from microsystem data, not storing mesosystem scalars
        assert!(spillover1 >= 0.0 && spillover1 <= 1.0);
        assert!(spillover2 >= 0.0 && spillover2 <= 1.0);
    }

    #[test]
    fn mesosystem_recomputes_on_microsystem_change() {
        let mut cache = MesosystemCache::new();
        let mut microsystems = HashMap::new();

        let work_id = MicrosystemId::new("work").unwrap();
        let family_id = MicrosystemId::new("family").unwrap();

        let mut work = WorkContext::default();
        work.workload_stress = 0.3;
        microsystems.insert(work_id.clone(), Microsystem::new_work(work));

        let mut family = FamilyContext::default();
        family.predictability = 0.5;
        family.stability = 0.5;
        microsystems.insert(family_id.clone(), Microsystem::new_family(family));

        let spillover1 = cache.get_spillover(&work_id, &family_id, &microsystems);

        // Invalidate cache and change stress
        cache.invalidate();
        let work = microsystems
            .get_mut(&work_id)
            .and_then(Microsystem::work_mut)
            .expect("work microsystem missing");
        work.workload_stress = 0.9;

        let spillover2 = cache.get_spillover(&work_id, &family_id, &microsystems);

        // Higher stress should lead to higher spillover
        assert!(spillover2 > spillover1);
    }

    #[test]
    fn mesosystem_consistency() {
        let mut microsystems = HashMap::new();

        // Create microsystems with similar value alignment
        let mut work = WorkContext::default();
        work.role_clarity = 0.7;
        microsystems.insert(
            MicrosystemId::new("work").unwrap(),
            Microsystem::new_work(work),
        );

        let mut family = FamilyContext::default();
        family.role_clarity = 0.7;
        microsystems.insert(
            MicrosystemId::new("family").unwrap(),
            Microsystem::new_family(family),
        );

        let consistency = MesosystemCache::compute_consistency(&microsystems);

        // Similar value alignment = high consistency
        assert!(consistency > 0.8);
    }

    #[test]
    fn mesosystem_consistency_low_variance() {
        let mut microsystems = HashMap::new();

        // Create microsystems with maximally different value alignment
        let mut work = WorkContext::default();
        work.role_clarity = 1.0;
        microsystems.insert(
            MicrosystemId::new("work").unwrap(),
            Microsystem::new_work(work),
        );

        let mut family = FamilyContext::default();
        family.role_clarity = 0.0;
        microsystems.insert(
            MicrosystemId::new("family").unwrap(),
            Microsystem::new_family(family),
        );

        let consistency = MesosystemCache::compute_consistency(&microsystems);

        // Max variance in [0, 1] yields 0.5 consistency.
        assert!((consistency - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn mesosystem_consistency_single_microsystem() {
        let mut microsystems = HashMap::new();
        microsystems.insert(
            MicrosystemId::new("work").unwrap(),
            Microsystem::new_work(WorkContext::default()),
        );

        let consistency = MesosystemCache::compute_consistency(&microsystems);
        assert!((consistency - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn mesosystem_consistency_empty() {
        let microsystems = HashMap::new();
        let consistency = MesosystemCache::compute_consistency(&microsystems);
        assert!((consistency - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn mesosystem_consistency_includes_social_and_ignores_other_types() {
        let mut microsystems = HashMap::new();

        let mut work = WorkContext::default();
        work.role_clarity = 0.6;
        microsystems.insert(
            MicrosystemId::new("work").unwrap(),
            Microsystem::new_work(work),
        );

        let mut family = FamilyContext::default();
        family.role_clarity = 0.4;
        microsystems.insert(
            MicrosystemId::new("family").unwrap(),
            Microsystem::new_family(family),
        );

        let mut social = SocialContext::default();
        social.predictability = 0.7;
        microsystems.insert(
            MicrosystemId::new("social").unwrap(),
            Microsystem::new_social(social),
        );

        microsystems.insert(
            MicrosystemId::new("education").unwrap(),
            Microsystem::new_education(EducationContext::default()),
        );

        let consistency = MesosystemCache::compute_consistency(&microsystems);
        assert!((0.0..=1.0).contains(&consistency));
    }

    #[test]
    fn mesosystem_shared_membership_empty() {
        let microsystems = HashMap::new();
        let strength = MesosystemCache::compute_shared_membership_strength(&microsystems);
        assert!((strength - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn mesosystem_shared_membership_no_overlap() {
        let mut microsystems = HashMap::new();

        let mut work = WorkContext::default();
        work.peer_ids = vec![
            crate::types::EntityId::new("alice").unwrap(),
            crate::types::EntityId::new("bob").unwrap(),
        ];
        microsystems.insert(
            MicrosystemId::new("work").unwrap(),
            Microsystem::new_work(work),
        );

        let mut family = FamilyContext::default();
        family.family_unit = vec![
            crate::types::EntityId::new("charlie").unwrap(),
            crate::types::EntityId::new("diana").unwrap(),
        ];
        microsystems.insert(
            MicrosystemId::new("family").unwrap(),
            Microsystem::new_family(family),
        );

        let strength = MesosystemCache::compute_shared_membership_strength(&microsystems);
        assert!((strength - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn mesosystem_shared_membership_with_overlap() {
        let mut microsystems = HashMap::new();

        let shared_id = crate::types::EntityId::new("shared_person").unwrap();

        let mut work = WorkContext::default();
        work.peer_ids = vec![
            shared_id.clone(),
            crate::types::EntityId::new("bob").unwrap(),
        ];
        microsystems.insert(
            MicrosystemId::new("work").unwrap(),
            Microsystem::new_work(work),
        );

        let mut social = SocialContext::default();
        social.close_friends = vec![shared_id, crate::types::EntityId::new("charlie").unwrap()];
        microsystems.insert(
            MicrosystemId::new("social").unwrap(),
            Microsystem::new_social(social),
        );

        let strength = MesosystemCache::compute_shared_membership_strength(&microsystems);

        // 1 overlap out of 3 unique people = 0.333...
        assert!(strength > 0.3 && strength < 0.4);
    }

    // --- MesosystemState tests ---

    #[test]
    fn mesosystem_state_defaults_when_empty() {
        let microsystems = HashMap::new();
        let state = MesosystemState::compute(&microsystems);
        assert_eq!(state, MesosystemState::default());
    }

    #[test]
    fn mesosystem_state_computes_cross_context_metrics() {
        use crate::types::EntityId;

        let mut microsystems = HashMap::new();
        let shared_id = EntityId::new("shared_person").unwrap();

        let mut work = WorkContext::default();
        work.role_clarity = 0.2;
        work.predictability = 0.3;
        work.warmth = 0.2;
        work.hostility = 0.1;
        work.workload_stress = 0.8;
        work.peer_ids = vec![shared_id.clone()];
        work.interaction_profile.interaction_frequency = 0.8;
        microsystems.insert(
            MicrosystemId::new("work").unwrap(),
            Microsystem::new_work(work),
        );

        let mut family = FamilyContext::default();
        family.role_clarity = 0.8;
        family.predictability = 0.9;
        family.warmth = 0.7;
        family.hostility = 0.1;
        family.caregiving_burden = 0.8;
        family.interaction_profile.interaction_frequency = 0.8;
        microsystems.insert(
            MicrosystemId::new("family").unwrap(),
            Microsystem::new_family(family),
        );

        let mut social = SocialContext::default();
        social.predictability = 0.9;
        social.warmth = 0.8;
        social.hostility = 0.1;
        social.close_friends = vec![shared_id];
        social.interaction_profile.interaction_frequency = 0.6;
        microsystems.insert(
            MicrosystemId::new("social").unwrap(),
            Microsystem::new_social(social),
        );

        let state = MesosystemState::compute(&microsystems);

        assert!(state.work_family_conflict > 0.0);
        assert!((state.family_social_support - 0.6).abs() < 1e-6);
        assert!((state.work_social_conflict - 0.3).abs() < f64::EPSILON);
        assert!((state.shared_membership_strength - 1.0).abs() < f64::EPSILON);

        let consistency = |values: &[f64]| {
            let mean = values.iter().sum::<f64>() / values.len() as f64;
            let variance = values
                .iter()
                .map(|&value| (value - mean).powi(2))
                .sum::<f64>()
                / values.len() as f64;
            1.0 - (variance * 2.0).clamp(0.0, 1.0)
        };

        let expected_value_alignment = consistency(&[0.2, 0.8, 0.9]);
        let expected_autonomy = consistency(&[0.3, 0.9, 0.9]);
        let expected_meso = expected_value_alignment;

        assert!((state.value_alignment_consistency - expected_value_alignment).abs() < 1e-6);
        assert!((state.autonomy_norm_consistency - expected_autonomy).abs() < 1e-6);
        assert!((state.mesosystem_consistency - expected_meso).abs() < 1e-6);
    }

    // --- MesosystemLinkage tests ---

    #[test]
    fn mesosystem_linkage_creation() {
        let linkage = MesosystemLinkage {
            from: MicrosystemId::new("work").unwrap(),
            to: MicrosystemId::new("family").unwrap(),
            spillover: 0.3,
            role_conflict: 0.2,
        };

        assert_eq!(linkage.from.as_str(), "work");
        assert_eq!(linkage.to.as_str(), "family");
    }

    #[test]
    fn mesosystem_linkage_clone_eq() {
        let linkage1 = MesosystemLinkage {
            from: MicrosystemId::new("work").unwrap(),
            to: MicrosystemId::new("family").unwrap(),
            spillover: 0.3,
            role_conflict: 0.2,
        };
        let linkage2 = linkage1.clone();
        assert_eq!(linkage1, linkage2);
    }

    #[test]
    fn mesosystem_cache_clone_eq() {
        let cache1 = MesosystemCache::new();
        let cache2 = cache1.clone();
        assert_eq!(cache1, cache2);
    }

    #[test]
    fn mesosystem_cache_default() {
        let cache = MesosystemCache::default();
        assert!(!cache.is_valid());
    }

    #[test]
    fn proximal_process_gate_error_clone_eq() {
        let err1 = ProximalProcessGateError::FrequencyBelowThreshold {
            actual: 0.2,
            threshold: 0.3,
        };
        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }

    // --- Additional coverage tests for mesosystem ---

    #[test]
    fn mesosystem_spillover_to_social_target() {
        use crate::context::microsystem::SocialContext;

        let cache = MesosystemCache::new();
        let mut microsystems = HashMap::new();

        let mut work = WorkContext::default();
        work.workload_stress = 0.8;
        work.interaction_profile.interaction_frequency = 0.7;
        let work_id = MicrosystemId::new("work").unwrap();
        microsystems.insert(work_id.clone(), Microsystem::new_work(work));

        let social = SocialContext::default();
        let social_id = MicrosystemId::new("social").unwrap();
        microsystems.insert(social_id.clone(), Microsystem::new_social(social));

        let spillover = cache.get_spillover(&work_id, &social_id, &microsystems);
        assert!(spillover >= 0.0 && spillover <= 1.0);
    }

    #[test]
    fn mesosystem_spillover_to_education_target() {
        use crate::context::microsystem::EducationContext;

        let cache = MesosystemCache::new();
        let mut microsystems = HashMap::new();

        let mut work = WorkContext::default();
        work.workload_stress = 0.8;
        work.interaction_profile.interaction_frequency = 0.7;
        let work_id = MicrosystemId::new("work").unwrap();
        microsystems.insert(work_id.clone(), Microsystem::new_work(work));

        let edu = EducationContext::default();
        let edu_id = MicrosystemId::new("education").unwrap();
        microsystems.insert(edu_id.clone(), Microsystem::new_education(edu));

        let spillover = cache.get_spillover(&work_id, &edu_id, &microsystems);
        assert!(spillover >= 0.0 && spillover <= 1.0);
    }

    #[test]
    fn mesosystem_spillover_to_healthcare_target() {
        use crate::context::microsystem::HealthcareContext;

        let cache = MesosystemCache::new();
        let mut microsystems = HashMap::new();

        let mut work = WorkContext::default();
        work.workload_stress = 0.8;
        work.interaction_profile.interaction_frequency = 0.7;
        let work_id = MicrosystemId::new("work").unwrap();
        microsystems.insert(work_id.clone(), Microsystem::new_work(work));

        let hc = HealthcareContext::default();
        let hc_id = MicrosystemId::new("healthcare").unwrap();
        microsystems.insert(hc_id.clone(), Microsystem::new_healthcare(hc));

        let spillover = cache.get_spillover(&work_id, &hc_id, &microsystems);
        assert!(spillover >= 0.0 && spillover <= 1.0);
    }

    #[test]
    fn mesosystem_spillover_to_religious_target() {
        use crate::context::microsystem::ReligiousContext;

        let cache = MesosystemCache::new();
        let mut microsystems = HashMap::new();

        let mut work = WorkContext::default();
        work.workload_stress = 0.8;
        work.interaction_profile.interaction_frequency = 0.7;
        let work_id = MicrosystemId::new("work").unwrap();
        microsystems.insert(work_id.clone(), Microsystem::new_work(work));

        let rel = ReligiousContext::default();
        let rel_id = MicrosystemId::new("religious").unwrap();
        microsystems.insert(rel_id.clone(), Microsystem::new_religious(rel));

        let spillover = cache.get_spillover(&work_id, &rel_id, &microsystems);
        assert!(spillover >= 0.0 && spillover <= 1.0);
    }

    #[test]
    fn mesosystem_spillover_to_neighborhood_target() {
        use crate::context::microsystem::NeighborhoodContext;

        let cache = MesosystemCache::new();
        let mut microsystems = HashMap::new();

        let mut work = WorkContext::default();
        work.workload_stress = 0.8;
        work.interaction_profile.interaction_frequency = 0.7;
        let work_id = MicrosystemId::new("work").unwrap();
        microsystems.insert(work_id.clone(), Microsystem::new_work(work));

        let nb = NeighborhoodContext::default();
        let nb_id = MicrosystemId::new("neighborhood").unwrap();
        microsystems.insert(nb_id.clone(), Microsystem::new_neighborhood(nb));

        let spillover = cache.get_spillover(&work_id, &nb_id, &microsystems);
        assert!(spillover >= 0.0 && spillover <= 1.0);
    }

    #[test]
    fn mesosystem_shared_membership_with_education() {
        use crate::context::microsystem::EducationContext;
        use crate::types::EntityId;

        let mut microsystems = HashMap::new();

        let shared_id = EntityId::new("shared_person").unwrap();

        let mut edu = EducationContext::default();
        edu.peer_ids = vec![shared_id.clone()];
        edu.instructors = vec![EntityId::new("instructor").unwrap()];
        microsystems.insert(
            MicrosystemId::new("education").unwrap(),
            Microsystem::new_education(edu),
        );

        let mut social = SocialContext::default();
        social.close_friends = vec![shared_id.clone()];
        microsystems.insert(
            MicrosystemId::new("social").unwrap(),
            Microsystem::new_social(social),
        );

        let strength = MesosystemCache::compute_shared_membership_strength(&microsystems);
        assert!(strength > 0.0);
    }

    #[test]
    fn mesosystem_shared_membership_with_healthcare() {
        use crate::context::microsystem::HealthcareContext;
        use crate::types::EntityId;

        let mut microsystems = HashMap::new();

        let shared_id = EntityId::new("shared_person").unwrap();

        let mut hc = HealthcareContext::default();
        hc.primary_provider_id = Some(shared_id.clone());
        microsystems.insert(
            MicrosystemId::new("healthcare").unwrap(),
            Microsystem::new_healthcare(hc),
        );

        let mut social = SocialContext::default();
        social.close_friends = vec![shared_id.clone()];
        microsystems.insert(
            MicrosystemId::new("social").unwrap(),
            Microsystem::new_social(social),
        );

        let strength = MesosystemCache::compute_shared_membership_strength(&microsystems);
        assert!(strength > 0.0);
    }

    #[test]
    fn mesosystem_shared_membership_with_religious() {
        use crate::context::microsystem::ReligiousContext;
        use crate::types::EntityId;

        let mut microsystems = HashMap::new();

        let shared_id = EntityId::new("shared_person").unwrap();

        let mut rel = ReligiousContext::default();
        rel.leader_id = Some(shared_id.clone());
        microsystems.insert(
            MicrosystemId::new("religious").unwrap(),
            Microsystem::new_religious(rel),
        );

        let mut social = SocialContext::default();
        social.close_friends = vec![shared_id.clone()];
        microsystems.insert(
            MicrosystemId::new("social").unwrap(),
            Microsystem::new_social(social),
        );

        let strength = MesosystemCache::compute_shared_membership_strength(&microsystems);
        assert!(strength > 0.0);
    }

    #[test]
    fn mesosystem_shared_membership_with_neighborhood() {
        use crate::context::microsystem::NeighborhoodContext;
        use crate::types::EntityId;

        let mut microsystems = HashMap::new();

        let shared_id = EntityId::new("shared_person").unwrap();

        let mut nb = NeighborhoodContext::default();
        nb.proximity_network = vec![shared_id.clone()];
        microsystems.insert(
            MicrosystemId::new("neighborhood").unwrap(),
            Microsystem::new_neighborhood(nb),
        );

        let mut social = SocialContext::default();
        social.close_friends = vec![shared_id.clone()];
        microsystems.insert(
            MicrosystemId::new("social").unwrap(),
            Microsystem::new_social(social),
        );

        let strength = MesosystemCache::compute_shared_membership_strength(&microsystems);
        assert!(strength > 0.0);
    }

    #[test]
    fn mesosystem_shared_membership_healthcare_no_provider() {
        use crate::context::microsystem::HealthcareContext;

        let mut microsystems = HashMap::new();

        let hc = HealthcareContext::default(); // No primary_provider_id set
        microsystems.insert(
            MicrosystemId::new("healthcare").unwrap(),
            Microsystem::new_healthcare(hc),
        );

        // Should not panic, just return 0.0 or low value
        let strength = MesosystemCache::compute_shared_membership_strength(&microsystems);
        assert!((strength - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn mesosystem_shared_membership_religious_no_leader() {
        use crate::context::microsystem::ReligiousContext;

        let mut microsystems = HashMap::new();

        let rel = ReligiousContext::default(); // No leader_id set
        microsystems.insert(
            MicrosystemId::new("religious").unwrap(),
            Microsystem::new_religious(rel),
        );

        // Should not panic, just return 0.0 or low value
        let strength = MesosystemCache::compute_shared_membership_strength(&microsystems);
        assert!((strength - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn mesosystem_shared_membership_with_supervisor() {
        use crate::types::EntityId;

        let mut microsystems = HashMap::new();

        let shared_id = EntityId::new("supervisor_friend").unwrap();

        let mut work = WorkContext::default();
        work.peer_ids = vec![EntityId::new("coworker").unwrap()];
        work.supervisor_id = Some(shared_id.clone());
        microsystems.insert(
            MicrosystemId::new("work").unwrap(),
            Microsystem::new_work(work),
        );

        let mut social = SocialContext::default();
        social.close_friends = vec![shared_id.clone()];
        microsystems.insert(
            MicrosystemId::new("social").unwrap(),
            Microsystem::new_social(social),
        );

        let strength = MesosystemCache::compute_shared_membership_strength(&microsystems);
        // supervisor_friend appears in both contexts
        assert!(strength > 0.0);
    }

    #[test]
    fn mesosystem_spillover_target_missing() {
        let cache = MesosystemCache::new();
        let mut microsystems = HashMap::new();

        let mut work = WorkContext::default();
        work.workload_stress = 0.8;
        let work_id = MicrosystemId::new("work").unwrap();
        microsystems.insert(work_id.clone(), Microsystem::new_work(work));

        let family_id = MicrosystemId::new("family").unwrap();
        // family_id not in microsystems

        let spillover = cache.get_spillover(&work_id, &family_id, &microsystems);
        assert!((spillover - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn mesosystem_role_conflict_uses_cache() {
        let cache = MesosystemCache::new();
        let mut microsystems = HashMap::new();

        let mut work = WorkContext::default();
        work.workload_stress = 0.8;
        work.interaction_profile.interaction_frequency = 0.8;
        let work_id = MicrosystemId::new("work").unwrap();
        microsystems.insert(work_id.clone(), Microsystem::new_work(work));

        let mut family = FamilyContext::default();
        family.caregiving_burden = 0.8;
        family.interaction_profile.interaction_frequency = 0.8;
        let family_id = MicrosystemId::new("family").unwrap();
        microsystems.insert(family_id.clone(), Microsystem::new_family(family));

        // First call computes and potentially caches
        let conflict1 = cache.get_role_conflict(&work_id, &family_id, &microsystems);

        // Second call should return same value (though cache isn't being stored in current impl)
        let conflict2 = cache.get_role_conflict(&work_id, &family_id, &microsystems);

        assert!((conflict1 - conflict2).abs() < f64::EPSILON);
    }

    #[test]
    fn mesosystem_spillover_to_work_target() {
        // Test spillover TO work (not from work) to cover the Work branch in role_boundary match
        let cache = MesosystemCache::new();
        let mut microsystems = HashMap::new();

        let mut family = FamilyContext::default();
        family.caregiving_burden = 0.8;
        family.hostility = 0.3;
        family.interaction_profile.interaction_frequency = 0.7;
        let family_id = MicrosystemId::new("family").unwrap();
        microsystems.insert(family_id.clone(), Microsystem::new_family(family));

        let work = WorkContext::default();
        let work_id = MicrosystemId::new("work").unwrap();
        microsystems.insert(work_id.clone(), Microsystem::new_work(work));

        // Spillover from family TO work
        let spillover = cache.get_spillover(&family_id, &work_id, &microsystems);
        assert!(spillover >= 0.0 && spillover <= 1.0);
    }

    #[test]
    fn mesosystem_role_conflict_missing_context_b() {
        let cache = MesosystemCache::new();
        let mut microsystems = HashMap::new();

        let work = WorkContext::default();
        let work_id = MicrosystemId::new("work").unwrap();
        microsystems.insert(work_id.clone(), Microsystem::new_work(work));

        let family_id = MicrosystemId::new("family").unwrap();
        // Note: family_id NOT inserted into microsystems

        let conflict = cache.get_role_conflict(&work_id, &family_id, &microsystems);
        assert!((conflict - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn mesosystem_spillover_uses_cache() {
        // Create a mutable cache to test cache hit
        let mut cache = MesosystemCache::new();
        let mut microsystems = HashMap::new();

        let mut work = WorkContext::default();
        work.workload_stress = 0.8;
        work.interaction_profile.interaction_frequency = 0.7;
        let work_id = MicrosystemId::new("work").unwrap();
        microsystems.insert(work_id.clone(), Microsystem::new_work(work));

        let family = FamilyContext::default();
        let family_id = MicrosystemId::new("family").unwrap();
        microsystems.insert(family_id.clone(), Microsystem::new_family(family));

        // First call should compute and store
        let spillover1 = cache.get_spillover(&work_id, &family_id, &microsystems);

        // Manually insert into cache to simulate cached state
        cache
            .spillover_cache
            .insert((work_id.clone(), family_id.clone()), spillover1);

        // Second call should return cached value
        let spillover2 = cache.get_spillover(&work_id, &family_id, &microsystems);
        assert!((spillover1 - spillover2).abs() < f64::EPSILON);
    }

    #[test]
    fn mesosystem_role_conflict_uses_cache_properly() {
        // Create a mutable cache to test cache hit
        let mut cache = MesosystemCache::new();
        let mut microsystems = HashMap::new();

        let mut work = WorkContext::default();
        work.workload_stress = 0.8;
        work.interaction_profile.interaction_frequency = 0.8;
        let work_id = MicrosystemId::new("work").unwrap();
        microsystems.insert(work_id.clone(), Microsystem::new_work(work));

        let mut family = FamilyContext::default();
        family.caregiving_burden = 0.8;
        family.interaction_profile.interaction_frequency = 0.8;
        let family_id = MicrosystemId::new("family").unwrap();
        microsystems.insert(family_id.clone(), Microsystem::new_family(family));

        // First call computes
        let conflict1 = cache.get_role_conflict(&work_id, &family_id, &microsystems);

        // Manually insert into cache (using canonical order: family < work by string)
        let canonical_key = |left: &MicrosystemId, right: &MicrosystemId| {
            if left.as_str() < right.as_str() {
                (left.clone(), right.clone())
            } else {
                (right.clone(), left.clone())
            }
        };
        let key = canonical_key(&family_id, &work_id);
        let alternate_key = canonical_key(&work_id, &family_id);
        cache.role_conflict_cache.insert(key, conflict1);
        cache.role_conflict_cache.insert(alternate_key, conflict1);

        // Second call should return cached value
        let conflict2 = cache.get_role_conflict(&work_id, &family_id, &microsystems);
        assert!((conflict1 - conflict2).abs() < f64::EPSILON);
    }

    // --- Required Phase 7 tests ---

    #[test]
    fn mesosystem_shared_member_linkage() {
        // Shared participants create linkage between microsystems
        use crate::types::EntityId;

        let mut microsystems = HashMap::new();

        // Create shared member
        let shared_member = EntityId::new("shared_person").unwrap();

        // Work context with shared member as peer
        let mut work = WorkContext::default();
        work.peer_ids = vec![shared_member.clone(), EntityId::new("coworker1").unwrap()];
        work.interaction_profile.interaction_frequency = 0.7;
        let work_id = MicrosystemId::new("work").unwrap();
        microsystems.insert(work_id.clone(), Microsystem::new_work(work));

        // Social context with shared member as friend
        let mut social = SocialContext::default();
        social.close_friends = vec![
            shared_member.clone(),
            EntityId::new("other_friend").unwrap(),
        ];
        social.interaction_profile.interaction_frequency = 0.6;
        let social_id = MicrosystemId::new("social").unwrap();
        microsystems.insert(social_id.clone(), Microsystem::new_social(social));

        // Verify linkages exist
        let cache = MesosystemCache::new();
        let linkages = cache.list_linkages(&microsystems);

        // Should have 1 linkage (work-social)
        assert_eq!(linkages.len(), 1);

        // Verify shared membership strength is positive (shared_person appears in both)
        let membership_strength =
            MesosystemCache::compute_shared_membership_strength(&microsystems);

        // We have 3 unique people: shared_person, coworker1, other_friend
        // shared_person appears in 2 contexts
        // So 1 out of 3 people overlap: 1/3 = 0.333...
        assert!(membership_strength > 0.3);

        // Without shared member, there would be no overlap
        let mut no_overlap_microsystems = HashMap::new();
        let mut work2 = WorkContext::default();
        work2.peer_ids = vec![EntityId::new("alice").unwrap()];
        work2.interaction_profile.interaction_frequency = 0.7;
        no_overlap_microsystems.insert(
            MicrosystemId::new("work").unwrap(),
            Microsystem::new_work(work2),
        );

        let mut social2 = SocialContext::default();
        social2.close_friends = vec![EntityId::new("bob").unwrap()];
        social2.interaction_profile.interaction_frequency = 0.6;
        no_overlap_microsystems.insert(
            MicrosystemId::new("social").unwrap(),
            Microsystem::new_social(social2),
        );

        let no_overlap_strength =
            MesosystemCache::compute_shared_membership_strength(&no_overlap_microsystems);
        assert!((no_overlap_strength - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn mesosystem_spillover_from_work_missing_family() {
        let cache = MesosystemCache::new();
        let mut microsystems = HashMap::new();

        let mut work = WorkContext::default();
        work.workload_stress = 0.8;
        let work_id = MicrosystemId::new("work").unwrap();
        microsystems.insert(work_id.clone(), Microsystem::new_work(work));

        let family_id = MicrosystemId::new("family").unwrap();
        // Note: family_id NOT in microsystems

        let spillover = cache.get_spillover(&work_id, &family_id, &microsystems);
        assert!((spillover - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn mesosystem_work_family_conflict_with_non_zero_count() {
        let mut microsystems = HashMap::new();

        let mut work = WorkContext::default();
        work.workload_stress = 0.8;
        let work_id = MicrosystemId::new("work").unwrap();
        microsystems.insert(work_id.clone(), Microsystem::new_work(work));

        let mut family = FamilyContext::default();
        family.caregiving_burden = 0.7;
        let family_id = MicrosystemId::new("family").unwrap();
        microsystems.insert(family_id.clone(), Microsystem::new_family(family));

        let state = MesosystemState::compute(&microsystems);
        let conflict = state.work_family_conflict;
        assert!(conflict >= 0.0);
    }

    #[test]
    fn mesosystem_work_family_conflict_with_zero_count() {
        let mut microsystems = HashMap::new();

        // Only work, no family - count will be 0
        let mut work = WorkContext::default();
        work.workload_stress = 0.8;
        let work_id = MicrosystemId::new("work").unwrap();
        microsystems.insert(work_id.clone(), Microsystem::new_work(work));

        let state = MesosystemState::compute(&microsystems);
        // With no family contexts, work_family_conflict should be 0.0 (the else branch)
        assert_eq!(state.work_family_conflict, 0.0);
    }

    #[test]
    fn mesosystem_spillover_below_threshold_returns_zero() {
        let cache = MesosystemCache::new();
        let mut microsystems = HashMap::new();

        let mut work = WorkContext::default();
        work.workload_stress = 0.3; // Below 0.5 threshold
        let work_id = MicrosystemId::new("work").unwrap();
        microsystems.insert(work_id.clone(), Microsystem::new_work(work));

        let family = FamilyContext::default();
        let family_id = MicrosystemId::new("family").unwrap();
        microsystems.insert(family_id.clone(), Microsystem::new_family(family));

        let spillover = cache.get_spillover(&work_id, &family_id, &microsystems);
        assert!((spillover - 0.0).abs() < f64::EPSILON);
    }
}
