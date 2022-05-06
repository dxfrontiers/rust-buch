#[macro_use] extern crate rocket;

use serde::{Deserialize};
use rocket::serde::{Serialize, json::Json};
use rocket::response::Responder;
use rocket::{Request, response, Response};
use rocket::http::Status;
use rocket::response::content;
use rocket::response::status::{Created};

use std::io::Cursor;

#[derive(Serialize, Deserialize)]
struct Item{
    id: u32,
    name: String
}

#[derive(Serialize, Deserialize)]
struct Item2{
    id: u32,
    name: String
}

enum ItemError{
    ItemLocked,
    ImATeapot,
}

impl<'r> Responder<'r, 'static> for Item2 {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        content::Plain(format!("Item2 hat ID {}",self.id)).respond_to(req)
    }
}
impl<'r> Responder<'r, 'static> for ItemError {
    fn respond_to(self, _req: &'r Request<'_>) -> response::Result<'static> {
        let (status, comment) = match self {
            ItemError::ItemLocked => (Status::Locked, "Seems to be locked"),
            ItemError::ImATeapot => (Status::ImATeapot, "We also support RFC 1149")
        };
        Response::build()
            .status(status)
            .sized_body(comment.len(), Cursor::new(comment))
            .ok()
    }
}

#[get("/item/<id>")]
fn get_item_handler_json(id: u32) -> Json<Item> {
    let item = Item{id: id, name: "Hans".into()};
    Json(item)
}
#[get("/item2/<id>")]
fn get_item_handler_diy(id: u32) -> Item2 {
    Item2{id: id, name: "Hans".into()}
}

#[post("/item", format = "json", data = "<item>")]
fn post_item_handler_created(item: Json<Item>) -> Created<Json<Item>> {
    Created::new("http://example.com").body(item)
}

#[get("/item3/<id>")]
fn get_item_handler_option(id: u32) -> Option<Json<Item>>  {
    match id {
        7 => Some(Json(Item{id: id, name: "Hans".into()})),
        _ => None
    }
}

#[get("/item4/<id>")]
fn get_item_handler_err1(id: u32) -> Result<Item2, Status> {
    match id {
        7 => Ok(Item2{id: id, name: "Hans".into()}),
        _ => Err(Status::ImATeapot)
    }
}

#[get("/item5/<id>")]
fn get_item_handler_err2(id: u32) -> Result<Option<Json<Item>>,ItemError> {
    match id {
        7 => Ok(Some(Json(Item{id: id, name: "Hans".into()}))),
        _ => Err(ItemError::ImATeapot)
    }
}



#[launch]
fn entry_point() -> _ {
    rocket::build().mount("/", routes![
        get_item_handler_json,
        get_item_handler_diy,
        post_item_handler_created,
        get_item_handler_option,
        get_item_handler_err1,
        get_item_handler_err2
    ])
}
