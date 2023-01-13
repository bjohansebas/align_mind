use super::emotion_service::get_emotion;
use super::think_service::get_think;

use align_mind_server::models::emotion_model::Emotion;
use align_mind_server::models::response_model::{ResponseError, ResponseSuccess};
use align_mind_server::models::think_emotion_model::{
    NewThinkEmotion, NewThinkEmotionDTO, ThinkEmotion,
};
use align_mind_server::models::think_model::Think;
use align_mind_server::schema::{emotions, think_emotions};

use diesel::prelude::*;
use diesel::result::Error;
use rocket::http::Status;
use uuid::Uuid;

pub fn get_think_emotions(
    uuid_think: Uuid,
    conn: &mut PgConnection,
) -> Result<Vec<Emotion>, ResponseError> {
    let think: Think = get_think(uuid_think, conn)?;

    let result_emotion: Result<Vec<Emotion>, Error> = ThinkEmotion::belonging_to(&think)
        .inner_join(emotions::table)
        .select(emotions::all_columns)
        .load::<Emotion>(conn);

    if result_emotion.is_err() {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Unknown error".to_string(),
        });
    }

    Ok(result_emotion.unwrap())
}

pub fn register_emotion(
    uuid_think: Uuid,
    payload: NewThinkEmotionDTO,
    conn: &mut PgConnection,
) -> Result<ResponseSuccess, ResponseError> {
    get_think(uuid_think, conn)?;

    let uuid_emotion: Result<Uuid, uuid::Error> =
        Uuid::parse_str(payload.emotion_id.unwrap().as_str());

    if uuid_emotion.is_err() {
        return Err(ResponseError {
            code: Status::NotFound.code,
            message: "The think not found".to_string(),
        });
    }

    let emotion: Emotion = get_emotion(uuid_emotion.unwrap(), conn)?;

    let result_emotion = ThinkEmotion::belonging_to(&emotion)
        .filter(think_emotions::think_id.eq(uuid_think))
        .first::<ThinkEmotion>(conn);

    if result_emotion.is_ok() {
        return Err(ResponseError {
            code: Status::NotFound.code,
            message: "The emotion exist".to_string(),
        });
    }

    let think_emotion_data: NewThinkEmotion = NewThinkEmotion {
        emotion_id: emotion.emotion_id,
        think_id: uuid_think,
    };

    let insert_action: bool = diesel::insert_into(think_emotions::table)
        .values(&think_emotion_data)
        .execute(conn)
        .is_ok();

    if !insert_action {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Unknow error".to_string(),
        });
    }

    Ok(ResponseSuccess {
        message: "The think had been created".to_string(),
        data: serde_json::to_value("").unwrap(),
    })
}

pub fn delete_think_emotions(
    uuid_think: Uuid,
    uuid_emotion: Uuid,
    conn: &mut PgConnection,
) -> Result<ResponseSuccess, ResponseError> {
    get_think(uuid_think, conn)?;

    let emotion: Emotion = get_emotion(uuid_emotion, conn)?;

    let result_emotion = ThinkEmotion::belonging_to(&emotion)
        .filter(think_emotions::think_id.eq(uuid_think))
        .first::<ThinkEmotion>(conn);

    if result_emotion.is_err() {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "The think not found".to_string(),
        });
    }

    let delete_action: bool = diesel::delete(&result_emotion.unwrap())
        .execute(conn)
        .is_ok();

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
