//! Memory maintenance operations for layer promotion and decay.
//!
//! This module provides functions for maintaining memory layers over time:
//! - Promoting high-salience memories to more permanent layers
//! - Decaying and removing low-salience memories
//! - Applying trauma salience boosts at encoding
//!
//! Memory maintenance is called periodically during simulation time advancement,
//! NOT during `state_at()` queries. The caller is responsible for invoking
//! maintenance at appropriate intervals (recommended: once per simulated day).
//!
//! # Layer Promotion Rules
//!
//! - Immediate -> Short-term: salience >= 0.3, 1 hour base window
//! - Short-term -> Long-term: salience >= 0.6, 24 hour base window
//! - Long-term -> Legacy: salience >= 0.9 AND `MemoryTag::Milestone` required
//! - Layer skipping is NOT allowed (must promote sequentially)
//!
//! # Arousal-Modulated Consolidation
//!
//! Consolidation windows follow an inverted-U relationship with arousal:
//! - Optimal arousal (0.0 raw, 0.5 normalized) = base window (fastest)
//! - Extreme arousal (-1.0 or 1.0 raw) = 2x base window (slowest)

use crate::memory::{MemoryEntry, MemoryLayer, MemoryLayers, MemoryTag};
use crate::types::{Duration, MemoryId};

/// Salience threshold for Immediate -> Short-term promotion.
const THRESHOLD_IMMEDIATE_TO_SHORT: f32 = 0.3;

/// Salience threshold for Short-term -> Long-term promotion.
const THRESHOLD_SHORT_TO_LONG: f32 = 0.6;

/// Salience threshold for Long-term -> Legacy promotion.
const THRESHOLD_LONG_TO_LEGACY: f32 = 0.9;

/// Salience threshold below which memories are removed from Short-term.
const DECAY_THRESHOLD: f32 = 0.2;

/// Base consolidation window for Immediate -> Short-term (in hours).
/// Used as the base value for arousal-modulated consolidation.
const BASE_WINDOW_IMMEDIATE_HOURS: f32 = 1.0;

/// Base consolidation window for Short-term -> Long-term (in hours).
/// Used as the base value for arousal-modulated consolidation.
const BASE_WINDOW_SHORT_HOURS: f32 = 24.0;

/// Trauma salience boost multiplier.
const TRAUMA_SALIENCE_BOOST: f32 = 1.3;

/// Maintenance interval threshold (1 day in seconds).
const MAINTENANCE_INTERVAL_SECONDS: u64 = 86400;

/// Error returned when memory maintenance operations fail.
#[derive(Debug, Clone, PartialEq)]
pub enum MaintenanceError {
    /// Memory with specified ID not found in any layer.
    MemoryNotFound { id: MemoryId },
    /// Invalid layer transition (e.g., Legacy -> anything, or skipping layers).
    /// Only sequential promotion is allowed: Immediate->Short->Long->Legacy.
    InvalidLayerTransition { from: MemoryLayer, to: MemoryLayer },
    /// Salience threshold not met for promotion.
    SalienceThresholdNotMet { required: f32, actual: f32 },
    /// Consolidation window has not elapsed.
    ConsolidationWindowNotElapsed {
        required: Duration,
        elapsed: Duration,
    },
    /// Legacy promotion requires MemoryTag::Milestone.
    MissingMilestoneTag,
    /// Memory is below decay threshold and should be removed.
    BelowDecayThreshold { salience: f32, threshold: f32 },
}

impl std::fmt::Display for MaintenanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaintenanceError::MemoryNotFound { id } => {
                write!(f, "Memory not found: {}", id.as_str())
            }
            MaintenanceError::InvalidLayerTransition { from, to } => {
                write!(f, "Invalid layer transition: {:?} -> {:?}", from, to)
            }
            MaintenanceError::SalienceThresholdNotMet { required, actual } => {
                write!(
                    f,
                    "Salience threshold not met: required {}, actual {}",
                    required, actual
                )
            }
            MaintenanceError::ConsolidationWindowNotElapsed { required, elapsed } => {
                write!(
                    f,
                    "Consolidation window not elapsed: required {:?}, elapsed {:?}",
                    required, elapsed
                )
            }
            MaintenanceError::MissingMilestoneTag => {
                write!(f, "Legacy promotion requires MemoryTag::Milestone")
            }
            MaintenanceError::BelowDecayThreshold {
                salience,
                threshold,
            } => {
                write!(
                    f,
                    "Memory below decay threshold: salience {}, threshold {}",
                    salience, threshold
                )
            }
        }
    }
}

impl std::error::Error for MaintenanceError {}

/// Summary of maintenance operations performed.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MaintenanceReport {
    /// Number of memories promoted to next layer.
    pub promoted: usize,
    /// Number of memories removed due to low salience.
    pub decayed: usize,
    /// IDs of memories that were promoted.
    pub promoted_ids: Vec<MemoryId>,
    /// IDs of memories that were removed.
    pub decayed_ids: Vec<MemoryId>,
}

impl MaintenanceReport {
    /// Creates a new empty maintenance report.
    #[must_use]
    pub fn new() -> Self {
        MaintenanceReport::default()
    }

    /// Returns true if any maintenance operations were performed.
    #[must_use]
    pub fn has_changes(&self) -> bool {
        self.promoted > 0 || self.decayed > 0
    }
}

/// Returns the next layer in the promotion sequence, if any.
///
/// Promotion is strictly sequential: Immediate -> Short-term -> Long-term -> Legacy.
/// Returns None for Legacy (cannot be promoted further).
fn next_layer(layer: MemoryLayer) -> Option<MemoryLayer> {
    match layer {
        MemoryLayer::Immediate => Some(MemoryLayer::ShortTerm),
        MemoryLayer::ShortTerm => Some(MemoryLayer::LongTerm),
        MemoryLayer::LongTerm => Some(MemoryLayer::Legacy),
        MemoryLayer::Legacy => None,
    }
}

