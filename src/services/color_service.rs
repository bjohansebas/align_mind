use align_mind_server::establish_connection;
use align_mind_server::models::color_model::*;
use align_mind_server::models::user_model::User;
use align_mind_server::schema::colors;

use chrono::Utc;
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

use super::users_service::get_user;

pub fn get_color(uuid_color: Uuid) -> Option<Color> {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_color: Result<Color, Error> = colors::table
        .filter(colors::color_id.eq(uuid_color))
        .first(connection);

    if let Ok(color) = result_color {
        return Some(color);
    }
    None
}

pub fn get_colors_with_user_uuid(uuid_user: Uuid) -> Option<Vec<Color>> {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_user: Option<User> = get_user(uuid_user);

    if let Some(user) = result_user {
        let result_colors: Result<Vec<Color>, _> =
            Color::belonging_to(&user).load::<Color>(connection);
        if let Ok(colors) = result_colors {
            return Some(colors);
        }
    }
    None
}

pub fn create_color(user_uuid: Option<Uuid>, mut payload: NewColor) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    if let Some(uuid) = user_uuid {
        let result_user: Option<User> = get_user(uuid);
        if let Some(user) = result_user {
            payload.user_id = Some(user.user_id);
            diesel::insert_into(colors::table)
                .values(&payload)
                .execute(connection)
                .is_ok()
        } else {
            false
        }
    } else {
        payload.user_id = None;
        diesel::insert_into(colors::table)
            .values(&payload)
            .execute(connection)
            .is_ok()
    }
}

pub fn update_color(uuid_color: Uuid, mut payload: UpdateColor) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_color: Option<Color> = get_color(uuid_color);

    if let Some(color) = result_color {
        payload.updated_at = Some(Utc::now().naive_utc());

        return diesel::update(&color)
            .set(&payload)
            .execute(connection)
            .is_ok();
    }

    false
}

pub fn delete_color(uuid_color: Uuid) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_color: Option<Color> = get_color(uuid_color);

    if let Some(color) = result_color {
        return diesel::delete(&color).execute(connection).is_ok();
    }
    false
}
