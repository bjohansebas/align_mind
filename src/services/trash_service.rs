use super::users_service::get_user;
use align_mind_server::establish_connection;
use align_mind_server::models::think_model::*;
use align_mind_server::models::user_model::User;
use align_mind_server::schema::{thinks, trash_thinks};

use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

pub fn get_trash_thinks_with_user_uuid(uuid_user: Uuid) -> Option<Vec<TrashThink>> {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_user: Option<User> = get_user(uuid_user);
    if let Some(user) = result_user {
        let result_thinks: Result<Vec<TrashThink>, Error> =
            TrashThink::belonging_to(&user).load::<TrashThink>(connection);
        if let Ok(thinks) = result_thinks {
            return Some(thinks);
        }
    }
    None
}

pub fn get_trash_think(uuid_trash_think: Uuid) -> Option<TrashThink> {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_trash_think: Result<TrashThink, Error> = trash_thinks::table
        .filter(trash_thinks::trash_th_id.eq(uuid_trash_think))
        .first(connection);

    if let Ok(trash) = result_trash_think {
        return Some(trash);
    }

    None
}

pub fn remove_of_trash(uuid_trash: Uuid) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_trash_think: Option<TrashThink> = get_trash_think(uuid_trash);

    if let Some(trash) = result_trash_think {
        let payload: NewThink = NewThink {
            text_think: trash.text_think,
            user_id: trash.user_id,
            place_id: trash.place_id,
            created_at: Some(trash.created_at),
            updated_at: Some(trash.updated_at),
            is_archive: Some(false),
        };

        let insert_think: bool = diesel::insert_into(thinks::table)
            .values(&payload)
            .execute(connection)
            .is_ok();

        if insert_think {
            delete_trash(uuid_trash);
            return true;
        }
    }

    false
}

pub fn delete_trash(uuid_trash: Uuid) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_trash: Option<TrashThink> = get_trash_think(uuid_trash);

    if let Some(trash) = result_trash {
        return diesel::delete(&trash).execute(connection).is_ok();
    }
    false
}
