use crate::jwt::UserToken;
use crate::services::statistics_service::{
    get_emotion_filter, get_postive_and_negative, get_postive_and_negative_filtre,
};
use crate::utils::responde_request::response_api_data;

use align_mind_server::establish_connection;
use align_mind_server::models::response_model::Response;

use diesel::PgConnection;
use rocket::response::status;
use rocket::serde::json::Json;
use uuid::Uuid;

#[get("/<uid_user>")]
pub fn getting_positive_and_negative(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_user: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_trash = get_postive_and_negative(uid_user, connection);
    response_api_data(result_trash)
}

#[get("/<uid_user>/negative")]
pub fn getting_negative(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_user: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_trash = get_emotion_filter(uid_user, "Negative", connection);
    response_api_data(result_trash)
}

#[get("/<uid_user>/positive")]
pub fn getting_positive(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_user: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_trash = get_emotion_filter(uid_user, "Positive", connection);
    response_api_data(result_trash)
}

#[get("/<uid_user>/all")]
pub fn getting_filter_all(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_user: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_trash = get_postive_and_negative_filtre(uid_user, connection);
    response_api_data(result_trash)
}
