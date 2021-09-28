use std::option::{Option};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
enum ValidPayloadType {
    String,
    F32,
    I32,
    Bool
}

#[derive(Debug, Serialize, Deserialize)]
struct EventPayload{
    key: String,
    tag: Option<String>,
    value: Option<ValidPayloadType>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublishSingleValueEventRequest  {
    source_id: String,
    payload: EventPayload
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublishMultiValueEventRequest {
    source_id: String,
    payloads: Option<EventPayload>
}
