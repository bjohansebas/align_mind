use align_mind_server::models::auth_model::{Login, LoginDTO};
use align_mind_server::models::response_model::Response;
use align_mind_server::models::user_model::{NewUser, NewUserDTO};

use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket_validation::Validated;

use crate::services::auth_service::{create_account, sign_in};

#[post("/signup", format = "application/json", data = "<payload>")]
pub fn sign_up(payload: Validated<Json<NewUserDTO>>) -> status::Custom<Json<Response>> {
    let data_new_user: NewUserDTO = payload.into_inner().into_inner();
    let new_user: NewUser = NewUser {
        username: data_new_user.username.unwrap(),
        password: data_new_user.password.unwrap(),
        email: data_new_user.email.unwrap(),
    };
    let response: bool = create_account(new_user);

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

#[post("/login", format = "application/json", data = "<payload>")]
pub fn login(payload: Validated<Json<LoginDTO>>) -> status::Custom<Json<Response>> {
    let data_login: LoginDTO = payload.into_inner().into_inner();

    let login: Login = Login {
        email: data_login.email.unwrap(),
        password: data_login.password.unwrap(),
    };

    if let Some(response) = sign_in(login) {
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
