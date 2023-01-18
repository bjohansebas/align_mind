use super::color_service::get_color;

use align_mind_server::models::color_model::Color;
use align_mind_server::models::emotion_model::*;
use align_mind_server::models::response_model::{ResponseError, ResponseSuccess};
use align_mind_server::schema::emotions;

use chrono::Utc;
use diesel::prelude::*;
use rocket::http::Status;
use uuid::Uuid;

pub fn get_emotion(uuid_emotion: Uuid, conn: &mut PgConnection) -> Result<Emotion, ResponseError> {
    emotions::table
        .filter(emotions::emotion_id.eq(uuid_emotion))
        .first::<Emotion>(conn)
        .map_err(|_| ResponseError {
            code: Status::NotFound.code,
            message: "The emotion not found".to_string(),
        })
}

pub fn get_emotion_with_textt(
    text_emotion: String,
    conn: &mut PgConnection,
) -> Result<Emotion, ResponseError> {
    emotions::table
        .filter(emotions::name_emotion.eq(text_emotion))
        .first::<Emotion>(conn)
        .map_err(|_| ResponseError {
            code: Status::NotFound.code,
            message: "The emotion not found".to_string(),
        })
}

pub fn get_all_emotion(conn: &mut PgConnection) -> Result<Vec<Emotion>, ResponseError> {
    emotions::table
        .load::<Emotion>(conn)
        .map_err(|_| ResponseError {
            code: Status::NotFound.code,
            message: "The emotion not found".to_string(),
        })
}

pub fn create_emotion(
    payload: NewEmotionDTO,
    conn: &mut PgConnection,
) -> Result<ResponseSuccess, ResponseError> {
    let result_emotion: Result<Emotion, ResponseError> =
        get_emotion_with_textt(payload.name_emotion.to_owned().unwrap(), conn);

    if result_emotion.is_ok() {
        return Err(ResponseError {
            code: Status::Conflict.code,
            message: "The emotion was already created".to_string(),
        });
    }

    let uuid_color: Result<Uuid, uuid::Error> = Uuid::parse_str(payload.color_id.unwrap().as_str());

    if uuid_color.is_err() {
        return Err(ResponseError {
            code: Status::NotFound.code,
            message: "The color not found".to_string(),
        });
    }

    let result_color: Color = get_color(uuid_color.unwrap(), conn)?;

    let emotion: NewEmotion = NewEmotion {
        color_id: result_color.color_id,
        type_emotion: payload.type_emotion.unwrap(),
        name_emotion: payload.name_emotion.unwrap(),
    };

    let insert_action: bool = diesel::insert_into(emotions::table)
        .values(&emotion)
        .execute(conn)
        .is_ok();

    if !insert_action {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Unknow error".to_string(),
        });
    }

    Ok(ResponseSuccess {
        message: "The color had been created".to_string(),
        data: serde_json::to_value("").unwrap(),
    })
}

pub fn update_emotion(
    uuid_emotion: Uuid,
    payload: UpdateEmotionDTO,
    conn: &mut PgConnection,
) -> Result<ResponseSuccess, ResponseError> {
    let result_emotion: Emotion = get_emotion(uuid_emotion, conn)?;

    let mut data_emotion: UpdateEmotion = UpdateEmotion {
        name_emotion: payload.name_emotion,
        type_emotion: payload.type_emotion,
        color_id: None,
        updated_at: Some(Utc::now().naive_utc()),
    };

    if let Some(color_id) = payload.color_id {
        let uuid_color: Result<Uuid, uuid::Error> = Uuid::parse_str(color_id.as_str());

        if uuid_color.is_err() {
            return Err(ResponseError {
                code: Status::NotFound.code,
                message: "The color not found".to_string(),
            });
        }

        let result_color: Color = get_color(uuid_color.unwrap(), conn)?;
        data_emotion.color_id = Some(result_color.color_id);
    }

    let update_action = diesel::update(&result_emotion)
        .set(&data_emotion)
        .execute(conn)
        .is_ok();

    if !update_action {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Unknown Error".to_string(),
        });
    }

    Ok(ResponseSuccess {
        message: "The emotion has been updated".to_string(),
        data: serde_json::to_value("").unwrap(),
    })
}

pub fn delete_emotion(
    uuid_emotion: Uuid,
    conn: &mut PgConnection,
) -> Result<ResponseSuccess, ResponseError> {
    let result_emotion: Emotion = get_emotion(uuid_emotion, conn)?;

    let delete_action: bool = diesel::delete(&result_emotion).execute(conn).is_ok();

    if !delete_action {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "The think hadn't been deleted".to_string(),
        });
    }

    Ok(ResponseSuccess {
        message: "The think had been deleted".to_string(),
        data: serde_json::to_value("").unwrap(),
    })
}
