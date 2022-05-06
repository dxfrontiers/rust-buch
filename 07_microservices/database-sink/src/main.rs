#[macro_use]
extern crate diesel;

use log::{debug};
use crate::persistence::connect_db;

mod config;
mod persistence;

use config::MyConfig;
use std::convert::{TryFrom};
use crate::persistence::model::{NewContactRequest};
use structopt::StructOpt;
use rustbuch_microservices_kafka_consumer::{subscribe_and_handle, MessageHandler, KafkaConfig, HeaderLookup, HandleError};
use diesel::PgConnection;
use opentelemetry::{global, trace};
use opentelemetry::trace::{SpanContext, Tracer, Span, get_active_span};
use opentelemetry::global::BoxedSpan;

struct DatabaseMessageHandler{
    db: PgConnection
}

fn get_span_from_headers(headers: HeaderLookup) -> Option<BoxedSpan>{
    if let Some(header) = headers.get_headers(&"tracing_blob").first(){
        if let Ok( sc ) = serde_json::from_slice::<SpanContext>(header){
            let tracer =  global::tracer("db_sink_handler");
            let mut span_builder = tracer.span_builder("subscribe");
            span_builder.trace_id = Some(sc.trace_id());
            let mut span = tracer.build(span_builder);
            debug!("tracing with tid {}", span.span_context().trace_id().to_hex());
            span.add_event("Picked up trace from Kafka".to_string(), vec![]);
            return Some(span);
        }
    }
    None
}

impl MessageHandler for DatabaseMessageHandler{
    fn handle(&mut self,message: &[u8], headers: HeaderLookup) -> Result<(),HandleError> {
        let tracer = global::tracer("db_sink_handler");

        let span = get_span_from_headers(headers)
            .unwrap_or_else(|| tracer.start("handler"));
        let _span = trace::mark_span_as_active(span);

        let request = NewContactRequest::try_from(message)
            .map_err(|_e| log_trace_error(format!("Message in queue not parseable")))?;

        // start a new span for the db tx
        let mut span = tracer.start("db_write");
        span.add_event("Message parsed".to_string(),vec![]);
        debug!("Saving message from {}",&request.email);

        persistence::save(&self.db, request)
            .map_err(|_e| log_trace_error(format!("Message persist failed")))?;

        // if we are here, everything worked, notify tracing
        span.add_event("Message persisted".to_string(),vec![]);
        Ok(())
    }
}

fn log_trace_error(msg: String) -> HandleError{
    get_active_span(|span| {
        span.add_event(msg.clone(),vec![])
    });
    HandleError(msg)
}


fn main() {

    env_logger::Builder::from_default_env().init();
    let config = MyConfig::from_args();

    // let (version_n, version_s) = get_rdkafka_version();
    // info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    let db = connect_db(&config);

    let databasae_message_handler = DatabaseMessageHandler{
        db
    };


    global::set_text_map_propagator(opentelemetry_zipkin::Propagator::new());
    let _tracer = opentelemetry_zipkin::new_pipeline()
        .with_service_name("db_sink")
        .with_collector_endpoint("http://zipkin:9411/api/v2/spans")
        .install_simple()
        .unwrap();



    subscribe_and_handle(
        Box::new(config) as Box<dyn KafkaConfig>,
        Box::new( databasae_message_handler) as Box<dyn MessageHandler>
    );

}
