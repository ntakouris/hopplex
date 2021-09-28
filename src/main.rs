#[macro_use] extern crate rocket;
use rocket::serde::json::Json;

mod contracts;
use contracts::request::events::publish_event_request::PublishSingleValueEventRequest;

// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }

#[post("/publish_event", format="json", data="<request>")]
pub fn publish_event(request: Json<PublishSingleValueEventRequest>) -> &'static str {
    
    "OK"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/publish_event", routes![publish_event])
}