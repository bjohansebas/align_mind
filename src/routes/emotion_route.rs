use crate::services::emotion_service::*;
use align_mind_server::models::emotion_model::*;

use rocket::serde::json::Json;
use uuid::Uuid;

#[get("/<id_emotion>")]
pub fn getting_emotion(id_emotion: String) -> Json<Emotion> {
    let uuid_emotion: Uuid = Uuid::parse_str(id_emotion.as_str()).unwrap();
    Json(get_emotion(uuid_emotion))
}

#[post("/", data = "<payload>")]
pub fn save_emotion(payload: Json<NewEmotion>) -> Json<Emotion> {
    Json(create_emotion(payload.into_inner()))
}

#[put("/<id_emotion>", data = "<payload>")]
pub fn updating_emotion(id_emotion: String, payload: Json<UpdateEmotion>) {
    let uuid_emotion: Uuid = Uuid::parse_str(id_emotion.as_str()).unwrap();
    update_emotion(uuid_emotion, payload.into_inner())
}

#[delete("/<id_emotion>")]
pub fn deleting_emotion(id_emotion: String) {
    let uuid_emotion: Uuid = Uuid::parse_str(id_emotion.as_str()).unwrap();
    delete_emotion(uuid_emotion)
}
