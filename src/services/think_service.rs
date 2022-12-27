use super::users_service::get_user;
use align_mind_server::establish_connection;
use align_mind_server::models::think_model::*;
use align_mind_server::schema::{thinks, trash_thinks};

use chrono::{Datelike, NaiveDate, Utc};
use diesel::prelude::*;
use uuid::Uuid;

pub fn get_thinks_with_user_uuid(uuid_user: Uuid) -> Vec<Think> {
    let connection: &mut PgConnection = &mut establish_connection();

    let user = get_user(uuid_user);

    Think::belonging_to(&user)
        .load::<Think>(connection)
        .expect("Error loading places")
}

pub fn get_think(uuid_think: Uuid) -> Think {
    let connection: &mut PgConnection = &mut establish_connection();

    thinks::table
        .filter(thinks::think_id.eq(uuid_think))
        .first(connection)
        .unwrap()
}

// poner ruta para ver los archivados y los desarchivados

pub fn create_think(uuid_user: Uuid, mut payload: NewThink) -> Think {
    let connection: &mut PgConnection = &mut establish_connection();

    payload.user_id = uuid_user;
    payload.is_archive = Some(false);
    payload.created_at = Some(Utc::now().naive_utc());
    payload.updated_at = Some(Utc::now().naive_utc());

    diesel::insert_into(thinks::table)
        .values(&payload)
        .get_result(connection)
        .unwrap()
}

pub fn update_think(uuid_think: Uuid, mut payload: UpdateThink) {
    let connection: &mut PgConnection = &mut establish_connection();

    let place: Think = get_think(uuid_think);

    payload.updated_at = Some(Utc::now().naive_utc());

    diesel::update(&place)
        .set(&payload)
        .execute(connection)
        .unwrap();
}

pub fn delete_think(uuid_think: Uuid) {
    let connection: &mut PgConnection = &mut establish_connection();

    let place: Think = get_think(uuid_think);

    diesel::delete(&place).execute(connection).unwrap();
}

pub fn get_trash_thinks_with_user_uuid(uuid_user: Uuid) -> Vec<TrashThink> {
    let connection: &mut PgConnection = &mut establish_connection();

    let user = get_user(uuid_user);

    TrashThink::belonging_to(&user)
        .load::<TrashThink>(connection)
        .expect("Error loading places")
}

pub fn get_trash_think(uuid_trash_think: Uuid) -> TrashThink {
    let connection: &mut PgConnection = &mut establish_connection();

    trash_thinks::table
        .filter(trash_thinks::trash_th_id.eq(uuid_trash_think))
        .first(connection)
        .unwrap()
}

pub fn delete_trash(uuid_trash: Uuid) {
    let connection: &mut PgConnection = &mut establish_connection();

    let trash: TrashThink = get_trash_think(uuid_trash);

    diesel::delete(&trash).execute(connection).unwrap();
}

pub fn move_think_to_trash(uuid_think: Uuid) -> TrashThink {
    let connection: &mut PgConnection = &mut establish_connection();

    let date_now = Utc::now().naive_utc();
    let date_start = NaiveDate::from_ymd_opt(date_now.year(), date_now.month(), date_now.day());
    let date_end = NaiveDate::from_ymd_opt(date_now.year(), date_now.month() + 1, date_now.day());

    let Think {
        text_think,
        user_id,
        place_id,
        created_at,
        updated_at,
        ..
    } = get_think(uuid_think);

    let payload: NewTrashThink = NewTrashThink {
        text_think,
        user_id,
        place_id,
        date_start,
        date_end,
        created_at,
        updated_at,
    };

    delete_think(uuid_think);

    diesel::insert_into(trash_thinks::table)
        .values(&payload)
        .get_result(connection)
        .unwrap()
}

pub fn remove_of_trash(uuid_trash: Uuid) -> Think {
    let connection: &mut PgConnection = &mut establish_connection();

    let TrashThink {
        text_think,
        user_id,
        place_id,
        created_at,
        updated_at,
        ..
    } = get_trash_think(uuid_trash);

    let payload = NewThink {
        text_think,
        user_id,
        place_id,
        created_at: Some(created_at),
        updated_at: Some(updated_at),
        is_archive: Some(false),
    };

    delete_trash(uuid_trash);

    diesel::insert_into(thinks::table)
        .values(&payload)
        .get_result::<Think>(connection)
        .unwrap()
}
