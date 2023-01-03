use crate::jwt::UserToken;
use crate::services::place_service::*;

use crate::utils::responde_request::{response_api_bool, response_api_entity};

use align_mind_server::models::place_model::*;
use align_mind_server::models::response_model::Response;

use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket_validation::Validated;

#[get("/<uid_place>")]
pub fn getting_place(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_place: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let result_place: Option<Place> = get_place(uid_place);
    response_api_entity(result_place)
}

#[post("/<uid_user>", format = "application/json", data = "<payload>")]
pub fn save_place(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_user: Uuid,
    payload: Validated<Json<NewPlaceDTO>>,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let result_action: bool = create_place(uid_user, payload.into_inner().into_inner());
    response_api_bool(result_action)
}

#[put("/<uid_place>", format = "application/json", data = "<payload>")]
pub fn updating_place(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_place: Uuid,
    payload: Validated<Json<UpdatePlaceDTO>>,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let result_action: bool = update_place(uid_place, payload.into_inner().into_inner());
    response_api_bool(result_action)
}

#[delete("/<uid_place>")]
pub fn deleting_place(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_place: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let result_action: bool = delete_place(uid_place);
    response_api_bool(result_action)
}
