use crate::{
    jwt::UserToken,
    services::place_service::*,
    utils::responde_request::{response_api_bool, response_api_entity},
};
use align_mind_server::models::{place_model::*, response_model::Response};

use rocket::{http::Status, response::status, serde::json::Json};
use uuid::Uuid;

#[get("/<id_place>")]
pub fn getting_place(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_place: String,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let uuid_place = Uuid::parse_str(id_place.as_str());
    if let Ok(uuid) = uuid_place {
        let result_place = get_place(uuid);
        response_api_entity(result_place)
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
pub fn save_place(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_user: String,
    payload: Json<NewPlace>,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let uuid_user = Uuid::parse_str(id_user.as_str());
    if let Ok(uuid) = uuid_user {
        let result_action = create_place(uuid, payload.into_inner());
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

#[put("/<id_place>", data = "<payload>")]
pub fn updating_place(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_place: String,
    payload: Json<UpdatePlace>,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let uuid_place = Uuid::parse_str(id_place.as_str());
    if let Ok(uuid) = uuid_place {
        let result_action = update_place(uuid, payload.into_inner());
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

#[delete("/<id_place>")]
pub fn deleting_place(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_place: String,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let uuid_place = Uuid::parse_str(id_place.as_str());
    if let Ok(uuid) = uuid_place {
        let result_action = delete_place(uuid);
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
