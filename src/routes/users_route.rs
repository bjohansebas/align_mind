use crate::{
    jwt::UserToken,
    services::{
        place_service::get_places_with_user_uuid,
        think_service::{get_thinks_with_user_uuid, get_trash_thinks_with_user_uuid},
        users_service::*,
    },
};
use align_mind_server::models::{
    place_model::Place,
    response_model::Response,
    think_model::{Think, TrashThink},
    user_model::*,
};

use rocket::{http::Status, response::status, serde::json::Json};
use uuid::Uuid;

#[get("/<id_user>")]
pub fn getting_user(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_user: String,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }
    let uuid_user: Uuid = Uuid::parse_str(id_user.as_str()).unwrap();
    let user = get_user(uuid_user);
    status::Custom(
        Status::from_code(Status::Ok.code).unwrap(),
        Json(Response {
            message: String::from(""),
            data: serde_json::to_value(user).unwrap(),
        }),
    )
}

#[get("/<id_user>/profile")]
pub fn getting_profile(id_user: String) -> Json<ProfileUser> {
    let uuid_user: Uuid = Uuid::parse_str(id_user.as_str()).unwrap();
    Json(get_user_profile(uuid_user))
}

#[get("/<id_user>/places")]
pub fn getting_places_of_user(id_user: String) -> Json<Vec<Place>> {
    let uuid_user: Uuid = Uuid::parse_str(id_user.as_str()).unwrap();
    Json(get_places_with_user_uuid(uuid_user))
}

#[get("/<id_user>/thinks")]
pub fn getting_thinks_of_user(id_user: String) -> Json<Vec<Think>> {
    let uuid_user: Uuid = Uuid::parse_str(id_user.as_str()).unwrap();
    Json(get_thinks_with_user_uuid(uuid_user))
}

#[get("/<id_user>/trash")]
pub fn getting_trash_of_user(id_user: String) -> Json<Vec<TrashThink>> {
    let uuid_user: Uuid = Uuid::parse_str(id_user.as_str()).unwrap();
    Json(get_trash_thinks_with_user_uuid(uuid_user))
}

#[delete("/<id_user>")]
pub fn delete_account(id_user: String) {
    let uuid_user: Uuid = Uuid::parse_str(id_user.as_str()).unwrap();
    delete_user_with_profile(uuid_user)
}

#[put("/<id_user>", data = "<payload>")]
pub fn updating_user(id_user: String, payload: Json<UpdateUser>) {
    let uuid_user: Uuid = Uuid::parse_str(id_user.as_str()).unwrap();
    update_user(uuid_user, payload.into_inner())
}

#[put("/<id_user>/profile", data = "<payload>")]
pub fn update_profile_user(id_user: String, payload: Json<UpdateProfileUser>) {
    let uuid_user: Uuid = Uuid::parse_str(id_user.as_str()).unwrap();
    update_profile(uuid_user, payload.into_inner())
}
