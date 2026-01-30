//! Core event structure for behavioral pathways.
//!
//! Events are occurrences that affect entity state. Each event has a type,
//! optional source and target, severity, tags, and type-specific payload.

use crate::enums::{EventCategory, EventPayload, EventTag, EventType, HexacoPath};
use crate::types::{Duration, EntityId, EventId, MicrosystemId};
use uuid::Uuid;

/// Generates a unique event ID using UUID v4.
fn generate_event_id() -> EventId {
    let uuid = Uuid::new_v4();
    // Safe to unwrap because UUIDs are never empty
    EventId::new(format!("evt_{uuid}")).unwrap()
}

/// An event that can affect entity state.
///
/// Events are the primary mechanism for state changes in the simulation.
/// Each event has a type that determines its category (for theoretical
/// linkage), and optionally a source (who caused it), target (who is
/// affected), and type-specific payload data.
///
/// # Event Category
///
/// The category is automatically derived from the event type and cannot
/// be set independently. This ensures consistency between type and
/// theoretical domain.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::event::{Event, EventBuilder};
/// use behavioral_pathways::enums::{EventType, EventCategory, EventTag};
///
/// let event = EventBuilder::new(EventType::SocialExclusion)
///     .severity(0.7)
///     .tag(EventTag::Negative)
///     .build()
///     .unwrap();
///
/// assert_eq!(event.event_type(), EventType::SocialExclusion);
/// assert_eq!(event.category(), EventCategory::SocialBelonging);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    /// Unique identifier for this event.
    id: EventId,
    /// Primary classification of the event.
    event_type: EventType,
    /// Theoretical domain (auto-derived from event_type).
    category: EventCategory,
    /// Entity that caused the event (None for environmental).
    source: Option<EntityId>,
    /// Entity affected by the event (None for broadcast).
    target: Option<EntityId>,
    /// Intensity of the event (0.0 to 1.0).
    severity: f64,
    /// Additional categorization tags.
    tags: Vec<EventTag>,
    /// Type-specific event data (required, use EventPayload::Empty if none).
    payload: EventPayload,
    /// When the event occurred (entity age at event time).
    timestamp: Duration,
    /// Microsystem context where event occurred.
    microsystem_context: Option<MicrosystemId>,
    /// Personality base shifts triggered by this event.
    /// Each entry is (trait, shift_amount) to be processed during simulation.
    base_shifts: Vec<(HexacoPath, f32)>,
}

impl Event {
    /// Creates a new event with the given type.
    ///
    /// This is an internal constructor. Use `EventBuilder` for public
    /// construction. Payload defaults to EventPayload::Empty.
    pub(crate) fn new(event_type: EventType) -> Self {
        Event {
            id: generate_event_id(),
            event_type,
            category: event_type.category(),
            source: None,
            target: None,
            severity: 0.5,
            tags: Vec::new(),
            payload: EventPayload::Empty,
            timestamp: Duration::zero(),
            microsystem_context: None,
            base_shifts: Vec::new(),
        }
    }

    /// Creates a new event with a specific ID (for testing/loading).
    #[must_use]
    pub fn with_id(id: EventId, event_type: EventType) -> Self {
        Event {
            id,
            event_type,
            category: event_type.category(),
            source: None,
            target: None,
            severity: 0.5,
            tags: Vec::new(),
            payload: EventPayload::Empty,
            timestamp: Duration::zero(),
            microsystem_context: None,
            base_shifts: Vec::new(),
        }
    }

    // Accessors

    /// Returns the event's unique identifier.
    #[must_use]
    pub fn id(&self) -> &EventId {
        &self.id
    }

    /// Returns the event type.
    #[must_use]
    pub fn event_type(&self) -> EventType {
        self.event_type
    }

    /// Returns the event category (auto-derived from type).
    #[must_use]
    pub fn category(&self) -> EventCategory {
        self.category
    }

    /// Returns the source entity, if any.
    #[must_use]
    pub fn source(&self) -> Option<&EntityId> {
        self.source.as_ref()
    }

    /// Returns the target entity, if any.
    #[must_use]
    pub fn target(&self) -> Option<&EntityId> {
        self.target.as_ref()
    }

    /// Returns the severity (0.0 to 1.0).
    #[must_use]
    pub fn severity(&self) -> f64 {
        self.severity
    }

    /// Returns the tags.
    #[must_use]
    pub fn tags(&self) -> &[EventTag] {
        &self.tags
    }

    /// Returns the payload.
    #[must_use]
    pub fn payload(&self) -> &EventPayload {
        &self.payload
    }

    /// Returns whether the payload has data (not Empty).
    #[must_use]
    pub fn has_payload_data(&self) -> bool {
        !matches!(self.payload, EventPayload::Empty)
    }

    /// Returns the timestamp.
    #[must_use]
    pub fn timestamp(&self) -> Duration {
        self.timestamp
    }

