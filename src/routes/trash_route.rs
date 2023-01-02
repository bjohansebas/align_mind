use crate::jwt::UserToken;
use crate::services::trash_service::*;
use crate::utils::responde_request::{response_api_bool, response_api_entity};
use align_mind_server::models::response_model::Response;
use align_mind_server::models::think_model::*;

use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use uuid::Uuid;

#[get("/<id_trash>")]
pub fn getting_trash(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_trash: String,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let uuid_trash = Uuid::parse_str(id_trash.as_str());
    if let Ok(uuid) = uuid_trash {
        let result_trash: Option<TrashThink> = get_trash_think(uuid);
        response_api_entity(result_trash)
    } else {
        status::Custom(
            Status::from_code(Status::BadRequest.code).unwrap(),
            Json(Response {
                message: String::from("That is not uuid"),
                data: serde_json::to_value("").unwrap(),
            }),
        )
    }
}

#[post("/<id_trash>")]
pub fn restore_think(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_trash: String,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let uuid_trash = Uuid::parse_str(id_trash.as_str());
    if let Ok(uuid) = uuid_trash {
        let result_action = remove_of_trash(uuid);
        response_api_bool(result_action)
    } else {
        status::Custom(
            Status::from_code(Status::BadRequest.code).unwrap(),
            Json(Response {
                message: String::from("That is not uuid"),
                data: serde_json::to_value("").unwrap(),
            }),
        )
    }
}
