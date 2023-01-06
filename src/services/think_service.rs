use super::place_service::get_place;
use super::users_service::get_user;

use align_mind_server::establish_connection;
use align_mind_server::models::place_model::Place;
use align_mind_server::models::response_model::ResponseError;
use align_mind_server::models::think_model::*;
use align_mind_server::models::user_model::User;
use align_mind_server::schema::{thinks, trash_thinks};

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

pub fn get_think(uuid_think: Uuid) -> Option<Think> {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_think: Result<Think, Error> = thinks::table
        .filter(thinks::think_id.eq(uuid_think))
        .first::<Think>(connection);
    if let Ok(think) = result_think {
        return Some(think);
    }
    None
}

// poner ruta para ver los archivados y los desarchivados

pub fn create_think(uuid_user: Uuid, payload: NewThinkDTO, conn: &mut PgConnection) -> bool {
    let result_user: Result<User, ResponseError> = get_user(uuid_user, conn);

    if let Err(_) = result_user {
        return false;
    }

    let uuid_place: Result<Uuid, uuid::Error> = Uuid::parse_str(payload.place_id.unwrap().as_str());

    if uuid_place.is_err() {
        return false;
    }

    let result_place: Option<Place> = get_place(uuid_place.unwrap());

    if let Some(place) = result_place {
        if place.user_id.eq(&uuid_user) {
            let think: NewThink = NewThink {
                user_id: uuid_user,
                is_archive: Some(false),
                place_id: place.place_id,
                text_think: payload.text_think.unwrap(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            };

            return diesel::insert_into(thinks::table)
                .values(&think)
                .execute(conn)
                .is_ok();
        }
    }
    false
}

pub fn update_think(uuid_think: Uuid, payload: UpdateThinkDTO) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_think: Option<Think> = get_think(uuid_think);
    if let Some(think) = result_think {
        let data_think: UpdateThink = UpdateThink {
            text_think: payload.text_think,
            is_archive: payload.is_archive,
            updated_at: Some(Utc::now().naive_utc()),
        };

        return diesel::update(&think)
            .set(&data_think)
            .execute(connection)
            .is_ok();
    }
    false
}

pub fn delete_think(uuid_think: Uuid) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_think: Option<Think> = get_think(uuid_think);

    if let Some(think) = result_think {
        return diesel::delete(&think).execute(connection).is_ok();
    }
    false
}

pub fn move_think_to_trash(uuid_think: Uuid) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_think: Option<Think> = get_think(uuid_think);

    if let Some(think) = result_think {
        let date_now: NaiveDateTime = Utc::now().naive_utc();
        let date_start: Option<NaiveDate> =
            NaiveDate::from_ymd_opt(date_now.year(), date_now.month(), date_now.day());
        let date_end: Option<NaiveDate> =
            NaiveDate::from_ymd_opt(date_now.year(), date_now.month() + 1, date_now.day());

        let payload: NewTrashThink = NewTrashThink {
            text_think: think.text_think,
            user_id: think.user_id,
            place_id: think.place_id,
            date_start,
            date_end,
            created_at: think.created_at,
            updated_at: think.updated_at,
        };

        let insert_trash: bool = diesel::insert_into(trash_thinks::table)
            .values(&payload)
            .execute(connection)
            .is_ok();

        if insert_trash {
            delete_think(uuid_think);
            return true;
        }
    }

    false
}
