use crate::jwt::UserToken;
use crate::services::color_service::*;
use crate::utils::responde_request::{response_message_api, response_value_api};

use align_mind_server::establish_connection;
use align_mind_server::models::color_model::*;
use align_mind_server::models::response_model::{ResponseMessage, ResponseValue};

use diesel::PgConnection;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket_validation::Validated;
use serde_json::Value;

#[get("/<uid_color>", format = "application/json")]
pub fn getting_color(
    token: Result<UserToken, status::Custom<Json<ResponseMessage>>>,
    uid_color: Uuid,
) -> Result<status::Custom<Json<Value>>, status::Custom<Json<ResponseMessage>>> {
    if let Err(e) = token {
        return Err(e);
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_color: Result<ResponseValue<Color>, ResponseMessage> =
        get_color(uid_color, connection);
    Ok(response_value_api(result_color))
}

#[post("/", format = "application/json", data = "<payload>")]
pub fn saving_color(
    token: Result<UserToken, status::Custom<Json<ResponseMessage>>>,
    payload: Validated<Json<NewColorDTO>>,
) -> status::Custom<Json<ResponseMessage>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_action: ResponseMessage = create_color(
        token.unwrap().sub,
        payload.into_inner().into_inner(),
        connection,
    );

    response_message_api(result_action)
}

#[put("/<uid_color>", format = "application/json", data = "<payload>")]
pub fn updating_color(
    token: Result<UserToken, status::Custom<Json<ResponseMessage>>>,
    uid_color: Uuid,
    payload: Validated<Json<UpdateColorDTO>>,
) -> status::Custom<Json<ResponseMessage>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_action: ResponseMessage =
        update_color(uid_color, payload.into_inner().into_inner(), connection);

    response_message_api(result_action)
}

#[delete("/<uid_color>")]
pub fn deleting_color(
    token: Result<UserToken, status::Custom<Json<ResponseMessage>>>,
    uid_color: Uuid,
) -> status::Custom<Json<ResponseMessage>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_action: ResponseMessage = delete_color(uid_color, connection);

    response_message_api(result_action)
}
