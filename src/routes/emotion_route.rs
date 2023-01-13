use crate::jwt::UserToken;
use crate::services::emotion_service::*;
use crate::utils::responde_request::{response_api, response_api_data};

use align_mind_server::establish_connection;
use align_mind_server::models::emotion_model::*;
use align_mind_server::models::response_model::{Response, ResponseError, ResponseSuccess};

use diesel::PgConnection;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket_validation::Validated;

#[get("/")]
pub fn getting_emotions(
    token: Result<UserToken, status::Custom<Json<Response>>>,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_emotion: Result<Vec<Emotion>, ResponseError> = get_all_emotion(connection);
    response_api_data(result_emotion)
}

#[get("/<uid_emotion>")]
pub fn getting_emotion(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_emotion: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_emotion: Result<Emotion, ResponseError> = get_emotion(uid_emotion, connection);
    response_api_data(result_emotion)
}

#[post("/", format = "application/json", data = "<payload>")]
pub fn save_emotion(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    payload: Validated<Json<NewEmotionDTO>>,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_action: Result<ResponseSuccess, ResponseError> =
        create_emotion(payload.into_inner().into_inner(), connection);

    response_api(result_action)
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

    let connection: &mut PgConnection = &mut establish_connection();

    let result_action: Result<ResponseSuccess, ResponseError> =
        update_emotion(uid_emotion, payload.into_inner().into_inner(), connection);

    response_api(result_action)
}

#[delete("/<uid_emotion>")]
pub fn deleting_emotion(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_emotion: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_action: Result<ResponseSuccess, ResponseError> =
        delete_emotion(uid_emotion, connection);

    response_api(result_action)
}
