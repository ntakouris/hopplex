#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rocket::http::Status;

mod contracts;
use contracts::request::events::publish_event_request::{PublishSingleValueEventRequest, EventValue};

// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }

#[post("/publish_event", format="application/json", data="<request>")]
pub fn publish_event(request: Json<PublishSingleValueEventRequest>) -> Status {
    let event = request.0;
    
    // validate user 
    // validate source_id, key, tag

    let evt_val: Option = match event.manifest {
        EventValue::String(x) => x.value,
        EventValue::Float(x) => x.value,
        EventValue::Int(x) => x.value,
        EventValue::Bool(x) => x.value
    };

    match evt_val {
        Some(x) => println!("Event value: {:?}", x),
        None => println!("No event value provided")
    }

    println!("{:?}", event.manifest);
    match event.manifest {
        EventValue::String(x) => {
            println!("String payload: {:?}", x.value);
        }

        EventValue::Float(x) => {
            println!("Float payload: {:?}", x.value);
        }

        EventValue::Int(x) => {
            println!("Int payload: {:?}", x.value);
        }
        
        EventValue::Bool(x) => {
            println!("Bool payload: {:?}", x.value);
        }
    }

    Status::Accepted
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![publish_event])
}