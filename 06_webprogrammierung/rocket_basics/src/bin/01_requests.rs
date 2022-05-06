#[macro_use] extern crate rocket;

// use serde::{Deserialize};
use rocket::serde::{Serialize, Deserialize, json::Json};

#[derive(Serialize, Deserialize)]
struct Item{
    id: u32,
    name: String
}

#[get("/hello")]
fn my_handler() -> String {
    "world".into()
}

#[get("/item/<id>")]
fn get_item_handler_path(id: u32) -> String {
    format!("item: {}", id)
}

#[get("/item?<id>")]
fn get_item_handler_param(id: u32) -> String {
    format!("item: {}", id)
}

#[get("/search?<start>")]
fn search_handler_param_optional(start: Option<u32>) -> String {
    format!("start: {:?}", start)
}

#[post("/item", format = "json", data = "<item>")]
fn post_item_handler(item: Json<Item>) -> String {
    format!("item_id: {}", item.id)
}

#[launch]
fn entry_point() -> _ {
    rocket::build().mount("/", routes![
        my_handler,
        get_item_handler_path,
        get_item_handler_param,
        search_handler_param_optional,
        post_item_handler
    ])
}
