use structopt::StructOpt;


use rustbuch_microservices_kafka_consumer::KafkaConfig;

#[derive(StructOpt, Debug)]
#[structopt(name = "mailer")]
pub struct MailerConfig {

    /// SMTP host to use
    #[structopt(short="h", long, env="SMTP_HOST", default_value = "localhost")]
    pub smtp_host: String,

    /// SMTP user to use
    #[structopt(short="u", long, env="SMTP_USER")]
    pub smtp_user: String,

    /// SMTP password to use
    #[structopt(short="p", long, env="SMTP_PASSWORD")]
    pub smtp_password: String,

    /// Recipient mail address
    #[structopt(short="r", long, env="EMAIL_RECIPIENT")]
    pub email_recipient: String,

    /// List of initial brokers of hostname:port
    #[structopt(long, env, default_value = "kafka:9092")]
    pub brokers: String,

    /// Group id to use for the consumer
    #[structopt(long, env, default_value = "mailer")]
    pub group_id: String,

    /// topics to subscribe to, can be used multiple times
    #[structopt(long, env, default_value = "contact_request")]
    pub topic: Vec<String>,
}

impl KafkaConfig for MailerConfig {
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