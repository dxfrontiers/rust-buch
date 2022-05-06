use rocket::http::Status;
use rocket::response::status::{Custom};
use crate::model::{ContactResponse, NewContactRequest};
use crate::persistence::{DbCon, request_repo };
use rocket_dyn_templates::Template;
use rocket::serde::json::Json;
use rocket::form::Form;
use serde_json::json;
use std::collections::HashMap;
use std::iter::FromIterator;
use serde::Serialize;

type HTTPContactRequestError = Custom<String>;


#[derive(Serialize)]
pub struct IndexLabel {
    message: String,
    email: String
}

/**
    Renders the index page from a template.
    The used Template does not need any context, hence we pass it an empty one.
*/
#[get("/")]
pub async fn index_template() -> Template {
    let labels = IndexLabel {
        message: "Nachricht".into(),
        email: "E-Mail".into()
    };
    Template::render("index", labels)
}


/**
   Public endpoint to accept a ContactRequest, encoded as Json in the HTTP Body.
   The Database connection is automatically inserted from the DB Fairing.
   We limit the content type strictly to application/json so that we can still accept the form
   submission on the same path with a different content type.
   The returned ContactResponse does implement responder directly by wrapping itself in a bit of json
*/
#[post("/", format = "application/json", data = "<request>")]
pub async fn index_json(
    con: DbCon,
    request: Json<NewContactRequest>,
) -> Result<ContactResponse, HTTPContactRequestError> {
    new_request(&con, request.into_inner()).await
}

/**
   Public endpoint to accept a ContactRequest, encoded as a Form.
*/
#[post("/", format = "application/x-www-form-urlencoded", data = "<request>")]
pub async fn index_form(
    con: DbCon,
    request: Form<NewContactRequest>,
) -> Result<ContactResponse, HTTPContactRequestError> {
    new_request(&con, request.into_inner()).await
}

/**
   Create a new Request, if the supplied request (NewType pattern) is valid.
   If there are database problems, we don't bother the user but rather return a generic 500
*/
pub async fn new_request(
    con: &DbCon,
    request: NewContactRequest,
) -> Result<ContactResponse, HTTPContactRequestError> {

    // Return early if the request is not valid.
    validate_new_request(&request).map_err(|e| Custom(Status::BadRequest, e))?;

    // Return a contact Response made from the id of the Request or a generic error
    request_repo::insert(&con, request).await
        .map(|result|  ContactResponse::new(result.id as usize))
        .map_err(|_| { Custom(Status::InternalServerError, "Database Server Error".into()) })
}


/**
    Very basic placeholder validation for the contact request
*/
fn validate_new_request(request: &NewContactRequest) -> Result<(), String> {
    let message_length = request.message.chars().count();

    return if message_length < 5 {
        Err("The message needs to be at least 5 chars".to_string())
    } else if !request.email.contains('@') {
        Err("💩 nope nope nope, there is an @ missing".to_string())
    } else {
        Ok(())
    }
}

