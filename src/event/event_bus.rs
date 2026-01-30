//! Event bus for dispatching and subscribing to events.
//!
//! The EventBus uses a pull-based model where events are queued
//! and subscribers poll for matching events. This avoids callback
//! complexity and allows controlled event processing.

use crate::enums::{EventCategory, EventScope, EventTag, EventType};
use crate::event::Event;
use crate::types::{EntityId, EventId, MicrosystemId, SubscriptionId};
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

/// Maximum depth for event cascades to prevent infinite loops.
pub const MAX_CASCADE_DEPTH: usize = 5;

/// Error returned by event bus operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventBusError {
    /// Description of the error.
    pub reason: String,
}

impl std::fmt::Display for EventBusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EventBus error: {}", self.reason)
    }
}

impl std::error::Error for EventBusError {}

/// Filter criteria for event subscriptions.
///
/// All criteria are optional. If a criterion is set, events must match it.
/// If not set, any value for that field is accepted.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EventFilter {
    /// Match events of specific types.
    pub event_types: Option<Vec<EventType>>,
    /// Match events of specific categories.
    pub categories: Option<Vec<EventCategory>>,
    /// Match events with specific tags.
    pub tags: Option<Vec<EventTag>>,
    /// Match events from a specific source.
    pub source: Option<EntityId>,
    /// Match events targeting a specific entity.
    pub target: Option<EntityId>,
    /// Match events in a specific microsystem.
    pub microsystem: Option<MicrosystemId>,
}

impl EventFilter {
    /// Creates a new empty filter (matches all events).
    #[must_use]
    pub fn new() -> Self {
        EventFilter::default()
    }

    /// Filters by event types.
    #[must_use]
    pub fn with_event_types(mut self, types: Vec<EventType>) -> Self {
        self.event_types = Some(types);
        self
    }

    /// Filters by a single event type.
    #[must_use]
    pub fn with_event_type(mut self, event_type: EventType) -> Self {
        self.event_types = Some(vec![event_type]);
        self
    }

    /// Filters by categories.
    #[must_use]
    pub fn with_categories(mut self, categories: Vec<EventCategory>) -> Self {
        self.categories = Some(categories);
        self
    }

    /// Filters by a single category.
    #[must_use]
    pub fn with_category(mut self, category: EventCategory) -> Self {
        self.categories = Some(vec![category]);
        self
    }

    /// Filters by tags.
    #[must_use]
    pub fn with_tags(mut self, tags: Vec<EventTag>) -> Self {
        self.tags = Some(tags);
        self
    }

    /// Filters by a single tag.
    #[must_use]
    pub fn with_tag(mut self, tag: EventTag) -> Self {
        self.tags = Some(vec![tag]);
        self
    }

    /// Filters by source entity.
    #[must_use]
    pub fn with_source(mut self, source: EntityId) -> Self {
        self.source = Some(source);
        self
    }

    /// Filters by target entity.
    #[must_use]
    pub fn with_target(mut self, target: EntityId) -> Self {
        self.target = Some(target);
        self
    }

    /// Filters by microsystem context.
    #[must_use]
    pub fn with_microsystem(mut self, microsystem: MicrosystemId) -> Self {
        self.microsystem = Some(microsystem);
        self
    }

    /// Checks if an event matches this filter.
    #[must_use]
    pub fn matches(&self, event: &Event) -> bool {
        // Check event types
        if let Some(ref types) = self.event_types {
            if !types.contains(&event.event_type()) {
                return false;
            }
        }

        // Check categories
        if let Some(ref categories) = self.categories {
            if !categories.contains(&event.category()) {
                return false;
            }
        }

        // Check tags (event must have at least one matching tag)
        if let Some(ref filter_tags) = self.tags {
            let has_matching_tag = filter_tags.iter().any(|tag| event.has_tag(*tag));
            if !has_matching_tag {
                return false;
            }
        }

        // Check source
        if let Some(ref source) = self.source {
            if event.source() != Some(source) {
                return false;
            }
        }

        // Check target
        if let Some(ref target) = self.target {
            if event.target() != Some(target) {
                return false;
            }
        }

        // Check microsystem
        if let Some(ref microsystem) = self.microsystem {
            if event.microsystem_context() != Some(microsystem) {
                return false;
            }
        }

        true
    }
}

