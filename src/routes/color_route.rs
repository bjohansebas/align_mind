use crate::jwt::UserToken;
use crate::services::color_service::*;
use crate::utils::responde_request::{response_api, response_api_data};
use align_mind_server::establish_connection;
use align_mind_server::models::color_model::*;
use align_mind_server::models::response_model::{Response, ResponseError, ResponseSuccess};

use diesel::PgConnection;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket_validation::Validated;

#[get("/<uid_color>")]
pub fn getting_color(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_color: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_color: Result<Color, ResponseError> = get_color(uid_color, connection);
    response_api_data(result_color)
}

#[post("/<uid_user>", format = "application/json", data = "<payload>")]
pub fn save_color(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_user: Uuid,
    payload: Validated<Json<NewColorDTO>>,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_action: Result<ResponseSuccess, ResponseError> =
        create_color(uid_user, payload.into_inner().into_inner(), connection);

    response_api(result_action)
}

#[put("/<uid_color>", format = "application/json", data = "<payload>")]
pub fn updating_color(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_color: Uuid,
    payload: Validated<Json<UpdateColorDTO>>,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_action: Result<ResponseSuccess, ResponseError> =
        update_color(uid_color, payload.into_inner().into_inner(), connection);

    response_api(result_action)
}

#[delete("/<uid_color>")]
pub fn deleting_color(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_color: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_action: Result<ResponseSuccess, ResponseError> = delete_color(uid_color, connection);

    response_api(result_action)
}
