use serde::{Deserialize, Serialize};
use serde;

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    #[serde(rename = "_id")]
    pub id : Option<String>,
    pub name: String,
    pub email : String
}