/// Returns the salience threshold required for promotion from the given layer.
fn promotion_threshold(from: MemoryLayer) -> f32 {
    match from {
        MemoryLayer::Immediate => THRESHOLD_IMMEDIATE_TO_SHORT,
        MemoryLayer::ShortTerm => THRESHOLD_SHORT_TO_LONG,
        MemoryLayer::LongTerm => THRESHOLD_LONG_TO_LEGACY,
        MemoryLayer::Legacy => f32::INFINITY, // Cannot promote from Legacy
    }
}

/// Returns the base consolidation window in hours for promotion from the given layer.
///
/// - Immediate -> Short-term: 1 hour
/// - Short-term -> Long-term: 24 hours
/// - Long-term -> Legacy: No time requirement (only salience + Milestone tag)
/// - Legacy: Cannot be promoted (returns 0)
fn base_consolidation_window_hours(from: MemoryLayer) -> f32 {
    match from {
        MemoryLayer::Immediate => BASE_WINDOW_IMMEDIATE_HOURS,
        MemoryLayer::ShortTerm => BASE_WINDOW_SHORT_HOURS,
        MemoryLayer::LongTerm => 0.0, // Legacy promotion has no time requirement
        MemoryLayer::Legacy => 0.0,   // Cannot promote from Legacy
    }
}

/// Promotes a memory to the next layer by ID.
///
/// Automatically determines the current layer and promotes to the next sequential layer.
/// Validates salience threshold requirements. For Legacy promotion, also requires
/// `MemoryTag::Milestone`.
///
/// # Arguments
///
/// * `layers` - The memory layers to modify
/// * `id` - The ID of the memory to promote
///
/// # Returns
///
/// Ok(()) on successful promotion, or an error if:
/// - Memory not found
/// - Already in Legacy layer (no next layer)
/// - Salience threshold not met
/// - Missing Milestone tag for Legacy promotion
///
/// # Examples
///
/// ```
/// use behavioral_pathways::memory::{
///     MemoryLayers, MemoryLayer, MemoryEntry, MemoryTag,
///     maintenance::promote_memory,
/// };
/// use behavioral_pathways::types::Duration;
///
/// let mut layers = MemoryLayers::new();
/// let entry = MemoryEntry::new(Duration::hours(2), "Important event")
///     .with_salience(0.5);
/// let id = entry.id().clone();
/// layers.add(MemoryLayer::Immediate, entry);
///
/// // Promote from Immediate to Short-term (requires salience >= 0.3)
/// let result = promote_memory(&mut layers, &id);
/// assert!(result.is_ok());
/// assert_eq!(layers.immediate_count(), 0);
/// assert_eq!(layers.short_term_count(), 1);
/// ```
pub fn promote_memory(layers: &mut MemoryLayers, id: &MemoryId) -> Result<(), MaintenanceError> {
    // Find which layer contains this memory
    let from_layer = layers
        .find_layer(id)
        .ok_or_else(|| MaintenanceError::MemoryNotFound { id: id.clone() })?;

    // Determine target layer
    let to_layer = next_layer(from_layer).ok_or(MaintenanceError::InvalidLayerTransition {
        from: from_layer,
        to: from_layer, // Same layer indicates no valid transition
    })?;

    // Get memory to check salience and tags
    // SAFETY: get_by_id must succeed since find_layer already located this memory.
    // Using expect instead of ok_or_else avoids an unreachable closure.
    let memory = layers
        .get_by_id(id)
        .expect("memory must exist since find_layer succeeded");

    let salience = memory.salience();
    let required_salience = promotion_threshold(from_layer);

    if salience < required_salience {
        return Err(MaintenanceError::SalienceThresholdNotMet {
            required: required_salience,
            actual: salience,
        });
    }

    // Legacy promotion requires Milestone tag
    if to_layer == MemoryLayer::Legacy && !memory.has_tag(MemoryTag::Milestone) {
        return Err(MaintenanceError::MissingMilestoneTag);
    }

    // Move the memory
    layers.move_to_layer(id, to_layer)
}

/// Checks if a memory should be removed due to low salience.
///
/// Returns Ok(()) if the memory survives (salience >= threshold),
/// or Err(BelowDecayThreshold) if it should be removed.
///
/// # Arguments
///
/// * `memory` - The memory entry to check
/// * `threshold` - The salience threshold (memories below this should be removed)
///
/// # Returns
///
/// Ok(()) if memory survives, Err(BelowDecayThreshold) if should be removed.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::memory::{MemoryEntry, maintenance::check_decay};
/// use behavioral_pathways::types::Duration;
///
/// let low_salience = MemoryEntry::new(Duration::days(1), "Fading memory")
///     .with_salience(0.1);
/// assert!(check_decay(&low_salience, 0.2).is_err());
///
/// let high_salience = MemoryEntry::new(Duration::days(1), "Strong memory")
///     .with_salience(0.5);
/// assert!(check_decay(&high_salience, 0.2).is_ok());
/// ```
pub fn check_decay(memory: &MemoryEntry, threshold: f32) -> Result<(), MaintenanceError> {
    let salience = memory.salience();
    if salience < threshold {
        Err(MaintenanceError::BelowDecayThreshold {
            salience,
            threshold,
        })
    } else {
        Ok(())
    }
}

