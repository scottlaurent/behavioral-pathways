//! Memory retrieval methods.
//!
//! This module provides retrieval functionality for memories. The primary
//! retrieval methods are implemented on [`MemoryLayers`] for convenience,
//! but this module contains the core retrieval logic and algorithms.
//!
//! # Retrieval Methods
//!
//! - `retrieve_by_salience(threshold)` - Memories above salience threshold
//! - `retrieve_mood_congruent(mood, min_congruence)` - Mood-matching memories
//! - `retrieve_by_tag(tag)` - Memories with specific tag
//! - `retrieve_by_participant(entity_id)` - Memories involving entity
//! - `retrieve_by_context(microsystem_id)` - Memories from context
//! - `retrieve_scored(query)` - Full scored retrieval with all factors
//!
//! # Mood-Congruent Retrieval
//!
//! Mood-congruent retrieval uses the Phase 6 congruence formula:
//!
//! ```text
//! valence_match = 1.0 - abs(memory_valence - mood_valence)
//! arousal_match = 1.0 - abs(memory_arousal - mood_arousal)
//! dominance_match = 1.0 - abs(memory_dominance - mood_dominance)
//! congruence = valence_match * 0.60 + arousal_match * 0.25 + dominance_match * 0.15
//! ```
//!
//! Match values are clamped to [0.0, 1.0].
//!
//! # Unified Retrieval Scoring
//!
//! The `retrieve_scored` function uses a weighted formula (Phase 8):
//!
//! ```text
//! score = relevance_to_tags * 0.25
//!       + participant_match * 0.20
//!       + salience * 0.15
//!       + recency * 0.10
//!       + mood_congruence * 0.10
//!       + context_congruence * 0.10
//!       + source_confidence * 0.05
//!       + base_score * 0.05
//! ```
//!
//! # Examples
//!
//! See [`MemoryLayers`] for usage examples.
//!
//! [`MemoryLayers`]: crate::memory::MemoryLayers

use crate::memory::{MemoryEntry, MemoryTag};
use crate::state::Mood;
use crate::types::{Duration, EntityId, MicrosystemId};

/// Default salience half-life in days for memory decay.
///
/// This is the number of days for a memory's salience to decay by half.
/// Species-based time_scale affects how quickly this half-life is reached.
pub const DEFAULT_SALIENCE_HALF_LIFE_DAYS: f32 = 30.0;

/// Weight for tag relevance in retrieval scoring.
pub const WEIGHT_TAG_RELEVANCE: f64 = 0.25;
/// Weight for participant match in retrieval scoring.
pub const WEIGHT_PARTICIPANT_MATCH: f64 = 0.20;
/// Weight for salience in retrieval scoring.
pub const WEIGHT_SALIENCE: f64 = 0.15;
/// Weight for recency in retrieval scoring.
pub const WEIGHT_RECENCY: f64 = 0.10;
/// Weight for mood congruence in retrieval scoring.
pub const WEIGHT_MOOD_CONGRUENCE: f64 = 0.10;
/// Weight for context congruence in retrieval scoring.
pub const WEIGHT_CONTEXT_CONGRUENCE: f64 = 0.10;
/// Weight for source confidence in retrieval scoring.
pub const WEIGHT_SOURCE_CONFIDENCE: f64 = 0.05;
/// Weight for base score in retrieval scoring.
pub const WEIGHT_BASE_SCORE: f64 = 0.05;

/// Query parameters for scored memory retrieval.
///
/// All fields are optional. Unspecified fields receive neutral scores.
#[derive(Debug, Clone)]
pub struct RetrievalQuery<'a> {
    /// Tags to match against memory tags.
    pub tags: Option<Vec<MemoryTag>>,
    /// Participant to match in memory participants.
    pub participant: Option<EntityId>,
    /// Current mood for congruence calculation.
    pub current_mood: Option<&'a Mood>,
    /// Current microsystem context for congruence boost.
    pub current_context: Option<MicrosystemId>,
    /// Maximum number of results to return.
    pub limit: usize,
    /// Current timestamp for recency calculation.
    pub current_time: Duration,
}

impl<'a> RetrievalQuery<'a> {
    /// Creates a new retrieval query with default values.
    ///
    /// # Arguments
    ///
    /// * `current_time` - The current timestamp for recency calculations
    #[must_use]
    pub fn new(current_time: Duration) -> Self {
        RetrievalQuery {
            tags: None,
            participant: None,
            current_mood: None,
            current_context: None,
            limit: 10,
            current_time,
        }
    }

