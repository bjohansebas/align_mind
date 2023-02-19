use rocket::serde::{Deserialize, Serialize};
use rocket_validation::Validate;

#[derive(Debug, Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginDTO {
    #[validate(email, required)]
    pub email: Option<String>,
    #[validate(length(min = 8, max = 30), required)]
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginInfo {
    pub id: String,
    pub email: String,
    #[serde(rename = "loginSession")]
    pub login_session: String,
}
