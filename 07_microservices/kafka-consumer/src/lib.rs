


use log::{warn, error,debug};
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::message::{Message};



use tokio::runtime::Runtime;

pub trait KafkaConfig : Send + Sync{
    fn group_id(&self) -> String;
    fn brokers(&self) -> String;
    fn topics(&self) -> Vec<String>;
}

impl Into<ClientConfig> for Box<dyn KafkaConfig>{
    fn into(self) -> ClientConfig {
        let mut cc = ClientConfig::new();
        cc.set("group.id", &self.group_id())
            .set("bootstrap.servers", &self.brokers())
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            .set_log_level(RDKafkaLogLevel::Warning);
        cc
    }
}

pub trait MessageHandler : Send{
    fn handle(&mut self, message: &[u8], headers: HeaderLookup) -> Result<(),HandleError>;
}

pub struct HandleError(pub String);

pub struct HeaderLookup<'a>{
    headers: Option<&'a BorrowedHeaders>
}

impl <'a> HeaderLookup<'a>{
    fn new(headers: Option<&'a BorrowedHeaders>) -> Self{
        HeaderLookup{ headers }
    }
    pub fn get_headers(&'a self, name: &str)-> Vec<&[u8]>{
        let mut result = Vec::new();
        if let Some(headers) = self.headers{
            for i in 0 .. headers.count(){
                if let Some(header) = headers.get(i){
                    if header.0.eq(name) {
                        result.push(header.1)
                    }
                }
            }
        };
        result
    }
}

pub use rdkafka::message::{Headers,BorrowedHeaders};


async fn subscribe_and_handle_task(
    config: Box<dyn KafkaConfig>,
    mut message_handler: Box<dyn MessageHandler>
) {
    let topics = config.topics();
    let topics = topics.iter().map(|s| s.as_str()).collect::<Vec<_>>();


    let cc : ClientConfig = config.into();
    let consumer: StreamConsumer = cc.create()
        .expect("Consumer creation failed");


    consumer
        .subscribe(topics.as_slice())
        .expect("Can't subscribe to specified topics");

    loop {
        match consumer.recv().await {
            Err(e) => error!("Kafka error: {}", e),
            Ok(m) => {
                if let Some(Ok(payload)) = m.payload_view::<[u8]>() {
                    debug!("Received[{:?}]: '{}'", m.offset(), hex::encode(payload));
                    match message_handler.handle(&payload, HeaderLookup::new(m.headers())){
                        Ok(()) => (),
                        Err(e) => warn!("Handler failed with {}",e.0)
                    }
                }
            }
        };
    }
}

pub fn subscribe_and_handle(
    config: Box<dyn KafkaConfig>,
    message_handler: Box<dyn MessageHandler>
    ) {
    let rt = Runtime::new().unwrap();
    rt.block_on(async move {
        subscribe_and_handle_task(
            config,
            message_handler
        ).await
    });
}
