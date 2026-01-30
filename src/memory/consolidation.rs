//! Memory consolidation for state evolution.
//!
//! This module applies memory consolidation effects to entity state during
//! temporal state computation. Effects include mood-congruent priming where
//! the current mood biases which memories contribute to state changes.
//!
//! # Phase 11 Implementation
//!
//! This phase implements mood-congruent priming effects on IndividualState:
//!
//! - Negative mood + negative memories -> negative valence priming delta
//! - Positive mood + positive memories -> positive valence priming delta
//! - High-salience memory retrieval -> arousal priming delta
//!
//! Memory layer mutations (promotion, decay removal) are deferred to Phase 12
//! as they require entity mutation which contradicts the pure `state_at()` model.

use crate::memory::{MemoryLayers, MemoryTag};
use crate::state::{IndividualState, Mood};
use crate::types::Duration;

/// Priming deltas computed from memory consolidation.
///
/// These values represent the mood changes induced by mood-congruent
/// memory retrieval during state computation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrimingDeltas {
    /// Change in valence from mood-congruent memory retrieval.
    /// Negative mood retrieves more negative memories, amplifying negative valence.
    pub valence_delta: f32,

    /// Change in arousal from high-salience memory activation.
    /// High-salience memories increase arousal when retrieved.
    pub arousal_delta: f32,
}

impl PrimingDeltas {
    /// Creates a new PrimingDeltas with zero values.
    #[must_use]
    pub fn zero() -> Self {
        PrimingDeltas {
            valence_delta: 0.0,
            arousal_delta: 0.0,
        }
    }

    /// Creates a new PrimingDeltas with specified values.
    #[must_use]
    pub fn new(valence_delta: f32, arousal_delta: f32) -> Self {
        PrimingDeltas {
            valence_delta,
            arousal_delta,
        }
    }
}

impl Default for PrimingDeltas {
    fn default() -> Self {
        PrimingDeltas::zero()
    }
}

/// Scaling factor for priming effects per day of duration.
/// This ensures priming scales with how long consolidation has occurred.
const PRIMING_SCALE_PER_DAY: f32 = 0.01;

/// Maximum priming effect that can be applied.
const MAX_PRIMING_EFFECT: f32 = 0.2;

/// Minimum salience threshold for memories to contribute to arousal priming.
const AROUSAL_PRIMING_SALIENCE_THRESHOLD: f32 = 0.5;

/// Arousal boost per high-salience memory (scaled by actual salience).
const AROUSAL_PRIMING_FACTOR: f32 = 0.02;

