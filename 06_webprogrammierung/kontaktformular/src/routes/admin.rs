use crate::model::{ContactRequest, NewContactRequest};
use crate::persistence::DbCon;
use crate::auth::User;
use log::{info};
use rocket::serde::json::Json;
use rocket_dyn_templates::Template;

/**
    List all requests contained in the database
    This handler acts upon requests accepting json responses
    We require there to be an authenticated user available
    Unfortunately we cannot direcrty implement Responder on `Vec<ContactRequest>`, hence we wrap in in json
*/
#[get("/requests", format = "application/json",rank = 1)]
pub async fn list_requests(con: DbCon, _user: User) -> Json<Vec<ContactRequest>> {
    let res = crate::persistence::request_repo::get_all(&con).await.unwrap_or_default();
    Json(res)
}

/**
    List all requests contained in the database by rendering them into a template
    This handler acts upon requests asking for HTML
*/
#[get("/requests", format = "text/html",rank = 2)]
pub async fn list_requests_template(con: DbCon, user: User) -> Template {
    info!("Loading requests for user {}",user.name);
    let res = crate::persistence::request_repo::get_all(&con).await.unwrap_or_default();
    Template::render("requests", &res)
}

/**
    We return a single request, if any, identified by an id in the path.
*/
#[get("/requests/<id>")]
pub async fn get_request_by_id(con: DbCon, id: i32, _user: User) -> Result<Option<ContactRequest>,String> {
    crate::persistence::request_repo::get_by_id(&con, id).await
}

/**
    Delete a single request, identified by an id in the path.
    We return either a response containing the number of deleted entries,
        None (leading to a 404 in rocket) if there was nothing deleted or
        Err with a description if there was an error while working with the db
*/
#[delete("/requests/<id>")]
pub async fn delete_request_by_id(con: DbCon, id: i32, _user: User) -> Result<Option<String>,String> {
    crate::persistence::request_repo::delete_by_id(&con, id).await.map(|num|{
        if num > 0 { Some( num.to_string()) }
        else { None }
    })
}

/**
    Updates a single request, identified by an id in the path. One cannot change the id of a request
    We return the new request if the modification was successful
*/
#[put("/requests/<id>", format = "application/json", data = "<request>")]
pub async fn update_request(
    con: DbCon,
    id: i32,
    request: Json<ContactRequest>,
    _user: User
) -> Result<ContactRequest, String> {
    crate::persistence::request_repo::update(&con, id,request.into_inner()).await
}

/**
    Creates a single request. This uses the NewType Pattern.
    Since we are on the admin backend, no other checks are performed
*/
#[post("/requests", format = "application/json", data = "<request>")]
pub async fn create_request(con: DbCon, request: Json<NewContactRequest>, _user: User
) -> Result<ContactRequest, String> {
    crate::persistence::request_repo::insert(&con, request.into_inner()).await
}

