
use structopt::StructOpt;
// use rdkafka::ClientConfig;
// use rdkafka::config::RDKafkaLogLevel;
use rustbuch_microservices_kafka_consumer::KafkaConfig;

#[derive(StructOpt, Debug)]
#[structopt(name = "database-sink")]
pub struct MyConfig {

    /// Database url to connect to
    #[structopt(long="db", env="DB_URL", default_value = "postgres://postgres:supersecure@postgres/postgres")]
    pub db_url: String,

    /// List of initial brokers of hostname:port
    #[structopt(long, env, default_value = "kafka:9092")]
    pub brokers: String,

    /// Group id to use for the consumer
    #[structopt(long, env, default_value = "database-sink")]
    pub group_id: String,

    /// topics to subscribe to, can be used multiple times
    #[structopt(long, env, default_value = "contact_request")]
    pub topic: Vec<String>,
}

// impl Into<ClientConfig> for &MyConfig{
//     fn into(self) -> ClientConfig {
//         let mut cc = ClientConfig::new();
//         cc.set("group.id", &self.group_id)
//             .set("bootstrap.servers", &self.brokers)
//             .set("enable.partition.eof", "false")
//             .set("session.timeout.ms", "6000")
//             .set("enable.auto.commit", "true")
//             .set_log_level(RDKafkaLogLevel::Warning);
//         cc
//     }
// }

impl KafkaConfig for MyConfig{
    fn group_id(&self) -> String {
        self.group_id.clone()
    }

    fn brokers(&self) -> String {
        self.brokers.clone()
    }
    fn topics(&self) -> Vec<String>{
        self.topic.clone()
    }
}