/// Computes priming deltas from mood-congruent memory retrieval.
///
/// This function implements mood-congruent priming: the current mood biases
/// which memories are mentally retrieved, and those retrieved memories in
/// turn affect mood.
///
/// Memory polarity is determined PRIMARILY by tags (via `is_negative()` and
/// `is_positive()` methods), with emotional snapshot valence as a secondary factor.
///
/// - Negative mood -> more negative-tagged memories retrieved -> negative valence delta
/// - Positive mood -> more positive-tagged memories retrieved -> positive valence delta
/// - High-salience memories -> increased arousal
///
/// # Arguments
///
/// * `memories` - The entity's memory layers
/// * `mood` - The current mood state
///
/// # Returns
///
/// Priming deltas to be applied to the state.
#[must_use]
pub fn compute_priming_deltas(memories: &MemoryLayers, mood: &Mood) -> PrimingDeltas {
    if memories.is_empty() {
        return PrimingDeltas::zero();
    }

    let valence = mood.valence_effective();
    let mut valence_delta = 0.0;
    let mut arousal_delta = 0.0;

    // Count memories for normalization - since we already checked is_empty(),
    // this will always be at least 1
    let memory_count = memories.all_memories().count();

    // Iterate through all memories to compute priming effects
    for memory in memories.all_memories() {
        let salience = memory.salience();

        // Determine memory polarity from tags (primary) and emotional snapshot (secondary)
        let (tag_polarity, has_polarity_tags) = compute_tag_polarity(memory.tags());
        let snapshot_valence = memory.emotional_snapshot().valence();

        // Memory valence is primarily from tags, with emotional snapshot as fallback/modifier
        // If tags indicate polarity, use that as primary (weight 0.7) with snapshot as secondary (0.3)
        // If no polarity tags, use emotional snapshot alone
        let memory_valence = if has_polarity_tags {
            tag_polarity * 0.7 + snapshot_valence * 0.3
        } else {
            snapshot_valence
        };

        // Mood-congruent priming: current mood biases retrieval
        // Negative mood retrieves more negative-tagged memories (higher weight)
        // Positive mood retrieves more positive-tagged memories (higher weight)
        let congruence = compute_mood_congruence(valence, tag_polarity, has_polarity_tags);

        // Memory contributes to valence delta weighted by congruence and salience
        valence_delta += memory_valence * salience * congruence * 0.1;

        // High-salience memories increase arousal (attention activation)
        if salience >= AROUSAL_PRIMING_SALIENCE_THRESHOLD {
            arousal_delta += salience * AROUSAL_PRIMING_FACTOR;
        }
    }

    // Normalize by memory count to prevent unbounded growth
    // Safe to divide directly since we already checked is_empty() above
    valence_delta /= memory_count as f32;

    // Clamp effects to reasonable bounds
    valence_delta = valence_delta.clamp(-MAX_PRIMING_EFFECT, MAX_PRIMING_EFFECT);
    arousal_delta = arousal_delta.clamp(0.0, MAX_PRIMING_EFFECT);

    PrimingDeltas::new(valence_delta, arousal_delta)
}

/// Computes the polarity of a memory based on its tags.
///
/// Returns a tuple of (polarity, has_polarity_tags) where:
/// - polarity is in range [-1.0, 1.0]: negative tags contribute -1, positive tags contribute +1
/// - has_polarity_tags is true if any positive or negative tags were found
///
/// Tags are the primary source of memory polarity per phase-11.md requirements.
fn compute_tag_polarity(tags: &[MemoryTag]) -> (f32, bool) {
    let mut negative_count = 0;
    let mut positive_count = 0;

    for tag in tags {
        if tag.is_negative() {
            negative_count += 1;
        }
        if tag.is_positive() {
            positive_count += 1;
        }
    }

    let total_polarity_tags = negative_count + positive_count;
    if total_polarity_tags == 0 {
        return (0.0, false);
    }

    // Net polarity: (positive - negative) / total, normalized to [-1, 1]
    let polarity = (positive_count as f32 - negative_count as f32) / total_polarity_tags as f32;

    (polarity, true)
}

/// Computes mood congruence factor based on mood valence and memory tag polarity.
///
/// When tag polarity matches mood sign, retrieval weight is boosted.
/// When they oppose, retrieval weight is reduced.
/// Neutral mood or no polarity tags results in neutral weighting.
fn compute_mood_congruence(mood_valence: f32, tag_polarity: f32, has_polarity_tags: bool) -> f32 {
    if !has_polarity_tags {
        // No polarity tags: use neutral congruence
        return 1.0;
    }

    if mood_valence.abs() < 0.1 {
        // Neutral mood: no bias
        return 1.0;
    }

    // Check if mood and tag polarity have same sign (congruent)
    let mood_negative = mood_valence < 0.0;
    let tag_negative = tag_polarity < 0.0;

    if mood_negative == tag_negative && tag_polarity.abs() > 0.1 {
        // Congruent: mood and memory tags have same polarity
        1.0 + mood_valence.abs() * 0.5 // Boost retrieval weight
    } else if tag_polarity.abs() < 0.1 {
        // Mixed tags (roughly neutral polarity): no strong bias
        1.0
    } else {
        // Incongruent: mood and memory tags have opposite polarity
        1.0 - mood_valence.abs() * 0.3 // Reduce retrieval weight
    }
}

