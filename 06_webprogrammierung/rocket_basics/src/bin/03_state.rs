#[macro_use] extern crate rocket;

use serde::{Deserialize, Serialize};

use rocket::{State};

use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Serialize, Deserialize)]
struct MyCounter(AtomicU64);
impl MyCounter{
    fn get_and_inc(&self) -> u64 {
        self.0.fetch_add(1,Ordering::SeqCst)
    }
}


#[get("/")]
fn get_shared_counter(state: &State<MyCounter>) -> String {
    let result = state.get_and_inc();
    format!("ctr: {}\n",result)
}



#[launch]
fn entry_point() -> _ {
    let counter = MyCounter(AtomicU64::default());
    rocket::build()
        .manage(counter)
        .mount("/", routes![
        get_shared_counter,
    ])
}
