use uuid::Uuid;

#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rocket::State;

mod contracts;
use contracts::events::{CaptionedImageValue, EventValue, EventEntity};

mod model;
use model::resource::{DataResource, DataResourcePayload, CaptionedImagePayload};

mod db;
use db::db::DB;
use db::in_memory::InMemoryDB;

#[post("/publish_event", format="application/json", data="<request>")]
pub fn publish_event(request: Json<EventEntity>, db: &State<Box<dyn DB>>) -> Result<(), String> {
    let event = request.0;

    /* map EventEntity -> DataResource */

    // alternatively, return a mutable DataResource
    // and map the id and other metadata afterwards
    let resource = match event.value {
        EventValue::String(x) => DataResource{ id: event.id, value: DataResourcePayload::String(x)},
        EventValue::Float(x) => DataResource{ id: event.id, value: DataResourcePayload::Float(x)},
        EventValue::Int(x) => DataResource{ id: event.id, value: DataResourcePayload::Int(x)},
        EventValue::Bool(x) => DataResource{ id: event.id, value: DataResourcePayload::Bool(x)},

        EventValue::CaptionedImage(x) => {
            // save to blob storage
            let ref_id = Uuid::new_v4().to_string();

            DataResource { 
                id: event.id,
                value: DataResourcePayload::CaptionedImage(
                    CaptionedImagePayload { caption: x.caption, blob_storage_ref_id: ref_id }
                )
            }
        }
    };

    match db.save(resource) {
        Ok(_) => Result::Ok(()),
        Err(x) => Result::Err(x)
    }
}

#[get("/retrieve_event/<id>")]
pub fn retrieve_event(id: String, db: &State<Box<dyn DB>>) -> Result<Json<EventEntity>, Option<String>> {
    match db.retrieve(&id) {
        Ok(c) => match c {
            Some(d) => {
                /* map DataResource -> EventEntity */
                let ret = match d.value {
                    DataResourcePayload::String(x) => EventEntity{ id: d.id, value: EventValue::String(x)},
                    DataResourcePayload::Float(x) => EventEntity{ id: d.id, value: EventValue::Float(x)},
                    DataResourcePayload::Int(x) => EventEntity{ id: d.id, value: EventValue::Int(x)},
                    DataResourcePayload::Bool(x) => EventEntity{ id: d.id, value: EventValue::Bool(x)},
                    DataResourcePayload::CaptionedImage(x) => {
                        // fetch from blob storage
                        let img_b64 = "<get from blob storage>".into();

                        EventEntity { 
                            id: d.id, 
                            value: EventValue::CaptionedImage(
                                CaptionedImageValue {
                                    caption: x.caption,
                                    contents_b64: img_b64
                                }
                            )
                        }
                    }
                };

                Result::Ok(Json(ret))
            },
            None => Result::Err(None)
        },
        Err(x) => Result::Err(Some(x))
    }
}

#[launch]
fn rocket() -> _ {
    let db_inst = InMemoryDB::new();
    let db_state = Box::new(db_inst) as Box<dyn DB>;

    rocket::build()
        .manage(db_state)
        .mount("/", routes![publish_event, retrieve_event])
}