/// A subscription to events matching a filter.
#[derive(Debug, Clone)]
struct Subscription {
    /// The subscription ID (used for lookup, stored for debugging).
    #[allow(dead_code)]
    id: SubscriptionId,
    /// The filter criteria for matching events.
    filter: EventFilter,
}

/// A processed event with its matched subscriptions.
///
/// When `process_pending()` is called, events are matched against all
/// subscriptions. This struct tracks which subscriptions matched each event.
#[derive(Debug, Clone)]
pub struct ProcessedEvent {
    /// The processed event.
    pub event: Event,
    /// Subscription IDs that matched this event.
    pub matched_subscriptions: Vec<SubscriptionId>,
}

/// Event bus for dispatching and receiving events.
///
/// Uses a pull-based model where events are queued and subscribers
/// poll for matching events. This avoids callback complexity and
/// enables controlled event processing.
///
/// # Usage
///
/// The `EventBus` is typically used internally by the simulation engine.
/// Events are dispatched with an explicit scope, and the bus delivers
/// them to matching subscriptions.
///
/// ```
/// use behavioral_pathways::event::{EventBus, EventBuilder};
/// use behavioral_pathways::enums::{EventType, EventScope};
///
/// let mut bus = EventBus::new();
///
/// // Dispatch an event with explicit scope
/// let event = EventBuilder::new(EventType::Violence).build().unwrap();
/// bus.dispatch(event, EventScope::Global);
/// ```
#[derive(Debug)]
pub struct EventBus {
    /// Pending events waiting to be dispatched.
    queue: VecDeque<Event>,
    /// Active subscriptions.
    subscriptions: HashMap<SubscriptionId, Subscription>,
    /// Events delivered to each subscription (subscription_id -> events).
    mailboxes: HashMap<SubscriptionId, VecDeque<Event>>,
    /// Current cascade depth for loop prevention.
    cascade_depth: usize,
}

impl EventBus {
    /// Creates a new empty event bus.
    #[must_use]
    pub fn new() -> Self {
        EventBus {
            queue: VecDeque::new(),
            subscriptions: HashMap::new(),
            mailboxes: HashMap::new(),
            cascade_depth: 0,
        }
    }

    /// Creates a subscription with the given filter.
    ///
    /// Returns a subscription ID for polling events.
    ///
    /// # Arguments
    ///
    /// * `filter` - Criteria for matching events
    ///
    /// # Returns
    ///
    /// The subscription ID for polling
    #[allow(dead_code)] // Used in unit tests
    pub(crate) fn subscribe(&mut self, filter: EventFilter) -> SubscriptionId {
        let id = generate_subscription_id();
        let subscription = Subscription {
            id: id.clone(),
            filter,
        };
        self.subscriptions.insert(id.clone(), subscription);
        self.mailboxes.insert(id.clone(), VecDeque::new());
        id
    }

    /// Removes a subscription.
    ///
    /// # Arguments
    ///
    /// * `id` - The subscription to remove
    ///
    /// # Returns
    ///
    /// True if the subscription existed and was removed
    pub fn unsubscribe(&mut self, id: &SubscriptionId) -> bool {
        self.mailboxes.remove(id);
        self.subscriptions.remove(id).is_some()
    }

    /// Dispatches an event to entities matching a scope.
    ///
    /// For Individual scope, sets the event's target before delivery.
    /// For Group/Microsystem/Global, the event is broadcast to all
    /// matching subscribers.
    ///
    /// # Arguments
    ///
    /// * `event` - The event to dispatch
    /// * `scope` - The target scope (required - be explicit about intent)
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::event::{EventBus, EventBuilder};
    /// use behavioral_pathways::enums::{EventType, EventScope};
    ///
    /// let mut bus = EventBus::new();
    ///
    /// // Broadcast to all subscribers
    /// let event = EventBuilder::new(EventType::Violence).build().unwrap();
    /// bus.dispatch(event, EventScope::Global);
    /// ```
    pub fn dispatch(&mut self, mut event: Event, scope: EventScope) {
        // For Individual scope, set the target
        if let EventScope::Individual(entity_id) = scope {
            event.set_target(Some(entity_id));
        }

        // Deliver to matching subscriptions
        self.deliver(event);
    }

