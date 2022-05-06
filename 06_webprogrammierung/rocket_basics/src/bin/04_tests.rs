#[macro_use] extern crate rocket;

use serde::{Deserialize, Serialize};

use std::sync::atomic::{AtomicU64, Ordering};
use rocket::State;

struct MyCounter(AtomicU64);
impl MyCounter{
    fn get_and_inc(&self) -> u64 {
        self.0.fetch_add(1,Ordering::SeqCst)
    }
}


#[derive(Serialize, Deserialize)]
struct Item{
    id: u32,
    name: String
}

#[get("/item/<id>")]
fn get_item_handler_path(id: u32) -> String {
    format!("item: {}", id)
}

#[get("/state")]
fn get_shared_counter( ctr: &State<MyCounter>) -> String {
    format!("ctr: {}", ctr.inner().get_and_inc())
}


#[launch]
fn entry_point() -> _ {
    let counter = MyCounter(AtomicU64::default());
    rocket::build()
        .manage(counter)
        .mount("/", routes![
        get_item_handler_path,
    ])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::{Status};
    use crate::{entry_point, get_item_handler_path, MyCounter, get_shared_counter};
    use std::sync::atomic::AtomicU64;
    use rocket::State;


    #[test]
    fn test_get_item_handler_path_returns_id_as_string() {
        let client = Client::tracked(entry_point())
            .expect("rocket test");
        let response = client.get("/item/99").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some("item: 99".into()));

    }
    #[test]
    fn test_get_item_handler_path_directly() {
        let result = get_item_handler_path(7);
        assert_eq!(result, "item: 7");
    }

    #[test]
    fn test_get_with_state() {
let ctr = MyCounter(AtomicU64::new(7));
// let rocket =
// rocket::custom(rocket::Config::default())
//     .manage(ctr);

let state = State::from(&ctr);
let result_a = get_shared_counter(state);
assert_eq!(result_a, "ctr: 7");

let state = State::from(&ctr);
let result_b = get_shared_counter(state);
assert_eq!(result_b, "ctr: 8");
    }
}
