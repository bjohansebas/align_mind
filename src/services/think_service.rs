use super::place_service::get_place;
use super::think_emotion_service::get_think_emotions;
use super::users_service::get_user;

use align_mind_server::models::emotion_model::Emotion;
use align_mind_server::models::place_model::Place;
use align_mind_server::models::response_model::{ResponseError, ResponseSuccess};
use align_mind_server::models::think_emotion_model::NewThinkTrashEmotion;
use align_mind_server::models::think_model::*;
use align_mind_server::models::user_model::User;
use align_mind_server::schema::{think_trash_emotions, thinks, trash_thinks};

use chrono::{Datelike, NaiveDate, NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::result::Error;
use rocket::http::Status;
use uuid::Uuid;

pub fn get_thinks_with_user_uuid(
    uuid_user: Uuid,
    conn: &mut PgConnection,
) -> Result<Vec<Think>, ResponseError> {
    let result_user: User = get_user(uuid_user, conn)?;

    let result_thinks: Result<Vec<Think>, Error> =
        Think::belonging_to(&result_user).load::<Think>(conn);

    if result_thinks.is_err() {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Unknown error".to_string(),
        });
    }

    Ok(result_thinks.unwrap())
}

pub fn get_think(uuid_think: Uuid, conn: &mut PgConnection) -> Result<Think, ResponseError> {
    thinks::table
        .filter(thinks::think_id.eq(uuid_think))
        .first::<Think>(conn)
        .map_err(|_| ResponseError {
            code: Status::NotFound.code,
            message: "The think not found".to_string(),
        })
}

pub fn get_archive_think(
    uuid_user: Uuid,
    conn: &mut PgConnection,
) -> Result<Vec<Think>, ResponseError> {
    thinks::table
        .filter(thinks::user_id.eq(uuid_user))
        .filter(thinks::is_archive.eq(true))
        .load::<Think>(conn)
        .map_err(|_| ResponseError {
            code: Status::NotFound.code,
            message: "Unknow error".to_string(),
        })
}

pub fn get_unarchive_think(
    uuid_user: Uuid,
    conn: &mut PgConnection,
) -> Result<Vec<Think>, ResponseError> {
    thinks::table
        .filter(thinks::user_id.eq(uuid_user))
        .filter(thinks::is_archive.eq(false))
        .load::<Think>(conn)
        .map_err(|_| ResponseError {
            code: Status::NotFound.code,
            message: "Unknow error".to_string(),
        })
}

pub fn create_think(
    uuid_user: Uuid,
    payload: NewThinkDTO,
    conn: &mut PgConnection,
) -> Result<ResponseSuccess, ResponseError> {
    get_user(uuid_user, conn)?;

    let uuid_place: Result<Uuid, uuid::Error> = Uuid::parse_str(payload.place_id.unwrap().as_str());

    if uuid_place.is_err() {
        return Err(ResponseError {
            code: Status::NotFound.code,
            message: "The place not found".to_string(),
        });
    }

    let result_place: Place = get_place(uuid_place.unwrap(), conn)?;

    if !result_place.user_id.eq(&uuid_user) {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "The place not own of user".to_string(),
        });
    }

    let think: NewThink = NewThink {
        think_id: Uuid::new_v4(),
        user_id: uuid_user,
        is_archive: Some(false),
        place_id: result_place.place_id,
        text_think: payload.text_think.unwrap(),
        created_at: Some(Utc::now().naive_utc()),
        updated_at: Some(Utc::now().naive_utc()),
    };

    let insert_action: bool = diesel::insert_into(thinks::table)
        .values(&think)
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

pub fn update_think(
    uuid_think: Uuid,
    payload: UpdateThinkDTO,
    conn: &mut PgConnection,
) -> Result<ResponseSuccess, ResponseError> {
    let result_think: Think = get_think(uuid_think, conn)?;

    let data_think: UpdateThink = UpdateThink {
        text_think: payload.text_think,
        is_archive: payload.is_archive,
        updated_at: Some(Utc::now().naive_utc()),
    };

    let update_action: bool = diesel::update(&result_think)
        .set(&data_think)
        .execute(conn)
        .is_ok();

    if !update_action {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Unknow error".to_string(),
        });
    }

    Ok(ResponseSuccess {
        message: "The think had been updated".to_string(),
        data: serde_json::to_value("").unwrap(),
    })
}

pub fn delete_think(
    uuid_think: Uuid,
    conn: &mut PgConnection,
) -> Result<ResponseSuccess, ResponseError> {
    let result_think: Think = get_think(uuid_think, conn)?;

    let delete_action = diesel::delete(&result_think).execute(conn).is_ok();

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

pub fn move_think_to_trash(
    uuid_think: Uuid,
    conn: &mut PgConnection,
) -> Result<ResponseSuccess, ResponseError> {
    let result_think: Think = get_think(uuid_think, conn)?;

    let date_now: NaiveDateTime = Utc::now().naive_utc();
    let date_start: Option<NaiveDate> =
        NaiveDate::from_ymd_opt(date_now.year(), date_now.month(), date_now.day());
    let date_end: Option<NaiveDate> =
        NaiveDate::from_ymd_opt(date_now.year(), date_now.month() + 1, date_now.day());

    let payload: NewTrashThink = NewTrashThink {
        trash_th_id: result_think.think_id,
        text_think: result_think.text_think,
        user_id: result_think.user_id,
        place_id: result_think.place_id,
        date_start,
        date_end,
        created_at: result_think.created_at,
        updated_at: result_think.updated_at,
    };

    let insert_trash: bool = diesel::insert_into(trash_thinks::table)
        .values(&payload)
        .execute(conn)
        .is_ok();

    if !insert_trash {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Unknown error".to_string(),
        });
    }

    let emotions: Vec<Emotion> = get_think_emotions(uuid_think, conn)?;

    for emotion in emotions.iter() {
        let think_emotion_data: NewThinkTrashEmotion = NewThinkTrashEmotion {
            emotion_id: emotion.emotion_id,
            trash_th_id: uuid_think,
        };

        let insert_action: bool = diesel::insert_into(think_trash_emotions::table)
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

    delete_think(uuid_think, conn)?;

    Ok(ResponseSuccess {
        message: "Think has moved to trash".to_string(),
        data: serde_json::to_value("").unwrap(),
    })
}
