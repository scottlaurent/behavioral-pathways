//! Memory entry structure representing a single memory.
//!
//! A memory entry captures an event or experience along with its emotional
//! context, participants, and effects on relationships.

use crate::memory::{DeltasApplied, EmotionalSnapshot, MemorySource, MemoryTag};
use crate::types::{Duration, EntityId, EventId, MemoryId, MicrosystemId};
use uuid::Uuid;

/// Generates a unique memory ID using UUID v4.
///
/// This matches the UUID pattern used for EntityId in Phase 1.
fn generate_memory_id() -> MemoryId {
    let uuid = Uuid::new_v4();
    // Safe to unwrap because UUIDs are never empty
    MemoryId::new(format!("mem_{uuid}")).unwrap()
}

/// A single memory entry representing a recorded experience.
///
/// Memory entries capture events along with their emotional context,
/// participants, source, and effects. They are stored in memory layers
/// and retrieved based on salience, mood congruence, tags, and context.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::memory::{MemoryEntry, MemorySource, MemoryTag, EmotionalSnapshot};
/// use behavioral_pathways::types::{Duration, EntityId};
///
/// let participant = EntityId::new("entity_001").unwrap();
/// let entry = MemoryEntry::new(
///     Duration::days(10),
///     "A significant conversation occurred.",
/// )
/// .with_participants(vec![participant])
/// .with_tags(vec![MemoryTag::Personal])
/// .with_source(MemorySource::Self_)
/// .with_salience(0.7);
///
/// assert_eq!(entry.summary(), "A significant conversation occurred.");
/// assert!((entry.salience() - 0.7).abs() < f32::EPSILON);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct MemoryEntry {
    /// Unique identifier for this memory.
    id: MemoryId,

    /// Optional reference to the originating event.
    event_id: Option<EventId>,

    /// Time since entity creation when the memory was formed.
    timestamp: Duration,

    /// Entities involved or who witnessed the event.
    participants: Vec<EntityId>,

    /// Categorization tags for retrieval.
    tags: Vec<MemoryTag>,

    /// How the entity learned about the event.
    source: MemorySource,

    /// Confidence weight based on source (0.0-1.0).
    source_confidence: f32,

    /// Emotional state at the time of encoding.
    emotional_snapshot: EmotionalSnapshot,

    /// Importance for promotion and recall (0.0-1.0).
    salience: f32,

    /// Changes that occurred as a result of this memory.
    deltas_applied: DeltasApplied,

    /// One-sentence summary of the memory.
    summary: String,

    /// Context where the memory was encoded.
    microsystem_context: Option<MicrosystemId>,
}

impl MemoryEntry {
    /// Creates a new memory entry with the given timestamp and summary.
    ///
    /// The memory ID is automatically generated. Use builder methods to
    /// set additional fields.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - Time since entity creation when memory formed
    /// * `summary` - One-sentence summary of the memory
    #[must_use]
    pub fn new(timestamp: Duration, summary: impl Into<String>) -> Self {
        MemoryEntry {
            id: generate_memory_id(),
            event_id: None,
            timestamp,
            participants: Vec::new(),
            tags: Vec::new(),
            source: MemorySource::default(),
            source_confidence: MemorySource::default().confidence(),
            emotional_snapshot: EmotionalSnapshot::default(),
            salience: 0.5, // Default to mid-range salience
            deltas_applied: DeltasApplied::default(),
            summary: summary.into(),
            microsystem_context: None,
        }
    }

    /// Creates a new memory entry with a specific ID.
    ///
    /// This is useful for testing or when loading memories from storage.
    #[must_use]
    pub fn with_id(id: MemoryId, timestamp: Duration, summary: impl Into<String>) -> Self {
        MemoryEntry {
            id,
            event_id: None,
            timestamp,
            participants: Vec::new(),
            tags: Vec::new(),
            source: MemorySource::default(),
            source_confidence: MemorySource::default().confidence(),
            emotional_snapshot: EmotionalSnapshot::default(),
            salience: 0.5,
            deltas_applied: DeltasApplied::default(),
            summary: summary.into(),
            microsystem_context: None,
        }
    }

    // Builder methods

    /// Sets the event ID reference.
    #[must_use]
    pub fn with_event_id(mut self, event_id: EventId) -> Self {
        self.event_id = Some(event_id);
        self
    }

