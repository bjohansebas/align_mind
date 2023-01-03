use crate::jwt::UserToken;
use crate::services::color_service::*;
use crate::utils::responde_request::{response_api_bool, response_api_entity};
use align_mind_server::models::color_model::*;
use align_mind_server::models::response_model::Response;

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

    let result_color: Option<Color> = get_color(uid_color);
    response_api_entity(result_color)
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

    let result_action: bool = create_color(uid_user, payload.into_inner().into_inner());
    response_api_bool(result_action)
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

    let result_action: bool = update_color(uid_color, payload.into_inner().into_inner());
    response_api_bool(result_action)
}

#[delete("/<uid_color>")]
pub fn deleting_color(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_color: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let result_action: bool = delete_color(uid_color);
    response_api_bool(result_action)
}
