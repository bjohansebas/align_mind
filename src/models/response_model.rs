use rocket::serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub message: String,
    pub data: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseSuccess {
    pub message: String,
    pub data: Value,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct ResponseError {
    pub code: u16,
    pub message: String,
}
