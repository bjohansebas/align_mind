use super::users_service::get_user;
use align_mind_server::establish_connection;
use align_mind_server::models::think_model::*;
use align_mind_server::models::user_model::User;
use align_mind_server::schema::{thinks, trash_thinks};

use chrono::{Datelike, NaiveDate, NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

pub fn get_thinks_with_user_uuid(uuid_user: Uuid) -> Option<Vec<Think>> {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_user: Option<User> = get_user(uuid_user);

    if let Some(user) = result_user {
        let result_thinks: Result<Vec<Think>, Error> =
            Think::belonging_to(&user).load::<Think>(connection);
        if let Ok(thinks) = result_thinks {
            return Some(thinks);
        }
    }
    None
}

pub fn get_think(uuid_think: Uuid) -> Option<Think> {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_think: Result<Think, Error> = thinks::table
        .filter(thinks::think_id.eq(uuid_think))
        .first(connection);
    if let Ok(think) = result_think {
        return Some(think);
    }
    None
}

// poner ruta para ver los archivados y los desarchivados

pub fn create_think(uuid_user: Uuid, mut payload: NewThink) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_user: Option<User> = get_user(uuid_user);

    if let Some(user) = result_user {
        payload.user_id = user.user_id;
        payload.is_archive = Some(false);
        payload.created_at = Some(Utc::now().naive_utc());
        payload.updated_at = Some(Utc::now().naive_utc());

        return diesel::insert_into(thinks::table)
            .values(&payload)
            .execute(connection)
            .is_ok();
    }
    false
}

pub fn update_think(uuid_think: Uuid, mut payload: UpdateThink) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_think: Option<Think> = get_think(uuid_think);
    if let Some(think) = result_think {
        payload.updated_at = Some(Utc::now().naive_utc());

        return diesel::update(&think)
            .set(&payload)
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
