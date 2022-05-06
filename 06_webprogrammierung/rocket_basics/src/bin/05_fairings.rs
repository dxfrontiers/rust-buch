#[macro_use] extern crate rocket;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Response, Request};

struct MyFairing;

#[rocket::async_trait]
impl Fairing for MyFairing{
    fn info(&self) -> Info {
        Info { name: "...", kind: Kind::Response }
    }
    async fn on_response<'r>(
        &self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_raw_header("X-MyFairing", "Foo");
    }
}

#[get("/")]
fn get_test() -> String {
    "Test".into()
}

#[launch]
fn entry_point() -> _ {
        rocket::build()
            .attach(MyFairing)
        .mount("/", routes![
        get_test,
    ])
}
