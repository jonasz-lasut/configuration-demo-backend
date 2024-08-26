use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::serialize::deserialize_checkbox;

#[derive(Deserialize)]
pub struct TodoNew {
    pub note: String,
}

#[derive(Serialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub note: String,
    pub status: bool,
}

#[derive(Deserialize)]
pub struct TodoUpdate {
    pub note: String,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_checkbox")]
    pub status: bool,
}

#[derive(Serialize)]
pub struct GenericResponse {
    pub message: String,
}
