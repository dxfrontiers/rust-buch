use super::schema::requests;
use serde::{Deserialize, Serialize};
use rocket::response::Responder;
use rocket::{Request, response};
use diesel::QueryResult;
use diesel::{self, prelude::*};
use rocket::serde::json::Json;

/*
   // TODO explain all the derives (and clean up)

   The request that start the communication process
   We pose nop constraints on the content here
*/
#[table_name = "requests"]
#[derive(
    Serialize,      //
    Deserialize,    //
    Clone,          //
    Debug,          //
    PartialEq,      //
    Identifiable,   //
    Queryable,      //
    Insertable,     //
    AsChangeset,    //
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
#[derive(Deserialize, Insertable, FromForm, Queryable)]
#[table_name = "requests"]
pub struct NewContactRequest {
    pub email: String,
    pub message: String,
}

/**
   The response carrying a ticket id for the user to refer to the original request later on
*/
#[derive(Serialize, Deserialize)]
pub struct ContactResponse {
    pub ticket_id: usize,
}

impl ContactResponse{
    pub fn new(id: usize) -> Self {
        ContactResponse{ticket_id: id}
    }
}

/**
    ContactResponse implements Responder by simply wrapping itself in json and handle that
    implementation care about the rest.
*/
impl<'r> Responder<'r, 'static> for ContactResponse {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        Json(self).respond_to(request)
    }
}