/// Computes the arousal-modulated consolidation window.
///
/// Per McGaugh (2004) and Yerkes-Dodson law, arousal at encoding affects memory
/// consolidation timing with an inverted-U relationship: moderate arousal optimizes
/// consolidation, while both very low and very high arousal impair it.
///
/// # Arguments
///
/// * `base_hours` - The base consolidation window in hours
/// * `arousal_at_encoding` - The arousal value at encoding time (-1.0 to 1.0)
///
/// # Returns
///
/// The effective consolidation window as a Duration.
///
/// # Arousal Effects
///
/// - Optimal arousal (0.0 raw) = base window (fastest consolidation)
/// - Extreme arousal (-1.0 or 1.0) = 2x base window (slowest consolidation)
///
/// # Examples
///
/// ```
/// use behavioral_pathways::memory::maintenance::compute_consolidation_window;
/// use behavioral_pathways::types::Duration;
///
/// // Optimal arousal = base window
/// let optimal = compute_consolidation_window(1.0, 0.0);
/// assert_eq!(optimal, Duration::hours(1));
///
/// // Extreme arousal = 2x base window
/// let extreme = compute_consolidation_window(1.0, 1.0);
/// assert_eq!(extreme, Duration::hours(2));
/// ```
#[must_use]
pub fn compute_consolidation_window(base_hours: f32, arousal_at_encoding: f32) -> Duration {
    // Normalize arousal from [-1, 1] to [0, 1]
    let normalized_arousal = (arousal_at_encoding + 1.0) / 2.0;

    // Inverted-U formula: peak efficiency at normalized = 0.5
    // Both extremes (0.0 and 1.0) impair consolidation
    let distance_from_optimal = (normalized_arousal - 0.5).abs() * 2.0;
    let impairment = distance_from_optimal * distance_from_optimal;

    // Modifier range: 0.5 (impaired) to 1.0 (optimal)
    let modifier = 0.5 + (0.5 * (1.0 - impairment));

    // Effective window = base / modifier
    // Smaller modifier (impairment) = longer window
    let effective_hours = base_hours / modifier;

    Duration::from_hours_f32(effective_hours)
}

/// Applies trauma salience boost at encoding time.
///
/// Per flashbulb memory research, trauma memories receive enhanced salience
/// at encoding. This function multiplies salience by 1.3x if the memory has
/// any trauma tags (Violence, Death, Crisis, Betrayal).
///
/// # Arguments
///
/// * `memory` - The memory entry to potentially boost
///
/// # Returns
///
/// Always returns Ok(()). The boost is only applied if the memory has trauma tags.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::memory::{MemoryEntry, MemoryTag, maintenance::apply_trauma_salience_boost};
/// use behavioral_pathways::types::Duration;
///
/// let mut trauma_memory = MemoryEntry::new(Duration::days(1), "Violent incident")
///     .add_tag(MemoryTag::Violence)
///     .with_salience(0.5);
///
/// apply_trauma_salience_boost(&mut trauma_memory).unwrap();
/// assert!((trauma_memory.salience() - 0.65).abs() < 0.01); // 0.5 * 1.3 = 0.65
/// ```
pub fn apply_trauma_salience_boost(memory: &mut MemoryEntry) -> Result<(), MaintenanceError> {
    // Check if any tag is a trauma tag
    let has_trauma = memory.tags().iter().any(|t| t.is_trauma());

    if has_trauma {
        let new_salience = memory.salience() * TRAUMA_SALIENCE_BOOST;
        memory.set_salience(new_salience); // set_salience clamps to [0, 1]
    }

    Ok(())
}

/// Removes all low-salience memories from the short-term layer.
///
/// Memories with salience below the threshold are removed from the short-term layer.
/// Immediate, long-term, and legacy layers are not affected.
///
/// # Arguments
///
/// * `memories` - The memory layers to modify
/// * `threshold` - The salience threshold (default is 0.2)
///
/// # Returns
///
/// Ok(count) where count is the number of memories removed.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::memory::{MemoryLayers, MemoryLayer, MemoryEntry, maintenance::decay_low_salience};
/// use behavioral_pathways::types::Duration;
///
/// let mut layers = MemoryLayers::new();
/// layers.add(MemoryLayer::ShortTerm, MemoryEntry::new(Duration::days(1), "Low").with_salience(0.1));
/// layers.add(MemoryLayer::ShortTerm, MemoryEntry::new(Duration::days(1), "High").with_salience(0.5));
///
/// let removed = decay_low_salience(&mut layers, 0.2).unwrap();
/// assert_eq!(removed, 1);
/// assert_eq!(layers.short_term_count(), 1);
/// ```
pub fn decay_low_salience(
    memories: &mut MemoryLayers,
    threshold: f32,
) -> Result<usize, MaintenanceError> {
    // Collect IDs of memories to remove (to avoid borrowing issues)
    let ids_to_remove: Vec<MemoryId> = memories
        .short_term()
        .iter()
        .filter(|m| m.salience() < threshold)
        .map(|m| m.id().clone())
        .collect();

    let count = ids_to_remove.len();

    // Remove each memory
    for id in ids_to_remove {
        memories.remove_by_id(&id);
    }

    Ok(count)
}

