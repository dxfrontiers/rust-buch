use serde::*;
use std::convert::TryFrom;

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

#[derive(Serialize, Deserialize)]
pub struct NewContactRequest {
    pub email: String,
    pub message: String,
}

impl TryFrom<&[u8]> for NewContactRequest {
    type Error = ();
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        serde_json::from_slice(value).map_err(|_e|())
    }
}