    /// Returns the microsystem context, if any.
    #[must_use]
    pub fn microsystem_context(&self) -> Option<&MicrosystemId> {
        self.microsystem_context.as_ref()
    }

    /// Returns the personality base shifts for this event.
    ///
    /// Each entry is a (trait, shift_amount) pair representing a permanent
    /// personality change to apply when this event is processed.
    #[must_use]
    pub fn base_shifts(&self) -> &[(HexacoPath, f32)] {
        &self.base_shifts
    }

    /// Returns true if this event has any personality base shifts.
    #[must_use]
    pub fn has_base_shifts(&self) -> bool {
        !self.base_shifts.is_empty()
    }

    /// Returns whether this event has the specified tag.
    #[must_use]
    pub fn has_tag(&self, tag: EventTag) -> bool {
        self.tags.contains(&tag)
    }

    /// Returns true if this is a trauma-category event.
    #[must_use]
    pub fn is_trauma(&self) -> bool {
        self.category == EventCategory::Trauma
    }

    /// Returns true if this event affects social belonging.
    #[must_use]
    pub fn affects_belonging(&self) -> bool {
        self.category == EventCategory::SocialBelonging
    }

    /// Returns true if this event affects burden perception.
    #[must_use]
    pub fn affects_burden(&self) -> bool {
        self.category == EventCategory::BurdenPerception
    }

    // Builder-style setters for internal use

    #[allow(dead_code)]
    pub(crate) fn set_id(&mut self, id: EventId) {
        self.id = id;
    }

    #[cfg(test)]
    pub(crate) fn set_category_for_test(&mut self, category: EventCategory) {
        self.category = category;
    }

    pub(crate) fn set_source(&mut self, source: Option<EntityId>) {
        self.source = source;
    }

    pub(crate) fn set_target(&mut self, target: Option<EntityId>) {
        self.target = target;
    }

    pub(crate) fn set_severity(&mut self, severity: f64) {
        self.severity = severity.clamp(0.0, 1.0);
    }

    pub(crate) fn set_tags(&mut self, tags: Vec<EventTag>) {
        self.tags = tags;
    }

    #[allow(dead_code)]
    pub(crate) fn add_tag(&mut self, tag: EventTag) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    pub(crate) fn set_payload(&mut self, payload: EventPayload) {
        self.payload = payload;
    }

    pub(crate) fn set_timestamp(&mut self, timestamp: Duration) {
        self.timestamp = timestamp;
    }

    pub(crate) fn set_microsystem_context(&mut self, context: Option<MicrosystemId>) {
        self.microsystem_context = context;
    }

