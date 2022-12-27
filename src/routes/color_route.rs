use crate::services::color_service::*;
use align_mind_server::models::color_model::*;

use rocket::serde::json::Json;
use uuid::Uuid;

#[get("/<id_color>")]
pub fn getting_color(id_color: String) -> Json<Color> {
    let uuid_color: Uuid = Uuid::parse_str(id_color.as_str()).unwrap();
    Json(get_color(uuid_color))
}

#[post("/", data = "<payload>")]
pub fn save_color(payload: Json<NewColor>) -> Json<Color> {
    Json(create_color(payload.into_inner()))
}

#[put("/<id_color>", data = "<payload>")]
pub fn updating_color(id_color: String, payload: Json<UpdateColor>) {
    let uuid_color: Uuid = Uuid::parse_str(id_color.as_str()).unwrap();
    update_color(uuid_color, payload.into_inner())
}

#[delete("/<id_color>")]
pub fn deleting_color(id_color: String) {
    let uuid_color: Uuid = Uuid::parse_str(id_color.as_str()).unwrap();
    delete_color(uuid_color)
}
