use super::users_service::get_user;
use align_mind_server::establish_connection;
use align_mind_server::models::place_model::*;
use align_mind_server::schema::places;

use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

pub fn get_places_with_user_uuid(uuid_user: Uuid) -> Vec<Place> {
    let connection: &mut PgConnection = &mut establish_connection();

    let user = get_user(uuid_user);

    Place::belonging_to(&user)
        .load::<Place>(connection)
        .expect("Error loading places")
}

pub fn get_place(uuid_place: Uuid) -> Place {
    let connection: &mut PgConnection = &mut establish_connection();

    places::table
        .filter(places::place_id.eq(uuid_place))
        .first(connection)
        .unwrap()
}

pub fn create_place(uuid_user: Uuid, mut payload: NewPlace) -> Place {
    let connection: &mut PgConnection = &mut establish_connection();

    payload.user_id = Some(uuid_user);

    diesel::insert_into(places::table)
        .values(&payload)
        .get_result(connection)
        .unwrap()
}

pub fn update_place(uuid_place: Uuid, mut payload: UpdatePlace) {
    let connection: &mut PgConnection = &mut establish_connection();

    let place: Place = get_place(uuid_place);

    payload.updated_at = Some(Utc::now().naive_utc());

    diesel::update(&place)
        .set(&payload)
        .execute(connection)
        .unwrap();
}

pub fn delete_place(uuid_place: Uuid) {
    let connection: &mut PgConnection = &mut establish_connection();

    let place: Place = get_place(uuid_place);

    diesel::delete(&place).execute(connection).unwrap();
}
