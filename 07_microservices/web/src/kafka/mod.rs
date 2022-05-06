mod fairing;


use rdkafka::error::KafkaError;


use rdkafka::message::{OwnedMessage};
use rocket::tokio::time::error::Elapsed;




use rocket::{Request, Response};
use rocket::http::{Status};
use rocket::fairing::{Fairing};
use crate::kafka::fairing::KafkaFairing;
use rocket::response::Responder;


pub enum KafkaResponse {
    Processed(Result<Result<(i32,i64), (KafkaError, OwnedMessage)>, Elapsed>),
    BadRequest(String)
}

impl KafkaResponse {

    pub fn fairing() -> impl Fairing
    {
        KafkaFairing {}
    }

    // pub fn process(request: NewContactRequest) -> KafkaResponse {
    //     KafkaResponse { contact_request: request }
    // }
}

impl<'r> Responder<'r, 'static> for KafkaResponse {
    fn respond_to(self, _req: &'r Request<'_>) -> rocket::response::Result<'static> {
        let mut response = Response::build();
        return match self{
            KafkaResponse::Processed(send_result) => match send_result {
                Ok(send_result) =>
                    match send_result {
                        Ok(v) => {
                            debug!("got response: {:?} {:?}", v.0, v.1);
                            response.status(Status::Created).ok()
                        },
                        Err(e) => {
                            warn!("error: {:?}, message {:?}", e.0, e.1);
                            Err(Status::InternalServerError)
                        }
                    },
                Err(e) => {
                    warn!("error: {:?}", e);
                    Err(Status::InternalServerError)
                }
            },
            KafkaResponse::BadRequest(pretty_reason) => {
                warn!("Request was Bad: {:?}", pretty_reason);
                Err(Status::BadRequest)
            }
        }

    }
}