    /// Internal: delivers an event to all matching subscriptions.
    fn deliver(&mut self, event: Event) {
        for (sub_id, subscription) in &self.subscriptions {
            if subscription.filter.matches(&event) {
                if let Some(mailbox) = self.mailboxes.get_mut(sub_id) {
                    mailbox.push_back(event.clone());
                }
            }
        }
    }

    /// Polls for events matching a subscription.
    ///
    /// Returns and removes all pending events from the mailbox.
    ///
    /// # Arguments
    ///
    /// * `subscription_id` - The subscription to poll
    ///
    /// # Returns
    ///
    /// All pending events for this subscription
    pub fn poll(&mut self, subscription_id: &SubscriptionId) -> Vec<Event> {
        self.mailboxes
            .get_mut(subscription_id)
            .map(|mailbox| mailbox.drain(..).collect())
            .unwrap_or_default()
    }

    /// Peeks at events without removing them.
    ///
    /// # Arguments
    ///
    /// * `subscription_id` - The subscription to peek
    ///
    /// # Returns
    ///
    /// References to pending events
    #[must_use]
    pub fn peek(&self, subscription_id: &SubscriptionId) -> Vec<&Event> {
        self.mailboxes
            .get(subscription_id)
            .map(|mailbox| mailbox.iter().collect())
            .unwrap_or_default()
    }

    /// Returns the number of pending events for a subscription.
    #[must_use]
    pub fn pending_count(&self, subscription_id: &SubscriptionId) -> usize {
        self.mailboxes
            .get(subscription_id)
            .map(|mailbox| mailbox.len())
            .unwrap_or(0)
    }

    /// Checks if dispatching another event would exceed cascade depth.
    ///
    /// # Returns
    ///
    /// True if cascade limit would be exceeded
    #[must_use]
    pub fn would_exceed_cascade_limit(&self) -> bool {
        self.cascade_depth >= MAX_CASCADE_DEPTH
    }

    /// Increments the cascade depth.
    ///
    /// # Returns
    ///
    /// Error if limit would be exceeded
    pub fn begin_cascade(&mut self) -> Result<(), EventBusError> {
        if self.cascade_depth >= MAX_CASCADE_DEPTH {
            return Err(EventBusError {
                reason: format!("Maximum cascade depth ({}) exceeded", MAX_CASCADE_DEPTH),
            });
        }
        self.cascade_depth += 1;
        Ok(())
    }

    /// Decrements the cascade depth.
    pub fn end_cascade(&mut self) {
        self.cascade_depth = self.cascade_depth.saturating_sub(1);
    }

    /// Returns the current cascade depth.
    #[must_use]
    pub fn cascade_depth(&self) -> usize {
        self.cascade_depth
    }

    /// Clears all events from all mailboxes.
    pub fn clear_all(&mut self) {
        for mailbox in self.mailboxes.values_mut() {
            mailbox.clear();
        }
        self.queue.clear();
    }

    /// Clears processed events from mailboxes after processing.
    ///
    /// This is typically called after `process_pending()` to clean up
    /// events that have been handled.
    pub fn clear_processed(&mut self) {
        for mailbox in self.mailboxes.values_mut() {
            mailbox.clear();
        }
    }

    /// Queues an event for later processing.
    ///
    /// Unlike `dispatch()`, this does not immediately deliver to mailboxes.
    /// Call `process_pending()` to match queued events against subscriptions.
    ///
    /// # Arguments
    ///
    /// * `event` - The event to queue
    pub fn queue(&mut self, event: Event) {
        self.queue.push_back(event);
    }

