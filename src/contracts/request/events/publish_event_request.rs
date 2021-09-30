use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CaptionedImage {
    pub caption: String,
    pub contents_b64: String
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum EventValue {
    String(String),
    Float(f32),
    Int(i32),
    Bool(bool),
    CaptionedImage(CaptionedImage)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublishSingleValueEventRequest {
    // whatever other metadata you need
    pub id: String,
    // flattening is optional, but we don't want to wrap 2 times
    pub value: EventValue
}