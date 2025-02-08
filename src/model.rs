use serde::{self, Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User{
    #[serde(rename = "_id")]
    pub id : Option<String>,
    pub name: String,
    pub email: String
}