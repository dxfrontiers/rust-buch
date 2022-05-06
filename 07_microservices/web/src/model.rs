use rocket::data::{FromData, Outcome};
use rocket::http::Status;
use serde::{Deserialize, Serialize};
use rocket::response::Responder;
use rocket::{Data, Request, response};
use rocket::serde::json::Json;
use rocket::form::FromForm;
use rocket::outcome::Outcome::*;

/*
   // TODO explain all the derives (and clean up)

   The request that start the communication process
   We pose nop constraints on the content here
*/
#[derive(
    Serialize,      //
    Deserialize,    //
    Clone,          //
    Debug,          //
    PartialEq,      //
)]
pub struct ContactRequest {
    pub id: i32,
    pub email: String,
    pub message: String,
}

/**
    ContactRequest implements Responder by simply wrapping itself in json and handle that
    implementation care about the rest.
*/
impl<'r> Responder<'r, 'static> for ContactRequest {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        Json(self).respond_to(request)
    }
}



/**
    NewContactRequest implements the NewType Pattern for ContactRequest.
    This is needed, since it originates a client request and the client cannot know the `id` field
    of ContactRequest in advance. Further, making `id`, being the primary key of the DB table,
    optional would violate the semantics of the key.
*/
#[derive(Serialize, Deserialize, FromForm)]
pub struct NewContactRequest {
    pub email: String,
    pub message: String,
}


#[rocket::async_trait]
impl<'r> FromData<'r> for NewContactRequest{
    type Error = String;
    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self>{

         match Json::from_data(req, data).await{
            Success(json) => Success(json.0),
            _a => Failure((Status::BadRequest,String::from("AAAAh"))),
         }
         
    }
}