/// Applies memory consolidation effects to state.
///
/// This function applies memory system effects to the state during temporal
/// evolution. It is called by `state_at()` AFTER decay, event processing,
/// developmental modifiers, and context effects (last in the chain).
///
/// # Arguments
///
/// * `state` - The current state after all prior processing
/// * `memories` - The entity's memory layers
/// * `duration` - The time elapsed for consolidation
///
/// # Returns
///
/// The state with memory consolidation effects applied.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::memory::{apply_memory_consolidation, MemoryLayers};
/// use behavioral_pathways::state::IndividualState;
/// use behavioral_pathways::types::Duration;
///
/// let state = IndividualState::new();
/// let memories = MemoryLayers::new();
/// let modified = apply_memory_consolidation(state.clone(), &memories, Duration::days(30));
///
/// // With no memories, state is unchanged
/// assert_eq!(modified.mood().valence_delta(), state.mood().valence_delta());
/// ```
#[must_use]
pub fn apply_memory_consolidation(
    mut state: IndividualState,
    memories: &MemoryLayers,
    duration: Duration,
) -> IndividualState {
    // No consolidation over zero duration
    if duration.is_zero() {
        return state;
    }

    // No consolidation with no memories
    if memories.is_empty() {
        return state;
    }

    // Compute priming deltas based on current mood and memories
    let priming = compute_priming_deltas(memories, state.mood());

    // Scale priming by duration (longer consolidation = stronger effect)
    let days = duration.as_days() as f32;
    let scale = (days * PRIMING_SCALE_PER_DAY).min(1.0);

    // Apply scaled priming deltas to state
    let scaled_valence = priming.valence_delta * scale;
    let scaled_arousal = priming.arousal_delta * scale;

    state.mood_mut().add_valence_delta(scaled_valence);
    state.mood_mut().add_arousal_delta(scaled_arousal);

    state
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::{EmotionalSnapshot, MemoryEntry, MemoryLayer, MemoryTag};

    fn create_memory_with_emotion(valence: f32, arousal: f32, salience: f32) -> MemoryEntry {
        MemoryEntry::new(Duration::days(1), "Test memory")
            .with_emotional_snapshot(EmotionalSnapshot::new(valence, arousal, 0.0))
            .with_salience(salience)
    }

    fn create_negative_memory(salience: f32) -> MemoryEntry {
        create_memory_with_emotion(-0.7, 0.3, salience).add_tag(MemoryTag::Loss)
    }

    fn create_positive_memory(salience: f32) -> MemoryEntry {
        create_memory_with_emotion(0.7, 0.3, salience).add_tag(MemoryTag::Achievement)
    }

    #[test]
    fn negative_mood_increases_negative_memory_priming() {
        let mut memories = MemoryLayers::new();
        memories.add(MemoryLayer::ShortTerm, create_negative_memory(0.7));
        memories.add(MemoryLayer::ShortTerm, create_positive_memory(0.7));

        // Negative mood
        let negative_mood = Mood::new().with_valence_base(-0.5);
        let negative_priming = compute_priming_deltas(&memories, &negative_mood);

        // Neutral mood
        let neutral_mood = Mood::new();
        let neutral_priming = compute_priming_deltas(&memories, &neutral_mood);

        // Negative mood should produce more negative valence delta
        // (negative memories are weighted higher)
        assert!(negative_priming.valence_delta < neutral_priming.valence_delta);
    }

    #[test]
    fn positive_mood_increases_positive_memory_priming() {
        let mut memories = MemoryLayers::new();
        memories.add(MemoryLayer::ShortTerm, create_negative_memory(0.7));
        memories.add(MemoryLayer::ShortTerm, create_positive_memory(0.7));

        // Positive mood
        let positive_mood = Mood::new().with_valence_base(0.5);
        let positive_priming = compute_priming_deltas(&memories, &positive_mood);

        // Neutral mood
        let neutral_mood = Mood::new();
        let neutral_priming = compute_priming_deltas(&memories, &neutral_mood);

        // Positive mood should produce more positive valence delta
        // (positive memories are weighted higher)
        assert!(positive_priming.valence_delta > neutral_priming.valence_delta);
    }

    #[test]
    fn priming_affects_valence_delta() {
        let mut memories = MemoryLayers::new();
        // Add several negative memories
        for _ in 0..5 {
            memories.add(MemoryLayer::ShortTerm, create_negative_memory(0.8));
        }

        let state = IndividualState::new();
        let original_valence = state.mood().valence_delta();

        // Apply consolidation over time
        let modified = apply_memory_consolidation(state, &memories, Duration::days(30));

        // Valence should have changed (negative priming from negative memories)
        assert!((modified.mood().valence_delta() - original_valence).abs() > f32::EPSILON);
    }

    #[test]
    fn priming_affects_arousal_delta() {
        let mut memories = MemoryLayers::new();
        // Add high-salience memories (above threshold)
        for _ in 0..5 {
            memories.add(
                MemoryLayer::ShortTerm,
                create_memory_with_emotion(0.0, 0.0, 0.8), // High salience
            );
        }

        let state = IndividualState::new();
        let original_arousal = state.mood().arousal_delta();

        // Apply consolidation
        let modified = apply_memory_consolidation(state, &memories, Duration::days(30));

        // Arousal should have increased (high-salience memories activate attention)
        assert!(modified.mood().arousal_delta() > original_arousal);
    }

    #[test]
    fn same_inputs_produce_same_output() {
        let mut memories = MemoryLayers::new();
        memories.add(MemoryLayer::ShortTerm, create_negative_memory(0.7));
        memories.add(MemoryLayer::ShortTerm, create_positive_memory(0.6));

        let state = IndividualState::new();
        let duration = Duration::days(30);

        // Run consolidation twice with same inputs
        let result1 = apply_memory_consolidation(state.clone(), &memories, duration);
        let result2 = apply_memory_consolidation(state, &memories, duration);

        // Results should be identical (deterministic)
        assert!(
            (result1.mood().valence_delta() - result2.mood().valence_delta()).abs() < f32::EPSILON
        );
        assert!(
            (result1.mood().arousal_delta() - result2.mood().arousal_delta()).abs() < f32::EPSILON
        );
    }

    #[test]
    fn zero_duration_produces_no_priming() {
        let mut memories = MemoryLayers::new();
        memories.add(MemoryLayer::ShortTerm, create_negative_memory(0.8));

        let state = IndividualState::new();
        let original_valence = state.mood().valence_delta();

        // Zero duration
        let result = apply_memory_consolidation(state, &memories, Duration::zero());

        // No change should occur
        assert!((result.mood().valence_delta() - original_valence).abs() < f32::EPSILON);
    }

    #[test]
    fn no_memories_produces_no_effect() {
        let memories = MemoryLayers::new(); // Empty
        let state = IndividualState::new();
        let original_valence = state.mood().valence_delta();
        let original_arousal = state.mood().arousal_delta();

        let result = apply_memory_consolidation(state, &memories, Duration::days(30));

        // No change should occur
        assert!((result.mood().valence_delta() - original_valence).abs() < f32::EPSILON);
        assert!((result.mood().arousal_delta() - original_arousal).abs() < f32::EPSILON);
    }

    #[test]
    fn priming_deltas_zero_creates_zero_values() {
        let deltas = PrimingDeltas::zero();
        assert!(deltas.valence_delta.abs() < f32::EPSILON);
        assert!(deltas.arousal_delta.abs() < f32::EPSILON);
    }

    #[test]
    fn priming_deltas_new_stores_values() {
        let deltas = PrimingDeltas::new(0.3, 0.2);
        assert!((deltas.valence_delta - 0.3).abs() < f32::EPSILON);
        assert!((deltas.arousal_delta - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn priming_deltas_default_is_zero() {
        let deltas = PrimingDeltas::default();
        assert!(deltas.valence_delta.abs() < f32::EPSILON);
        assert!(deltas.arousal_delta.abs() < f32::EPSILON);
    }

    #[test]
    fn priming_deltas_clone_and_equality() {
        let deltas1 = PrimingDeltas::new(0.5, 0.3);
        let deltas2 = deltas1;
        assert_eq!(deltas1, deltas2);
    }

    #[test]
    fn priming_deltas_debug_format() {
        let deltas = PrimingDeltas::new(0.1, 0.2);
        let debug = format!("{:?}", deltas);
        assert!(debug.contains("PrimingDeltas"));
    }

    #[test]
    fn compute_priming_deltas_empty_memories_returns_zero() {
        let memories = MemoryLayers::new();
        let mood = Mood::new();

        let deltas = compute_priming_deltas(&memories, &mood);

        assert!(deltas.valence_delta.abs() < f32::EPSILON);
        assert!(deltas.arousal_delta.abs() < f32::EPSILON);
    }

    #[test]
    fn arousal_priming_only_from_high_salience() {
        let mut low_salience_memories = MemoryLayers::new();
        low_salience_memories.add(
            MemoryLayer::ShortTerm,
            create_memory_with_emotion(0.0, 0.0, 0.2), // Low salience
        );

        let mut high_salience_memories = MemoryLayers::new();
        high_salience_memories.add(
            MemoryLayer::ShortTerm,
            create_memory_with_emotion(0.0, 0.0, 0.8), // High salience
        );

        let mood = Mood::new();

        let low_deltas = compute_priming_deltas(&low_salience_memories, &mood);
        let high_deltas = compute_priming_deltas(&high_salience_memories, &mood);

        // Low salience should not contribute to arousal
        assert!(low_deltas.arousal_delta.abs() < f32::EPSILON);

        // High salience should contribute to arousal
        assert!(high_deltas.arousal_delta > 0.0);
    }

    #[test]
    fn consolidation_scales_with_duration() {
        let mut memories = MemoryLayers::new();
        memories.add(MemoryLayer::ShortTerm, create_negative_memory(0.8));

        let state = IndividualState::new();

        // Short duration
        let short_result = apply_memory_consolidation(state.clone(), &memories, Duration::days(1));

        // Long duration
        let long_result = apply_memory_consolidation(state, &memories, Duration::days(100));

        // Longer duration should produce larger effect (or saturated at max)
        let short_effect = short_result.mood().valence_delta().abs();
        let long_effect = long_result.mood().valence_delta().abs();

        assert!(long_effect >= short_effect);
    }

    #[test]
    fn priming_is_clamped_to_bounds() {
        let mut memories = MemoryLayers::new();
        // Add many extreme negative memories
        for _ in 0..100 {
            memories.add(
                MemoryLayer::ShortTerm,
                create_memory_with_emotion(-1.0, 0.0, 1.0),
            );
        }

        let negative_mood = Mood::new().with_valence_base(-1.0);
        let deltas = compute_priming_deltas(&memories, &negative_mood);

        // Valence delta should be clamped
        assert!(deltas.valence_delta >= -MAX_PRIMING_EFFECT);
        assert!(deltas.valence_delta <= MAX_PRIMING_EFFECT);
    }

    // Tests for compute_tag_polarity helper function

    #[test]
    fn compute_tag_polarity_empty_tags_returns_zero_and_false() {
        let tags: Vec<MemoryTag> = vec![];
        let (polarity, has_tags) = compute_tag_polarity(&tags);
        assert!(polarity.abs() < f32::EPSILON);
        assert!(!has_tags);
    }

    #[test]
    fn compute_tag_polarity_negative_tags_returns_negative() {
        let tags = vec![MemoryTag::Loss, MemoryTag::Violence];
        let (polarity, has_tags) = compute_tag_polarity(&tags);
        assert!(polarity < 0.0);
        assert!(has_tags);
        // Two negative tags: (-2) / 2 = -1.0
        assert!((polarity - (-1.0)).abs() < f32::EPSILON);
    }

    #[test]
    fn compute_tag_polarity_positive_tags_returns_positive() {
        let tags = vec![MemoryTag::Achievement, MemoryTag::Support];
        let (polarity, has_tags) = compute_tag_polarity(&tags);
        assert!(polarity > 0.0);
        assert!(has_tags);
        // Two positive tags: 2 / 2 = 1.0
        assert!((polarity - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn compute_tag_polarity_mixed_tags_returns_net() {
        // One negative, one positive should net to 0
        let tags = vec![MemoryTag::Loss, MemoryTag::Achievement];
        let (polarity, has_tags) = compute_tag_polarity(&tags);
        assert!(polarity.abs() < f32::EPSILON);
        assert!(has_tags);
    }

    #[test]
    fn compute_tag_polarity_neutral_tags_returns_zero_and_false() {
        // Mission and Personal are neutral tags
        let tags = vec![MemoryTag::Mission, MemoryTag::Personal];
        let (polarity, has_tags) = compute_tag_polarity(&tags);
        assert!(polarity.abs() < f32::EPSILON);
        assert!(!has_tags);
    }

    #[test]
    fn compute_tag_polarity_mixed_with_neutral_ignores_neutral() {
        // Neutral tags should not affect polarity calculation
        let tags = vec![MemoryTag::Mission, MemoryTag::Loss, MemoryTag::Personal];
        let (polarity, has_tags) = compute_tag_polarity(&tags);
        assert!(polarity < 0.0);
        assert!(has_tags);
        // Only Loss is a polarity tag: -1 / 1 = -1.0
        assert!((polarity - (-1.0)).abs() < f32::EPSILON);
    }

    // Tests for compute_mood_congruence helper function

    #[test]
    fn compute_mood_congruence_no_polarity_tags_returns_one() {
        let congruence = compute_mood_congruence(-0.5, 0.0, false);
        assert!((congruence - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn compute_mood_congruence_neutral_mood_returns_one() {
        // Mood is near zero (neutral)
        let congruence = compute_mood_congruence(0.05, -0.5, true);
        assert!((congruence - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn compute_mood_congruence_congruent_negative_boosts() {
        // Negative mood, negative tag polarity -> boost
        let congruence = compute_mood_congruence(-0.5, -0.5, true);
        assert!(congruence > 1.0);
        // Expected: 1.0 + 0.5 * 0.5 = 1.25
        assert!((congruence - 1.25).abs() < f32::EPSILON);
    }

    #[test]
    fn compute_mood_congruence_congruent_positive_boosts() {
        // Positive mood, positive tag polarity -> boost
        let congruence = compute_mood_congruence(0.6, 0.8, true);
        assert!(congruence > 1.0);
        // Expected: 1.0 + 0.6 * 0.5 = 1.3
        assert!((congruence - 1.3).abs() < f32::EPSILON);
    }

    #[test]
    fn compute_mood_congruence_incongruent_reduces() {
        // Negative mood, positive tag polarity -> reduce
        let congruence = compute_mood_congruence(-0.5, 0.5, true);
        assert!(congruence < 1.0);
        // Expected: 1.0 - 0.5 * 0.3 = 0.85
        assert!((congruence - 0.85).abs() < f32::EPSILON);
    }

    #[test]
    fn compute_mood_congruence_mixed_tags_neutral_polarity() {
        // Tag polarity near zero (mixed tags) -> neutral congruence
        let congruence = compute_mood_congruence(-0.5, 0.05, true);
        assert!((congruence - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn compute_mood_congruence_same_sign_but_weak_polarity_negative() {
        // Same sign (both negative) but tag polarity is weak (between 0 and 0.1)
        // Should fall through to neutral because polarity is too weak to matter
        let congruence = compute_mood_congruence(-0.5, -0.08, true);
        assert!((congruence - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn compute_mood_congruence_same_sign_but_weak_polarity_positive() {
        // Same sign (both positive) but tag polarity is weak (between 0 and 0.1)
        // Should fall through to neutral because polarity is too weak to matter
        let congruence = compute_mood_congruence(0.5, 0.08, true);
        assert!((congruence - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn compute_mood_congruence_opposite_signs() {
        // Positive mood but negative tag polarity (incongruent)
        let congruence = compute_mood_congruence(0.5, -0.5, true);
        assert!(congruence < 1.0);
        // Expected: 1.0 - 0.5 * 0.3 = 0.85
        assert!((congruence - 0.85).abs() < f32::EPSILON);
    }

    // Tests for tag-based priming behavior

    #[test]
    fn tag_polarity_drives_priming_over_snapshot() {
        // Memory with negative tag but positive emotional snapshot
        // Tag polarity should dominate the priming calculation
        let mut memories = MemoryLayers::new();
        memories.add(
            MemoryLayer::ShortTerm,
            MemoryEntry::new(Duration::days(1), "Negative tagged, positive emotion")
                .with_emotional_snapshot(EmotionalSnapshot::new(0.8, 0.0, 0.0)) // Positive snapshot
                .add_tag(MemoryTag::Loss) // Negative tag
                .with_salience(0.7),
        );

        let negative_mood = Mood::new().with_valence_base(-0.5);
        let priming = compute_priming_deltas(&memories, &negative_mood);

        // Should be negative because tag polarity dominates (0.7 weight for tags)
        // Tag polarity: -1.0, Snapshot: 0.8
        // Combined: -1.0 * 0.7 + 0.8 * 0.3 = -0.7 + 0.24 = -0.46
        // With negative mood congruence boost and salience scaling, should be negative
        assert!(priming.valence_delta < 0.0);
    }

    #[test]
    fn memories_without_tags_use_snapshot_only() {
        // Memory with no polarity tags should fall back to emotional snapshot
        let mut memories = MemoryLayers::new();
        memories.add(
            MemoryLayer::ShortTerm,
            MemoryEntry::new(Duration::days(1), "No polarity tags")
                .with_emotional_snapshot(EmotionalSnapshot::new(-0.5, 0.0, 0.0))
                .add_tag(MemoryTag::Mission) // Neutral tag, not a polarity tag
                .with_salience(0.7),
        );

        let mood = Mood::new();
        let priming = compute_priming_deltas(&memories, &mood);

        // Should be negative because snapshot is negative and no tags to override
        assert!(priming.valence_delta < 0.0);
    }

    #[test]
    fn positive_tagged_memories_with_positive_mood_amplify() {
        let mut memories = MemoryLayers::new();
        // Add purely positive-tagged memories
        for _ in 0..3 {
            memories.add(
                MemoryLayer::ShortTerm,
                MemoryEntry::new(Duration::days(1), "Achievement")
                    .with_emotional_snapshot(EmotionalSnapshot::new(0.5, 0.0, 0.0))
                    .add_tag(MemoryTag::Achievement)
                    .with_salience(0.7),
            );
        }

        let positive_mood = Mood::new().with_valence_base(0.6);
        let neutral_mood = Mood::new();

        let positive_priming = compute_priming_deltas(&memories, &positive_mood);
        let neutral_priming = compute_priming_deltas(&memories, &neutral_mood);

        // Positive mood should amplify positive-tagged memory priming
        assert!(positive_priming.valence_delta > neutral_priming.valence_delta);
    }
}
