use crate::{
    jwt::UserToken,
    services::emotion_service::*,
    utils::responde_request::{response_api_bool, response_api_entity},
};
use align_mind_server::models::{emotion_model::*, response_model::Response};

use rocket::{http::Status, response::status, serde::json::Json};
use uuid::Uuid;

#[get("/<id_emotion>")]
pub fn getting_emotion(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_emotion: String,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let uuid_emotion = Uuid::parse_str(id_emotion.as_str());
    if let Ok(uuid) = uuid_emotion {
        let result_emotion: Option<Emotion> = get_emotion(uuid);
        response_api_entity(result_emotion)
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

#[post("/", data = "<payload>")]
pub fn save_emotion(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    payload: Json<NewEmotion>,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let result_action: bool = create_emotion(payload.into_inner());

    if result_action {
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

#[put("/<id_emotion>", data = "<payload>")]
pub fn updating_emotion(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_emotion: String,
    payload: Json<UpdateEmotion>,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let uuid_emotion = Uuid::parse_str(id_emotion.as_str());
    if let Ok(uuid) = uuid_emotion {
        let result_action: bool = update_emotion(uuid, payload.into_inner());
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

#[delete("/<id_emotion>")]
pub fn deleting_emotion(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_emotion: String,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let uuid_emotion = Uuid::parse_str(id_emotion.as_str());
    if let Ok(uuid) = uuid_emotion {
        let result_action = delete_emotion(uuid);
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
