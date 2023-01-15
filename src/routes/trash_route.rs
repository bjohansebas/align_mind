use crate::jwt::UserToken;
use crate::services::trash_service::*;
use crate::utils::responde_request::{response_api, response_api_data};

use align_mind_server::establish_connection;
use align_mind_server::models::emotion_model::Emotion;
use align_mind_server::models::response_model::{Response, ResponseError, ResponseSuccess};
use align_mind_server::models::think_model::*;

use diesel::PgConnection;
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

    let connection: &mut PgConnection = &mut establish_connection();

    let result_trash: Result<TrashThink, ResponseError> = get_trash_think(uid_trash, connection);
    response_api_data(result_trash)
}

#[get("/<uid_trash>/emotions")]
pub fn getting_trash_emotions(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_trash: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_think: Result<Vec<Emotion>, ResponseError> =
        get_trash_emotions(uid_trash, connection);
    response_api_data(result_think)
}

#[post("/<uid_trash>")]
pub fn restore_think(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_trash: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_action: Result<ResponseSuccess, ResponseError> =
        remove_of_trash(uid_trash, connection);
    response_api(result_action)
}

#[delete("/<uid_trash>")]
pub fn deleting_trash(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_trash: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_action: Result<ResponseSuccess, ResponseError> = delete_trash(uid_trash, connection);
    response_api(result_action)
}
