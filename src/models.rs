use serde::{Deserialize, Serialize};

/// An event to prefill in the system event editor; times are Unix epoch milliseconds.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEvent {
    pub title: String,
    pub start: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_day: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<Recurrence>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Frequency {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

/// Repeats the event; ends after `count` occurrences or at `until` (epoch milliseconds), otherwise forever.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Recurrence {
    pub frequency: Frequency,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AddEventResult {
    /// Whether the user saved the event; only known on iOS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saved: Option<bool>,
}
