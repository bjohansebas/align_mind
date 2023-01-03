use crate::jwt::UserToken;
use crate::services::trash_service::*;
use crate::utils::responde_request::{response_api_bool, response_api_entity};
use align_mind_server::models::response_model::Response;
use align_mind_server::models::think_model::*;

use rocket::response::status;
use rocket::serde::json::Json;
use uuid::Uuid;

#[get("/<uid_trash>")]
pub fn getting_trash(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_trash: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let result_trash: Option<TrashThink> = get_trash_think(uid_trash);
    response_api_entity(result_trash)
}

#[post("/<uid_trash>")]
pub fn restore_think(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_trash: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let result_action: bool = remove_of_trash(uid_trash);
    response_api_bool(result_action)
}