    /// Sets the participants.
    #[must_use]
    pub fn with_participants(mut self, participants: Vec<EntityId>) -> Self {
        self.participants = participants;
        self
    }

    /// Adds a participant.
    #[must_use]
    pub fn add_participant(mut self, participant: EntityId) -> Self {
        self.participants.push(participant);
        self
    }

    /// Sets the tags.
    #[must_use]
    pub fn with_tags(mut self, tags: Vec<MemoryTag>) -> Self {
        self.tags = tags;
        self
    }

    /// Adds a tag.
    #[must_use]
    pub fn add_tag(mut self, tag: MemoryTag) -> Self {
        self.tags.push(tag);
        self
    }

    /// Sets the memory source and updates source_confidence accordingly.
    #[must_use]
    pub fn with_source(mut self, source: MemorySource) -> Self {
        self.source_confidence = source.confidence();
        self.source = source;
        self
    }

    /// Sets the emotional snapshot.
    #[must_use]
    pub fn with_emotional_snapshot(mut self, snapshot: EmotionalSnapshot) -> Self {
        self.emotional_snapshot = snapshot;
        self
    }

    /// Sets the salience (clamped to 0.0-1.0).
    #[must_use]
    pub fn with_salience(mut self, salience: f32) -> Self {
        self.salience = salience.clamp(0.0, 1.0);
        self
    }

    /// Sets the deltas applied.
    #[must_use]
    pub fn with_deltas_applied(mut self, deltas: DeltasApplied) -> Self {
        self.deltas_applied = deltas;
        self
    }

    /// Sets the microsystem context.
    #[must_use]
    pub fn with_microsystem_context(mut self, context: MicrosystemId) -> Self {
        self.microsystem_context = Some(context);
        self
    }

    // Accessors

    /// Returns the memory's unique identifier.
    #[must_use]
    pub fn id(&self) -> &MemoryId {
        &self.id
    }

    /// Returns the optional event ID reference.
    #[must_use]
    pub fn event_id(&self) -> Option<&EventId> {
        self.event_id.as_ref()
    }

    /// Returns the timestamp when the memory was formed.
    #[must_use]
    pub fn timestamp(&self) -> Duration {
        self.timestamp
    }

    /// Returns the participants.
    #[must_use]
    pub fn participants(&self) -> &[EntityId] {
        &self.participants
    }

    /// Returns the tags.
    #[must_use]
    pub fn tags(&self) -> &[MemoryTag] {
        &self.tags
    }

    /// Returns the memory source.
    #[must_use]
    pub fn source(&self) -> MemorySource {
        self.source
    }

    /// Returns the source confidence.
    #[must_use]
    pub fn source_confidence(&self) -> f32 {
        self.source_confidence
    }

    /// Returns the emotional snapshot.
    #[must_use]
    pub fn emotional_snapshot(&self) -> &EmotionalSnapshot {
        &self.emotional_snapshot
    }

    /// Returns the salience.
    #[must_use]
    pub fn salience(&self) -> f32 {
        self.salience
    }

    /// Returns the deltas applied.
    #[must_use]
    pub fn deltas_applied(&self) -> &DeltasApplied {
        &self.deltas_applied
    }

    /// Returns the summary.
    #[must_use]
    pub fn summary(&self) -> &str {
        &self.summary
    }

    /// Returns the microsystem context.
    #[must_use]
    pub fn microsystem_context(&self) -> Option<&MicrosystemId> {
        self.microsystem_context.as_ref()
    }

    /// Returns whether this memory contains the specified tag.
    #[must_use]
    pub fn has_tag(&self, tag: MemoryTag) -> bool {
        self.tags.contains(&tag)
    }

    /// Returns whether this memory involves the specified entity.
    #[must_use]
    pub fn involves_participant(&self, entity_id: &EntityId) -> bool {
        self.participants.contains(entity_id)
    }

    /// Returns whether this memory was formed in the specified context.
    #[must_use]
    pub fn is_in_context(&self, context: &MicrosystemId) -> bool {
        self.microsystem_context.as_ref() == Some(context)
    }

