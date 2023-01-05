use align_mind_server::establish_connection;
use align_mind_server::models::auth_model::LoginDTO;
use align_mind_server::models::response_model::{Response, ResponseError, ResponseSuccess};
use align_mind_server::models::user_model::NewUserDTO;

use diesel::PgConnection;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket_validation::Validated;

use crate::services::auth_service::{create_account, sign_in};
use crate::utils::responde_request::response_api;

#[post("/signup", format = "application/json", data = "<payload>")]
pub fn sign_up(payload: Validated<Json<NewUserDTO>>) -> status::Custom<Json<Response>> {
    let connection: &mut PgConnection = &mut establish_connection();

    let response_action: Result<ResponseSuccess, ResponseError> =
        create_account(payload.into_inner().into_inner(), connection);

    response_api(response_action)
}

#[post("/login", format = "application/json", data = "<payload>")]
pub fn login(payload: Validated<Json<LoginDTO>>) -> status::Custom<Json<Response>> {
    let connection: &mut PgConnection = &mut establish_connection();

    let response_action = sign_in(payload.into_inner().into_inner(), connection);

    response_api(response_action)
}
