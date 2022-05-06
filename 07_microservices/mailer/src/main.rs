mod config;
mod model;
#[cfg(feature = "smtp")]
mod smtp;
#[cfg(not(feature = "smtp"))]
mod stub;

use lettre::{Transport, Message};
use lettre::error::Error as EmailError;
use crate::config::MailerConfig;
use crate::model::NewContactRequest;
use std::convert::TryFrom;
use rustbuch_microservices_kafka_consumer::{MessageHandler, KafkaConfig, HeaderLookup, HandleError, subscribe_and_handle};
use log::{warn,debug};use structopt::StructOpt;

#[cfg(feature = "smtp")]
use smtp::prepare_transport;

#[cfg(not(feature = "smtp"))]
use stub::prepare_transport;

use opentelemetry::global::BoxedSpan;
use opentelemetry::{global, trace};
use opentelemetry::trace::{SpanContext, Tracer, Span};

struct LettreMessageHandler<T: Transport> {
    email_recipient: String,
    lettre_transport: T
}

impl<T: Transport> LettreMessageHandler<T> {
    fn prepare_mail(&self, request: NewContactRequest) -> Result<Message, EmailError> {
        return Message::builder()
            .from(request.email.parse().map_err(|_| EmailError::MissingFrom)?)
            .to(self.email_recipient.parse().map_err(|_| EmailError::MissingTo)?)
            .subject("Contact request")
            .body(request.message);
    }

    fn send_mail(&mut self, message: Message) {
        match self.lettre_transport.send(&message) {
            Ok(_response) => {
                debug!("Mail was sent successfully");
            }
            Err(_e) => {
                debug!("Mail could not be sent.");
            }
        }
    }
}

impl<T: Transport + Send> MessageHandler for LettreMessageHandler<T> {
    fn handle(&mut self, message: &[u8], headers: HeaderLookup) -> Result<(),HandleError> {
        let tracer = global::tracer("mail_handler");

        let span = get_span_from_headers(headers)
            .unwrap_or_else(|| tracer.start("handler"));
        let _span = trace::mark_span_as_active(span);

        match NewContactRequest::try_from(message){
            Ok(request) => {
                debug!("About sending mail from {}", &request.email);
                match self.prepare_mail(request) {
                    Ok(message) =>  tracer.in_span(
                        "mail_delivery",
                        |_cx|  self.send_mail(message)),
                    Err(error) => {
                        warn!("Could not sent mail because of: {}", error)
                    }
                }
            }
            _ => warn!("Unparseable message in queue")
        }
        Ok(())

    }
}


fn main() {

    env_logger::Builder::from_default_env().init();
    let config = MailerConfig::from_args();

    // let (version_n, version_s) = get_rdkafka_version();
    // info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    global::set_text_map_propagator(opentelemetry_zipkin::Propagator::new());
    let _tracer = opentelemetry_zipkin::new_pipeline()
        .with_service_name("mail_sink")
        .with_collector_endpoint("http://zipkin:9411/api/v2/spans")
        .install_simple()
        .unwrap();

    let transport = prepare_transport(&config);

    let message_handler = LettreMessageHandler {
        lettre_transport: transport,
        email_recipient: config.email_recipient.clone()
    };

    subscribe_and_handle(
        Box::new(config) as Box<dyn KafkaConfig>,
        Box::new(message_handler) as Box<dyn MessageHandler>
    );

}



// DUPLICATE FROM DATABASE-SINK
fn get_span_from_headers(headers: HeaderLookup) -> Option<BoxedSpan>{
    if let Some(header) = headers.get_headers(&"tracing_blob").first(){
        if let Ok( sc ) = serde_json::from_slice::<SpanContext>(header){
            let tracer =  global::tracer("mail_handler");
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