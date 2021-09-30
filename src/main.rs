use std::collections::HashMap;
use std::sync::Mutex;

use uuid::Uuid;

#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rocket::State;

mod contracts;
use contracts::request::events::publish_event_request::{PublishSingleValueEventRequest, EventValue};

mod model;
use model::resource::{DataResource, DataResourcePayload, CaptionedImagePayload};

mod db;
use db::db::DB;
use db::in_memory::InMemoryDB;

#[post("/publish_event", format="application/json", data="<request>")]
pub fn publish_event(request: Json<PublishSingleValueEventRequest>, db: &State<Box<dyn DB>>) -> Result<(), String> {
    let event = request.0;

    println!("{:?}", event.value);

    // alternatively, return a mutable DataResource
    // and map the id and other metadata afterwards
    let resource = match event.value {
        EventValue::String(x) => DataResource{ id: event.id, value: DataResourcePayload::String(x)},
        EventValue::Float(x) => DataResource{ id: event.id, value: DataResourcePayload::Float(x)},
        EventValue::Int(x) => DataResource{ id: event.id, value: DataResourcePayload::Int(x)},
        EventValue::Bool(x) => DataResource{ id: event.id, value: DataResourcePayload::Bool(x)},

        EventValue::CaptionedImage(x) => {
            // assume this connects to some blob storage service to store the image and receive a key back
            println!("Saving image b64 encoded: {} to blob storage", x.contents_b64);
            let ref_id = Uuid::new_v4().to_string();

            DataResource { 
                id: event.id,
                value: DataResourcePayload::CaptionedImage(
                    CaptionedImagePayload { caption: x.caption, blob_storage_ref_id: ref_id }
                )
            }
        }
    };

    match db.save(event.id, resource) {
        Ok(_) => Result::Ok(()),
        Err(x) => Result::Err(x)
    }
}

#[get("/retrieve_event/<id>")]
pub fn retrieve_event(id: String, db: &State<Box<dyn DB>>) -> Result<Json<DataResourcePayload>, Option<String>> {
    match db.retrieve(id) {
        Ok(c) => match c {
            Some(x) => Result::Ok(Json(x.value)),
            None => Result::Err(None)
        },
        Err(x) => Result::Err(Some(x))
    }
}

#[launch]
fn rocket() -> _ {
    let db_inst = Box::new(InMemoryDB{ map: HashMap::new()}) as Box<dyn DB>;

    rocket::build()
        .manage(db_inst)
        .mount("/", routes![publish_event, retrieve_event])
}
