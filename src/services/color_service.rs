use align_mind_server::establish_connection;
use align_mind_server::models::color_model::*;
use align_mind_server::schema::colors;

use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

pub fn get_color(uuid_color: Uuid) -> Color {
    let connection: &mut PgConnection = &mut establish_connection();

    colors::table
        .filter(colors::color_id.eq(uuid_color))
        .first(connection)
        .unwrap()
}

pub fn create_color(payload: NewColor) -> Color {
    let connection: &mut PgConnection = &mut establish_connection();

    diesel::insert_into(colors::table)
        .values(&payload)
        .get_result(connection)
        .unwrap()
}

pub fn update_color(uuid_color: Uuid, mut payload: UpdateColor) {
    let connection: &mut PgConnection = &mut establish_connection();

    let color: Color = get_color(uuid_color);

    payload.updated_at = Some(Utc::now().naive_utc());

    diesel::update(&color)
        .set(&payload)
        .execute(connection)
        .unwrap();
}

pub fn delete_color(uuid_color: Uuid) {
    let connection: &mut PgConnection = &mut establish_connection();

    let color: Color = get_color(uuid_color);

    diesel::delete(&color).execute(connection).unwrap();
}