/// Main maintenance entry point - processes promotions and decay.
///
/// This function should be called periodically during simulation time advancement
/// (recommended: once per simulated day). It is NOT called during `state_at()` queries.
///
/// # Operations Performed
///
/// 1. Removes low-salience memories from short-term layer (threshold: 0.2)
/// 2. Promotes eligible memories between layers based on salience thresholds
///
/// # Arguments
///
/// * `memories` - The memory layers to maintain
/// * `elapsed` - The time elapsed since the memory was formed (used for consolidation windows)
///
/// # Returns
///
/// A MaintenanceReport summarizing what operations were performed.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::memory::{MemoryLayers, MemoryLayer, MemoryEntry, maintenance::apply_memory_maintenance};
/// use behavioral_pathways::types::Duration;
///
/// let mut layers = MemoryLayers::new();
/// layers.add(MemoryLayer::ShortTerm, MemoryEntry::new(Duration::days(1), "Test").with_salience(0.7));
///
/// let report = apply_memory_maintenance(&mut layers, Duration::days(2)).unwrap();
/// // Report contains counts of promoted and decayed memories
/// ```
pub fn apply_memory_maintenance(
    memories: &mut MemoryLayers,
    elapsed: Duration,
) -> Result<MaintenanceReport, MaintenanceError> {
    let mut report = MaintenanceReport::new();

    // Step 1: Decay low-salience memories from short-term
    let decay_ids: Vec<MemoryId> = memories
        .short_term()
        .iter()
        .filter(|m| m.salience() < DECAY_THRESHOLD)
        .map(|m| m.id().clone())
        .collect();

    for id in &decay_ids {
        memories.remove_by_id(id);
        report.decayed_ids.push(id.clone());
    }
    report.decayed = decay_ids.len();

    // Step 2: Collect candidates for promotion from each layer
    // We collect IDs first to avoid borrowing issues during mutation
    // Each candidate must meet BOTH salience threshold AND consolidation window requirements

    // Immediate -> Short-term candidates
    let immediate_candidates: Vec<MemoryId> = memories
        .immediate()
        .iter()
        .filter(|m| {
            // Check salience threshold
            if m.salience() < THRESHOLD_IMMEDIATE_TO_SHORT {
                return false;
            }
            // Check consolidation window
            let base_hours = base_consolidation_window_hours(MemoryLayer::Immediate);
            let arousal = m.emotional_snapshot().arousal();
            let required_window = compute_consolidation_window(base_hours, arousal);
            let time_since_encoding = elapsed.saturating_sub(m.timestamp());
            time_since_encoding >= required_window
        })
        .map(|m| m.id().clone())
        .collect();

    // Short-term -> Long-term candidates
    let short_candidates: Vec<MemoryId> = memories
        .short_term()
        .iter()
        .filter(|m| {
            // Check salience threshold
            if m.salience() < THRESHOLD_SHORT_TO_LONG {
                return false;
            }
            // Check consolidation window
            let base_hours = base_consolidation_window_hours(MemoryLayer::ShortTerm);
            let arousal = m.emotional_snapshot().arousal();
            let required_window = compute_consolidation_window(base_hours, arousal);
            let time_since_encoding = elapsed.saturating_sub(m.timestamp());
            time_since_encoding >= required_window
        })
        .map(|m| m.id().clone())
        .collect();

    // Long-term -> Legacy candidates (requires both high salience AND milestone tag)
    // No time requirement for Legacy promotion (only salience + tag)
    let long_candidates: Vec<MemoryId> = memories
        .long_term()
        .iter()
        .filter(|m| m.salience() >= THRESHOLD_LONG_TO_LEGACY && m.has_tag(MemoryTag::Milestone))
        .map(|m| m.id().clone())
        .collect();

    // Apply promotions
    // Note: promote_memory should succeed for all candidates since we pre-filtered
    // for salience threshold, consolidation window, and milestone tag requirements.
    for id in immediate_candidates {
        promote_memory(memories, &id)
            .expect("immediate candidate should always promote (pre-filtered for requirements)");
        report.promoted += 1;
        report.promoted_ids.push(id);
    }

    for id in short_candidates {
        promote_memory(memories, &id)
            .expect("short-term candidate should always promote (pre-filtered for requirements)");
        report.promoted += 1;
        report.promoted_ids.push(id);
    }

    for id in long_candidates {
        promote_memory(memories, &id)
            .expect("long-term candidate should always promote (pre-filtered for requirements)");
        report.promoted += 1;
        report.promoted_ids.push(id);
    }

    Ok(report)
}

