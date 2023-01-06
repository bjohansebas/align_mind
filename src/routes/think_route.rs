use crate::jwt::UserToken;
use crate::services::think_service::*;
use crate::utils::responde_request::{response_api, response_api_data};
use align_mind_server::establish_connection;
use align_mind_server::models::response_model::{Response, ResponseError, ResponseSuccess};
use align_mind_server::models::think_model::*;

use diesel::PgConnection;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket_validation::Validated;

#[get("/<uid_think>")]
pub fn getting_think(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_think: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_think: Result<Think, ResponseError> = get_think(uid_think, connection);
    response_api_data(result_think)
}

#[post("/<uid_user>", format = "application/json", data = "<payload>")]
pub fn save_think(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_user: Uuid,
    payload: Validated<Json<NewThinkDTO>>,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }
    let connection: &mut PgConnection = &mut establish_connection();

    let data_think: NewThinkDTO = payload.into_inner().into_inner();
    let result_action: Result<ResponseSuccess, ResponseError> =
        create_think(uid_user, data_think, connection);

    response_api(result_action)
}

#[post("/<uid_think>/trash")]
pub fn move_to_trash(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_think: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_action: Result<ResponseSuccess, ResponseError> =
        move_think_to_trash(uid_think, connection);

    response_api(result_action)
}

#[put("/<uid_think>", format = "application/json", data = "<payload>")]
pub fn updating_think(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_think: Uuid,
    payload: Validated<Json<UpdateThinkDTO>>,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_action: Result<ResponseSuccess, ResponseError> =
        update_think(uid_think, payload.into_inner().into_inner(), connection);

    response_api(result_action)
}

#[delete("/<uid_think>")]
pub fn deleting_think(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_think: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_action: Result<ResponseSuccess, ResponseError> = delete_think(uid_think, connection);
    response_api(result_action)
}