    /// Processes all pending events in the queue.
    ///
    /// Events are matched against ALL subscriptions. Returns a list of
    /// processed events with their matched subscription IDs.
    ///
    /// This enforces cascade depth limits - if processing would exceed
    /// MAX_CASCADE_DEPTH, an error is returned.
    ///
    /// # Returns
    ///
    /// A list of processed events with their matched subscriptions, or
    /// an error if cascade depth would be exceeded.
    pub fn process_pending(&mut self) -> Result<Vec<ProcessedEvent>, EventBusError> {
        if self.cascade_depth >= MAX_CASCADE_DEPTH {
            return Err(EventBusError {
                reason: format!("Maximum cascade depth ({}) exceeded", MAX_CASCADE_DEPTH),
            });
        }

        self.cascade_depth += 1;
        let mut results = Vec::new();

        while let Some(event) = self.queue.pop_front() {
            let mut matched = Vec::new();

            for (sub_id, subscription) in &self.subscriptions {
                if subscription.filter.matches(&event) {
                    matched.push(sub_id.clone());
                    if let Some(mailbox) = self.mailboxes.get_mut(sub_id) {
                        mailbox.push_back(event.clone());
                    }
                }
            }

            results.push(ProcessedEvent {
                event,
                matched_subscriptions: matched,
            });
        }

        self.cascade_depth = self.cascade_depth.saturating_sub(1);
        Ok(results)
    }

