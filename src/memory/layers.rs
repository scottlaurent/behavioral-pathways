//! Memory layer storage with capacity limits and eviction policy.
//!
//! Memories are organized into four layers with different capacities and
//! time horizons. When a layer reaches capacity, the lowest-salience
//! memory is evicted.

use crate::memory::retrieval::{compute_retrieval_score, RetrievalQuery};
use crate::memory::{MemoryEntry, MemoryTag};
use crate::state::Mood;
use crate::types::{Duration, EntityId, MemoryId, MicrosystemId};

/// Capacity of the immediate memory layer.
pub const IMMEDIATE_CAPACITY: usize = 10;

/// Capacity of the short-term memory layer.
pub const SHORT_TERM_CAPACITY: usize = 20;

/// Capacity of the long-term memory layer.
pub const LONG_TERM_CAPACITY: usize = 50;

/// Container for memory entries organized by temporal layer.
///
/// Memories are stored in four layers with different capacities:
/// - Immediate: 10 entries (minutes-hours horizon)
/// - Short-term: 20 entries (days-weeks horizon)
/// - Long-term: 50 entries (months-years horizon)
/// - Legacy: Unlimited (milestone-triggered)
///
/// When a layer reaches capacity, the lowest-salience entry is evicted.
/// If there's a tie, the oldest entry (lowest timestamp) is removed.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::memory::{MemoryLayers, MemoryEntry, MemoryLayer};
/// use behavioral_pathways::types::Duration;
///
/// let mut layers = MemoryLayers::new();
/// let entry = MemoryEntry::new(Duration::days(1), "Test memory");
/// layers.add(MemoryLayer::Immediate, entry);
///
/// assert_eq!(layers.immediate_count(), 1);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct MemoryLayers {
    /// Immediate memories (capacity: 10, horizon: minutes-hours).
    immediate: Vec<MemoryEntry>,

    /// Short-term memories (capacity: 20, horizon: days-weeks).
    short_term: Vec<MemoryEntry>,

    /// Long-term memories (capacity: 50, horizon: months-years).
    long_term: Vec<MemoryEntry>,

    /// Legacy memories (unlimited, milestone-triggered).
    legacy: Vec<MemoryEntry>,
}

/// Specifies which memory layer to operate on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryLayer {
    /// Immediate memory layer (capacity: 10).
    Immediate,
    /// Short-term memory layer (capacity: 20).
    ShortTerm,
    /// Long-term memory layer (capacity: 50).
    LongTerm,
    /// Legacy memory layer (unlimited).
    Legacy,
}

impl MemoryLayer {
    /// Returns the capacity for this layer, or None for unlimited.
    #[must_use]
    pub fn capacity(&self) -> Option<usize> {
        match self {
            MemoryLayer::Immediate => Some(IMMEDIATE_CAPACITY),
            MemoryLayer::ShortTerm => Some(SHORT_TERM_CAPACITY),
            MemoryLayer::LongTerm => Some(LONG_TERM_CAPACITY),
            MemoryLayer::Legacy => None,
        }
    }
}

impl MemoryLayers {
    /// Creates a new empty MemoryLayers.
    #[must_use]
    pub fn new() -> Self {
        MemoryLayers {
            immediate: Vec::new(),
            short_term: Vec::new(),
            long_term: Vec::new(),
            legacy: Vec::new(),
        }
    }

    /// Returns the number of memories in the immediate layer.
    #[must_use]
    pub fn immediate_count(&self) -> usize {
        self.immediate.len()
    }

    /// Returns the number of memories in the short-term layer.
    #[must_use]
    pub fn short_term_count(&self) -> usize {
        self.short_term.len()
    }

    /// Returns the number of memories in the long-term layer.
    #[must_use]
    pub fn long_term_count(&self) -> usize {
        self.long_term.len()
    }

    /// Returns the number of memories in the legacy layer.
    #[must_use]
    pub fn legacy_count(&self) -> usize {
        self.legacy.len()
    }

    /// Returns the total number of memories across all layers.
    #[must_use]
    pub fn total_count(&self) -> usize {
        self.immediate.len() + self.short_term.len() + self.long_term.len() + self.legacy.len()
    }

