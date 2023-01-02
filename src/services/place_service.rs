use super::users_service::get_user;
use align_mind_server::establish_connection;
use align_mind_server::models::place_model::*;
use align_mind_server::models::user_model::User;
use align_mind_server::schema::places;

use chrono::Utc;
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

pub fn get_places_with_user_uuid(uuid_user: Uuid) -> Option<Vec<Place>> {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_user: Option<User> = get_user(uuid_user);

    if let Some(user) = result_user {
        let result_places: Result<Vec<Place>, Error> =
            Place::belonging_to(&user).load::<Place>(connection);
        if let Ok(places) = result_places {
            return Some(places);
        }
    }

    None
}

pub fn get_place(uuid_place: Uuid) -> Option<Place> {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_place: Result<Place, Error> = places::table
        .filter(places::place_id.eq(uuid_place))
        .first(connection);

    if let Ok(place) = result_place {
        return Some(place);
    }

    None
}

pub fn create_place(uuid_user: Uuid, mut payload: NewPlace) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_user: Option<User> = get_user(uuid_user);

    if let Some(user) = result_user {
        payload.user_id = Some(user.user_id);

        return diesel::insert_into(places::table)
            .values(&payload)
            .execute(connection)
            .is_ok();
    }

    false
}

pub fn update_place(uuid_place: Uuid, mut payload: UpdatePlace) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_place: Option<Place> = get_place(uuid_place);

    if let Some(place) = result_place {
        payload.updated_at = Some(Utc::now().naive_utc());

        return diesel::update(&place)
            .set(&payload)
            .execute(connection)
            .is_ok();
    }

    false
}

pub fn delete_place(uuid_place: Uuid) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_place: Option<Place> = get_place(uuid_place);

    if let Some(place) = result_place {
        return diesel::delete(&place).execute(connection).is_ok();
    }

    false
}
