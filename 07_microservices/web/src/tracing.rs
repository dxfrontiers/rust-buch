use rocket::fairing::{Fairing, Info, Kind, Result};
use rocket::{Rocket, Build};
use opentelemetry::global;

pub struct TracingFairing {}

#[rocket::async_trait]
impl Fairing for TracingFairing {
    fn info(&self) -> Info {
        let kind = Kind::Ignite;

        Info { kind, name: "Tracing" }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result {

        global::set_text_map_propagator(opentelemetry_zipkin::Propagator::new());

        let _tracer = opentelemetry_zipkin::new_pipeline()
            .with_service_name("web")
            .with_collector_endpoint("http://zipkin:9411/api/v2/spans")
            .install_simple()
            .expect("Could not install tracer");
        Ok(rocket)
    }
}