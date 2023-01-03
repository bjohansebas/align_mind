use super::color_service::get_color;
use super::users_service::get_user;
use align_mind_server::establish_connection;
use align_mind_server::models::color_model::Color;
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
        .first::<Place>(connection);

    if let Ok(place) = result_place {
        return Some(place);
    }

    None
}

pub fn create_place(uuid_user: Uuid, payload: NewPlaceDTO) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_user: Option<User> = get_user(uuid_user);

    if let Some(user) = result_user {
        let uuid_color: Result<Uuid, uuid::Error> =
            Uuid::parse_str(payload.color_id.unwrap().as_str());

        if let Ok(uuid) = uuid_color {
            let result_color: Option<Color> = get_color(uuid);

            if let Some(color) = result_color {
                if color.user_id.eq(&Some(uuid_user)) {
                    let place: NewPlace = NewPlace {
                        name_place: payload.name_place.unwrap(),
                        user_id: user.user_id,
                        color_id: color.color_id,
                    };

                    return diesel::insert_into(places::table)
                        .values(&place)
                        .execute(connection)
                        .is_ok();
                }
            }
        }
    }

    false
}

pub fn update_place(uuid_place: Uuid, payload: UpdatePlaceDTO) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_place: Option<Place> = get_place(uuid_place);

    if let Some(place) = result_place {
        let mut data_place: UpdatePlace = UpdatePlace {
            name_place: payload.name_place,
            color_id: None,
            updated_at: Some(Utc::now().naive_utc()),
        };

        if let Some(color_id) = payload.color_id {
            let uuid_color: Result<Uuid, uuid::Error> = Uuid::parse_str(color_id.as_str());
            if let Ok(uuid) = uuid_color {
                let result_color: Option<Color> = get_color(uuid);
                if let Some(color) = result_color {
                    if color.user_id.eq(&Some(place.user_id)) {
                        data_place.color_id = Some(color.color_id);
                    } else {
                        return false;
                    }
                }
            } else {
                return false;
            }
        }

        return diesel::update(&place)
            .set(&data_place)
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
