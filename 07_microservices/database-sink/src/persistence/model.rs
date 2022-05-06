use diesel::{self, Identifiable, Queryable, Insertable, AsChangeset};
use serde::*;
use crate::persistence::schema::requests;
use std::convert::TryFrom;

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

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="requests"]
pub struct NewContactRequest {
    pub email: String,
    pub message: String,
}

impl TryFrom<&[u8]> for NewContactRequest{
    type Error = ();
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        serde_json::from_slice(value).map_err(|_e|())
    }
}