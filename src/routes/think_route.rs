use crate::jwt::UserToken;
use crate::services::think_service::*;
use crate::utils::responde_request::{response_api_bool, response_api_entity};
use align_mind_server::models::response_model::Response;
use align_mind_server::models::think_model::*;

use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use uuid::Uuid;

#[get("/<id_think>")]
pub fn getting_think(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_think: String,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let uuid_think = Uuid::parse_str(id_think.as_str());
    if let Ok(uuid) = uuid_think {
        let result_think = get_think(uuid);
        response_api_entity(result_think)
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

#[post("/<id_user>", data = "<payload>")]
pub fn save_think(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_user: String,
    payload: Json<NewThink>,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let uuid_user = Uuid::parse_str(id_user.as_str());
    if let Ok(uuid) = uuid_user {
        let result_action = create_think(uuid, payload.into_inner());
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

#[post("/<id_think>/trash")]
pub fn move_to_trash(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_think: String,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }
    let uuid_think = Uuid::parse_str(id_think.as_str());
    if let Ok(uuid) = uuid_think {
        let result_action = move_think_to_trash(uuid);
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

#[put("/<id_think>", data = "<payload>")]
pub fn updating_think(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_think: String,
    payload: Json<UpdateThink>,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let uuid_think = Uuid::parse_str(id_think.as_str());

    if let Ok(uuid) = uuid_think {
        let result_action = update_think(uuid, payload.into_inner());
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

#[delete("/<id_think>")]
pub fn deleting_think(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_think: String,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }
    let uuid_think = Uuid::parse_str(id_think.as_str());
    if let Ok(uuid) = uuid_think {
        let result_action = delete_think(uuid);
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
