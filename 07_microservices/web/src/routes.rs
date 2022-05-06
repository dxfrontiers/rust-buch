use std::time::Duration;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rocket::tokio::time::timeout;
use rocket::{State, post, get};
use opentelemetry::trace::{Span, TraceContextExt, Tracer};
use opentelemetry::{trace::FutureExt, Context, KeyValue, global};
use crate::model::NewContactRequest;
use crate::kafka::{KafkaResponse};
use opentelemetry::Value;
use sha2::Sha256;
use sha2::Digest;

/**
   Public endpoint to accept a ContactRequest, encoded as Json in the HTTP Body.
*/
#[post("/", format = "application/json", data = "<request>")]
pub async fn index_json(
    request: NewContactRequest,
    producer: &State<FutureProducer>
) -> KafkaResponse {

    // This is where a request enter the system, so we can define a new span and tracing_id
    let mut span = global::tracer("Handler").start("request_ingestion");
    debug!("Handler with tid: {}", span.span_context().trace_id().to_hex());

    span.add_event("Handler call".to_string(), vec![
        KeyValue::new("Remote",Value::from(request.email.clone()))
    ]);

    publish_to_kafka(request, producer)
        .with_context(Context::current_with_span(span))
        .await  
}


async fn publish_to_kafka(
    contact_request: NewContactRequest, 
    producer: &FutureProducer)
     -> KafkaResponse {

    let mut sha = Sha256::new();
    sha.update(&contact_request.email);
    let key = sha.finalize();
    let payload =  match serde_json::to_vec(&contact_request) {
        Ok(payload) => payload,
        _ => { return KafkaResponse::BadRequest("Request not processable".to_string())}
    };
    Context::current().span().add_event("Payload parsed".to_string(),vec![]);

    // encode the serializable context parts into a string to add to a kafka message as a header
    let tracing_blob = serde_json::to_string(Context::current().span().span_context())
        .unwrap_or("{}".into());

    // create the record for kafka
    let record = FutureRecord::to("contact_request")
        .payload(&payload)
        .key(&key[..])
        .headers(OwnedHeaders::new().add("tracing_blob", &tracing_blob));

    debug!("{:?}",record);

    // create tasks and await
    // wraps the send task into an extra timeout since the build-in one does not work
    let send_result = timeout(
        Duration::from_millis(500),
        producer.send(record, Duration::from_millis(0)))
        .await;

    // Notify the tracer about the send result
    match &send_result{
        Ok(Ok(send_result)) =>
            Context::current().span().add_event("Publish done".to_string(),vec![
                KeyValue::new("Partition",Value::from(send_result.0 as i64)),
                KeyValue::new("Offset",Value::from(send_result.1))
            ]),
        _ => Context::current().span().add_event("Publish failed".to_string(),vec![])
    }

    KafkaResponse::Processed(send_result)
}

#[get("/health")]
pub async fn health()-> Result< &'static str, &'static str>{
    Ok("healthy")
}
