use crate::jwt::UserToken;
use crate::services::color_service::get_colors_with_user_uuid;
use crate::services::place_service::get_places_with_user_uuid;
use crate::services::think_service::{
    get_archive_think, get_thinks_with_user_uuid, get_unarchive_think,
};
use crate::services::trash_service::get_trash_thinks_with_user_uuid;
use crate::services::users_service::*;
use crate::utils::responde_request::{response_message_api, response_value_api};

use align_mind_server::establish_connection;
use align_mind_server::models::color_model::Color;
use align_mind_server::models::place_model::Place;
use align_mind_server::models::response_model::{ResponseMessage, ResponseValue};
use align_mind_server::models::think_model::{Think, TrashThink};
use align_mind_server::models::user_model::*;

use diesel::PgConnection;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket_validation::Validated;
use serde_json::Value;

#[get("/account", format = "application/json")]
pub fn getting_user(
    token: Result<UserToken, status::Custom<Json<ResponseMessage>>>,
) -> Result<status::Custom<Json<Value>>, status::Custom<Json<ResponseMessage>>> {
    let token: UserToken = token?;

    let connection: &mut PgConnection = &mut establish_connection();

    let result_user: Result<ResponseValue<User>, ResponseMessage> =
        get_user_account(token.sub, connection);

    Ok(response_value_api(result_user))
}

#[get("/profile", format = "application/json")]
pub fn getting_profile(
    token: Result<UserToken, status::Custom<Json<ResponseMessage>>>,
) -> Result<status::Custom<Json<Value>>, status::Custom<Json<ResponseMessage>>> {
    let token: UserToken = token?;

    let connection: &mut PgConnection = &mut establish_connection();

    let result_profile: Result<ResponseValue<ProfileUser>, ResponseMessage> =
        get_user_profile(token.sub, connection);

    Ok(response_value_api(result_profile))
}

#[get("/places")]
pub fn getting_places_of_user(
    token: Result<UserToken, status::Custom<Json<ResponseMessage>>>,
) -> Result<status::Custom<Json<Value>>, status::Custom<Json<ResponseMessage>>> {
    if let Err(e) = token {
        return Err(e);
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_place: Result<ResponseValue<Vec<Place>>, ResponseMessage> =
        get_places_with_user_uuid(token.unwrap().sub, connection);

    Ok(response_value_api(result_place))
}

#[get("/colors")]
pub fn getting_colors_of_user(
    token: Result<UserToken, status::Custom<Json<ResponseMessage>>>,
) -> Result<status::Custom<Json<Value>>, status::Custom<Json<ResponseMessage>>> {
    if let Err(e) = token {
        return Err(e);
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_color: Result<ResponseValue<Vec<Color>>, ResponseMessage> =
        get_colors_with_user_uuid(token.unwrap().sub, connection);

    Ok(response_value_api(result_color))
}

#[get("/thinks")]
pub fn getting_thinks_of_user(
    token: Result<UserToken, status::Custom<Json<ResponseMessage>>>,
) -> Result<status::Custom<Json<Value>>, status::Custom<Json<ResponseMessage>>> {
    if let Err(e) = token {
        return Err(e);
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_think: Result<ResponseValue<Vec<Think>>, ResponseMessage> =
        get_thinks_with_user_uuid(token.unwrap().sub, connection);

    Ok(response_value_api(result_think))
}

#[get("/trash")]
pub fn getting_trash_of_user(
    token: Result<UserToken, status::Custom<Json<ResponseMessage>>>,
) -> Result<status::Custom<Json<Value>>, status::Custom<Json<ResponseMessage>>> {
    if let Err(e) = token {
        return Err(e);
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_trash: Result<ResponseValue<Vec<TrashThink>>, ResponseMessage> =
        get_trash_thinks_with_user_uuid(token.unwrap().sub, connection);

    Ok(response_value_api(result_trash))
}

#[get("/unarchives")]
pub fn getting_unarchive_think(
    token: Result<UserToken, status::Custom<Json<ResponseMessage>>>,
) -> Result<status::Custom<Json<Value>>, status::Custom<Json<ResponseMessage>>> {
    if let Err(e) = token {
        return Err(e);
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_think: Result<ResponseValue<Vec<Think>>, ResponseMessage> =
        get_unarchive_think(token.unwrap().sub, connection);

    Ok(response_value_api(result_think))
}

#[get("/archives")]
pub fn getting_archive_think(
    token: Result<UserToken, status::Custom<Json<ResponseMessage>>>,
) -> Result<status::Custom<Json<Value>>, status::Custom<Json<ResponseMessage>>> {
    if let Err(e) = token {
        return Err(e);
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_think: Result<ResponseValue<Vec<Think>>, ResponseMessage> =
        get_archive_think(token.unwrap().sub, connection);

    Ok(response_value_api(result_think))
}

#[post("/profile", format = "application/json", data = "<payload>")]
pub fn saving_profile(
    token: Result<UserToken, status::Custom<Json<ResponseMessage>>>,
    payload: Validated<Json<NewProfileUserDTO>>,
) -> status::Custom<Json<ResponseMessage>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let action: ResponseMessage = create_profile(
        token.unwrap().sub,
        payload.into_inner().into_inner(),
        connection,
    );

    response_message_api(action)
}

#[put("/account", format = "application/json", data = "<payload>")]
pub fn updating_user(
    token: Result<UserToken, status::Custom<Json<ResponseMessage>>>,
    payload: Validated<Json<UpdateUserDTO>>,
) -> status::Custom<Json<ResponseMessage>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let action: ResponseMessage = update_user(
        token.unwrap().sub,
        payload.into_inner().into_inner(),
        connection,
    );

    response_message_api(action)
}

#[put("/profile", format = "application/json", data = "<payload>")]
pub fn updating_profile_user(
    token: Result<UserToken, status::Custom<Json<ResponseMessage>>>,
    payload: Validated<Json<UpdateProfileUserDTO>>,
) -> status::Custom<Json<ResponseMessage>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let action: ResponseMessage = update_profile(
        token.unwrap().sub,
        payload.into_inner().into_inner(),
        connection,
    );

    response_message_api(action)
}

#[put("/password", format = "application/json", data = "<payload>")]
pub fn updating_password_user(
    token: Result<UserToken, status::Custom<Json<ResponseMessage>>>,
    payload: Validated<Json<UpdatePasswordDTO>>,
) -> status::Custom<Json<ResponseMessage>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let action: ResponseMessage = update_password(
        token.unwrap().sub,
        payload.into_inner().into_inner(),
        connection,
    );
    response_message_api(action)
}

#[delete("/")]
pub fn delete_account(
    token: Result<UserToken, status::Custom<Json<ResponseMessage>>>,
) -> status::Custom<Json<ResponseMessage>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let action: ResponseMessage = delete_user_with_profile(token.unwrap().sub, connection);
    response_message_api(action)
}
