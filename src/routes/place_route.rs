use crate::services::place_service::*;
use align_mind_server::models::place_model::*;

use rocket::serde::json::Json;
use uuid::Uuid;

#[get("/<id_place>")]
pub fn getting_place(id_place: String) -> Json<Place> {
    let uuid_place: Uuid = Uuid::parse_str(id_place.as_str()).unwrap();
    Json(get_place(uuid_place))
}

#[post("/<id_user>", data = "<payload>")]
pub fn save_place(id_user: String, payload: Json<NewPlace>) -> Json<Place> {
    let uuid_user: Uuid = Uuid::parse_str(id_user.as_str()).unwrap();
    Json(create_place(uuid_user, payload.into_inner()))
}

#[put("/<id_place>", data = "<payload>")]
pub fn updating_place(id_place: String, payload: Json<UpdatePlace>) {
    let uuid_place: Uuid = Uuid::parse_str(id_place.as_str()).unwrap();
    update_place(uuid_place, payload.into_inner())
}

#[delete("/<id_place>")]
pub fn deleting_place(id_place: String) {
    let uuid_place: Uuid = Uuid::parse_str(id_place.as_str()).unwrap();
    delete_place(uuid_place)
}
