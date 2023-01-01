use align_mind_server::models::{
    auth_model::LoginDTO, response_model::Response, user_model::NewUser,
};
use rocket::{http::Status, response::status, serde::json::Json};

use crate::services::auth_service::{create_account, sign_in};

#[post("/signup", format = "json", data = "<payload>")]
pub fn sign_up(payload: Json<NewUser>) -> status::Custom<Json<Response>> {
    let response = create_account(payload.into_inner());

    if response {
        status::Custom(
            Status::from_code(Status::Ok.code).unwrap(),
            Json(Response {
                message: String::from("signup successfully"),
                data: serde_json::to_value("").unwrap(),
            }),
        )
    } else {
        status::Custom(
            Status::from_code(Status::BadRequest.code).unwrap(),
            Json(Response {
                message: String::from("error when signing up, please try again"),
                data: serde_json::to_value("").unwrap(),
            }),
        )
    }
}

#[post("/login", format = "json", data = "<payload>")]
pub fn login(payload: Json<LoginDTO>) -> status::Custom<Json<Response>> {
    if let Some(response) = sign_in(payload.into_inner()) {
        status::Custom(
            Status::from_code(Status::Ok.code).unwrap(),
            Json(Response {
                message: String::from("signup successfully"),
                data: serde_json::to_value(response).unwrap(),
            }),
        )
    } else {
        status::Custom(
            Status::from_code(Status::BadRequest.code).unwrap(),
            Json(Response {
                message: String::from("error when signing up, please try again"),
                data: serde_json::to_value("").unwrap(),
            }),
        )
    }
}