    pub(crate) fn set_base_shifts(&mut self, shifts: Vec<(HexacoPath, f32)>) {
        self.base_shifts = shifts;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_creation_with_type() {
        let event = Event::new(EventType::Violence);
        assert_eq!(event.event_type(), EventType::Violence);
        assert_eq!(event.category(), EventCategory::Trauma);
    }

    #[test]
    fn event_type_to_category_mapping() {
        let event = Event::new(EventType::SocialExclusion);
        assert_eq!(event.category(), EventCategory::SocialBelonging);

        let event2 = Event::new(EventType::BurdenFeedback);
        assert_eq!(event2.category(), EventCategory::BurdenPerception);

        let event3 = Event::new(EventType::Humiliation);
        assert_eq!(event3.category(), EventCategory::Control);
    }

    #[test]
    fn event_source_target_specification() {
        let mut event = Event::new(EventType::Conflict);
        let source = EntityId::new("attacker").unwrap();
        let target = EntityId::new("victim").unwrap();

        event.set_source(Some(source.clone()));
        event.set_target(Some(target.clone()));

        assert_eq!(event.source(), Some(&source));
        assert_eq!(event.target(), Some(&target));
    }

    #[test]
    fn event_id_auto_generated() {
        let event1 = Event::new(EventType::Interaction);
        let event2 = Event::new(EventType::Interaction);

        assert_ne!(event1.id(), event2.id());
        assert!(event1.id().as_str().starts_with("evt_"));
    }

    #[test]
    fn event_set_id_overrides_existing_id() {
        let mut event = Event::new(EventType::Interaction);
        let new_id = EventId::new("custom_event_override").unwrap();

        event.set_id(new_id.clone());

        assert_eq!(event.id(), &new_id);
    }

    #[test]
    fn event_with_id_sets_specific_id() {
        let id = EventId::new("custom_event").unwrap();
        let event = Event::with_id(id.clone(), EventType::Support);
        assert_eq!(event.id(), &id);
    }

    #[test]
    fn event_severity_clamped() {
        let mut event = Event::new(EventType::Violence);
        event.set_severity(1.5);
        assert!((event.severity() - 1.0).abs() < f64::EPSILON);

        event.set_severity(-0.5);
        assert!(event.severity().abs() < f64::EPSILON);
    }

    #[test]
    fn event_tags_work() {
        let mut event = Event::new(EventType::Conflict);
        event.add_tag(EventTag::Negative);
        event.add_tag(EventTag::HighStakes);

        assert!(event.has_tag(EventTag::Negative));
        assert!(event.has_tag(EventTag::HighStakes));
        assert!(!event.has_tag(EventTag::Positive));
    }

    #[test]
    fn event_tags_no_duplicates() {
        let mut event = Event::new(EventType::Conflict);
        event.add_tag(EventTag::Negative);
        event.add_tag(EventTag::Negative);

        assert_eq!(event.tags().len(), 1);
    }

    #[test]
    fn event_payload_access() {
        let mut event = Event::new(EventType::Violence);
        let payload = EventPayload::Violence {
            weapon: None,
            injury_severity: 0.5,
        };
        event.set_payload(payload.clone());

        assert_eq!(event.payload(), &payload);
        assert!(event.has_payload_data());
    }

    #[test]
    fn event_empty_payload() {
        let event = Event::new(EventType::Violence);
        assert!(!event.has_payload_data());
        assert_eq!(event.payload(), &EventPayload::Empty);
    }

    #[test]
    fn event_is_trauma() {
        let violence = Event::new(EventType::Violence);
        assert!(violence.is_trauma());

        let exposure = Event::new(EventType::TraumaticExposure);
        assert!(exposure.is_trauma());

        let conflict = Event::new(EventType::Conflict);
        assert!(!conflict.is_trauma());
    }

    #[test]
    fn event_affects_belonging() {
        let exclusion = Event::new(EventType::SocialExclusion);
        assert!(exclusion.affects_belonging());

        let inclusion = Event::new(EventType::SocialInclusion);
        assert!(inclusion.affects_belonging());

        let violence = Event::new(EventType::Violence);
        assert!(!violence.affects_belonging());
    }

    #[test]
    fn event_affects_burden() {
        let feedback = Event::new(EventType::BurdenFeedback);
        assert!(feedback.affects_burden());

        let violence = Event::new(EventType::Violence);
        assert!(!violence.affects_burden());
    }

    #[test]
    fn event_timestamp_works() {
        let mut event = Event::new(EventType::Interaction);
        event.set_timestamp(Duration::days(100));
        assert_eq!(event.timestamp().as_days(), 100);
    }

    #[test]
    fn event_microsystem_context_works() {
        let mut event = Event::new(EventType::Interaction);
        let context = MicrosystemId::new("work_001").unwrap();
        event.set_microsystem_context(Some(context.clone()));
        assert_eq!(event.microsystem_context(), Some(&context));
    }

    #[test]
    fn event_clone() {
        let mut event = Event::new(EventType::Violence);
        event.set_severity(0.8);
        event.add_tag(EventTag::Negative);

        let cloned = event.clone();
        assert_eq!(event, cloned);
    }

    #[test]
    fn event_debug_format() {
        let event = Event::new(EventType::Violence);
        let debug = format!("{:?}", event);
        assert!(debug.contains("Event"));
        assert!(debug.contains("Violence"));
    }

    #[test]
    fn event_default_values() {
        let event = Event::new(EventType::Interaction);

        assert!(event.source().is_none());
        assert!(event.target().is_none());
        assert!((event.severity() - 0.5).abs() < f64::EPSILON);
        assert!(event.tags().is_empty());
        assert_eq!(event.payload(), &EventPayload::Empty);
        assert_eq!(event.timestamp().as_seconds(), 0);
        assert!(event.microsystem_context().is_none());
    }

    #[test]
    fn set_tags_replaces_all() {
        let mut event = Event::new(EventType::Conflict);
        event.add_tag(EventTag::Personal);

        event.set_tags(vec![EventTag::Work, EventTag::HighStakes]);

        assert!(!event.has_tag(EventTag::Personal));
        assert!(event.has_tag(EventTag::Work));
        assert!(event.has_tag(EventTag::HighStakes));
    }

    #[test]
    fn event_default_has_no_base_shifts() {
        let event = Event::new(EventType::Violence);
        assert!(!event.has_base_shifts());
        assert!(event.base_shifts().is_empty());
    }

    #[test]
    fn event_with_base_shifts() {
        let mut event = Event::new(EventType::Violence);
        event.set_base_shifts(vec![
            (HexacoPath::Neuroticism, 0.25),
            (HexacoPath::Agreeableness, -0.15),
        ]);

        assert!(event.has_base_shifts());
        assert_eq!(event.base_shifts().len(), 2);
        assert_eq!(event.base_shifts()[0], (HexacoPath::Neuroticism, 0.25));
        assert_eq!(event.base_shifts()[1], (HexacoPath::Agreeableness, -0.15));
    }
}
