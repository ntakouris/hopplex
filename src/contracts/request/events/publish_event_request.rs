use std::option::{Option};
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct EventPayload<T>{
    pub key: String,
    pub tag: Option<String>,
    pub value: Option<T>
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum EventValue {
    String(EventPayload<String>),
    Float(EventPayload<f32>),
    Int(EventPayload<i32>),
    Bool(EventPayload<bool>)
}

#[derive(Deserialize, Debug)]
pub struct PublishSingleValueEventRequest {
    pub source_id: String,
    #[serde(flatten)]
    pub manifest: EventValue
}