    /// Returns true if all layers are empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.total_count() == 0
    }

    /// Adds a memory to the specified layer.
    ///
    /// If the layer is at capacity, evicts the lowest-salience entry first.
    /// Legacy layer has unlimited capacity.
    pub fn add(&mut self, layer: MemoryLayer, entry: MemoryEntry) {
        let (vec, capacity) = match layer {
            MemoryLayer::Immediate => (&mut self.immediate, Some(IMMEDIATE_CAPACITY)),
            MemoryLayer::ShortTerm => (&mut self.short_term, Some(SHORT_TERM_CAPACITY)),
            MemoryLayer::LongTerm => (&mut self.long_term, Some(LONG_TERM_CAPACITY)),
            MemoryLayer::Legacy => (&mut self.legacy, None),
        };

        // Evict if at capacity
        if let Some(cap) = capacity {
            if vec.len() >= cap {
                Self::evict_lowest_salience(vec);
            }
        }

        vec.push(entry);
    }

    /// Evicts the lowest-salience entry from the given vector.
    /// On salience tie, removes the oldest (lowest timestamp).
    fn evict_lowest_salience(vec: &mut Vec<MemoryEntry>) {
        if vec.is_empty() {
            return;
        }

        let mut min_idx = 0;
        let mut min_salience = vec[0].salience();
        let mut min_timestamp = vec[0].timestamp();

        for (i, entry) in vec.iter().enumerate().skip(1) {
            let salience = entry.salience();
            let timestamp = entry.timestamp();

            if salience < min_salience || (salience == min_salience && timestamp < min_timestamp) {
                min_idx = i;
                min_salience = salience;
                min_timestamp = timestamp;
            }
        }

        vec.remove(min_idx);
    }

    /// Returns a reference to the immediate layer.
    #[must_use]
    pub fn immediate(&self) -> &[MemoryEntry] {
        &self.immediate
    }

    /// Returns a reference to the short-term layer.
    #[must_use]
    pub fn short_term(&self) -> &[MemoryEntry] {
        &self.short_term
    }

    /// Returns a reference to the long-term layer.
    #[must_use]
    pub fn long_term(&self) -> &[MemoryEntry] {
        &self.long_term
    }

    /// Returns a reference to the legacy layer.
    #[must_use]
    pub fn legacy(&self) -> &[MemoryEntry] {
        &self.legacy
    }

    /// Returns an iterator over all memories in all layers.
    pub fn all_memories(&self) -> impl Iterator<Item = &MemoryEntry> {
        self.immediate
            .iter()
            .chain(self.short_term.iter())
            .chain(self.long_term.iter())
            .chain(self.legacy.iter())
    }

    // Retrieval methods

    /// Retrieves memories with salience >= threshold.
    ///
    /// Returns memories ordered by salience (highest first).
    #[must_use]
    pub fn retrieve_by_salience(&self, threshold: f32) -> Vec<&MemoryEntry> {
        let mut results: Vec<_> = self
            .all_memories()
            .filter(|m| m.salience() >= threshold)
            .collect();

        // Sort by salience descending
        results.sort_by(|a, b| {
            b.salience()
                .partial_cmp(&a.salience())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        results
    }

    /// Retrieves memories congruent with the given mood.
    ///
    /// Uses weighted PAD congruence formula (0.60/0.25/0.15).
    /// Returns memories where congruence >= min_congruence, ordered by salience.
    ///
    /// Note: `min_congruence >= 1.0` returns empty because perfect float match
    /// is practically impossible.
    ///
    /// # Arguments
    ///
    /// * `mood` - The current mood to compare against
    /// * `min_congruence` - Minimum congruence threshold (0.0-1.0)
    #[must_use]
    pub fn retrieve_mood_congruent(&self, mood: &Mood, min_congruence: f32) -> Vec<&MemoryEntry> {
        // Perfect match (1.0) is practically impossible with floats
        if min_congruence >= 1.0 {
            return Vec::new();
        }

        let mut results: Vec<_> = self
            .all_memories()
            .filter(|m| m.emotional_snapshot().compute_congruence_with_mood(mood) >= min_congruence)
            .collect();

        // Sort by salience descending
        results.sort_by(|a, b| {
            b.salience()
                .partial_cmp(&a.salience())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        results
    }

    /// Retrieves memories with the specified tag.
    ///
    /// Returns memories ordered by salience (highest first).
    #[must_use]
    pub fn retrieve_by_tag(&self, tag: MemoryTag) -> Vec<&MemoryEntry> {
        let mut results: Vec<_> = self.all_memories().filter(|m| m.has_tag(tag)).collect();

        // Sort by salience descending
        results.sort_by(|a, b| {
            b.salience()
                .partial_cmp(&a.salience())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        results
    }

    /// Retrieves memories involving the specified participant.
    ///
    /// Returns memories ordered by salience (highest first).
    #[must_use]
    pub fn retrieve_by_participant(&self, entity_id: &EntityId) -> Vec<&MemoryEntry> {
        let mut results: Vec<_> = self
            .all_memories()
            .filter(|m| m.involves_participant(entity_id))
            .collect();

        // Sort by salience descending
        results.sort_by(|a, b| {
            b.salience()
                .partial_cmp(&a.salience())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        results
    }

    /// Retrieves memories formed in the specified microsystem context.
    ///
    /// Returns memories ordered by salience (highest first).
    #[must_use]
    pub fn retrieve_by_context(&self, context: &MicrosystemId) -> Vec<&MemoryEntry> {
        let mut results: Vec<_> = self
            .all_memories()
            .filter(|m| m.is_in_context(context))
            .collect();

        // Sort by salience descending
        results.sort_by(|a, b| {
            b.salience()
                .partial_cmp(&a.salience())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        results
    }

    /// Retrieves memories with full scoring using Phase 8 formula.
    ///
    /// Uses a weighted combination of:
    /// - Tag relevance (0.25)
    /// - Participant match (0.20)
    /// - Salience (0.15)
    /// - Recency (0.10)
    /// - Mood congruence (0.10)
    /// - Context congruence (0.10)
    /// - Source confidence (0.05)
    /// - Base score (0.05)
    ///
    /// # Arguments
    ///
    /// * `query` - The retrieval query with match criteria
    ///
    /// # Returns
    ///
    /// A vector of (MemoryEntry reference, score) tuples, sorted by score descending,
    /// limited to `query.limit` results.
    #[must_use]
    pub fn retrieve_scored<'a>(&'a self, query: &RetrievalQuery) -> Vec<(&'a MemoryEntry, f64)> {
        let mut scored: Vec<_> = self
            .all_memories()
            .map(|entry| {
                let score = compute_retrieval_score(entry, query);
                (entry, score)
            })
            .collect();

        // Sort by score descending
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Apply limit
        scored.truncate(query.limit);
        scored
    }

    /// Applies salience decay to all memories across all layers except Legacy.
    ///
    /// Memory salience decays over time, scaled by species. This method
    /// applies the decay formula to every memory in Immediate, Short-term,
    /// and Long-term layers. Legacy memories are immutable and never decay.
    ///
    /// # Arguments
    ///
    /// * `duration` - The elapsed time
    /// * `time_scale` - Species-based time scaling factor (Human: 1.0, Dog: 6.7)
    /// * `half_life_days` - Days for salience to decay by half
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::memory::{MemoryLayers, MemoryLayer, MemoryEntry};
    /// use behavioral_pathways::types::Duration;
    ///
    /// let mut layers = MemoryLayers::new();
    /// layers.add(
    ///     MemoryLayer::Immediate,
    ///     MemoryEntry::new(Duration::days(0), "Test").with_salience(0.8)
    /// );
    ///
    /// // Decay all memories (human scale, 30-day half-life)
    /// layers.apply_salience_decay_all(Duration::days(30), 1.0, 30.0);
    ///
    /// assert!((layers.immediate()[0].salience() - 0.4).abs() < 0.01);
    /// ```
    pub fn apply_salience_decay_all(
        &mut self,
        duration: Duration,
        time_scale: f32,
        half_life_days: f32,
    ) {
        for entry in &mut self.immediate {
            entry.apply_salience_decay(duration, time_scale, half_life_days);
        }
        for entry in &mut self.short_term {
            entry.apply_salience_decay(duration, time_scale, half_life_days);
        }
        for entry in &mut self.long_term {
            entry.apply_salience_decay(duration, time_scale, half_life_days);
        }
        // Legacy memories are immutable and never decay
    }

    /// Finds which layer contains a memory by ID.
    ///
    /// Searches all layers for a memory with the given ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The memory ID to search for
    ///
    /// # Returns
    ///
    /// The layer containing the memory, or None if not found.
    #[must_use]
    pub fn find_layer(&self, id: &MemoryId) -> Option<MemoryLayer> {
        if self.immediate.iter().any(|m| m.id() == id) {
            return Some(MemoryLayer::Immediate);
        }
        if self.short_term.iter().any(|m| m.id() == id) {
            return Some(MemoryLayer::ShortTerm);
        }
        if self.long_term.iter().any(|m| m.id() == id) {
            return Some(MemoryLayer::LongTerm);
        }
        if self.legacy.iter().any(|m| m.id() == id) {
            return Some(MemoryLayer::Legacy);
        }
        None
    }

    /// Gets a reference to a memory by ID, searching all layers.
    ///
    /// # Arguments
    ///
    /// * `id` - The memory ID to search for
    ///
    /// # Returns
    ///
    /// A reference to the memory, or None if not found.
    #[must_use]
    pub fn get_by_id(&self, id: &MemoryId) -> Option<&MemoryEntry> {
        self.immediate
            .iter()
            .find(|m| m.id() == id)
            .or_else(|| self.short_term.iter().find(|m| m.id() == id))
            .or_else(|| self.long_term.iter().find(|m| m.id() == id))
            .or_else(|| self.legacy.iter().find(|m| m.id() == id))
    }

    /// Gets a mutable reference to a memory by ID, searching all layers.
    ///
    /// # Arguments
    ///
    /// * `id` - The memory ID to search for
    ///
    /// # Returns
    ///
    /// A mutable reference to the memory, or None if not found.
    pub fn get_by_id_mut(&mut self, id: &MemoryId) -> Option<&mut MemoryEntry> {
        if let Some(entry) = self.immediate.iter_mut().find(|m| m.id() == id) {
            return Some(entry);
        }
        if let Some(entry) = self.short_term.iter_mut().find(|m| m.id() == id) {
            return Some(entry);
        }
        if let Some(entry) = self.long_term.iter_mut().find(|m| m.id() == id) {
            return Some(entry);
        }
        if let Some(entry) = self.legacy.iter_mut().find(|m| m.id() == id) {
            return Some(entry);
        }
        None
    }

    /// Removes a memory by ID, returning it if found.
    ///
    /// Searches all layers for the memory and removes it from its layer.
    ///
    /// # Arguments
    ///
    /// * `id` - The memory ID to remove
    ///
    /// # Returns
    ///
    /// The removed memory, or None if not found.
    pub fn remove_by_id(&mut self, id: &MemoryId) -> Option<MemoryEntry> {
        if let Some(pos) = self.immediate.iter().position(|m| m.id() == id) {
            return Some(self.immediate.remove(pos));
        }
        if let Some(pos) = self.short_term.iter().position(|m| m.id() == id) {
            return Some(self.short_term.remove(pos));
        }
        if let Some(pos) = self.long_term.iter().position(|m| m.id() == id) {
            return Some(self.long_term.remove(pos));
        }
        if let Some(pos) = self.legacy.iter().position(|m| m.id() == id) {
            return Some(self.legacy.remove(pos));
        }
        None
    }

    /// Moves a memory from its current layer to a target layer.
    ///
    /// This is used by memory promotion. The memory is removed from its
    /// current layer and added to the target layer.
    ///
    /// # Arguments
    ///
    /// * `id` - The memory ID to move
    /// * `to` - The target layer
    ///
    /// # Returns
    ///
    /// Ok(()) on success, or an error if the memory is not found.
    pub fn move_to_layer(
        &mut self,
        id: &MemoryId,
        to: MemoryLayer,
    ) -> Result<(), crate::memory::maintenance::MaintenanceError> {
        let entry = self.remove_by_id(id).ok_or_else(|| {
            crate::memory::maintenance::MaintenanceError::MemoryNotFound { id: id.clone() }
        })?;

        self.add(to, entry);
        Ok(())
    }
}

impl Default for MemoryLayers {
    fn default() -> Self {
        MemoryLayers::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::EmotionalSnapshot;
    use crate::types::Duration;

    fn create_entry(days: u64, salience: f32) -> MemoryEntry {
        MemoryEntry::new(Duration::days(days), format!("Memory at day {days}"))
            .with_salience(salience)
    }

    #[test]
    fn evict_lowest_salience_on_empty_vec_does_nothing() {
        let mut empty_vec: Vec<MemoryEntry> = Vec::new();
        MemoryLayers::evict_lowest_salience(&mut empty_vec);
        assert!(empty_vec.is_empty());
    }

    #[test]
    fn layer_capacity_immediate_is_10() {
        assert_eq!(IMMEDIATE_CAPACITY, 10);
        assert_eq!(MemoryLayer::Immediate.capacity(), Some(10));
    }

    #[test]
    fn layer_capacity_short_term_is_20() {
        assert_eq!(SHORT_TERM_CAPACITY, 20);
        assert_eq!(MemoryLayer::ShortTerm.capacity(), Some(20));
    }

    #[test]
    fn layer_capacity_long_term_is_50() {
        assert_eq!(LONG_TERM_CAPACITY, 50);
        assert_eq!(MemoryLayer::LongTerm.capacity(), Some(50));
    }

    #[test]
    fn layer_capacity_legacy_unlimited() {
        assert_eq!(MemoryLayer::Legacy.capacity(), None);
    }

    #[test]
    fn new_layers_are_empty() {
        let layers = MemoryLayers::new();
        assert!(layers.is_empty());
        assert_eq!(layers.total_count(), 0);
        assert_eq!(layers.immediate_count(), 0);
        assert_eq!(layers.short_term_count(), 0);
        assert_eq!(layers.long_term_count(), 0);
        assert_eq!(layers.legacy_count(), 0);
    }

    #[test]
    fn add_to_layers() {
        let mut layers = MemoryLayers::new();

        layers.add(MemoryLayer::Immediate, create_entry(1, 0.5));
        assert_eq!(layers.immediate_count(), 1);

        layers.add(MemoryLayer::ShortTerm, create_entry(2, 0.5));
        assert_eq!(layers.short_term_count(), 1);

        layers.add(MemoryLayer::LongTerm, create_entry(3, 0.5));
        assert_eq!(layers.long_term_count(), 1);

        layers.add(MemoryLayer::Legacy, create_entry(4, 0.5));
        assert_eq!(layers.legacy_count(), 1);

        assert_eq!(layers.total_count(), 4);
    }

    #[test]
    fn layer_eviction_removes_lowest_salience() {
        let mut layers = MemoryLayers::new();

        // Fill immediate layer with 10 entries, salience increasing
        for i in 0..10 {
            layers.add(MemoryLayer::Immediate, create_entry(i, (i as f32) / 10.0));
        }
        assert_eq!(layers.immediate_count(), 10);

        // Add one more - should evict the lowest salience (day 0, salience 0.0)
        layers.add(MemoryLayer::Immediate, create_entry(100, 0.5));
        assert_eq!(layers.immediate_count(), 10);

        // Verify day 0 was evicted
        let has_day_0 = layers
            .immediate()
            .iter()
            .any(|m| m.timestamp().as_days() == 0);
        assert!(!has_day_0);

        // Verify day 100 is present
        let has_day_100 = layers
            .immediate()
            .iter()
            .any(|m| m.timestamp().as_days() == 100);
        assert!(has_day_100);
    }

    #[test]
    fn layer_eviction_finds_lowest_in_middle() {
        let mut layers = MemoryLayers::new();

        // Add 10 entries where the lowest salience is in the middle (not at start)
        // Index 0-4: high salience (0.8)
        // Index 5: low salience (0.1) - should be evicted
        // Index 6-9: high salience (0.8)
        for i in 0..5 {
            layers.add(MemoryLayer::Immediate, create_entry(i, 0.8));
        }
        layers.add(MemoryLayer::Immediate, create_entry(5, 0.1)); // Lowest salience
        for i in 6..10 {
            layers.add(MemoryLayer::Immediate, create_entry(i, 0.8));
        }
        assert_eq!(layers.immediate_count(), 10);

        // Add one more - should evict the lowest salience (day 5, salience 0.1)
        layers.add(MemoryLayer::Immediate, create_entry(100, 0.5));
        assert_eq!(layers.immediate_count(), 10);

        // Verify day 5 (lowest salience at index 5) was evicted
        let has_day_5 = layers
            .immediate()
            .iter()
            .any(|m| m.timestamp().as_days() == 5);
        assert!(!has_day_5);

        // Verify day 100 is present
        let has_day_100 = layers
            .immediate()
            .iter()
            .any(|m| m.timestamp().as_days() == 100);
        assert!(has_day_100);
    }

    #[test]
    fn layer_eviction_timestamp_tiebreaker_in_middle() {
        let mut layers = MemoryLayers::new();

        // Add entries where oldest (lowest timestamp) is at index 5, not index 0
        // All have same salience, so timestamp decides
        // Newer entries first (high timestamps), then old one, then newer again
        for i in 0..5 {
            // Days 100-104 (newer) - indices 0-4
            layers.add(MemoryLayer::Immediate, create_entry(100 + i, 0.5));
        }
        // Day 1 (oldest) - index 5, should be evicted
        layers.add(MemoryLayer::Immediate, create_entry(1, 0.5));
        for i in 0..4 {
            // Days 200-203 (newest) - indices 6-9
            layers.add(MemoryLayer::Immediate, create_entry(200 + i, 0.5));
        }
        assert_eq!(layers.immediate_count(), 10);

        // Add one more - should evict the oldest (day 1 at index 5)
        layers.add(MemoryLayer::Immediate, create_entry(300, 0.5));
        assert_eq!(layers.immediate_count(), 10);

        // Verify day 1 (oldest, in middle) was evicted
        let has_day_1 = layers
            .immediate()
            .iter()
            .any(|m| m.timestamp().as_days() == 1);
        assert!(!has_day_1);

        // Verify day 300 is present
        let has_day_300 = layers
            .immediate()
            .iter()
            .any(|m| m.timestamp().as_days() == 300);
        assert!(has_day_300);
    }

    #[test]
    fn layer_eviction_oldest_on_tie() {
        let mut layers = MemoryLayers::new();

        // Fill immediate layer with 10 entries, all same salience, different timestamps
        for i in 0..10 {
            layers.add(MemoryLayer::Immediate, create_entry(i, 0.5));
        }
        assert_eq!(layers.immediate_count(), 10);

        // Add one more with same salience - should evict oldest (day 0)
        layers.add(MemoryLayer::Immediate, create_entry(100, 0.5));
        assert_eq!(layers.immediate_count(), 10);

        // Verify day 0 (oldest) was evicted
        let has_day_0 = layers
            .immediate()
            .iter()
            .any(|m| m.timestamp().as_days() == 0);
        assert!(!has_day_0);

        // Verify day 100 is present
        let has_day_100 = layers
            .immediate()
            .iter()
            .any(|m| m.timestamp().as_days() == 100);
        assert!(has_day_100);
    }

    #[test]
    fn eviction_does_not_remove_newly_added_memory() {
        let mut layers = MemoryLayers::new();

        // Fill immediate layer with 10 high-salience entries
        for i in 0..10 {
            layers.add(MemoryLayer::Immediate, create_entry(i, 0.9));
        }

        // Add a low-salience entry - should not be immediately evicted
        // because we add after eviction
        layers.add(MemoryLayer::Immediate, create_entry(100, 0.1));
        assert_eq!(layers.immediate_count(), 10);

        // The new entry should still be there (lowest was evicted, not the new one)
        let has_day_100 = layers
            .immediate()
            .iter()
            .any(|m| m.timestamp().as_days() == 100);
        assert!(has_day_100);
    }

    #[test]
    fn legacy_no_eviction() {
        let mut layers = MemoryLayers::new();

        // Add 100 entries to legacy - should all stay
        for i in 0..100 {
            layers.add(MemoryLayer::Legacy, create_entry(i, 0.5));
        }
        assert_eq!(layers.legacy_count(), 100);
    }

    #[test]
    fn retrieve_by_salience_threshold() {
        let mut layers = MemoryLayers::new();

        layers.add(MemoryLayer::Immediate, create_entry(1, 0.2));
        layers.add(MemoryLayer::Immediate, create_entry(2, 0.5));
        layers.add(MemoryLayer::Immediate, create_entry(3, 0.8));

        let results = layers.retrieve_by_salience(0.5);
        assert_eq!(results.len(), 2);
        // Should be ordered by salience descending
        assert!((results[0].salience() - 0.8).abs() < f32::EPSILON);
        assert!((results[1].salience() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn retrieve_mood_congruent_within_tolerance() {
        let mut layers = MemoryLayers::new();

        // Add memory with specific emotional snapshot
        let entry = MemoryEntry::new(Duration::days(1), "Test")
            .with_emotional_snapshot(EmotionalSnapshot::new(0.5, 0.3, -0.2))
            .with_salience(0.7);
        layers.add(MemoryLayer::Immediate, entry);

        // Create matching mood
        let mood = Mood::new()
            .with_valence_base(0.5)
            .with_arousal_base(0.3)
            .with_dominance_base(-0.2);

        let results = layers.retrieve_mood_congruent(&mood, 0.9);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn retrieve_mood_congruent_uses_pad_weights_60_25_15() {
        let mut layers = MemoryLayers::new();

        // Memory at one extreme
        let entry = MemoryEntry::new(Duration::days(1), "Test")
            .with_emotional_snapshot(EmotionalSnapshot::new(1.0, 1.0, 1.0))
            .with_salience(0.7);
        layers.add(MemoryLayer::Immediate, entry);

        // Mood with only valence different (-1.0 vs 1.0 = max diff)
        // Expected congruence = 0.0 * 0.60 + 1.0 * 0.25 + 1.0 * 0.15 = 0.40
        let mood = Mood::new()
            .with_valence_base(-1.0)
            .with_arousal_base(1.0)
            .with_dominance_base(1.0);

        // Should pass at 0.4, fail at 0.5
        let results_pass = layers.retrieve_mood_congruent(&mood, 0.4);
        assert_eq!(results_pass.len(), 1);

        let results_fail = layers.retrieve_mood_congruent(&mood, 0.5);
        assert_eq!(results_fail.len(), 0);
    }

    #[test]
    fn retrieve_mood_congruent_returns_empty_vec_on_no_match() {
        let mut layers = MemoryLayers::new();

        // Memory at one extreme
        let entry = MemoryEntry::new(Duration::days(1), "Test")
            .with_emotional_snapshot(EmotionalSnapshot::new(1.0, 1.0, 1.0))
            .with_salience(0.7);
        layers.add(MemoryLayer::Immediate, entry);

        // Mood at opposite extreme
        let mood = Mood::new()
            .with_valence_base(-1.0)
            .with_arousal_base(-1.0)
            .with_dominance_base(-1.0);

        // With high threshold, no match
        let results = layers.retrieve_mood_congruent(&mood, 0.5);
        assert!(results.is_empty());
    }

    #[test]
    fn retrieve_mood_congruent_ordered_by_salience() {
        let mut layers = MemoryLayers::new();

        let snapshot = EmotionalSnapshot::neutral();

        layers.add(
            MemoryLayer::Immediate,
            MemoryEntry::new(Duration::days(1), "Low")
                .with_emotional_snapshot(snapshot)
                .with_salience(0.3),
        );
        layers.add(
            MemoryLayer::Immediate,
            MemoryEntry::new(Duration::days(2), "High")
                .with_emotional_snapshot(snapshot)
                .with_salience(0.9),
        );
        layers.add(
            MemoryLayer::Immediate,
            MemoryEntry::new(Duration::days(3), "Mid")
                .with_emotional_snapshot(snapshot)
                .with_salience(0.6),
        );

        let mood = Mood::new(); // Neutral mood matches neutral snapshots perfectly
        let results = layers.retrieve_mood_congruent(&mood, 0.5);

        assert_eq!(results.len(), 3);
        assert!((results[0].salience() - 0.9).abs() < f32::EPSILON);
        assert!((results[1].salience() - 0.6).abs() < f32::EPSILON);
        assert!((results[2].salience() - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn retrieve_mood_congruent_min_congruence_zero_returns_all() {
        let mut layers = MemoryLayers::new();

        // Add memories with different emotional states
        layers.add(
            MemoryLayer::Immediate,
            MemoryEntry::new(Duration::days(1), "Positive")
                .with_emotional_snapshot(EmotionalSnapshot::new(1.0, 1.0, 1.0))
                .with_salience(0.5),
        );
        layers.add(
            MemoryLayer::Immediate,
            MemoryEntry::new(Duration::days(2), "Negative")
                .with_emotional_snapshot(EmotionalSnapshot::new(-1.0, -1.0, -1.0))
                .with_salience(0.5),
        );

        let mood = Mood::new(); // Neutral
        let results = layers.retrieve_mood_congruent(&mood, 0.0);

        // All memories should be returned since min_congruence is 0
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn retrieve_mood_congruent_min_congruence_one_returns_empty() {
        let mut layers = MemoryLayers::new();

        // Add a memory with neutral emotional snapshot (would be "perfect match")
        layers.add(
            MemoryLayer::Immediate,
            MemoryEntry::new(Duration::days(1), "Perfect match")
                .with_emotional_snapshot(EmotionalSnapshot::neutral())
                .with_salience(0.5),
        );

        // Add a memory with slightly different values
        layers.add(
            MemoryLayer::Immediate,
            MemoryEntry::new(Duration::days(2), "Not perfect")
                .with_emotional_snapshot(EmotionalSnapshot::new(0.1, 0.0, 0.0))
                .with_salience(0.5),
        );

        // min_congruence = 1.0 should return empty (perfect float match is impossible)
        let mood = Mood::new();
        let results = layers.retrieve_mood_congruent(&mood, 1.0);
        assert!(results.is_empty());

        // Above 1.0 should also return nothing
        let results_above = layers.retrieve_mood_congruent(&mood, 1.001);
        assert!(results_above.is_empty());

        // Just below 1.0 should return the perfect match
        let results_below = layers.retrieve_mood_congruent(&mood, 0.999);
        assert_eq!(results_below.len(), 1);
        assert_eq!(results_below[0].summary(), "Perfect match");
    }

    #[test]
    fn retrieve_by_tag_single() {
        let mut layers = MemoryLayers::new();

        layers.add(
            MemoryLayer::Immediate,
            MemoryEntry::new(Duration::days(1), "Personal").add_tag(MemoryTag::Personal),
        );
        layers.add(
            MemoryLayer::Immediate,
            MemoryEntry::new(Duration::days(2), "Mission").add_tag(MemoryTag::Mission),
        );

        let results = layers.retrieve_by_tag(MemoryTag::Personal);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].summary(), "Personal");
    }

    #[test]
    fn retrieve_by_tag_multiple() {
        let mut layers = MemoryLayers::new();

        layers.add(
            MemoryLayer::Immediate,
            MemoryEntry::new(Duration::days(1), "Personal 1")
                .add_tag(MemoryTag::Personal)
                .with_salience(0.5),
        );
        layers.add(
            MemoryLayer::Immediate,
            MemoryEntry::new(Duration::days(2), "Personal 2")
                .add_tag(MemoryTag::Personal)
                .with_salience(0.8),
        );

        let results = layers.retrieve_by_tag(MemoryTag::Personal);
        assert_eq!(results.len(), 2);
        // Should be ordered by salience
        assert!((results[0].salience() - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn retrieve_by_participant() {
        let mut layers = MemoryLayers::new();

        let p1 = EntityId::new("entity_001").unwrap();
        let p2 = EntityId::new("entity_002").unwrap();

        layers.add(
            MemoryLayer::Immediate,
            MemoryEntry::new(Duration::days(1), "With p1").add_participant(p1.clone()),
        );
        layers.add(
            MemoryLayer::Immediate,
            MemoryEntry::new(Duration::days(2), "With p2").add_participant(p2),
        );

        let results = layers.retrieve_by_participant(&p1);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].summary(), "With p1");
    }

    #[test]
    fn retrieve_by_participant_multiple_sorted_by_salience() {
        let mut layers = MemoryLayers::new();

        let p1 = EntityId::new("entity_001").unwrap();

        layers.add(
            MemoryLayer::Immediate,
            MemoryEntry::new(Duration::days(1), "Low salience")
                .add_participant(p1.clone())
                .with_salience(0.3),
        );
        layers.add(
            MemoryLayer::Immediate,
            MemoryEntry::new(Duration::days(2), "High salience")
                .add_participant(p1.clone())
                .with_salience(0.9),
        );

        let results = layers.retrieve_by_participant(&p1);
        assert_eq!(results.len(), 2);
        // Should be sorted by salience descending
        assert!((results[0].salience() - 0.9).abs() < f32::EPSILON);
        assert!((results[1].salience() - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn retrieve_by_context_returns_matching_memories() {
        let mut layers = MemoryLayers::new();

        let work = MicrosystemId::new("work_001").unwrap();
        let home = MicrosystemId::new("home_001").unwrap();

        layers.add(
            MemoryLayer::Immediate,
            MemoryEntry::new(Duration::days(1), "At work").with_microsystem_context(work.clone()),
        );
        layers.add(
            MemoryLayer::Immediate,
            MemoryEntry::new(Duration::days(2), "At home").with_microsystem_context(home),
        );

        let results = layers.retrieve_by_context(&work);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].summary(), "At work");
    }

    #[test]
    fn retrieve_by_context_excludes_other_contexts() {
        let mut layers = MemoryLayers::new();

        let work = MicrosystemId::new("work_001").unwrap();
        let school = MicrosystemId::new("school_001").unwrap();

        layers.add(
            MemoryLayer::Immediate,
            MemoryEntry::new(Duration::days(1), "At work").with_microsystem_context(work.clone()),
        );
        layers.add(
            MemoryLayer::Immediate,
            MemoryEntry::new(Duration::days(2), "No context"),
        );

        let results = layers.retrieve_by_context(&school);
        assert!(results.is_empty());
    }

    #[test]
    fn retrieve_by_context_multiple_sorted_by_salience() {
        let mut layers = MemoryLayers::new();

        let work = MicrosystemId::new("work_001").unwrap();

        layers.add(
            MemoryLayer::Immediate,
            MemoryEntry::new(Duration::days(1), "Low salience work")
                .with_microsystem_context(work.clone())
                .with_salience(0.2),
        );
        layers.add(
            MemoryLayer::Immediate,
            MemoryEntry::new(Duration::days(2), "High salience work")
                .with_microsystem_context(work.clone())
                .with_salience(0.8),
        );

        let results = layers.retrieve_by_context(&work);
        assert_eq!(results.len(), 2);
        // Should be sorted by salience descending
        assert!((results[0].salience() - 0.8).abs() < f32::EPSILON);
        assert!((results[1].salience() - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn retrieve_across_all_layers() {
        let mut layers = MemoryLayers::new();

        layers.add(
            MemoryLayer::Immediate,
            MemoryEntry::new(Duration::days(1), "Immediate").add_tag(MemoryTag::Personal),
        );
        layers.add(
            MemoryLayer::ShortTerm,
            MemoryEntry::new(Duration::days(2), "Short-term").add_tag(MemoryTag::Personal),
        );
        layers.add(
            MemoryLayer::LongTerm,
            MemoryEntry::new(Duration::days(3), "Long-term").add_tag(MemoryTag::Personal),
        );
        layers.add(
            MemoryLayer::Legacy,
            MemoryEntry::new(Duration::days(4), "Legacy").add_tag(MemoryTag::Personal),
        );

        let results = layers.retrieve_by_tag(MemoryTag::Personal);
        assert_eq!(results.len(), 4);
    }

    #[test]
    fn layer_access() {
        let mut layers = MemoryLayers::new();

        layers.add(MemoryLayer::Immediate, create_entry(1, 0.5));
        layers.add(MemoryLayer::ShortTerm, create_entry(2, 0.5));
        layers.add(MemoryLayer::LongTerm, create_entry(3, 0.5));
        layers.add(MemoryLayer::Legacy, create_entry(4, 0.5));

        assert_eq!(layers.immediate().len(), 1);
        assert_eq!(layers.short_term().len(), 1);
        assert_eq!(layers.long_term().len(), 1);
        assert_eq!(layers.legacy().len(), 1);
    }

    #[test]
    fn default_is_empty() {
        let layers = MemoryLayers::default();
        assert!(layers.is_empty());
    }

    #[test]
    fn clone() {
        let mut layers = MemoryLayers::new();
        layers.add(MemoryLayer::Immediate, create_entry(1, 0.5));

        let cloned = layers.clone();
        assert_eq!(layers, cloned);
    }

    #[test]
    fn debug_format() {
        let layers = MemoryLayers::new();
        let debug = format!("{:?}", layers);
        assert!(debug.contains("MemoryLayers"));
    }

    #[test]
    fn apply_salience_decay_all_affects_non_legacy_layers() {
        let mut layers = MemoryLayers::new();

        // Add entries to all layers with same salience
        layers.add(MemoryLayer::Immediate, create_entry(0, 0.8));
        layers.add(MemoryLayer::ShortTerm, create_entry(0, 0.8));
        layers.add(MemoryLayer::LongTerm, create_entry(0, 0.8));
        layers.add(MemoryLayer::Legacy, create_entry(0, 0.8));

        // Apply decay (one half-life at human scale)
        layers.apply_salience_decay_all(Duration::days(30), 1.0, 30.0);

        // Immediate, Short-term, Long-term should be at ~0.4
        assert!((layers.immediate()[0].salience() - 0.4).abs() < 0.01);
        assert!((layers.short_term()[0].salience() - 0.4).abs() < 0.01);
        assert!((layers.long_term()[0].salience() - 0.4).abs() < 0.01);

        // Legacy should NOT decay - still at 0.8
        assert!((layers.legacy()[0].salience() - 0.8).abs() < 0.01);
    }

    #[test]
    fn apply_salience_decay_all_respects_time_scale() {
        // Human memories
        let mut human_layers = MemoryLayers::new();
        human_layers.add(MemoryLayer::Immediate, create_entry(0, 0.8));

        // Dog memories
        let mut dog_layers = MemoryLayers::new();
        dog_layers.add(MemoryLayer::Immediate, create_entry(0, 0.8));

        // Same real time, different time scales
        human_layers.apply_salience_decay_all(Duration::days(30), 1.0, 30.0);
        dog_layers.apply_salience_decay_all(Duration::days(30), 6.7, 30.0);

        // Human: 1 half-life -> 0.4
        assert!((human_layers.immediate()[0].salience() - 0.4).abs() < 0.01);

        // Dog: 6.7 half-lives -> much lower
        assert!(dog_layers.immediate()[0].salience() < 0.02);
    }

    #[test]
    fn apply_salience_decay_all_on_empty_layers_does_nothing() {
        let mut layers = MemoryLayers::new();
        layers.apply_salience_decay_all(Duration::days(30), 1.0, 30.0);
        assert!(layers.is_empty());
    }

    #[test]
    fn retrieve_scored_returns_limited_results() {
        let mut layers = MemoryLayers::new();

        // Add more than limit
        for i in 0..10 {
            layers.add(MemoryLayer::Immediate, create_entry(i, 0.5));
        }

        let query = RetrievalQuery::new(Duration::days(100)).with_limit(5);
        let results = layers.retrieve_scored(&query);

        assert_eq!(results.len(), 5);
    }

    #[test]
    fn retrieve_scored_sorted_by_score() {
        let mut layers = MemoryLayers::new();

        // Add entries with different saliences
        layers.add(
            MemoryLayer::Immediate,
            MemoryEntry::new(Duration::days(1), "Low").with_salience(0.2),
        );
        layers.add(
            MemoryLayer::Immediate,
            MemoryEntry::new(Duration::days(1), "High").with_salience(0.9),
        );
        layers.add(
            MemoryLayer::Immediate,
            MemoryEntry::new(Duration::days(1), "Mid").with_salience(0.5),
        );

        let query = RetrievalQuery::new(Duration::days(1));
        let results = layers.retrieve_scored(&query);

        // Should be sorted by score descending
        // Higher salience = higher score (all else equal)
        assert!(results[0].1 >= results[1].1);
        assert!(results[1].1 >= results[2].1);
    }

    #[test]
    fn retrieve_scored_context_congruence_boosts_matching() {
        let mut layers = MemoryLayers::new();

        let work = MicrosystemId::new("work").unwrap();
        let home = MicrosystemId::new("home").unwrap();

        // Add work memory and home memory with same salience
        layers.add(
            MemoryLayer::Immediate,
            MemoryEntry::new(Duration::days(1), "At work")
                .with_microsystem_context(work.clone())
                .with_salience(0.5),
        );
        layers.add(
            MemoryLayer::Immediate,
            MemoryEntry::new(Duration::days(1), "At home")
                .with_microsystem_context(home)
                .with_salience(0.5),
        );

        // Query from work context
        let query = RetrievalQuery::new(Duration::days(1)).with_context(work);
        let results = layers.retrieve_scored(&query);

        // Work memory should score higher
        assert_eq!(results[0].0.summary(), "At work");
    }

    #[test]
    fn retrieve_scored_empty_layers_returns_empty() {
        let layers = MemoryLayers::new();
        let query = RetrievalQuery::new(Duration::days(100));
        let results = layers.retrieve_scored(&query);
        assert!(results.is_empty());
    }

    // Tests for helper methods added in Phase 12

    #[test]
    fn find_layer_returns_correct_layer() {
        let mut layers = MemoryLayers::new();

        let imm_entry = MemoryEntry::new(Duration::days(1), "Immediate");
        let imm_id = imm_entry.id().clone();
        layers.add(MemoryLayer::Immediate, imm_entry);

        let short_entry = MemoryEntry::new(Duration::days(1), "ShortTerm");
        let short_id = short_entry.id().clone();
        layers.add(MemoryLayer::ShortTerm, short_entry);

        let long_entry = MemoryEntry::new(Duration::days(1), "LongTerm");
        let long_id = long_entry.id().clone();
        layers.add(MemoryLayer::LongTerm, long_entry);

        let legacy_entry = MemoryEntry::new(Duration::days(1), "Legacy");
        let legacy_id = legacy_entry.id().clone();
        layers.add(MemoryLayer::Legacy, legacy_entry);

        assert_eq!(layers.find_layer(&imm_id), Some(MemoryLayer::Immediate));
        assert_eq!(layers.find_layer(&short_id), Some(MemoryLayer::ShortTerm));
        assert_eq!(layers.find_layer(&long_id), Some(MemoryLayer::LongTerm));
        assert_eq!(layers.find_layer(&legacy_id), Some(MemoryLayer::Legacy));
    }

    #[test]
    fn find_layer_returns_none_for_unknown_id() {
        let layers = MemoryLayers::new();
        let unknown_id = MemoryId::new("unknown_123").unwrap();
        assert_eq!(layers.find_layer(&unknown_id), None);
    }

    #[test]
    fn get_by_id_returns_correct_entry() {
        let mut layers = MemoryLayers::new();

        let entry = MemoryEntry::new(Duration::days(1), "Test entry").with_salience(0.7);
        let id = entry.id().clone();
        layers.add(MemoryLayer::ShortTerm, entry);

        let found = layers.get_by_id(&id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().summary(), "Test entry");
        assert!((found.unwrap().salience() - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn get_by_id_returns_none_for_unknown_id() {
        let layers = MemoryLayers::new();
        let unknown_id = MemoryId::new("unknown_456").unwrap();
        assert!(layers.get_by_id(&unknown_id).is_none());
    }

    #[test]
    fn get_by_id_mut_allows_mutation() {
        let mut layers = MemoryLayers::new();

        let entry = MemoryEntry::new(Duration::days(1), "Mutable").with_salience(0.5);
        let id = entry.id().clone();
        layers.add(MemoryLayer::Immediate, entry);

        // Mutate the entry - use unwrap since we know the entry exists
        layers.get_by_id_mut(&id).unwrap().set_salience(0.9);

        // Verify mutation
        let found = layers.get_by_id(&id).unwrap();
        assert!((found.salience() - 0.9).abs() < f32::EPSILON);
    }

    #[test]
    fn get_by_id_mut_returns_none_for_unknown_id() {
        let mut layers = MemoryLayers::new();
        let unknown_id = MemoryId::new("unknown_789").unwrap();
        assert!(layers.get_by_id_mut(&unknown_id).is_none());
    }

    #[test]
    fn remove_by_id_removes_and_returns_entry() {
        let mut layers = MemoryLayers::new();

        let entry = MemoryEntry::new(Duration::days(1), "To remove");
        let id = entry.id().clone();
        layers.add(MemoryLayer::LongTerm, entry);
        assert_eq!(layers.long_term_count(), 1);

        let removed = layers.remove_by_id(&id);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().summary(), "To remove");
        assert_eq!(layers.long_term_count(), 0);
    }

    #[test]
    fn remove_by_id_returns_none_for_unknown_id() {
        let mut layers = MemoryLayers::new();
        let unknown_id = MemoryId::new("unknown_abc").unwrap();
        assert!(layers.remove_by_id(&unknown_id).is_none());
    }

    #[test]
    fn move_to_layer_moves_memory_between_layers() {
        let mut layers = MemoryLayers::new();

        let entry = MemoryEntry::new(Duration::days(1), "Moving");
        let id = entry.id().clone();
        layers.add(MemoryLayer::Immediate, entry);
        assert_eq!(layers.immediate_count(), 1);
        assert_eq!(layers.short_term_count(), 0);

        let result = layers.move_to_layer(&id, MemoryLayer::ShortTerm);
        assert!(result.is_ok());
        assert_eq!(layers.immediate_count(), 0);
        assert_eq!(layers.short_term_count(), 1);

        // Verify the moved entry
        let found = layers.get_by_id(&id).unwrap();
        assert_eq!(found.summary(), "Moving");
    }

    #[test]
    fn move_to_layer_returns_error_for_unknown_id() {
        let mut layers = MemoryLayers::new();
        let unknown_id = MemoryId::new("unknown_def").unwrap();

        let result = layers.move_to_layer(&unknown_id, MemoryLayer::ShortTerm);
        assert!(result.is_err());
    }

    #[test]
    fn get_by_id_searches_all_layers() {
        let mut layers = MemoryLayers::new();

        // Add entries to different layers
        let imm = MemoryEntry::new(Duration::days(1), "Imm");
        let imm_id = imm.id().clone();
        layers.add(MemoryLayer::Immediate, imm);

        let short = MemoryEntry::new(Duration::days(2), "Short");
        let short_id = short.id().clone();
        layers.add(MemoryLayer::ShortTerm, short);

        let long = MemoryEntry::new(Duration::days(3), "Long");
        let long_id = long.id().clone();
        layers.add(MemoryLayer::LongTerm, long);

        let legacy = MemoryEntry::new(Duration::days(4), "Legacy");
        let legacy_id = legacy.id().clone();
        layers.add(MemoryLayer::Legacy, legacy);

        // Can find each entry
        assert_eq!(layers.get_by_id(&imm_id).unwrap().summary(), "Imm");
        assert_eq!(layers.get_by_id(&short_id).unwrap().summary(), "Short");
        assert_eq!(layers.get_by_id(&long_id).unwrap().summary(), "Long");
        assert_eq!(layers.get_by_id(&legacy_id).unwrap().summary(), "Legacy");
    }

    #[test]
    fn remove_by_id_removes_from_correct_layer() {
        let mut layers = MemoryLayers::new();

        // Add entries to all layers
        let imm = MemoryEntry::new(Duration::days(1), "Imm");
        layers.add(MemoryLayer::Immediate, imm);

        let short = MemoryEntry::new(Duration::days(2), "Short");
        let short_id = short.id().clone();
        layers.add(MemoryLayer::ShortTerm, short);

        let long = MemoryEntry::new(Duration::days(3), "Long");
        layers.add(MemoryLayer::LongTerm, long);

        let legacy = MemoryEntry::new(Duration::days(4), "Legacy");
        layers.add(MemoryLayer::Legacy, legacy);

        assert_eq!(layers.total_count(), 4);

        // Remove only the short-term entry
        layers.remove_by_id(&short_id);

        assert_eq!(layers.total_count(), 3);
        assert_eq!(layers.immediate_count(), 1);
        assert_eq!(layers.short_term_count(), 0);
        assert_eq!(layers.long_term_count(), 1);
        assert_eq!(layers.legacy_count(), 1);
    }

    #[test]
    fn get_by_id_mut_finds_entry_in_short_term() {
        let mut layers = MemoryLayers::new();

        let entry = MemoryEntry::new(Duration::days(1), "ShortTerm").with_salience(0.5);
        let id = entry.id().clone();
        layers.add(MemoryLayer::ShortTerm, entry);

        // Find and mutate - use unwrap since we know the entry exists
        layers.get_by_id_mut(&id).unwrap().set_salience(0.9);

        assert!((layers.short_term()[0].salience() - 0.9).abs() < f32::EPSILON);
    }

    #[test]
    fn get_by_id_mut_finds_entry_in_long_term() {
        let mut layers = MemoryLayers::new();

        let entry = MemoryEntry::new(Duration::days(1), "LongTerm").with_salience(0.5);
        let id = entry.id().clone();
        layers.add(MemoryLayer::LongTerm, entry);

        // Find and mutate - use unwrap since we know the entry exists
        layers.get_by_id_mut(&id).unwrap().set_salience(0.8);

        assert!((layers.long_term()[0].salience() - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn get_by_id_mut_finds_entry_in_legacy() {
        let mut layers = MemoryLayers::new();

        let entry = MemoryEntry::new(Duration::days(1), "Legacy").with_salience(0.5);
        let id = entry.id().clone();
        layers.add(MemoryLayer::Legacy, entry);

        // Find and mutate - use unwrap since we know the entry exists
        layers.get_by_id_mut(&id).unwrap().set_salience(0.95);

        assert!((layers.legacy()[0].salience() - 0.95).abs() < f32::EPSILON);
    }

    #[test]
    fn remove_by_id_removes_from_legacy() {
        let mut layers = MemoryLayers::new();

        let entry = MemoryEntry::new(Duration::days(1), "LegacyRemove");
        let id = entry.id().clone();
        layers.add(MemoryLayer::Legacy, entry);
        assert_eq!(layers.legacy_count(), 1);

        let removed = layers.remove_by_id(&id);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().summary(), "LegacyRemove");
        assert_eq!(layers.legacy_count(), 0);
    }

    #[test]
    fn remove_by_id_removes_from_long_term() {
        let mut layers = MemoryLayers::new();

        let entry = MemoryEntry::new(Duration::days(1), "LongTermRemove");
        let id = entry.id().clone();
        layers.add(MemoryLayer::LongTerm, entry);
        assert_eq!(layers.long_term_count(), 1);

        let removed = layers.remove_by_id(&id);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().summary(), "LongTermRemove");
        assert_eq!(layers.long_term_count(), 0);
    }
}
