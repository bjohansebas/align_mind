use crate::jwt::UserToken;
use crate::services::color_service::get_colors_with_user_uuid;
use crate::services::place_service::get_places_with_user_uuid;
use crate::services::think_service::get_thinks_with_user_uuid;
use crate::services::trash_service::get_trash_thinks_with_user_uuid;
use crate::services::users_service::*;
use crate::utils::responde_request::{response_api_bool, response_api_entity};

use align_mind_server::models::color_model::Color;
use align_mind_server::models::place_model::Place;
use align_mind_server::models::response_model::Response;
use align_mind_server::models::think_model::{Think, TrashThink};
use align_mind_server::models::user_model::*;

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

    let result_user: Option<User> = get_user(uid_user);
    response_api_entity(result_user)
}

#[get("/<uid_user>/profile")]
pub fn getting_profile(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_user: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let result_profile: Option<ProfileUser> = get_user_profile(uid_user);
    response_api_entity(result_profile)
}

#[get("/<uid_user>/places")]
pub fn getting_places_of_user(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_user: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let result_place: Option<Vec<Place>> = get_places_with_user_uuid(uid_user);
    response_api_entity(result_place)
}

#[get("/<uid_user>/colors")]
pub fn getting_colors_of_user(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_user: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let result_color: Option<Vec<Color>> = get_colors_with_user_uuid(uid_user);
    response_api_entity(result_color)
}

#[get("/<uid_user>/thinks")]
pub fn getting_thinks_of_user(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_user: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let result_think: Option<Vec<Think>> = get_thinks_with_user_uuid(uid_user);
    response_api_entity(result_think)
}

#[get("/<uid_user>/trash")]
pub fn getting_trash_of_user(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_user: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let result_trash: Option<Vec<TrashThink>> = get_trash_thinks_with_user_uuid(uid_user);
    response_api_entity(result_trash)
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
    let data_new_profile: NewProfileUserDTO = payload.into_inner().into_inner();
    let new_profile: NewProfileUser = NewProfileUser {
        first_name: data_new_profile.first_name.unwrap(),
        gender: data_new_profile.gender.unwrap(),
        preference_lang: data_new_profile.preference_lang.unwrap(),
        last_name: data_new_profile.last_name,
        photo_url: data_new_profile.photo_url,
        user_id: uid_user,
        years_old: data_new_profile.years_old,
    };
    let action: bool = create_profile(uid_user, new_profile);
    response_api_bool(action)
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

    let data_update: UpdateUserDTO = payload.into_inner().into_inner();
    let data_user: UpdateUser = UpdateUser {
        username: data_update.username,
        email: data_update.email,
        password: data_update.password,
        changed_password_at: None,
        updated_at: None,
    };
    let action: bool = update_user(uid_user, data_user);
    response_api_bool(action)
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

    let data_update: UpdateProfileUserDTO = payload.into_inner().into_inner();
    let data_profile: UpdateProfileUser = UpdateProfileUser {
        first_name: data_update.first_name,
        gender: data_update.gender,
        last_name: data_update.last_name,
        photo_url: data_update.photo_url,
        preference_lang: data_update.preference_lang,
        updated_at: None,
        years_old: data_update.years_old,
    };

    let action: bool = update_profile(uid_user, data_profile);
    response_api_bool(action)
}

#[delete("/<uid_user>")]
pub fn delete_account(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    uid_user: Uuid,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let action: bool = delete_user_with_profile(uid_user);
    response_api_bool(action)
}
