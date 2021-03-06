use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CaptionedImagePayload {
    pub caption: String,
    pub blob_storage_ref_id: String 
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PrimitiveValuePayload<T> {
    pub value: T
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DataResourcePayload {
    String(String),
    Float(f32),
    Int(i32),
    Bool(bool),
    CaptionedImage(CaptionedImagePayload)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataResource {
    // whatever other metadata you need
    pub id: String,

    // flattening is optional, but we don't want to box 2 times
    pub value: DataResourcePayload
}