    /// Applies salience decay over the given duration.
    ///
    /// Memory salience decays over time using an exponential decay formula.
    /// The time_scale parameter adjusts the decay rate based on species:
    /// - Human: time_scale = 1.0 (base rate)
    /// - Dog: time_scale = 6.7 (6.7x faster decay)
    ///
    /// # Arguments
    ///
    /// * `duration` - The elapsed time
    /// * `time_scale` - Species-based time scaling factor
    /// * `half_life_days` - Days for salience to decay by half (default: 30 days)
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::memory::MemoryEntry;
    /// use behavioral_pathways::types::Duration;
    ///
    /// let mut entry = MemoryEntry::new(Duration::days(0), "Test").with_salience(0.8);
    ///
    /// // After one half-life, salience should be roughly half
    /// entry.apply_salience_decay(Duration::days(30), 1.0, 30.0);
    /// assert!((entry.salience() - 0.4).abs() < 0.01);
    /// ```
    pub fn apply_salience_decay(
        &mut self,
        duration: Duration,
        time_scale: f32,
        half_life_days: f32,
    ) {
        // Decay formula: new_salience = salience * 2^(-t/half_life)
        // where t is scaled elapsed time in days
        let elapsed_days = duration.as_days() as f32 * time_scale;
        let decay_factor = 0.5_f32.powf(elapsed_days / half_life_days);
        self.salience = (self.salience * decay_factor).clamp(0.0, 1.0);
    }

