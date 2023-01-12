use crate::jwt::UserToken;
use crate::services::color_service::get_colors_with_user_uuid;
use crate::services::place_service::get_places_with_user_uuid;
use crate::services::think_service::{
    get_archive_think, get_thinks_with_user_uuid, get_unarchive_think,
};
use crate::services::trash_service::get_trash_thinks_with_user_uuid;
use crate::services::users_service::*;
use crate::utils::responde_request::{response_api, response_api_data};

use align_mind_server::establish_connection;
use align_mind_server::models::color_model::Color;
use align_mind_server::models::place_model::Place;
use align_mind_server::models::response_model::{Response, ResponseError, ResponseSuccess};
use align_mind_server::models::think_model::{Think, TrashThink};
use align_mind_server::models::user_model::*;

use diesel::PgConnection;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket_validation::Validated;

#[get("/<uid_user>")]
pub fn getting_user(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_user: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_user: Result<User, ResponseError> = get_user(uid_user, connection);
    response_api_data(result_user)
}

#[get("/<uid_user>/profile")]
pub fn getting_profile(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_user: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_profile: Result<ProfileUser, ResponseError> = get_user_profile(uid_user, connection);
    response_api_data(result_profile)
}

#[get("/<uid_user>/places")]
pub fn getting_places_of_user(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_user: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_place: Result<Vec<Place>, ResponseError> =
        get_places_with_user_uuid(uid_user, connection);

    response_api_data(result_place)
}

#[get("/<uid_user>/colors")]
pub fn getting_colors_of_user(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_user: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_color: Result<Vec<Color>, ResponseError> =
        get_colors_with_user_uuid(uid_user, connection);
    response_api_data(result_color)
}

#[get("/<uid_user>/thinks")]
pub fn getting_thinks_of_user(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_user: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_think: Result<Vec<Think>, ResponseError> =
        get_thinks_with_user_uuid(uid_user, connection);
    response_api_data(result_think)
}

#[get("/<uid_user>/trash")]
pub fn getting_trash_of_user(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_user: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_trash: Result<Vec<TrashThink>, ResponseError> =
        get_trash_thinks_with_user_uuid(uid_user, connection);
    response_api_data(result_trash)
}

#[get("/<uid_user>/unarchives")]
pub fn getting_unarchive_think(
    uid_user: Uuid,
    token: Result<UserToken, status::Custom<Json<Response>>>,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_think: Result<Vec<Think>, ResponseError> = get_unarchive_think(uid_user, connection);
    response_api_data(result_think)
}

#[get("/<uid_user>/archives")]
pub fn getting_archive_think(
    uid_user: Uuid,
    token: Result<UserToken, status::Custom<Json<Response>>>,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let result_think: Result<Vec<Think>, ResponseError> = get_archive_think(uid_user, connection);
    response_api_data(result_think)
}

#[post("/<uid_user>/profile", format = "application/json", data = "<payload>")]
pub fn save_profile(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_user: Uuid,
    payload: Validated<Json<NewProfileUserDTO>>,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let action: Result<ResponseSuccess, ResponseError> =
        create_profile(uid_user, payload.into_inner().into_inner(), connection);
    response_api(action)
}

#[put("/<uid_user>", format = "application/json", data = "<payload>")]
pub fn updating_user(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_user: Uuid,
    payload: Validated<Json<UpdateUserDTO>>,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let action: Result<ResponseSuccess, ResponseError> =
        update_user(uid_user, payload.into_inner().into_inner(), connection);
    response_api(action)
}

#[put("/<uid_user>/profile", format = "application/json", data = "<payload>")]
pub fn update_profile_user(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_user: Uuid,
    payload: Validated<Json<UpdateProfileUserDTO>>,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let action: Result<ResponseSuccess, ResponseError> =
        update_profile(uid_user, payload.into_inner().into_inner(), connection);
    response_api(action)
}

#[delete("/<uid_user>")]
pub fn delete_account(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_user: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let connection: &mut PgConnection = &mut establish_connection();

    let action: Result<ResponseSuccess, ResponseError> =
        delete_user_with_profile(uid_user, connection);
    response_api(action)
}
