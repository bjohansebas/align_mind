use crate::jwt::UserToken;
use crate::services::think_service::*;
use crate::utils::responde_request::{response_api_bool, response_api_entity};
use align_mind_server::models::response_model::Response;
use align_mind_server::models::think_model::*;

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

    let result_think: Option<Think> = get_think(uid_think);
    response_api_entity(result_think)
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

    let data_think: NewThinkDTO = payload.into_inner().into_inner();
    let result_action: bool = create_think(uid_user, data_think);
    response_api_bool(result_action)
}

#[post("/<uid_think>/trash")]
pub fn move_to_trash(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_think: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }
    let result_action: bool = move_think_to_trash(uid_think);
    response_api_bool(result_action)
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

    let result_action: bool = update_think(uid_think, payload.into_inner().into_inner());
    response_api_bool(result_action)
}

#[delete("/<uid_think>")]
pub fn deleting_think(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_think: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let result_action: bool = delete_think(uid_think);
    response_api_bool(result_action)
}
