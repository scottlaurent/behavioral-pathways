//! Fluent builder for event construction.
//!
//! The builder pattern allows clean construction of events with many
//! optional fields. Category is auto-derived from EventType.

use crate::enums::{EventPayload, EventTag, EventType, HexacoPath};
use crate::event::Event;
use crate::types::{Duration, EntityId, EventId, MicrosystemId};
use std::fmt;

/// Error returned when event building fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventBuildError {
    /// Description of what went wrong.
    pub reason: String,
}

impl fmt::Display for EventBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Event build error: {}", self.reason)
    }
}

impl std::error::Error for EventBuildError {}

/// Fluent builder for constructing events.
///
/// Use this to create events with clean, readable syntax. The category
/// is automatically derived from the event type and cannot be set
/// independently to prevent mismatches.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::event::EventBuilder;
/// use behavioral_pathways::enums::{EventType, EventTag, EventPayload, SupportType};
/// use behavioral_pathways::types::{EntityId, Duration};
///
/// let helper = EntityId::new("helper_001").unwrap();
/// let recipient = EntityId::new("recipient_001").unwrap();
///
/// let event = EventBuilder::new(EventType::Support)
///     .source(helper)
///     .target(recipient)
///     .severity(0.8)
///     .tag(EventTag::Positive)
///     .timestamp(Duration::days(100))
///     .payload(EventPayload::Support {
///         support_type: SupportType::Emotional,
///         effectiveness: 0.9,
///     })
///     .build()
///     .unwrap();
///
/// assert_eq!(event.event_type(), EventType::Support);
/// assert!((event.severity() - 0.8).abs() < f64::EPSILON);
/// ```
#[derive(Debug, Clone)]
pub struct EventBuilder {
    event_type: EventType,
    id: Option<EventId>,
    source: Option<EntityId>,
    target: Option<EntityId>,
    severity: f64,
    tags: Vec<EventTag>,
    payload: Option<EventPayload>,
    timestamp: Duration,
    microsystem_context: Option<MicrosystemId>,
    base_shifts: Vec<(HexacoPath, f32)>,
}

impl EventBuilder {
    /// Creates a new builder for the given event type.
    ///
    /// The category is automatically derived from the event type.
    ///
    /// # Arguments
    ///
    /// * `event_type` - The type of event to build
    #[must_use]
    pub fn new(event_type: EventType) -> Self {
        EventBuilder {
            event_type,
            id: None,
            source: None,
            target: None,
            severity: 0.5,
            tags: Vec::new(),
            payload: None,
            timestamp: Duration::zero(),
            microsystem_context: None,
            base_shifts: Vec::new(),
        }
    }

    /// Sets a specific event ID (for testing or loading).
    ///
    /// If not set, an ID will be auto-generated.
    #[must_use]
    pub fn id(mut self, id: EventId) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the source entity (who caused the event).
    #[must_use]
    pub fn source(mut self, entity_id: EntityId) -> Self {
        self.source = Some(entity_id);
        self
    }

    /// Sets the target entity (who is affected).
    #[must_use]
    pub fn target(mut self, entity_id: EntityId) -> Self {
        self.target = Some(entity_id);
        self
    }

    /// Sets the severity (0.0 to 1.0).
    ///
    /// Values are clamped to the valid range.
    #[must_use]
    pub fn severity(mut self, severity: f64) -> Self {
        self.severity = severity.clamp(0.0, 1.0);
        self
    }

    /// Adds a single tag.
    #[must_use]
    pub fn tag(mut self, tag: EventTag) -> Self {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
        self
    }

    /// Sets multiple tags at once.
    #[must_use]
    pub fn tags(mut self, tags: Vec<EventTag>) -> Self {
        self.tags = tags;
        self
    }

    /// Sets the type-specific payload.
    #[must_use]
    pub fn payload(mut self, payload: EventPayload) -> Self {
        self.payload = Some(payload);
        self
    }

    /// Sets the timestamp (entity age at event time).
    #[must_use]
    pub fn timestamp(mut self, timestamp: Duration) -> Self {
        self.timestamp = timestamp;
        self
    }

    /// Sets the microsystem context where the event occurred.
    #[must_use]
    pub fn context(mut self, microsystem: MicrosystemId) -> Self {
        self.microsystem_context = Some(microsystem);
        self
    }

