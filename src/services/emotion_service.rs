use super::color_service::get_color;

use align_mind_server::models::color_model::Color;
use align_mind_server::models::emotion_model::*;
use align_mind_server::models::response_model::{ResponseMessage, ResponseValue};
use align_mind_server::schema::emotions;

use chrono::Utc;
use diesel::prelude::*;
use rocket::http::Status;
use uuid::Uuid;

pub fn get_emotion(
    uuid_emotion: Uuid,
    conn: &mut PgConnection,
) -> Result<ResponseValue<Emotion>, ResponseMessage> {
    emotions::table
        .filter(emotions::emotion_id.eq(uuid_emotion))
        .first::<Emotion>(conn)
        .map(|emotion| ResponseValue {
            code: Status::Accepted.code,
            value: emotion,
        })
        .map_err(|_| ResponseMessage {
            code: Some(Status::NotFound.code),
            message: "The emotion not found".to_string(),
        })
}

pub fn get_emotion_with_text(
    text_emotion: String,
    conn: &mut PgConnection,
) -> Result<ResponseValue<Emotion>, ResponseMessage> {
    emotions::table
        .filter(emotions::name_emotion.eq(text_emotion))
        .first::<Emotion>(conn)
        .map(|emotion| ResponseValue {
            code: Status::Accepted.code,
            value: emotion,
        })
        .map_err(|_| ResponseMessage {
            code: Some(Status::NotFound.code),
            message: "The emotion not found".to_string(),
        })
}

pub fn get_all_emotion(
    conn: &mut PgConnection,
) -> Result<ResponseValue<Vec<Emotion>>, ResponseMessage> {
    emotions::table
        .load::<Emotion>(conn)
        .map(|emotion| ResponseValue {
            code: Status::Accepted.code,
            value: emotion,
        })
        .map_err(|_| ResponseMessage {
            code: Some(Status::NotFound.code),
            message: "The emotion not found".to_string(),
        })
}

pub fn create_emotion(payload: NewEmotionDTO, conn: &mut PgConnection) -> ResponseMessage {
    let result_emotion: Result<ResponseValue<Emotion>, ResponseMessage> =
        get_emotion_with_text(payload.name_emotion.to_owned().unwrap(), conn);

    if result_emotion.is_ok() {
        return ResponseMessage {
            code: Some(Status::Conflict.code),
            message: "The emotion was already created".to_string(),
        };
    }

    let uuid_color: Result<Uuid, uuid::Error> = Uuid::parse_str(payload.color_id.unwrap().as_str());

    if uuid_color.is_err() {
        return ResponseMessage {
            code: Some(Status::NotFound.code),
            message: "The color not found".to_string(),
        };
    }

    let result_color = get_color(uuid_color.unwrap(), conn);

    if let Err(e) = result_color {
        return e;
    }

    let color = result_color.unwrap().value;

    let emotion: NewEmotion = NewEmotion {
        color_id: color.color_id,
        type_emotion: payload.type_emotion.unwrap(),
        name_emotion: payload.name_emotion.unwrap(),
    };

    let insert_action: bool = diesel::insert_into(emotions::table)
        .values(&emotion)
        .execute(conn)
        .is_ok();

    if !insert_action {
        return ResponseMessage {
            code: Some(Status::BadRequest.code),
            message: "Unknow error".to_string(),
        };
    }

    ResponseMessage {
        code: Some(Status::Created.code),
        message: "The emotion has been created".to_string(),
    }
}

pub fn update_emotion(
    uuid_emotion: Uuid,
    payload: UpdateEmotionDTO,
    conn: &mut PgConnection,
) -> ResponseMessage {
    let result_emotion: Result<ResponseValue<Emotion>, ResponseMessage> =
        get_emotion(uuid_emotion, conn);

    if let Err(e) = result_emotion {
        return e;
    }

    let emotion: Emotion = result_emotion.unwrap().value;

    let mut data_emotion: UpdateEmotion = UpdateEmotion {
        name_emotion: payload.name_emotion,
        type_emotion: payload.type_emotion,
        color_id: None,
        updated_at: Some(Utc::now().naive_utc()),
    };

    if let Some(color_id) = payload.color_id {
        let uuid_color: Result<Uuid, uuid::Error> = Uuid::parse_str(color_id.as_str());

        if uuid_color.is_err() {
            return ResponseMessage {
                code: Some(Status::NotFound.code),
                message: "The color not found".to_string(),
            };
        }

        let result_color: Result<ResponseValue<Color>, ResponseMessage> =
            get_color(uuid_color.to_owned().unwrap(), conn);

        if let Err(e) = result_color {
            return e;
        }

        data_emotion.color_id = Some(uuid_color.unwrap());
    }

    let update_action = diesel::update(&emotion)
        .set(&data_emotion)
        .execute(conn)
        .is_ok();

    if !update_action {
        return ResponseMessage {
            code: Some(Status::BadRequest.code),
            message: "Unknown Error".to_string(),
        };
    }

    ResponseMessage {
        code: Some(Status::Accepted.code),
        message: "The emotion has been updated".to_string(),
    }
}

pub fn delete_emotion(uuid_emotion: Uuid, conn: &mut PgConnection) -> ResponseMessage {
    let result_emotion: Result<ResponseValue<Emotion>, ResponseMessage> =
        get_emotion(uuid_emotion, conn);

    if let Err(e) = result_emotion {
        return e;
    }

    let delete_action: bool = diesel::delete(&result_emotion.unwrap().value)
        .execute(conn)
        .is_ok();

    if !delete_action {
        return ResponseMessage {
            code: Some(Status::BadRequest.code),
            message: "The think hadn't been deleted".to_string(),
        };
    }

    ResponseMessage {
        code: Some(Status::BadRequest.code),
        message: "The think had been deleted".to_string(),
    }
}