    /// Sets the tags to match.
    #[must_use]
    pub fn with_tags(mut self, tags: Vec<MemoryTag>) -> Self {
        self.tags = Some(tags);
        self
    }

    /// Sets the participant to match.
    #[must_use]
    pub fn with_participant(mut self, participant: EntityId) -> Self {
        self.participant = Some(participant);
        self
    }

    /// Sets the current mood for congruence.
    #[must_use]
    pub fn with_mood(mut self, mood: &'a Mood) -> Self {
        self.current_mood = Some(mood);
        self
    }

    /// Sets the current context for congruence.
    #[must_use]
    pub fn with_context(mut self, context: MicrosystemId) -> Self {
        self.current_context = Some(context);
        self
    }

    /// Sets the result limit.
    #[must_use]
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }
}

/// Computes the retrieval score for a single memory entry.
///
/// Uses the Phase 8 weighted formula with context congruence.
///
/// # Arguments
///
/// * `entry` - The memory entry to score
/// * `query` - The retrieval query parameters
///
/// # Returns
///
/// A score between 0.0 and 1.0
#[must_use]
pub fn compute_retrieval_score(entry: &MemoryEntry, query: &RetrievalQuery) -> f64 {
    // 1. Tag relevance (0.25)
    let tag_score = if let Some(ref query_tags) = query.tags {
        if query_tags.is_empty() {
            0.5 // Neutral if no tags specified
        } else {
            let matching = query_tags.iter().filter(|t| entry.has_tag(**t)).count();
            matching as f64 / query_tags.len() as f64
        }
    } else {
        0.5 // Neutral if no tags in query
    };

    // 2. Participant match (0.20)
    let participant_score = if let Some(ref participant) = query.participant {
        if entry.involves_participant(participant) {
            1.0
        } else {
            0.0
        }
    } else {
        0.5 // Neutral if no participant in query
    };

    // 3. Salience (0.15)
    let salience_score = entry.salience() as f64;

    // 4. Recency (0.10) - more recent = higher score
    let recency_score = compute_recency_score(entry.timestamp(), query.current_time);

    // 5. Mood congruence (0.10)
    let mood_score = if let Some(mood) = query.current_mood {
        entry
            .emotional_snapshot()
            .compute_congruence_with_mood(mood) as f64
    } else {
        0.5 // Neutral if no mood specified
    };

    // 6. Context congruence (0.10) - NEW in Phase 8
    let context_score = if let Some(ref context) = query.current_context {
        if entry.is_in_context(context) {
            1.0
        } else {
            0.0
        }
    } else {
        0.5 // Neutral if no context specified
    };

    // 7. Source confidence (0.05) - use the memory source's built-in confidence
    let source_score = entry.source().confidence() as f64;

    // 8. Base score (0.05) - ensures non-zero for weak matches
    let base_score = 1.0;

    // Weighted sum
    tag_score * WEIGHT_TAG_RELEVANCE
        + participant_score * WEIGHT_PARTICIPANT_MATCH
        + salience_score * WEIGHT_SALIENCE
        + recency_score * WEIGHT_RECENCY
        + mood_score * WEIGHT_MOOD_CONGRUENCE
        + context_score * WEIGHT_CONTEXT_CONGRUENCE
        + source_score * WEIGHT_SOURCE_CONFIDENCE
        + base_score * WEIGHT_BASE_SCORE
}

/// Computes recency score based on memory age.
///
/// More recent memories get higher scores. Uses exponential decay.
fn compute_recency_score(memory_time: Duration, current_time: Duration) -> f64 {
    let age_days = (current_time.as_days() as i64 - memory_time.as_days() as i64).max(0) as f64;

    // Exponential decay with 30-day half-life
    let half_life = DEFAULT_SALIENCE_HALF_LIFE_DAYS as f64;
    (0.5_f64).powf(age_days / half_life)
}

