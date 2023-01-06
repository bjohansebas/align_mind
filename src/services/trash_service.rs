use super::users_service::get_user;
use align_mind_server::establish_connection;
use align_mind_server::models::response_model::ResponseError;
use align_mind_server::models::think_model::*;
use align_mind_server::models::user_model::User;
use align_mind_server::schema::{thinks, trash_thinks};

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

pub fn get_trash_think(uuid_trash_think: Uuid) -> Option<TrashThink> {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_trash_think: Result<TrashThink, Error> = trash_thinks::table
        .filter(trash_thinks::trash_th_id.eq(uuid_trash_think))
        .first::<TrashThink>(connection);

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
