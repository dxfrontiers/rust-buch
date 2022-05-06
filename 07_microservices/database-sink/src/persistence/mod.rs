use diesel::{PgConnection, Connection};
use crate::MyConfig;
use crate::persistence::model::{ContactRequest, NewContactRequest};
use log::{trace, debug};

use schema::requests;
use diesel::prelude::RunQueryDsl;

pub mod model;
pub mod schema;


pub (crate) fn connect_db(conf: &MyConfig) -> PgConnection {
    trace!("About to connect to DB");

    let con = match PgConnection::establish(&conf.db_url) {
        Ok(con) => Some(con),
        Err(e) =>  {   debug!("Error: {}", e);  None }
    };

    trace!("Finished to connect to DB");
    return con.unwrap();
}

pub (crate) fn save(con: &PgConnection, request: NewContactRequest)
    -> Result<ContactRequest,String> {
    diesel::insert_into(requests::table)
        .values(&request)
        .get_result(con)
        .map_err(|e| format!("DB Failure: {}",e))
}