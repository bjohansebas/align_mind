use crate::{
    jwt::UserToken,
    services::emotion_service::*,
    utils::responde_request::{response_api_bool, response_api_entity},
};
use align_mind_server::models::{emotion_model::*, response_model::Response};

use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket_validation::Validated;

#[get("/<uid_emotion>")]
pub fn getting_emotion(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_emotion: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let result_emotion: Option<Emotion> = get_emotion(uid_emotion);
    response_api_entity(result_emotion)
}

#[post("/", format = "application/json", data = "<payload>")]
pub fn save_emotion(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    payload: Validated<Json<NewEmotionDTO>>,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let result_action: bool = create_emotion(payload.into_inner().into_inner());
    response_api_bool(result_action)
}

#[put("/<uid_emotion>", format = "application/json", data = "<payload>")]
pub fn updating_emotion(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_emotion: Uuid,
    payload: Validated<Json<UpdateEmotionDTO>>,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let result_action: bool = update_emotion(uid_emotion, payload.into_inner().into_inner());
    response_api_bool(result_action)
}

#[delete("/<uid_emotion>")]
pub fn deleting_emotion(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_emotion: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let result_action: bool = delete_emotion(uid_emotion);
    response_api_bool(result_action)
}