/// Determines if maintenance should run based on elapsed time since last run.
///
/// Maintenance is recommended to run once per simulated day. This helper function
/// returns true if at least one day has elapsed since the last maintenance run.
///
/// # Arguments
///
/// * `last_run` - Duration since entity creation when maintenance last ran
/// * `elapsed` - Current duration since entity creation
///
/// # Returns
///
/// True if maintenance should run (at least 1 day since last run).
///
/// # Examples
///
/// ```
/// use behavioral_pathways::memory::maintenance::should_run_maintenance;
/// use behavioral_pathways::types::Duration;
///
/// // No time has passed since last run
/// assert!(!should_run_maintenance(Duration::days(10), Duration::days(10)));
///
/// // Less than a day has passed
/// assert!(!should_run_maintenance(Duration::days(10), Duration::hours(10 * 24 + 12)));
///
/// // Exactly one day has passed
/// assert!(should_run_maintenance(Duration::days(10), Duration::days(11)));
///
/// // More than a day has passed
/// assert!(should_run_maintenance(Duration::days(10), Duration::days(15)));
/// ```
#[must_use]
pub fn should_run_maintenance(last_run: Duration, elapsed: Duration) -> bool {
    let diff_seconds = elapsed.as_seconds().saturating_sub(last_run.as_seconds());
    diff_seconds >= MAINTENANCE_INTERVAL_SECONDS
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_memory(salience: f32) -> MemoryEntry {
        MemoryEntry::new(Duration::days(1), "Test memory").with_salience(salience)
    }

    /// Creates a memory encoded at timestamp 0 for testing consolidation windows.
    fn create_memory_at_zero(salience: f32) -> MemoryEntry {
        MemoryEntry::new(Duration::zero(), "Test memory").with_salience(salience)
    }

    // === Promotion Tests ===

    #[test]
    fn promote_memory_by_id_succeeds() {
        let mut layers = MemoryLayers::new();
        let entry = create_memory(0.5);
        let id = entry.id().clone();
        layers.add(MemoryLayer::Immediate, entry);

        let result = promote_memory(&mut layers, &id);
        assert!(result.is_ok());
        assert_eq!(layers.immediate_count(), 0);
        assert_eq!(layers.short_term_count(), 1);
    }

    #[test]
    fn promote_memory_returns_error_for_unknown_id() {
        let mut layers = MemoryLayers::new();
        let unknown_id = MemoryId::new("unknown_123").unwrap();

        let result = promote_memory(&mut layers, &unknown_id);
        assert!(result.is_err());
        // Use error Display to verify the error type and content
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Memory not found"));
        assert!(err_msg.contains("unknown_123"));
    }

    #[test]
    fn high_salience_promotes_to_long_term() {
        let mut layers = MemoryLayers::new();
        let entry = create_memory(0.7);
        let id = entry.id().clone();
        layers.add(MemoryLayer::ShortTerm, entry);

        let result = promote_memory(&mut layers, &id);
        assert!(result.is_ok());
        assert_eq!(layers.short_term_count(), 0);
        assert_eq!(layers.long_term_count(), 1);
    }

    #[test]
    fn promotion_requires_both_salience_and_time() {
        // This test verifies salience requirement (time requirement would be
        // checked in integration with consolidation windows)
        let mut layers = MemoryLayers::new();
        let entry = create_memory(0.2); // Below threshold
        let id = entry.id().clone();
        layers.add(MemoryLayer::Immediate, entry);

        let result = promote_memory(&mut layers, &id);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Salience threshold not met"));
        assert!(err_msg.contains("0.3")); // required
        assert!(err_msg.contains("0.2")); // actual
    }

    #[test]
    fn immediate_to_short_term_threshold() {
        let mut layers = MemoryLayers::new();

        // Below threshold (0.29 < 0.3)
        let entry_low = create_memory(0.29);
        let id_low = entry_low.id().clone();
        layers.add(MemoryLayer::Immediate, entry_low);

        let result = promote_memory(&mut layers, &id_low);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Salience threshold not met"));
        assert!(err_msg.contains("0.3")); // required
        assert!(err_msg.contains("0.29")); // actual

        // At threshold (0.3 >= 0.3)
        let entry_ok = create_memory(0.3);
        let id_ok = entry_ok.id().clone();
        layers.add(MemoryLayer::Immediate, entry_ok);

        let result = promote_memory(&mut layers, &id_ok);
        assert!(result.is_ok());
    }

    #[test]
    fn short_term_to_long_term_threshold() {
        let mut layers = MemoryLayers::new();

        // Below threshold (0.59 < 0.6)
        let entry_low = create_memory(0.59);
        let id_low = entry_low.id().clone();
        layers.add(MemoryLayer::ShortTerm, entry_low);

        let result = promote_memory(&mut layers, &id_low);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Salience threshold not met"));
        assert!(err_msg.contains("0.6")); // required
        assert!(err_msg.contains("0.59")); // actual

        // At threshold (0.6 >= 0.6)
        let entry_ok = create_memory(0.6);
        let id_ok = entry_ok.id().clone();
        layers.add(MemoryLayer::ShortTerm, entry_ok);

        let result = promote_memory(&mut layers, &id_ok);
        assert!(result.is_ok());
    }

    #[test]
    fn legacy_requires_milestone_trigger() {
        let mut layers = MemoryLayers::new();

        // High salience but no milestone tag
        let entry = create_memory(0.95);
        let id = entry.id().clone();
        layers.add(MemoryLayer::LongTerm, entry);

        let result = promote_memory(&mut layers, &id);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Milestone"));

        // With milestone tag
        let entry_milestone = create_memory(0.95).add_tag(MemoryTag::Milestone);
        let id_milestone = entry_milestone.id().clone();
        layers.add(MemoryLayer::LongTerm, entry_milestone);

        let result = promote_memory(&mut layers, &id_milestone);
        assert!(result.is_ok());
        assert_eq!(layers.legacy_count(), 1);
    }

    #[test]
    fn layer_skipping_not_allowed() {
        // This is implicit in the design - promote_memory always goes to next layer
        // There's no way to skip layers directly. We test that promoting from Immediate
        // goes to ShortTerm, not directly to LongTerm or Legacy.
        let mut layers = MemoryLayers::new();
        let entry = create_memory(0.95); // High salience
        let id = entry.id().clone();
        layers.add(MemoryLayer::Immediate, entry);

        // First promotion: Immediate -> ShortTerm
        let result = promote_memory(&mut layers, &id);
        assert!(result.is_ok());
        assert_eq!(layers.short_term_count(), 1);
        assert_eq!(layers.long_term_count(), 0);

        // Second promotion: ShortTerm -> LongTerm
        let result = promote_memory(&mut layers, &id);
        assert!(result.is_ok());
        assert_eq!(layers.short_term_count(), 0);
        assert_eq!(layers.long_term_count(), 1);
    }

    // === Decay Tests ===

    #[test]
    fn low_salience_decays_from_short_term() {
        let mut layers = MemoryLayers::new();
        layers.add(MemoryLayer::ShortTerm, create_memory(0.1)); // Below 0.2
        layers.add(MemoryLayer::ShortTerm, create_memory(0.5)); // Above 0.2

        let removed = decay_low_salience(&mut layers, 0.2).unwrap();
        assert_eq!(removed, 1);
        assert_eq!(layers.short_term_count(), 1);
    }

    #[test]
    fn check_decay_returns_error_below_threshold() {
        let entry = create_memory(0.15);
        let result = check_decay(&entry, 0.2);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("decay threshold"));
        assert!(err_msg.contains("0.15")); // salience
        assert!(err_msg.contains("0.2")); // threshold
    }

    #[test]
    fn legacy_memories_never_decay() {
        // Legacy layer is not affected by decay_low_salience
        let mut layers = MemoryLayers::new();
        layers.add(MemoryLayer::Legacy, create_memory(0.1)); // Would be below threshold

        let removed = decay_low_salience(&mut layers, 0.2).unwrap();
        assert_eq!(removed, 0);
        assert_eq!(layers.legacy_count(), 1);
    }

    // === Arousal/Consolidation Tests (Inverted-U) ===

    #[test]
    fn optimal_arousal_gives_shortest_window() {
        // Optimal arousal is 0.0 raw (0.5 normalized) = 1.0x base
        let window = compute_consolidation_window(1.0, 0.0);
        assert_eq!(window, Duration::hours(1));
    }

    #[test]
    fn extreme_low_arousal_impairs_consolidation() {
        // Extreme low arousal (-1.0 raw, 0.0 normalized) = 2.0x base
        let window = compute_consolidation_window(1.0, -1.0);
        assert_eq!(window, Duration::hours(2));
    }

    #[test]
    fn extreme_high_arousal_impairs_consolidation() {
        // Extreme high arousal (1.0 raw, 1.0 normalized) = 2.0x base
        let window = compute_consolidation_window(1.0, 1.0);
        assert_eq!(window, Duration::hours(2));
    }

    #[test]
    fn moderate_arousal_near_optimal() {
        // Arousal in range 0.3-0.7 normalized should be near-optimal
        // -0.4 raw = 0.3 normalized
        let window_low = compute_consolidation_window(1.0, -0.4);
        // +0.4 raw = 0.7 normalized
        let window_high = compute_consolidation_window(1.0, 0.4);

        // Both should be less than 2 hours but close to 1 hour
        assert!(window_low.as_hours() < 2);
        assert!(window_low.as_hours() >= 1);
        assert!(window_high.as_hours() < 2);
        assert!(window_high.as_hours() >= 1);
    }

    // === Trauma Tests ===

    #[test]
    fn trauma_enhanced_salience_at_encoding() {
        let mut entry = MemoryEntry::new(Duration::days(1), "Violent event")
            .add_tag(MemoryTag::Violence)
            .with_salience(0.5);

        apply_trauma_salience_boost(&mut entry).unwrap();

        // 0.5 * 1.3 = 0.65
        assert!((entry.salience() - 0.65).abs() < 0.01);
    }

    #[test]
    fn apply_trauma_boost_returns_ok() {
        // Non-trauma memory - boost not applied but still returns Ok
        let mut entry = create_memory(0.5);
        let result = apply_trauma_salience_boost(&mut entry);
        assert!(result.is_ok());
        assert!((entry.salience() - 0.5).abs() < f32::EPSILON);

        // Trauma memory - boost applied and returns Ok
        let mut trauma_entry = create_memory(0.5).add_tag(MemoryTag::Crisis);
        let result = apply_trauma_salience_boost(&mut trauma_entry);
        assert!(result.is_ok());
    }

    // === Maintenance Report Tests ===

    #[test]
    fn maintenance_returns_report() {
        let mut layers = MemoryLayers::new();
        layers.add(MemoryLayer::ShortTerm, create_memory_at_zero(0.1)); // Will decay
        layers.add(MemoryLayer::Immediate, create_memory_at_zero(0.5)); // Will promote

        // Elapsed = 2 hours is enough to exceed the 1 hour consolidation window
        let report = apply_memory_maintenance(&mut layers, Duration::hours(2)).unwrap();

        assert_eq!(report.decayed, 1);
        assert_eq!(report.promoted, 1);
    }

    #[test]
    fn maintenance_report_includes_ids() {
        let mut layers = MemoryLayers::new();

        let decay_entry = create_memory_at_zero(0.1);
        let decay_id = decay_entry.id().clone();
        layers.add(MemoryLayer::ShortTerm, decay_entry);

        let promote_entry = create_memory_at_zero(0.5);
        let promote_id = promote_entry.id().clone();
        layers.add(MemoryLayer::Immediate, promote_entry);

        // Elapsed = 2 hours is enough to exceed the 1 hour consolidation window
        let report = apply_memory_maintenance(&mut layers, Duration::hours(2)).unwrap();

        assert!(report.decayed_ids.contains(&decay_id));
        assert!(report.promoted_ids.contains(&promote_id));
    }

    // === Additional Tests for Coverage ===

    #[test]
    fn maintenance_report_new_is_empty() {
        let report = MaintenanceReport::new();
        assert_eq!(report.promoted, 0);
        assert_eq!(report.decayed, 0);
        assert!(report.promoted_ids.is_empty());
        assert!(report.decayed_ids.is_empty());
    }

    #[test]
    fn maintenance_report_has_changes() {
        let mut report = MaintenanceReport::new();
        assert!(!report.has_changes());

        report.promoted = 1;
        assert!(report.has_changes());

        report.promoted = 0;
        report.decayed = 1;
        assert!(report.has_changes());
    }

    #[test]
    fn maintenance_error_display() {
        let id = MemoryId::new("test_id").unwrap();
        let err = MaintenanceError::MemoryNotFound { id };
        assert!(err.to_string().contains("test_id"));

        let err = MaintenanceError::InvalidLayerTransition {
            from: MemoryLayer::Immediate,
            to: MemoryLayer::ShortTerm,
        };
        assert!(err.to_string().contains("Immediate"));

        let err = MaintenanceError::SalienceThresholdNotMet {
            required: 0.5,
            actual: 0.3,
        };
        assert!(err.to_string().contains("0.5"));

        let err = MaintenanceError::ConsolidationWindowNotElapsed {
            required: Duration::hours(1),
            elapsed: Duration::minutes(30),
        };
        assert!(err.to_string().contains("elapsed"));

        let err = MaintenanceError::MissingMilestoneTag;
        assert!(err.to_string().contains("Milestone"));

        let err = MaintenanceError::BelowDecayThreshold {
            salience: 0.1,
            threshold: 0.2,
        };
        assert!(err.to_string().contains("0.1"));
    }

    #[test]
    fn should_run_maintenance_respects_interval() {
        // Same time - no maintenance needed
        assert!(!should_run_maintenance(
            Duration::days(10),
            Duration::days(10)
        ));

        // Less than a day passed
        assert!(!should_run_maintenance(
            Duration::days(10),
            Duration::seconds(10 * 86400 + 43200)
        )); // 10.5 days

        // Exactly one day passed
        assert!(should_run_maintenance(
            Duration::days(10),
            Duration::days(11)
        ));

        // More than a day passed
        assert!(should_run_maintenance(
            Duration::days(10),
            Duration::days(15)
        ));
    }

    #[test]
    fn next_layer_returns_correct_sequence() {
        assert_eq!(
            next_layer(MemoryLayer::Immediate),
            Some(MemoryLayer::ShortTerm)
        );
        assert_eq!(
            next_layer(MemoryLayer::ShortTerm),
            Some(MemoryLayer::LongTerm)
        );
        assert_eq!(next_layer(MemoryLayer::LongTerm), Some(MemoryLayer::Legacy));
        assert_eq!(next_layer(MemoryLayer::Legacy), None);
    }

    #[test]
    fn promotion_threshold_returns_correct_values() {
        assert!((promotion_threshold(MemoryLayer::Immediate) - 0.3).abs() < f32::EPSILON);
        assert!((promotion_threshold(MemoryLayer::ShortTerm) - 0.6).abs() < f32::EPSILON);
        assert!((promotion_threshold(MemoryLayer::LongTerm) - 0.9).abs() < f32::EPSILON);
        assert!(promotion_threshold(MemoryLayer::Legacy).is_infinite());
    }

    #[test]
    fn cannot_promote_from_legacy() {
        let mut layers = MemoryLayers::new();
        let entry = create_memory(1.0).add_tag(MemoryTag::Milestone);
        let id = entry.id().clone();
        layers.add(MemoryLayer::Legacy, entry);

        let result = promote_memory(&mut layers, &id);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Invalid layer transition"));
        assert!(err_msg.contains("Legacy"));
    }

    #[test]
    fn trauma_boost_clamped_to_one() {
        // High salience that would exceed 1.0 after boost
        let mut entry = create_memory(0.9).add_tag(MemoryTag::Death);
        apply_trauma_salience_boost(&mut entry).unwrap();

        // 0.9 * 1.3 = 1.17, but should be clamped to 1.0
        assert!((entry.salience() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn check_decay_at_threshold_survives() {
        let entry = create_memory(0.2); // Exactly at threshold
        let result = check_decay(&entry, 0.2);
        assert!(result.is_ok());
    }

    #[test]
    fn apply_memory_maintenance_empty_layers() {
        let mut layers = MemoryLayers::new();
        let report = apply_memory_maintenance(&mut layers, Duration::days(1)).unwrap();
        assert!(!report.has_changes());
    }

    #[test]
    fn apply_memory_maintenance_promotes_through_chain() {
        let mut layers = MemoryLayers::new();

        // Add a high-salience memory with milestone tag to immediate, encoded at time 0
        let entry = create_memory_at_zero(0.95).add_tag(MemoryTag::Milestone);
        layers.add(MemoryLayer::Immediate, entry);

        // First maintenance: Immediate -> ShortTerm (needs 1 hour consolidation window)
        // Use elapsed = 2 hours to exceed the window
        let report = apply_memory_maintenance(&mut layers, Duration::hours(2)).unwrap();
        assert_eq!(report.promoted, 1);
        assert_eq!(layers.short_term_count(), 1);

        // Second maintenance: ShortTerm -> LongTerm (needs 24 hours consolidation window)
        // Use elapsed = 2 days to exceed the window
        let report = apply_memory_maintenance(&mut layers, Duration::days(2)).unwrap();
        assert_eq!(report.promoted, 1);
        assert_eq!(layers.long_term_count(), 1);

        // Third maintenance: LongTerm -> Legacy (has Milestone tag, no time requirement)
        let report = apply_memory_maintenance(&mut layers, Duration::days(3)).unwrap();
        assert_eq!(report.promoted, 1);
        assert_eq!(layers.legacy_count(), 1);
    }

    #[test]
    fn maintenance_report_equality() {
        let report1 = MaintenanceReport::new();
        let report2 = MaintenanceReport::default();
        assert_eq!(report1, report2);
    }

    #[test]
    fn maintenance_report_clone() {
        let mut report = MaintenanceReport::new();
        report.promoted = 5;
        report.decayed = 3;
        report.promoted_ids.push(MemoryId::new("test1").unwrap());

        let cloned = report.clone();
        assert_eq!(report, cloned);
    }

    #[test]
    fn maintenance_report_debug() {
        let report = MaintenanceReport::new();
        let debug = format!("{:?}", report);
        assert!(debug.contains("MaintenanceReport"));
    }

    #[test]
    fn maintenance_error_clone_and_equality() {
        let err1 = MaintenanceError::MissingMilestoneTag;
        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }

    #[test]
    fn decay_low_salience_immediate_layer_not_affected() {
        let mut layers = MemoryLayers::new();
        layers.add(MemoryLayer::Immediate, create_memory(0.1)); // Low salience in immediate

        let removed = decay_low_salience(&mut layers, 0.2).unwrap();
        assert_eq!(removed, 0);
        assert_eq!(layers.immediate_count(), 1);
    }

    #[test]
    fn decay_low_salience_long_term_not_affected() {
        let mut layers = MemoryLayers::new();
        layers.add(MemoryLayer::LongTerm, create_memory(0.1)); // Low salience in long-term

        let removed = decay_low_salience(&mut layers, 0.2).unwrap();
        assert_eq!(removed, 0);
        assert_eq!(layers.long_term_count(), 1);
    }

    #[test]
    fn consolidation_window_24_hour_base() {
        // Short-term to Long-term uses 24 hour base
        let optimal = compute_consolidation_window(24.0, 0.0);
        assert_eq!(optimal, Duration::hours(24));

        let extreme = compute_consolidation_window(24.0, 1.0);
        assert_eq!(extreme, Duration::hours(48));
    }

    #[test]
    fn trauma_tags_all_trigger_boost() {
        // Violence
        let mut v = create_memory(0.5).add_tag(MemoryTag::Violence);
        apply_trauma_salience_boost(&mut v).unwrap();
        assert!((v.salience() - 0.65).abs() < 0.01);

        // Death
        let mut d = create_memory(0.5).add_tag(MemoryTag::Death);
        apply_trauma_salience_boost(&mut d).unwrap();
        assert!((d.salience() - 0.65).abs() < 0.01);

        // Crisis
        let mut c = create_memory(0.5).add_tag(MemoryTag::Crisis);
        apply_trauma_salience_boost(&mut c).unwrap();
        assert!((c.salience() - 0.65).abs() < 0.01);

        // Betrayal
        let mut b = create_memory(0.5).add_tag(MemoryTag::Betrayal);
        apply_trauma_salience_boost(&mut b).unwrap();
        assert!((b.salience() - 0.65).abs() < 0.01);
    }

    #[test]
    fn maintenance_error_is_std_error() {
        use std::error::Error;

        let err = MaintenanceError::MissingMilestoneTag;

        // Verify it implements std::error::Error
        let _: &dyn Error = &err;

        // Verify source is None (no underlying cause)
        assert!(err.source().is_none());
    }

    // === Time-Gated Promotion Tests ===

    #[test]
    fn memory_not_promoted_before_consolidation_window() {
        // Memory encoded at time 0, we check at 30 minutes (less than 1 hour window)
        let mut layers = MemoryLayers::new();
        let entry = create_memory_at_zero(0.5); // High salience, meets threshold
        layers.add(MemoryLayer::Immediate, entry);

        // 30 minutes is less than the 1 hour consolidation window for Immediate->Short-term
        let report = apply_memory_maintenance(&mut layers, Duration::minutes(30)).unwrap();

        // Should not be promoted (window not elapsed)
        assert_eq!(report.promoted, 0);
        assert_eq!(layers.immediate_count(), 1);
        assert_eq!(layers.short_term_count(), 0);
    }

    #[test]
    fn memory_promoted_after_consolidation_window() {
        // Memory encoded at time 0, we check at 2 hours (more than 1 hour window)
        let mut layers = MemoryLayers::new();
        let entry = create_memory_at_zero(0.5); // High salience, meets threshold
        layers.add(MemoryLayer::Immediate, entry);

        // 2 hours exceeds the 1 hour consolidation window for Immediate->Short-term
        let report = apply_memory_maintenance(&mut layers, Duration::hours(2)).unwrap();

        // Should be promoted (window elapsed)
        assert_eq!(report.promoted, 1);
        assert_eq!(layers.immediate_count(), 0);
        assert_eq!(layers.short_term_count(), 1);
    }

    #[test]
    fn extreme_arousal_requires_longer_consolidation_window() {
        use crate::memory::EmotionalSnapshot;

        let mut layers = MemoryLayers::new();

        // Memory with extreme arousal (1.0) requires 2x the base window (2 hours for Immediate->Short-term)
        let entry = create_memory_at_zero(0.5)
            .with_emotional_snapshot(EmotionalSnapshot::new(0.0, 1.0, 0.0)); // Extreme high arousal
        layers.add(MemoryLayer::Immediate, entry);

        // At 1.5 hours, should NOT be promoted (needs 2 hours due to extreme arousal)
        let report = apply_memory_maintenance(&mut layers, Duration::minutes(90)).unwrap();
        assert_eq!(report.promoted, 0);
        assert_eq!(layers.immediate_count(), 1);

        // At 2.5 hours, SHOULD be promoted (exceeds 2 hour window)
        let report = apply_memory_maintenance(&mut layers, Duration::minutes(150)).unwrap();
        assert_eq!(report.promoted, 1);
        assert_eq!(layers.short_term_count(), 1);
    }

    #[test]
    fn optimal_arousal_uses_base_consolidation_window() {
        use crate::memory::EmotionalSnapshot;

        let mut layers = MemoryLayers::new();

        // Memory with optimal arousal (0.0) uses base window (1 hour for Immediate->Short-term)
        let entry = create_memory_at_zero(0.5)
            .with_emotional_snapshot(EmotionalSnapshot::new(0.0, 0.0, 0.0)); // Optimal arousal
        layers.add(MemoryLayer::Immediate, entry);

        // At exactly 1 hour, should be promoted (meets 1 hour window)
        let report = apply_memory_maintenance(&mut layers, Duration::hours(1)).unwrap();
        assert_eq!(report.promoted, 1);
        assert_eq!(layers.short_term_count(), 1);
    }

    #[test]
    fn short_term_to_long_term_uses_24_hour_window() {
        let mut layers = MemoryLayers::new();

        // Memory with high salience (meets 0.6 threshold)
        let entry = create_memory_at_zero(0.7);
        layers.add(MemoryLayer::ShortTerm, entry);

        // At 12 hours, should NOT be promoted (needs 24 hours)
        let report = apply_memory_maintenance(&mut layers, Duration::hours(12)).unwrap();
        assert_eq!(report.promoted, 0);
        assert_eq!(layers.short_term_count(), 1);

        // At 25 hours, SHOULD be promoted (exceeds 24 hour window)
        let report = apply_memory_maintenance(&mut layers, Duration::hours(25)).unwrap();
        assert_eq!(report.promoted, 1);
        assert_eq!(layers.long_term_count(), 1);
    }

    #[test]
    fn base_consolidation_window_hours_returns_correct_values() {
        assert!(
            (base_consolidation_window_hours(MemoryLayer::Immediate) - 1.0).abs() < f32::EPSILON
        );
        assert!(
            (base_consolidation_window_hours(MemoryLayer::ShortTerm) - 24.0).abs() < f32::EPSILON
        );
        assert!(base_consolidation_window_hours(MemoryLayer::LongTerm).abs() < f32::EPSILON);
        assert!(base_consolidation_window_hours(MemoryLayer::Legacy).abs() < f32::EPSILON);
    }

    // === Coverage gap tests: filter closures in apply_memory_maintenance ===

    #[test]
    fn immediate_memory_below_salience_threshold_not_promoted_even_after_window() {
        // This test covers the `return false` branch at line 494:
        // Immediate layer memory with salience < THRESHOLD_IMMEDIATE_TO_SHORT (0.3)
        // but enough time has passed for consolidation window
        let mut layers = MemoryLayers::new();

        // Salience 0.25 is below the 0.3 threshold for Immediate -> Short-term
        let entry = create_memory_at_zero(0.25);
        layers.add(MemoryLayer::Immediate, entry);

        // Elapsed time is well past the consolidation window (2 hours > 1 hour base)
        // but the memory should NOT be promoted due to low salience
        let report = apply_memory_maintenance(&mut layers, Duration::hours(2)).unwrap();

        assert_eq!(report.promoted, 0);
        assert_eq!(layers.immediate_count(), 1); // Still in immediate
        assert_eq!(layers.short_term_count(), 0); // Not promoted
    }

    #[test]
    fn short_term_memory_below_salience_threshold_not_promoted_even_after_window() {
        // This test covers the `return false` branch at line 513:
        // Short-term layer memory with salience < THRESHOLD_SHORT_TO_LONG (0.6)
        // but above decay threshold (0.2), and enough time has passed
        let mut layers = MemoryLayers::new();

        // Salience 0.4 is:
        // - Above decay threshold (0.2), so won't be decayed
        // - Below promotion threshold (0.6) for Short-term -> Long-term
        let entry = create_memory_at_zero(0.4);
        layers.add(MemoryLayer::ShortTerm, entry);

        // Elapsed time is well past the consolidation window (48 hours > 24 hour base)
        // but the memory should NOT be promoted due to salience below threshold
        let report = apply_memory_maintenance(&mut layers, Duration::hours(48)).unwrap();

        assert_eq!(report.promoted, 0);
        assert_eq!(report.decayed, 0); // Not decayed (salience 0.4 > 0.2)
        assert_eq!(layers.short_term_count(), 1); // Still in short-term
        assert_eq!(layers.long_term_count(), 0); // Not promoted
    }
}
