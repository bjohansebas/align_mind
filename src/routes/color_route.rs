use crate::{
    jwt::UserToken,
    services::color_service::*,
    utils::responde_request::{response_api_bool, response_api_entity},
};
use align_mind_server::models::{color_model::*, response_model::Response};

use rocket::{http::Status, response::status, serde::json::Json};
use uuid::Uuid;

#[get("/<id_color>")]
pub fn getting_color(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_color: String,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let uuid_color = Uuid::parse_str(id_color.as_str());
    if let Ok(uuid) = uuid_color {
        let result_color = get_color(uuid);
        response_api_entity(result_color)
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
pub fn save_color(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_user: String,
    payload: Json<NewColor>,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let uuid_user = Uuid::parse_str(id_user.as_str());

    if let Ok(uuid) = uuid_user {
        let result_action = create_color(Some(uuid), payload.into_inner());
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

#[put("/<id_color>", data = "<payload>")]
pub fn updating_color(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_color: String,
    payload: Json<UpdateColor>,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let uuid_color = Uuid::parse_str(id_color.as_str());
    if let Ok(uuid) = uuid_color {
        let result_action = update_color(uuid, payload.into_inner());
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

#[delete("/<id_color>")]
pub fn deleting_color(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_color: String,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let uuid_color = Uuid::parse_str(id_color.as_str());
    if let Ok(uuid) = uuid_color {
        let result_action = delete_color(uuid);
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
