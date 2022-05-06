use rocket::{launch, Rocket, Build};
use rocket::routes;

use crate::monitoring::Prometheus;
use crate::kafka::KafkaResponse;
use crate::tracing::TracingFairing;


mod model;
mod routes;
mod monitoring;
mod kafka;
mod tracing;

#[macro_use]
extern crate log;

#[launch]
fn rocket() -> Rocket<Build> {

    env_logger::Builder::from_default_env().init();
    info!("Logger configured");

    let prometheus = Prometheus::new();

    rocket::build()
        .attach(TracingFairing {})
        .attach(KafkaResponse::fairing())
        // .attach(Template::fairing())
        .attach(prometheus.clone())
        .mount(
            "/",
            routes![
                routes::index_json,
                routes::health
            ],
        )
        .mount("/metrics", prometheus)
}

