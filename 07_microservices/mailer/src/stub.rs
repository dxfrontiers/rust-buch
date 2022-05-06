use crate::config::MailerConfig;
use lettre::transport::stub::StubTransport;

pub fn prepare_transport(_config: &MailerConfig) -> StubTransport {
    return StubTransport::new_ok();
}