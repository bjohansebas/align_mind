use crate::jwt::UserToken;
use crate::services::emotion_service::*;
use crate::utils::responde_request::{response_message_api, response_value_api};

use align_mind_server::establish_connection;
use align_mind_server::models::emotion_model::*;
use align_mind_server::models::response_model::{ResponseMessage, ResponseValue};

use diesel::PgConnection;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket_validation::Validated;
use serde_json::Value;

#[get("/")]
pub fn getting_emotions(
    token: Result<UserToken, status::Custom<Json<ResponseMessage>>>,
) -> Result<status::Custom<Json<Value>>, status::Custom<Json<ResponseMessage>>> {
    if let Err(e) = token {
        return Err(e);
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_emotion: Result<ResponseValue<Vec<Emotion>>, ResponseMessage> =
        get_all_emotion(connection);

    Ok(response_value_api(result_emotion))
}

#[get("/<uid_emotion>")]
pub fn getting_emotion(
    token: Result<UserToken, status::Custom<Json<ResponseMessage>>>,
    uid_emotion: Uuid,
) -> Result<status::Custom<Json<Value>>, status::Custom<Json<ResponseMessage>>> {
    if let Err(e) = token {
        return Err(e);
    }
    let connection: &mut PgConnection = &mut establish_connection();

    let result_emotion: Result<ResponseValue<Emotion>, ResponseMessage> =
        get_emotion(uid_emotion, connection);

    Ok(response_value_api(result_emotion))
}

#[post("/", format = "application/json", data = "<payload>")]
pub fn save_emotion(
    token: Result<UserToken, status::Custom<Json<ResponseMessage>>>,
    payload: Validated<Json<NewEmotionDTO>>,
) -> status::Custom<Json<ResponseMessage>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_action: ResponseMessage =
        create_emotion(payload.into_inner().into_inner(), connection);

    response_message_api(result_action)
}

#[put("/<uid_emotion>", format = "application/json", data = "<payload>")]
pub fn updating_emotion(
    token: Result<UserToken, status::Custom<Json<ResponseMessage>>>,
    uid_emotion: Uuid,
    payload: Validated<Json<UpdateEmotionDTO>>,
) -> status::Custom<Json<ResponseMessage>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_action: ResponseMessage =
        update_emotion(uid_emotion, payload.into_inner().into_inner(), connection);

    response_message_api(result_action)
}

#[delete("/<uid_emotion>")]
pub fn deleting_emotion(
    token: Result<UserToken, status::Custom<Json<ResponseMessage>>>,
    uid_emotion: Uuid,
) -> status::Custom<Json<ResponseMessage>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_action: ResponseMessage = delete_emotion(uid_emotion, connection);

    response_message_api(result_action)
}