    /// Sets the salience directly (useful for testing or manual adjustments).
    ///
    /// Value is clamped to [0.0, 1.0].
    pub fn set_salience(&mut self, salience: f32) {
        self.salience = salience.clamp(0.0, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_entry_creation_with_all_fields() {
        let participant = EntityId::new("entity_001").unwrap();
        let event_id = EventId::new("evt_001").unwrap();
        let context = MicrosystemId::new("work_001").unwrap();
        let snapshot = EmotionalSnapshot::new(0.5, 0.3, -0.2);
        let deltas = DeltasApplied::new();

        let entry = MemoryEntry::new(Duration::days(10), "Test memory")
            .with_event_id(event_id.clone())
            .with_participants(vec![participant.clone()])
            .with_tags(vec![MemoryTag::Personal])
            .with_source(MemorySource::Self_)
            .with_emotional_snapshot(snapshot)
            .with_salience(0.7)
            .with_deltas_applied(deltas)
            .with_microsystem_context(context.clone());

        assert_eq!(entry.event_id(), Some(&event_id));
        assert_eq!(entry.participants().len(), 1);
        assert_eq!(entry.participants()[0], participant);
        assert_eq!(entry.tags().len(), 1);
        assert_eq!(entry.tags()[0], MemoryTag::Personal);
        assert_eq!(entry.source(), MemorySource::Self_);
        assert!((entry.source_confidence() - 1.0).abs() < f32::EPSILON);
        assert!((entry.emotional_snapshot().valence() - 0.5).abs() < f32::EPSILON);
        assert!((entry.salience() - 0.7).abs() < f32::EPSILON);
        assert_eq!(entry.summary(), "Test memory");
        assert_eq!(entry.microsystem_context(), Some(&context));
        // Verify deltas_applied accessor
        assert!(!entry.deltas_applied().has_changes());
    }

    #[test]
    fn memory_entry_optional_event_id() {
        let entry = MemoryEntry::new(Duration::days(10), "Memory without event");
        assert!(entry.event_id().is_none());

        let event_id = EventId::new("evt_001").unwrap();
        let entry_with_event = entry.with_event_id(event_id.clone());
        assert_eq!(entry_with_event.event_id(), Some(&event_id));
    }

    #[test]
    fn memory_entry_requires_memory_id() {
        let entry = MemoryEntry::new(Duration::days(10), "Test");
        // ID should not be empty
        assert!(!entry.id().as_str().is_empty());
    }

    #[test]
    fn memory_entry_id_auto_generated() {
        let entry1 = MemoryEntry::new(Duration::days(10), "Test 1");
        let entry2 = MemoryEntry::new(Duration::days(10), "Test 2");

        // Each entry should have a unique ID
        assert_ne!(entry1.id(), entry2.id());
    }

    #[test]
    fn memory_entry_id_uses_uuid_pattern() {
        let entry = MemoryEntry::new(Duration::days(10), "Test");
        let id_str = entry.id().as_str();

        // ID should start with "mem_"
        assert!(id_str.starts_with("mem_"));

        // The UUID portion should be after "mem_" and be 36 chars (standard UUID format)
        let uuid_portion = &id_str[4..];
        assert_eq!(uuid_portion.len(), 36);

        // UUID format: 8-4-4-4-12 with hyphens
        let parts: Vec<&str> = uuid_portion.split('-').collect();
        assert_eq!(parts.len(), 5);
        assert_eq!(parts[0].len(), 8);
        assert_eq!(parts[1].len(), 4);
        assert_eq!(parts[2].len(), 4);
        assert_eq!(parts[3].len(), 4);
        assert_eq!(parts[4].len(), 12);
    }

    #[test]
    fn memory_entry_timestamp_is_duration() {
        let entry = MemoryEntry::new(Duration::days(365), "One year later");
        assert_eq!(entry.timestamp().as_days(), 365);
    }

    #[test]
    fn memory_entry_optional_microsystem_context() {
        let entry = MemoryEntry::new(Duration::days(10), "Test");
        assert!(entry.microsystem_context().is_none());

        let context = MicrosystemId::new("home_001").unwrap();
        let entry_with_context = entry.with_microsystem_context(context.clone());
        assert_eq!(entry_with_context.microsystem_context(), Some(&context));
    }

    #[test]
    fn with_id_sets_specific_id() {
        let id = MemoryId::new("custom_id").unwrap();
        let entry = MemoryEntry::with_id(id.clone(), Duration::days(10), "Test");
        assert_eq!(entry.id(), &id);
    }

    #[test]
    fn add_participant_appends() {
        let p1 = EntityId::new("entity_001").unwrap();
        let p2 = EntityId::new("entity_002").unwrap();

        let entry = MemoryEntry::new(Duration::days(10), "Test")
            .add_participant(p1.clone())
            .add_participant(p2.clone());

        assert_eq!(entry.participants().len(), 2);
        assert!(entry.involves_participant(&p1));
        assert!(entry.involves_participant(&p2));
    }

    #[test]
    fn add_tag_appends() {
        let entry = MemoryEntry::new(Duration::days(10), "Test")
            .add_tag(MemoryTag::Personal)
            .add_tag(MemoryTag::Achievement);

        assert_eq!(entry.tags().len(), 2);
        assert!(entry.has_tag(MemoryTag::Personal));
        assert!(entry.has_tag(MemoryTag::Achievement));
    }

    #[test]
    fn source_updates_confidence() {
        let entry_self =
            MemoryEntry::new(Duration::days(10), "Test").with_source(MemorySource::Self_);
        assert!((entry_self.source_confidence() - 1.0).abs() < f32::EPSILON);

        let entry_witness =
            MemoryEntry::new(Duration::days(10), "Test").with_source(MemorySource::Witness);
        assert!((entry_witness.source_confidence() - 0.7).abs() < f32::EPSILON);

        let entry_rumor =
            MemoryEntry::new(Duration::days(10), "Test").with_source(MemorySource::Rumor);
        assert!((entry_rumor.source_confidence() - 0.4).abs() < f32::EPSILON);
    }

    #[test]
    fn salience_clamped() {
        let entry_high = MemoryEntry::new(Duration::days(10), "Test").with_salience(1.5);
        assert!((entry_high.salience() - 1.0).abs() < f32::EPSILON);

        let entry_low = MemoryEntry::new(Duration::days(10), "Test").with_salience(-0.5);
        assert!(entry_low.salience().abs() < f32::EPSILON);
    }

    #[test]
    fn has_tag_returns_correct_result() {
        let entry = MemoryEntry::new(Duration::days(10), "Test")
            .with_tags(vec![MemoryTag::Betrayal, MemoryTag::Personal]);

        assert!(entry.has_tag(MemoryTag::Betrayal));
        assert!(entry.has_tag(MemoryTag::Personal));
        assert!(!entry.has_tag(MemoryTag::Achievement));
    }

    #[test]
    fn involves_participant_returns_correct_result() {
        let p1 = EntityId::new("entity_001").unwrap();
        let p2 = EntityId::new("entity_002").unwrap();
        let p3 = EntityId::new("entity_003").unwrap();

        let entry = MemoryEntry::new(Duration::days(10), "Test")
            .with_participants(vec![p1.clone(), p2.clone()]);

        assert!(entry.involves_participant(&p1));
        assert!(entry.involves_participant(&p2));
        assert!(!entry.involves_participant(&p3));
    }

    #[test]
    fn is_in_context_returns_correct_result() {
        let context1 = MicrosystemId::new("work_001").unwrap();
        let context2 = MicrosystemId::new("home_001").unwrap();

        let entry =
            MemoryEntry::new(Duration::days(10), "Test").with_microsystem_context(context1.clone());

        assert!(entry.is_in_context(&context1));
        assert!(!entry.is_in_context(&context2));
    }

    #[test]
    fn is_in_context_false_when_no_context() {
        let entry = MemoryEntry::new(Duration::days(10), "Test");
        let context = MicrosystemId::new("work_001").unwrap();

        assert!(!entry.is_in_context(&context));
    }

    #[test]
    fn clone() {
        let entry = MemoryEntry::new(Duration::days(10), "Test")
            .with_salience(0.8)
            .add_tag(MemoryTag::Personal);

        let cloned = entry.clone();
        assert_eq!(entry, cloned);
    }

    #[test]
    fn debug_format() {
        let entry = MemoryEntry::new(Duration::days(10), "Test");
        let debug = format!("{:?}", entry);
        assert!(debug.contains("MemoryEntry"));
    }

    #[test]
    fn default_values() {
        let entry = MemoryEntry::new(Duration::days(10), "Test");

        // Default source is Self_
        assert_eq!(entry.source(), MemorySource::Self_);
        assert!((entry.source_confidence() - 1.0).abs() < f32::EPSILON);

        // Default salience is 0.5
        assert!((entry.salience() - 0.5).abs() < f32::EPSILON);

        // Default emotional snapshot is neutral
        assert!(entry.emotional_snapshot().valence().abs() < f32::EPSILON);

        // Default participants and tags are empty
        assert!(entry.participants().is_empty());
        assert!(entry.tags().is_empty());

        // Default optional fields are None
        assert!(entry.event_id().is_none());
        assert!(entry.microsystem_context().is_none());
    }

    #[test]
    fn apply_salience_decay_halves_at_half_life() {
        let mut entry = MemoryEntry::new(Duration::days(0), "Test").with_salience(0.8);

        // After exactly one half-life (30 days), salience should be half
        entry.apply_salience_decay(Duration::days(30), 1.0, 30.0);
        assert!((entry.salience() - 0.4).abs() < 0.01);
    }

    #[test]
    fn apply_salience_decay_respects_time_scale() {
        // Human with time_scale 1.0
        let mut human_memory = MemoryEntry::new(Duration::days(0), "Human").with_salience(0.8);
        human_memory.apply_salience_decay(Duration::days(30), 1.0, 30.0);

        // Dog with time_scale 6.7 decays much faster
        let mut dog_memory = MemoryEntry::new(Duration::days(0), "Dog").with_salience(0.8);
        dog_memory.apply_salience_decay(Duration::days(30), 6.7, 30.0);

        // Human at 30 days should be at ~0.4 (one half-life)
        assert!((human_memory.salience() - 0.4).abs() < 0.01);

        // Dog at 30 days experiences 30 * 6.7 = 201 scaled days
        // That's 201/30 = ~6.7 half-lives, so 0.8 * 0.5^6.7 = ~0.008
        assert!(dog_memory.salience() < 0.02);
    }

    #[test]
    fn apply_salience_decay_clamps_to_zero() {
        let mut entry = MemoryEntry::new(Duration::days(0), "Test").with_salience(0.5);

        // After many half-lives, salience should be effectively zero but clamped
        entry.apply_salience_decay(Duration::days(1000), 1.0, 30.0);
        assert!(entry.salience() >= 0.0);
        assert!(entry.salience() < 0.001);
    }

    #[test]
    fn apply_salience_decay_zero_duration_no_change() {
        let mut entry = MemoryEntry::new(Duration::days(0), "Test").with_salience(0.7);

        entry.apply_salience_decay(Duration::days(0), 1.0, 30.0);
        assert!((entry.salience() - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn set_salience_clamps() {
        let mut entry = MemoryEntry::new(Duration::days(0), "Test");

        entry.set_salience(1.5);
        assert!((entry.salience() - 1.0).abs() < f32::EPSILON);

        entry.set_salience(-0.5);
        assert!(entry.salience().abs() < f32::EPSILON);
    }
}
