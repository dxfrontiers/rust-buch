#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use persistence::DbCon;
use crate::auth::SessionStorage;
use rocket_dyn_templates::Template;

mod model;
mod persistence;
mod routes;
mod schema;
mod auth;
mod catchers;

/**
    Start the rocket
    The `launch` macro makes this method asynchronous and spawns it in a worker.
    Further, the macro takes care, that the function is called from the main function.

    The SessionStore is used for storing Sessions for cookie-based authentication and acts as a
        demonstration of how to use managed state. Since Rocket requires its managed State to be
        thread-safe, session-store uses an RwLock internally an can be used from any thread.
    The PgDbCon fairing acts as a wrapper around diesels PgConnection but has a annotation from
        rocket, so that it can be configured by the Rocket.toml configuration file.
    The routes mounted are implemented in their respective module files for maintainability.
    The single implemented custom error catcher shal demonstrate how this feature works in rocket.
*/

#[launch]
fn launch() -> _ {

    let session_store = SessionStorage::new();

    rocket::build()
        .attach(DbCon::fairing())
        .attach(Template::fairing())
        .manage(session_store)
        .mount(
            "/public/",
            routes![
                routes::public::index_json,
                routes::public::index_template,
                routes::public::index_form,
            ],
        )
        .mount(
            "/admin/",
            routes![
                routes::admin::list_requests,
                routes::admin::get_request_by_id,
                routes::admin::delete_request_by_id,
                routes::admin::update_request,
                routes::admin::create_request,
                routes::admin::list_requests_template,
            ],
        )
        .mount(
            "/auth/",
            routes![
                auth::login_template,
                auth::logout_template,
                auth::login_form,
            ],
        )

        /*
         * Register is used to register catchers in the Rocket instance.
         * Catchers are handlers that are invoked, when errors occur.
         * Catchers will be invoked, if no other handlers match (e.g. 404) or some handler
         * returned some Ourtcome other than Success.
         * Returning an Outcome::Failure or Outcome::Forward and handling them with catchers
         * is extremely powerfull, since one can externalize Error handling from the "real"
         * handlers.
         *
         * The catchers! macro creates a Vec of Catchers with their respective routes.
         * The routes for each catcher can be defined by using the mcaro #[catch(...)]
         */
        .register(
            "/",catchers![catchers::unauthorized]
        )
}

#[cfg(test)]
mod test {
    use super::launch;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn hello_world() {
        let client = Client::tracked(launch()).expect("valid rocket instance");
        let mut response = client.get("/public").dispatch();
        assert_eq!(response.status(), Status::Ok);
        insta::assert_yaml_snapshot!(response.into_string().unwrap());
    }
}
