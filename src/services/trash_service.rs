use super::users_service::get_user;

use align_mind_server::models::emotion_model::Emotion;
use align_mind_server::models::response_model::{ResponseError, ResponseSuccess};
use align_mind_server::models::think_emotion_model::{NewThinkEmotion, ThinkTrashEmotion};
use align_mind_server::models::think_model::*;
use align_mind_server::models::user_model::User;
use align_mind_server::schema::{emotions, think_emotions, thinks, trash_thinks};

use diesel::prelude::*;
use diesel::result::Error;
use rocket::http::Status;
use uuid::Uuid;

pub fn get_trash_thinks_with_user_uuid(
    uuid_user: Uuid,
    conn: &mut PgConnection,
) -> Result<Vec<TrashThink>, ResponseError> {
    let result_user: User = get_user(uuid_user, conn)?;

    let result_thinks: Result<Vec<TrashThink>, Error> =
        TrashThink::belonging_to(&result_user).load::<TrashThink>(conn);

    if result_thinks.is_err() {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Unknown error".to_string(),
        });
    }

    Ok(result_thinks.unwrap())
}

pub fn get_trash_think(
    uuid_trash_think: Uuid,
    conn: &mut PgConnection,
) -> Result<TrashThink, ResponseError> {
    trash_thinks::table
        .filter(trash_thinks::trash_th_id.eq(uuid_trash_think))
        .first::<TrashThink>(conn)
        .map_err(|_| ResponseError {
            code: Status::NotFound.code,
            message: "The think not found".to_string(),
        })
}

pub fn get_trash_emotions(
    uuid_trash_think: Uuid,
    conn: &mut PgConnection,
) -> Result<Vec<Emotion>, ResponseError> {
    let trash: TrashThink = get_trash_think(uuid_trash_think, conn)?;

    let result_emotion: Result<Vec<Emotion>, Error> = ThinkTrashEmotion::belonging_to(&trash)
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

pub fn remove_of_trash(
    uuid_trash_think: Uuid,
    conn: &mut PgConnection,
) -> Result<ResponseSuccess, ResponseError> {
    let result_trash: TrashThink = get_trash_think(uuid_trash_think, conn)?;

    let payload: NewThink = NewThink {
        think_id: result_trash.trash_th_id,
        text_think: result_trash.text_think,
        user_id: result_trash.user_id,
        place_id: result_trash.place_id,
        created_at: Some(result_trash.created_at),
        updated_at: Some(result_trash.updated_at),
        is_archive: Some(false),
    };

    let insert_think: bool = diesel::insert_into(thinks::table)
        .values(&payload)
        .execute(conn)
        .is_ok();

    if !insert_think {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Unknown error".to_string(),
        });
    }

    let emotions: Vec<Emotion> = get_trash_emotions(uuid_trash_think, conn)?;

    for emotion in emotions.iter() {
        let think_emotion_data: NewThinkEmotion = NewThinkEmotion {
            emotion_id: emotion.emotion_id,
            think_id: uuid_trash_think,
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
    }

    delete_trash(uuid_trash_think, conn)?;

    Ok(ResponseSuccess {
        message: "Think has restored of trash".to_string(),
        data: serde_json::to_value("").unwrap(),
    })
}

pub fn delete_trash(
    uuid_trash_think: Uuid,
    conn: &mut PgConnection,
) -> Result<ResponseSuccess, ResponseError> {
    let result_trash: TrashThink = get_trash_think(uuid_trash_think, conn)?;

    let action_result: bool = diesel::delete(&result_trash).execute(conn).is_ok();

    if !action_result {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Not think delete".to_string(),
        });
    }

    Ok(ResponseSuccess {
        message: "Think delete".to_string(),
        data: serde_json::to_value("").unwrap(),
    })
}