/// Sorts memory references by salience in descending order.
///
/// This is a utility function used by all retrieval methods to ensure
/// consistent ordering of results.
pub fn sort_by_salience_descending(memories: &mut [&MemoryEntry]) {
    memories.sort_by(|a, b| {
        b.salience()
            .partial_cmp(&a.salience())
            .unwrap_or(std::cmp::Ordering::Equal)
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::EmotionalSnapshot;

    #[test]
    fn sort_by_salience_descending_orders_correctly() {
        let entry1 = MemoryEntry::new(Duration::days(1), "Low").with_salience(0.3);
        let entry2 = MemoryEntry::new(Duration::days(2), "High").with_salience(0.9);
        let entry3 = MemoryEntry::new(Duration::days(3), "Mid").with_salience(0.6);

        let mut refs: Vec<&MemoryEntry> = vec![&entry1, &entry2, &entry3];
        sort_by_salience_descending(&mut refs);

        assert!((refs[0].salience() - 0.9).abs() < f32::EPSILON);
        assert!((refs[1].salience() - 0.6).abs() < f32::EPSILON);
        assert!((refs[2].salience() - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn sort_by_salience_descending_empty_vec() {
        let mut refs: Vec<&MemoryEntry> = vec![];
        sort_by_salience_descending(&mut refs);
        assert!(refs.is_empty());
    }

    #[test]
    fn sort_by_salience_descending_single_element() {
        let entry = MemoryEntry::new(Duration::days(1), "Single").with_salience(0.5);
        let mut refs: Vec<&MemoryEntry> = vec![&entry];
        sort_by_salience_descending(&mut refs);
        assert_eq!(refs.len(), 1);
    }

    #[test]
    fn default_half_life_is_30_days() {
        assert!((DEFAULT_SALIENCE_HALF_LIFE_DAYS - 30.0).abs() < f32::EPSILON);
    }

    #[test]
    fn retrieval_weights_sum_to_one() {
        let sum = WEIGHT_TAG_RELEVANCE
            + WEIGHT_PARTICIPANT_MATCH
            + WEIGHT_SALIENCE
            + WEIGHT_RECENCY
            + WEIGHT_MOOD_CONGRUENCE
            + WEIGHT_CONTEXT_CONGRUENCE
            + WEIGHT_SOURCE_CONFIDENCE
            + WEIGHT_BASE_SCORE;
        assert!((sum - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn retrieval_query_builder() {
        let mood = Mood::new();
        let context = MicrosystemId::new("work").unwrap();
        let participant = EntityId::new("person").unwrap();

        let query = RetrievalQuery::new(Duration::days(100))
            .with_tags(vec![MemoryTag::Personal])
            .with_participant(participant.clone())
            .with_mood(&mood)
            .with_context(context.clone())
            .with_limit(5);

        assert!(query.tags.is_some());
        assert_eq!(query.participant, Some(participant));
        assert!(query.current_mood.is_some());
        assert_eq!(query.current_context, Some(context));
        assert_eq!(query.limit, 5);
    }

    #[test]
    fn compute_retrieval_score_basic() {
        let entry = MemoryEntry::new(Duration::days(10), "Test")
            .with_salience(0.8)
            .add_tag(MemoryTag::Personal);

        let query = RetrievalQuery::new(Duration::days(10));

        let score = compute_retrieval_score(&entry, &query);

        // Should be positive since base_score is always 1.0
        assert!(score > 0.0);
        assert!(score <= 1.0);
    }

    #[test]
    fn compute_retrieval_score_with_matching_tags() {
        let entry = MemoryEntry::new(Duration::days(10), "Test")
            .with_salience(0.5)
            .add_tag(MemoryTag::Personal);

        let query_match =
            RetrievalQuery::new(Duration::days(10)).with_tags(vec![MemoryTag::Personal]);

        let query_no_match =
            RetrievalQuery::new(Duration::days(10)).with_tags(vec![MemoryTag::Mission]);

        let score_match = compute_retrieval_score(&entry, &query_match);
        let score_no_match = compute_retrieval_score(&entry, &query_no_match);

        // Matching tags should score higher
        assert!(score_match > score_no_match);
    }

    #[test]
    fn compute_retrieval_score_with_empty_tags_is_neutral() {
        let entry = MemoryEntry::new(Duration::days(10), "Test")
            .with_salience(0.5)
            .add_tag(MemoryTag::Personal);

        let query_empty = RetrievalQuery::new(Duration::days(10)).with_tags(vec![]);
        let query_none = RetrievalQuery::new(Duration::days(10));

        let score_empty = compute_retrieval_score(&entry, &query_empty);
        let score_none = compute_retrieval_score(&entry, &query_none);

        assert!((score_empty - score_none).abs() < 0.01);
    }

    #[test]
    fn compute_retrieval_score_with_participant() {
        let participant = EntityId::new("person").unwrap();
        let entry = MemoryEntry::new(Duration::days(10), "Test")
            .with_salience(0.5)
            .add_participant(participant.clone());

        let query_match =
            RetrievalQuery::new(Duration::days(10)).with_participant(participant.clone());

        let other = EntityId::new("other").unwrap();
        let query_no_match = RetrievalQuery::new(Duration::days(10)).with_participant(other);

        let score_match = compute_retrieval_score(&entry, &query_match);
        let score_no_match = compute_retrieval_score(&entry, &query_no_match);

        // Matching participant should score higher
        assert!(score_match > score_no_match);
    }

    #[test]
    fn compute_retrieval_score_context_congruence() {
        let context = MicrosystemId::new("work").unwrap();
        let entry = MemoryEntry::new(Duration::days(10), "Test")
            .with_salience(0.5)
            .with_microsystem_context(context.clone());

        let query_match = RetrievalQuery::new(Duration::days(10)).with_context(context.clone());

        let other = MicrosystemId::new("home").unwrap();
        let query_no_match = RetrievalQuery::new(Duration::days(10)).with_context(other);

        let score_match = compute_retrieval_score(&entry, &query_match);
        let score_no_match = compute_retrieval_score(&entry, &query_no_match);

        // Matching context should score higher
        assert!(score_match > score_no_match);
    }

    #[test]
    fn compute_retrieval_score_mood_congruence() {
        let happy_mood = Mood::new().with_valence_base(0.8);
        let sad_mood = Mood::new().with_valence_base(-0.8);

        let happy_entry = MemoryEntry::new(Duration::days(10), "Happy memory")
            .with_salience(0.5)
            .with_emotional_snapshot(EmotionalSnapshot::new(0.8, 0.0, 0.0));

        let query_happy = RetrievalQuery::new(Duration::days(10)).with_mood(&happy_mood);
        let query_sad = RetrievalQuery::new(Duration::days(10)).with_mood(&sad_mood);

        let score_congruent = compute_retrieval_score(&happy_entry, &query_happy);
        let score_incongruent = compute_retrieval_score(&happy_entry, &query_sad);

        // Congruent mood should score higher
        assert!(score_congruent > score_incongruent);
    }

    #[test]
    fn compute_retrieval_score_recency() {
        let recent_entry = MemoryEntry::new(Duration::days(95), "Recent").with_salience(0.5);

        let old_entry = MemoryEntry::new(Duration::days(10), "Old").with_salience(0.5);

        let query = RetrievalQuery::new(Duration::days(100));

        let score_recent = compute_retrieval_score(&recent_entry, &query);
        let score_old = compute_retrieval_score(&old_entry, &query);

        // More recent should score higher
        assert!(score_recent > score_old);
    }

    #[test]
    fn compute_retrieval_score_source_confidence() {
        use crate::memory::MemorySource;

        let self_entry = MemoryEntry::new(Duration::days(10), "Self")
            .with_salience(0.5)
            .with_source(MemorySource::Self_);

        let rumor_entry = MemoryEntry::new(Duration::days(10), "Rumor")
            .with_salience(0.5)
            .with_source(MemorySource::Rumor);

        let query = RetrievalQuery::new(Duration::days(10));

        let score_self = compute_retrieval_score(&self_entry, &query);
        let score_rumor = compute_retrieval_score(&rumor_entry, &query);

        // Self-sourced should have higher confidence than rumor
        assert!(score_self > score_rumor);
    }

    #[test]
    fn recency_score_at_same_time_is_one() {
        let score = compute_recency_score(Duration::days(100), Duration::days(100));
        assert!((score - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn recency_score_decays_with_time() {
        let score_fresh = compute_recency_score(Duration::days(100), Duration::days(100));
        let score_half_life = compute_recency_score(Duration::days(70), Duration::days(100));
        let score_old = compute_recency_score(Duration::days(40), Duration::days(100));

        assert!(score_fresh > score_half_life);
        assert!(score_half_life > score_old);

        // At one half-life, score should be ~0.5
        assert!((score_half_life - 0.5).abs() < 0.01);
    }

    #[test]
    fn retrieval_query_debug() {
        let query = RetrievalQuery::new(Duration::days(100));
        let debug = format!("{:?}", query);
        assert!(debug.contains("RetrievalQuery"));
    }

    #[test]
    fn retrieval_query_clone() {
        let query = RetrievalQuery::new(Duration::days(100)).with_limit(5);
        let cloned = query.clone();
        assert_eq!(cloned.limit, 5);
    }
}
