use rocket::fairing::{Fairing, Info, Result, Kind};
use rocket::{Build, Rocket, Orbit, Config};
use rocket::serde::Deserialize;
use rdkafka::producer::FutureProducer;
use rdkafka::config::ClientConfig;

pub struct KafkaFairing {}

#[rocket::async_trait]
impl Fairing for KafkaFairing {
    fn info(&self) -> Info {
        let kind = Kind::Ignite;

        Info { kind, name: "Kafka" }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result {
        

        #[derive(Deserialize)]
        struct KafkaConfig {
            bootstrap_server: String,
            queue_buffering_max_ms: i32,
        }

        #[derive(Deserialize)]
        struct MyConfig {
            kafka: KafkaConfig
        }

        // extract the entire config any `Deserialize` value
        let config = Config::figment().extract::<MyConfig>()
            .expect("Insufficient Configuration");

        match ClientConfig::new()
            .set("bootstrap.servers", config.kafka.bootstrap_server)
            .set("queue.buffering.max.ms", config.kafka.queue_buffering_max_ms.to_string()) // Do not buffer
            .create::<FutureProducer>() {
            Ok(client) => Ok(rocket.manage(client)),
            Err(e) => {
                error!("The Kafka client creation returned an error:");
                error!("{}", e);
                Err(rocket)
            }
        }
    }

    async fn on_liftoff(&self, rocket: &Rocket<Orbit>) {
        use rocket::{log::PaintExt, yansi::Paint};

        let _client = rocket.state::<FutureProducer>()
            .expect("Kafka Client registered in on_ignite");

        info!("{}{}:", Paint::emoji("📐 "), Paint::red("Kafka"));
    }
}