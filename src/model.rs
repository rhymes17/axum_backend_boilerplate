use serde::{Deserialize, Serialize};
use serde;

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    #[serde(rename = "_id")]
    id : Option<String>,
    name: String,
    email : String
}