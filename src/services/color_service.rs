use align_mind_server::establish_connection;
use align_mind_server::models::color_model::*;
use align_mind_server::models::response_model::ResponseError;
use align_mind_server::models::user_model::User;
use align_mind_server::schema::colors;

use chrono::Utc;
use diesel::prelude::*;
use diesel::result::Error;
use rocket::http::Status;
use uuid::Uuid;

use super::users_service::get_user;

pub fn get_color(uuid_color: Uuid) -> Option<Color> {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_color: Result<Color, Error> = colors::table
        .filter(colors::color_id.eq(uuid_color))
        .first::<Color>(connection);

    if let Ok(color) = result_color {
        return Some(color);
    }
    None
}

pub fn get_colors_with_user_uuid(
    uuid_user: Uuid,
    conn: &mut PgConnection,
) -> Result<Vec<Color>, ResponseError> {
    let result_user: Result<User, ResponseError> = get_user(uuid_user, conn);

    if let Err(e) = result_user {
        return Err(e);
    }

    let result_colors: Result<Vec<Color>, Error> =
        Color::belonging_to(&result_user.unwrap()).load::<Color>(conn);

    if result_colors.is_err() {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Unknown error".to_string(),
        });
    }

    Ok(result_colors.unwrap())
}

pub fn create_color(user_uuid: Uuid, payload: NewColorDTO) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_user: Result<User, ResponseError> = get_user(user_uuid, connection);
    if let Err(_) = result_user {
        return false;
    }

    let color: NewColor = NewColor {
        user_id: user_uuid,
        code_color: payload.code_color.unwrap(),
        name_color: payload.name_color.unwrap(),
    };

    diesel::insert_into(colors::table)
        .values(&color)
        .execute(connection)
        .is_ok()
}

pub fn update_color(uuid_color: Uuid, payload: UpdateColorDTO) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_color: Option<Color> = get_color(uuid_color);

    if let Some(color) = result_color {
        let data_color: UpdateColor = UpdateColor {
            code_color: payload.code_color,
            name_color: payload.name_color,
            updated_at: Some(Utc::now().naive_utc()),
        };

        return diesel::update(&color)
            .set(&data_color)
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