    /// Gets events for a specific subscription.
    ///
    /// Returns all pending events that matched this subscription.
    /// Does not remove events from the mailbox - use `poll()` to consume.
    ///
    /// # Arguments
    ///
    /// * `subscription_id` - The subscription to get events for
    ///
    /// # Returns
    ///
    /// A list of events for this subscription (cloned)
    #[must_use]
    pub fn get_events_for(&self, subscription_id: &SubscriptionId) -> Vec<Event> {
        self.mailboxes
            .get(subscription_id)
            .map(|mailbox| mailbox.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Returns the number of active subscriptions.
    #[must_use]
    pub fn subscription_count(&self) -> usize {
        self.subscriptions.len()
    }

    /// Returns true if a subscription exists.
    #[must_use]
    pub fn has_subscription(&self, id: &SubscriptionId) -> bool {
        self.subscriptions.contains_key(id)
    }

    /// Retrieves an event by ID from any mailbox.
    ///
    /// This is a convenience method for debugging/testing.
    #[must_use]
    pub fn find_event_by_id(&self, event_id: &EventId) -> Option<&Event> {
        for mailbox in self.mailboxes.values() {
            for event in mailbox {
                if event.id() == event_id {
                    return Some(event);
                }
            }
        }
        None
    }
}

impl Default for EventBus {
    fn default() -> Self {
        EventBus::new()
    }
}

/// Generates a unique subscription ID.
#[allow(dead_code)] // Used by subscribe() which is used in unit tests
fn generate_subscription_id() -> SubscriptionId {
    let uuid = Uuid::new_v4();
    SubscriptionId::new(format!("sub_{uuid}")).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::EventBuilder;
    use crate::types::GroupId;

    fn create_violence_event() -> Event {
        EventBuilder::new(EventType::Violence).build().unwrap()
    }

    fn create_support_event() -> Event {
        EventBuilder::new(EventType::Support).build().unwrap()
    }

    #[test]
    fn event_bus_subscribe_and_poll() {
        let mut bus = EventBus::new();

        let sub_id = bus.subscribe(EventFilter::new());

        let event = create_violence_event();
        bus.dispatch(event, EventScope::Global);

        let events = bus.poll(&sub_id);
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn event_bus_filter_by_type() {
        let mut bus = EventBus::new();

        let violence_sub = bus.subscribe(EventFilter::new().with_event_type(EventType::Violence));
        let support_sub = bus.subscribe(EventFilter::new().with_event_type(EventType::Support));

        bus.dispatch(create_violence_event(), EventScope::Global);

        assert_eq!(bus.poll(&violence_sub).len(), 1);
        assert_eq!(bus.poll(&support_sub).len(), 0);
    }

    #[test]
    fn event_bus_filter_by_category() {
        let mut bus = EventBus::new();

        let trauma_sub = bus.subscribe(EventFilter::new().with_category(EventCategory::Trauma));

        bus.dispatch(create_violence_event(), EventScope::Global); // Trauma
        bus.dispatch(create_support_event(), EventScope::Global); // Not trauma

        assert_eq!(bus.poll(&trauma_sub).len(), 1);
    }

    #[test]
    fn event_bus_filter_by_tag() {
        let mut bus = EventBus::new();

        let negative_sub = bus.subscribe(EventFilter::new().with_tag(EventTag::Negative));

        let negative_event = EventBuilder::new(EventType::Violence)
            .tag(EventTag::Negative)
            .build()
            .unwrap();
        let neutral_event = EventBuilder::new(EventType::Interaction).build().unwrap();

        bus.dispatch(negative_event, EventScope::Global);
        bus.dispatch(neutral_event, EventScope::Global);

        assert_eq!(bus.poll(&negative_sub).len(), 1);
    }

    #[test]
    fn event_bus_filter_by_tags_vector() {
        let mut bus = EventBus::new();

        let tagged_sub =
            bus.subscribe(EventFilter::new().with_tags(vec![EventTag::Positive, EventTag::Social]));

        let positive_event = EventBuilder::new(EventType::Support)
            .tag(EventTag::Positive)
            .build()
            .unwrap();
        let untagged_event = EventBuilder::new(EventType::Support).build().unwrap();

        bus.dispatch(positive_event, EventScope::Global);
        bus.dispatch(untagged_event, EventScope::Global);

        assert_eq!(bus.poll(&tagged_sub).len(), 1);
    }

    #[test]
    fn event_bus_filter_by_source() {
        let mut bus = EventBus::new();

        let source = EntityId::new("attacker").unwrap();
        let source_sub = bus.subscribe(EventFilter::new().with_source(source.clone()));

        let with_source = EventBuilder::new(EventType::Violence)
            .source(source)
            .build()
            .unwrap();
        let without_source = EventBuilder::new(EventType::Violence).build().unwrap();

        bus.dispatch(with_source, EventScope::Global);
        bus.dispatch(without_source, EventScope::Global);

        assert_eq!(bus.poll(&source_sub).len(), 1);
    }

    #[test]
    fn event_bus_filter_by_target() {
        let mut bus = EventBus::new();

        let target = EntityId::new("victim").unwrap();
        let target_sub = bus.subscribe(EventFilter::new().with_target(target.clone()));

        let with_target = EventBuilder::new(EventType::Violence)
            .target(target)
            .build()
            .unwrap();
        let without_target = EventBuilder::new(EventType::Violence).build().unwrap();

        bus.dispatch(with_target, EventScope::Global);
        bus.dispatch(without_target, EventScope::Global);

        assert_eq!(bus.poll(&target_sub).len(), 1);
    }

    #[test]
    fn event_bus_unsubscribe() {
        let mut bus = EventBus::new();

        let sub_id = bus.subscribe(EventFilter::new());
        assert!(bus.has_subscription(&sub_id));

        let removed = bus.unsubscribe(&sub_id);
        assert!(removed);
        assert!(!bus.has_subscription(&sub_id));
    }

    #[test]
    fn deliver_skips_missing_mailbox() {
        let mut bus = EventBus::new();
        let sub_id = bus.subscribe(EventFilter::new());

        bus.mailboxes.remove(&sub_id);
        bus.dispatch(create_violence_event(), EventScope::Global);

        assert_eq!(bus.pending_count(&sub_id), 0);
    }

    #[test]
    fn event_bus_peek_does_not_remove() {
        let mut bus = EventBus::new();

        let sub_id = bus.subscribe(EventFilter::new());
        bus.dispatch(create_violence_event(), EventScope::Global);

        // Peek
        let peeked = bus.peek(&sub_id);
        assert_eq!(peeked.len(), 1);

        // Still there after peek
        let polled = bus.poll(&sub_id);
        assert_eq!(polled.len(), 1);

        // Now gone
        assert_eq!(bus.poll(&sub_id).len(), 0);
    }

    #[test]
    fn event_bus_pending_count() {
        let mut bus = EventBus::new();

        let sub_id = bus.subscribe(EventFilter::new());
        assert_eq!(bus.pending_count(&sub_id), 0);

        bus.dispatch(create_violence_event(), EventScope::Global);
        assert_eq!(bus.pending_count(&sub_id), 1);

        bus.dispatch(create_support_event(), EventScope::Global);
        assert_eq!(bus.pending_count(&sub_id), 2);
    }

    #[test]
    fn event_bus_cascade_depth() {
        let mut bus = EventBus::new();

        assert_eq!(bus.cascade_depth(), 0);

        bus.begin_cascade().unwrap();
        assert_eq!(bus.cascade_depth(), 1);

        bus.end_cascade();
        assert_eq!(bus.cascade_depth(), 0);
    }

    #[test]
    fn event_bus_cascade_limit() {
        let mut bus = EventBus::new();

        for _ in 0..MAX_CASCADE_DEPTH {
            bus.begin_cascade().unwrap();
        }

        // Should fail on the next one
        let result = bus.begin_cascade();
        assert!(result.is_err());
    }

    #[test]
    fn event_bus_would_exceed_cascade_limit() {
        let mut bus = EventBus::new();

        assert!(!bus.would_exceed_cascade_limit());

        for _ in 0..MAX_CASCADE_DEPTH {
            bus.begin_cascade().unwrap();
        }

        assert!(bus.would_exceed_cascade_limit());
    }

    #[test]
    fn event_bus_clear_all() {
        let mut bus = EventBus::new();

        let sub_id = bus.subscribe(EventFilter::new());
        bus.dispatch(create_violence_event(), EventScope::Global);
        bus.dispatch(create_support_event(), EventScope::Global);

        assert_eq!(bus.pending_count(&sub_id), 2);

        bus.clear_all();
        assert_eq!(bus.pending_count(&sub_id), 0);
    }

    #[test]
    fn event_bus_subscription_count() {
        let mut bus = EventBus::new();

        assert_eq!(bus.subscription_count(), 0);

        let sub1 = bus.subscribe(EventFilter::new());
        assert_eq!(bus.subscription_count(), 1);

        let _ = bus.subscribe(EventFilter::new());
        assert_eq!(bus.subscription_count(), 2);

        bus.unsubscribe(&sub1);
        assert_eq!(bus.subscription_count(), 1);
    }

    #[test]
    fn dispatch_individual_scope() {
        let mut bus = EventBus::new();

        let entity = EntityId::new("person_001").unwrap();
        let sub_id = bus.subscribe(EventFilter::new().with_target(entity.clone()));

        let event = create_violence_event();
        bus.dispatch(event, EventScope::Individual(entity));

        let events = bus.poll(&sub_id);
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn dispatch_global_scope() {
        let mut bus = EventBus::new();

        let sub_id = bus.subscribe(EventFilter::new());

        let event = create_violence_event();
        bus.dispatch(event, EventScope::Global);

        let events = bus.poll(&sub_id);
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn event_filter_default_matches_all() {
        let filter = EventFilter::new();
        let event = create_violence_event();
        assert!(filter.matches(&event));
    }

    #[test]
    fn event_filter_multiple_criteria() {
        let source = EntityId::new("source").unwrap();
        let target = EntityId::new("target").unwrap();

        let filter = EventFilter::new()
            .with_event_type(EventType::Violence)
            .with_source(source.clone())
            .with_target(target.clone());

        let matching = EventBuilder::new(EventType::Violence)
            .source(source)
            .target(target)
            .build()
            .unwrap();

        let wrong_type = EventBuilder::new(EventType::Support).build().unwrap();

        assert!(filter.matches(&matching));
        assert!(!filter.matches(&wrong_type));
    }

    #[test]
    fn event_filter_multiple_types() {
        let filter =
            EventFilter::new().with_event_types(vec![EventType::Violence, EventType::Conflict]);

        let violence = create_violence_event();
        let conflict = EventBuilder::new(EventType::Conflict).build().unwrap();
        let support = create_support_event();

        assert!(filter.matches(&violence));
        assert!(filter.matches(&conflict));
        assert!(!filter.matches(&support));
    }

    #[test]
    fn event_filter_multiple_categories() {
        let filter =
            EventFilter::new().with_categories(vec![EventCategory::Trauma, EventCategory::Social]);

        let violence = create_violence_event(); // Trauma
        let support = create_support_event(); // Social
        let exclusion = EventBuilder::new(EventType::SocialExclusion)
            .build()
            .unwrap(); // SocialBelonging

        assert!(filter.matches(&violence));
        assert!(filter.matches(&support));
        assert!(!filter.matches(&exclusion));
    }

    #[test]
    fn event_bus_find_event_by_id() {
        let mut bus = EventBus::new();
        let _sub_id = bus.subscribe(EventFilter::new());

        let event = create_violence_event();
        let event_id = event.id().clone();

        bus.dispatch(event, EventScope::Global);

        let found = bus.find_event_by_id(&event_id);
        assert!(found.is_some());

        let not_found_id = EventId::new("nonexistent").unwrap();
        assert!(bus.find_event_by_id(&not_found_id).is_none());
    }

    #[test]
    fn event_bus_default() {
        let bus = EventBus::default();
        assert_eq!(bus.subscription_count(), 0);
        assert_eq!(bus.cascade_depth(), 0);
    }

    #[test]
    fn event_bus_error_display() {
        let error = EventBusError {
            reason: "test error".to_string(),
        };
        let display = format!("{}", error);
        assert!(display.contains("test error"));
    }

    #[test]
    fn event_filter_by_microsystem() {
        let mut bus = EventBus::new();

        let work = MicrosystemId::new("work").unwrap();
        let work_sub = bus.subscribe(EventFilter::new().with_microsystem(work.clone()));

        let work_event = EventBuilder::new(EventType::Interaction)
            .context(work)
            .build()
            .unwrap();
        let home_event = EventBuilder::new(EventType::Interaction)
            .context(MicrosystemId::new("home").unwrap())
            .build()
            .unwrap();

        bus.dispatch(work_event, EventScope::Global);
        bus.dispatch(home_event, EventScope::Global);

        assert_eq!(bus.poll(&work_sub).len(), 1);
    }

    #[test]
    fn event_filter_clone() {
        let filter = EventFilter::new().with_event_type(EventType::Violence);
        let cloned = filter.clone();
        assert_eq!(filter, cloned);
    }

    #[test]
    fn poll_unknown_subscription_returns_empty() {
        let mut bus = EventBus::new();
        let fake_id = SubscriptionId::new("unknown").unwrap();

        let events = bus.poll(&fake_id);
        assert!(events.is_empty());
    }

    #[test]
    fn peek_unknown_subscription_returns_empty() {
        let bus = EventBus::new();
        let fake_id = SubscriptionId::new("unknown").unwrap();

        let events = bus.peek(&fake_id);
        assert!(events.is_empty());
    }

    #[test]
    fn pending_count_unknown_subscription_returns_zero() {
        let bus = EventBus::new();
        let fake_id = SubscriptionId::new("unknown").unwrap();

        assert_eq!(bus.pending_count(&fake_id), 0);
    }

    #[test]
    fn unsubscribe_unknown_returns_false() {
        let mut bus = EventBus::new();
        let fake_id = SubscriptionId::new("unknown").unwrap();

        assert!(!bus.unsubscribe(&fake_id));
    }

    #[test]
    fn dispatch_group_scope() {
        let mut bus = EventBus::new();
        let sub_id = bus.subscribe(EventFilter::new());

        let group = GroupId::new("team").unwrap();
        let event = create_violence_event();
        bus.dispatch(event, EventScope::Group(group));

        // Should still receive the event (broadcast)
        assert_eq!(bus.poll(&sub_id).len(), 1);
    }

    #[test]
    fn dispatch_microsystem_scope() {
        let mut bus = EventBus::new();
        let sub_id = bus.subscribe(EventFilter::new());

        let micro = MicrosystemId::new("work").unwrap();
        let event = create_violence_event();
        bus.dispatch(event, EventScope::Microsystem(micro));

        // Should still receive the event (broadcast)
        assert_eq!(bus.poll(&sub_id).len(), 1);
    }

    #[test]
    fn max_cascade_depth_constant() {
        assert_eq!(MAX_CASCADE_DEPTH, 5);
    }

    #[test]
    fn queue_and_process_pending() {
        let mut bus = EventBus::new();
        let sub_id = bus.subscribe(EventFilter::new());

        let event = create_violence_event();
        bus.queue(event);

        // Event should not be in mailbox yet
        assert_eq!(bus.pending_count(&sub_id), 0);

        // Process queued events
        let processed = bus.process_pending().unwrap();
        assert_eq!(processed.len(), 1);
        assert_eq!(processed[0].matched_subscriptions.len(), 1);
        assert_eq!(processed[0].matched_subscriptions[0], sub_id);

        // Now should be in mailbox
        assert_eq!(bus.pending_count(&sub_id), 1);
    }

    #[test]
    fn process_pending_matches_multiple_subscriptions() {
        let mut bus = EventBus::new();
        let sub1 = bus.subscribe(EventFilter::new());
        let sub2 = bus.subscribe(EventFilter::new().with_event_type(EventType::Violence));

        bus.queue(create_violence_event());
        let processed = bus.process_pending().unwrap();

        assert_eq!(processed.len(), 1);
        assert_eq!(processed[0].matched_subscriptions.len(), 2);
        assert!(processed[0].matched_subscriptions.contains(&sub1));
        assert!(processed[0].matched_subscriptions.contains(&sub2));
    }

    #[test]
    fn process_pending_skips_non_matching_subscriptions() {
        let mut bus = EventBus::new();
        let violence_sub = bus.subscribe(EventFilter::new().with_event_type(EventType::Violence));
        let support_sub = bus.subscribe(EventFilter::new().with_event_type(EventType::Support));

        bus.queue(create_violence_event());
        let processed = bus.process_pending().unwrap();

        assert_eq!(processed.len(), 1);
        assert!(processed[0].matched_subscriptions.contains(&violence_sub));
        assert!(!processed[0].matched_subscriptions.contains(&support_sub));
        assert_eq!(bus.pending_count(&violence_sub), 1);
        assert_eq!(bus.pending_count(&support_sub), 0);
    }

    #[test]
    fn process_pending_skips_missing_mailbox() {
        let mut bus = EventBus::new();
        let sub_id = bus.subscribe(EventFilter::new());

        bus.mailboxes.remove(&sub_id);
        bus.queue(create_violence_event());

        let processed = bus.process_pending().unwrap();
        assert_eq!(processed.len(), 1);
        assert!(processed[0].matched_subscriptions.contains(&sub_id));
        assert_eq!(bus.pending_count(&sub_id), 0);
    }

    #[test]
    fn process_pending_respects_cascade_depth() {
        let mut bus = EventBus::new();

        // Max out cascade depth
        for _ in 0..MAX_CASCADE_DEPTH {
            bus.begin_cascade().unwrap();
        }

        bus.queue(create_violence_event());
        let result = bus.process_pending();
        assert!(result.is_err());
    }

    #[test]
    fn get_events_for_returns_matching_events() {
        let mut bus = EventBus::new();
        let sub_id = bus.subscribe(EventFilter::new());

        bus.dispatch(create_violence_event(), EventScope::Global);
        bus.dispatch(create_support_event(), EventScope::Global);

        let events = bus.get_events_for(&sub_id);
        assert_eq!(events.len(), 2);
    }

    #[test]
    fn get_events_for_does_not_consume() {
        let mut bus = EventBus::new();
        let sub_id = bus.subscribe(EventFilter::new());

        bus.dispatch(create_violence_event(), EventScope::Global);

        // Get events twice - should return same results
        let events1 = bus.get_events_for(&sub_id);
        let events2 = bus.get_events_for(&sub_id);
        assert_eq!(events1.len(), events2.len());

        // Also still available via poll
        assert_eq!(bus.poll(&sub_id).len(), 1);
    }

    #[test]
    fn get_events_for_unknown_subscription() {
        let bus = EventBus::new();
        let fake_id = SubscriptionId::new("unknown").unwrap();
        assert!(bus.get_events_for(&fake_id).is_empty());
    }

    #[test]
    fn clear_processed_clears_mailboxes() {
        let mut bus = EventBus::new();
        let sub_id = bus.subscribe(EventFilter::new());

        bus.dispatch(create_violence_event(), EventScope::Global);
        assert_eq!(bus.pending_count(&sub_id), 1);

        bus.clear_processed();
        assert_eq!(bus.pending_count(&sub_id), 0);
    }

    #[test]
    fn processed_event_debug_and_clone() {
        let event = create_violence_event();
        let processed = ProcessedEvent {
            event,
            matched_subscriptions: vec![SubscriptionId::new("sub_1").unwrap()],
        };

        let cloned = processed.clone();
        assert_eq!(cloned.matched_subscriptions.len(), 1);

        let debug = format!("{:?}", processed);
        assert!(debug.contains("ProcessedEvent"));
    }
}
