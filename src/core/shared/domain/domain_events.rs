use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;
use uuid::Uuid;

pub type DomainEventAttributes = JsonValue;

#[derive(Debug, Clone)]
pub struct DomainEventBase {
    pub aggregate_id: String,
    pub event_id: String,
    pub occurred_on: DateTime<Utc>,
    pub event_name: String,
}

impl DomainEventBase {
    pub fn new(
        aggregate_id: String,
        event_name: String,
        event_id: Option<String>,
        occurred_on: Option<DateTime<Utc>>,
    ) -> Self {
        DomainEventBase {
            aggregate_id,
            event_name,
            event_id: event_id.unwrap_or_else(|| Uuid::new_v4().to_string()),
            occurred_on: occurred_on.unwrap_or_else(Utc::now),
        }
    }
}

pub trait DomainEvent {
    fn base(&self) -> &DomainEventBase;
    fn to_primitives(&self) -> DomainEventAttributes;
}