    /// Adds a personality base shift to this event.
    ///
    /// Base shifts represent permanent personality changes triggered by
    /// formative events. Multiple shifts can be added to the same event.
    ///
    /// # Arguments
    ///
    /// * `trait_path` - Which HEXACO trait to shift
    /// * `amount` - Shift amount (clamped to -1.0 to 1.0)
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::event::EventBuilder;
    /// use behavioral_pathways::enums::{EventType, HexacoPath};
    ///
    /// let event = EventBuilder::new(EventType::Violence)
    ///     .severity(0.9)
    ///     .with_base_shift(HexacoPath::Neuroticism, 0.25)
    ///     .with_base_shift(HexacoPath::Agreeableness, -0.15)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert!(event.has_base_shifts());
    /// assert_eq!(event.base_shifts().len(), 2);
    /// ```
    #[must_use]
    pub fn with_base_shift(mut self, trait_path: HexacoPath, amount: f32) -> Self {
        let clamped = amount.clamp(-1.0, 1.0);
        self.base_shifts.push((trait_path, clamped));
        self
    }

    /// Builds the event, consuming the builder.
    ///
    /// # Returns
    ///
    /// The constructed event, or an error if validation fails.
    ///
    /// # Notes
    ///
    /// If no payload was set, defaults to `EventPayload::Empty`.
    pub fn build(self) -> Result<Event, EventBuildError> {
        let mut event = if let Some(id) = self.id {
            Event::with_id(id, self.event_type)
        } else {
            Event::new(self.event_type)
        };

        event.set_source(self.source);
        event.set_target(self.target);
        event.set_severity(self.severity);
        event.set_tags(self.tags);
        // Use provided payload or default to Empty
        event.set_payload(self.payload.unwrap_or(EventPayload::Empty));
        event.set_timestamp(self.timestamp);
        event.set_microsystem_context(self.microsystem_context);
        event.set_base_shifts(self.base_shifts);

        Ok(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::{EventCategory, SupportType};

    #[test]
    fn event_builder_minimal() {
        let event = EventBuilder::new(EventType::Violence).build().unwrap();

        assert_eq!(event.event_type(), EventType::Violence);
        assert_eq!(event.category(), EventCategory::Trauma);
        assert!(event.source().is_none());
        assert!(event.target().is_none());
        assert!((event.severity() - 0.5).abs() < f64::EPSILON);
        assert!(event.tags().is_empty());
        // Empty payload is the default
        assert_eq!(event.payload(), &EventPayload::Empty);
        assert!(!event.has_payload_data());
    }

    #[test]
    fn event_builder_full() {
        let source = EntityId::new("source_001").unwrap();
        let target = EntityId::new("target_001").unwrap();
        let context = MicrosystemId::new("work_001").unwrap();
        let custom_id = EventId::new("custom_evt").unwrap();

        let event = EventBuilder::new(EventType::Support)
            .id(custom_id.clone())
            .source(source.clone())
            .target(target.clone())
            .severity(0.9)
            .tag(EventTag::Positive)
            .tag(EventTag::Personal)
            .timestamp(Duration::days(50))
            .context(context.clone())
            .payload(EventPayload::Support {
                support_type: SupportType::Emotional,
                effectiveness: 0.85,
            })
            .build()
            .unwrap();

        assert_eq!(event.id(), &custom_id);
        assert_eq!(event.event_type(), EventType::Support);
        assert_eq!(event.category(), EventCategory::Social);
        assert_eq!(event.source(), Some(&source));
        assert_eq!(event.target(), Some(&target));
        assert!((event.severity() - 0.9).abs() < f64::EPSILON);
        assert!(event.has_tag(EventTag::Positive));
        assert!(event.has_tag(EventTag::Personal));
        assert_eq!(event.timestamp().as_days(), 50);
        assert_eq!(event.microsystem_context(), Some(&context));
        assert!(event.has_payload_data());
    }

    #[test]
    fn event_builder_fluent_chain() {
        let event = EventBuilder::new(EventType::Conflict)
            .severity(0.7)
            .tag(EventTag::Negative)
            .tag(EventTag::Work)
            .timestamp(Duration::hours(24))
            .build()
            .unwrap();

        assert_eq!(event.event_type(), EventType::Conflict);
        assert!((event.severity() - 0.7).abs() < f64::EPSILON);
        assert_eq!(event.tags().len(), 2);
    }

    #[test]
    fn event_builder_severity_clamped() {
        let event_high = EventBuilder::new(EventType::Violence)
            .severity(1.5)
            .build()
            .unwrap();
        assert!((event_high.severity() - 1.0).abs() < f64::EPSILON);

        let event_low = EventBuilder::new(EventType::Violence)
            .severity(-0.5)
            .build()
            .unwrap();
        assert!(event_low.severity().abs() < f64::EPSILON);
    }

    #[test]
    fn event_builder_tag_no_duplicates() {
        let event = EventBuilder::new(EventType::Conflict)
            .tag(EventTag::Negative)
            .tag(EventTag::Negative)
            .build()
            .unwrap();

        assert_eq!(event.tags().len(), 1);
    }

    #[test]
    fn event_builder_tags_replaces_all() {
        let event = EventBuilder::new(EventType::Conflict)
            .tag(EventTag::Personal)
            .tags(vec![EventTag::Work, EventTag::HighStakes])
            .build()
            .unwrap();

        assert!(!event.has_tag(EventTag::Personal));
        assert!(event.has_tag(EventTag::Work));
        assert!(event.has_tag(EventTag::HighStakes));
    }

    #[test]
    fn event_builder_auto_generates_id() {
        let event1 = EventBuilder::new(EventType::Interaction).build().unwrap();
        let event2 = EventBuilder::new(EventType::Interaction).build().unwrap();

        assert_ne!(event1.id(), event2.id());
    }

    #[test]
    fn event_builder_category_auto_derived() {
        // Category should be derived from event_type, not settable
        let event = EventBuilder::new(EventType::SocialExclusion)
            .build()
            .unwrap();
        assert_eq!(event.category(), EventCategory::SocialBelonging);

        let event2 = EventBuilder::new(EventType::TraumaticExposure)
            .build()
            .unwrap();
        assert_eq!(event2.category(), EventCategory::Trauma);
    }

    #[test]
    fn event_build_error_display() {
        let error = EventBuildError {
            reason: "test error".to_string(),
        };
        let display = format!("{}", error);
        assert!(display.contains("test error"));
    }

    #[test]
    fn event_builder_clone() {
        let builder = EventBuilder::new(EventType::Violence)
            .severity(0.8)
            .tag(EventTag::Negative);

        let cloned = builder.clone();

        let event1 = builder.build().unwrap();
        let event2 = cloned.build().unwrap();

        assert_eq!(event1.event_type(), event2.event_type());
        assert!((event1.severity() - event2.severity()).abs() < f64::EPSILON);
    }

    #[test]
    fn event_builder_debug_format() {
        let builder = EventBuilder::new(EventType::Violence);
        let debug = format!("{:?}", builder);
        assert!(debug.contains("EventBuilder"));
        assert!(debug.contains("Violence"));
    }

    #[test]
    fn event_builder_with_base_shift() {
        let event = EventBuilder::new(EventType::Violence)
            .severity(0.9)
            .with_base_shift(HexacoPath::Neuroticism, 0.25)
            .with_base_shift(HexacoPath::Agreeableness, -0.15)
            .build()
            .unwrap();

        assert!(event.has_base_shifts());
        assert_eq!(event.base_shifts().len(), 2);
        assert_eq!(event.base_shifts()[0], (HexacoPath::Neuroticism, 0.25));
        assert_eq!(event.base_shifts()[1], (HexacoPath::Agreeableness, -0.15));
    }

    #[test]
    fn event_builder_no_base_shifts_by_default() {
        let event = EventBuilder::new(EventType::Violence).build().unwrap();
        assert!(!event.has_base_shifts());
    }

    #[test]
    fn event_builder_base_shift_clamps_values() {
        let event = EventBuilder::new(EventType::Violence)
            .with_base_shift(HexacoPath::Openness, 2.0)
            .with_base_shift(HexacoPath::Extraversion, -2.0)
            .build()
            .unwrap();

        assert!((event.base_shifts()[0].1 - 1.0).abs() < f32::EPSILON);
        assert!((event.base_shifts()[1].1 - (-1.0)).abs() < f32::EPSILON);
    }
}
