use rocket::serde::{Deserialize, Serialize};
use rocket_validation::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginDTO {
    #[validate(email, required)]
    pub email: Option<String>,
    #[validate(length(min = 8, max = 50), required)]
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginInfo {
    pub email: String,
    pub login_session: String,
}
