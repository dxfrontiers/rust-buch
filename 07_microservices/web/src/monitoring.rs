use rocket::fairing::{Fairing, Info, Kind};
use opentelemetry::{
    global,
    metrics::{BoundCounter, BoundValueRecorder},
    KeyValue,
};
use opentelemetry_prometheus::PrometheusExporter;
use rocket::{Response, Request, Data, Route};
use rocket::route::Handler;
use rocket::route::Outcome;
use rocket::http::{Method};
use rocket::response::content::{Plain};
use prometheus::{TextEncoder, Encoder};
use lazy_static::lazy_static;
use opentelemetry::metrics::Counter;


#[derive(Clone)]
pub struct Prometheus {
    exporter: PrometheusExporter,
    http_counter: BoundCounter<'static, u64>,
    http_body_gauge: BoundValueRecorder<'static, u64>,
    http_counter_individual: Counter<u64>,

}

lazy_static! {
    static ref HANDLER_ALL: [KeyValue; 1] = [KeyValue::new("handler", "all")];
}

impl Prometheus {
    /// Create a new `PrometheusMetrics`.
    pub fn new() -> Self {
        let exporter = opentelemetry_prometheus::exporter().init();

        let meter = global::meter("rustbuch.de/opentele-prometheus");

        // meter.

        Prometheus {
            exporter,
            http_counter: meter
                .u64_counter("example.http_requests_total")
                .with_description("Total number of HTTP requests made.")
                .init()
                .bind(HANDLER_ALL.as_ref()),
            http_body_gauge: meter
                .u64_value_recorder("example.http_response_size_bytes")
                .with_description("The HTTP response sizes in bytes.")
                .init()
                .bind(HANDLER_ALL.as_ref()),
            http_counter_individual: meter
                .u64_counter("example.http_counter_individual")
                .with_description("The count of callsd for each handler")
                .init()
        }
    }
}

#[rocket::async_trait]
impl Fairing for Prometheus {
    fn info(&self) -> Info {
        Info {
            name: "Prometheus metric collection",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, _response: &mut Response<'r>) {
        self.http_counter.add(1);
        self.http_counter_individual.add(1,&[
            KeyValue::new("url", request.uri().path().as_str().to_string()),
            KeyValue::new("remote", request.remote().unwrap().ip().to_string()),
        ]);

    }
}

#[rocket::async_trait]
impl Handler for Prometheus {
    async fn handle<'r>(&self, request: &'r Request<'_>, _: Data<'r>) -> Outcome<'r> {
        // Gather the metrics.
        let mut buffer = vec![];

        let encoder = TextEncoder::new();
        let metric_families = self.exporter.registry().gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        let body = String::from_utf8(buffer).unwrap();
        let responder = Plain(body);
        Outcome::from(
            request,
            responder
        )
    }
}

impl Into<Vec<Route>> for Prometheus {
    fn into(self) -> Vec<Route> {
        vec![Route::new(Method::Get, "/", self)]
    }
}