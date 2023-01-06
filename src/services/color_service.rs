use align_mind_server::models::color_model::*;
use align_mind_server::models::response_model::{ResponseError, ResponseSuccess};
use align_mind_server::models::user_model::User;
use align_mind_server::schema::colors;

use chrono::Utc;
use diesel::prelude::*;
use diesel::result::Error;
use rocket::http::Status;
use uuid::Uuid;

use super::users_service::get_user;

pub fn get_color(uuid_color: Uuid, conn: &mut PgConnection) -> Result<Color, ResponseError> {
    colors::table
        .filter(colors::color_id.eq(uuid_color))
        .first::<Color>(conn)
        .map_err(|_| ResponseError {
            code: Status::NotFound.code,
            message: "The color not found".to_string(),
        })
}

pub fn get_colors_with_user_uuid(
    uuid_user: Uuid,
    conn: &mut PgConnection,
) -> Result<Vec<Color>, ResponseError> {
    let result_user: User = get_user(uuid_user, conn)?;

    let result_colors: Result<Vec<Color>, Error> =
        Color::belonging_to(&result_user).load::<Color>(conn);

    if result_colors.is_err() {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Unknown error".to_string(),
        });
    }

    Ok(result_colors.unwrap())
}

pub fn create_color(
    user_uuid: Uuid,
    payload: NewColorDTO,
    conn: &mut PgConnection,
) -> Result<ResponseSuccess, ResponseError> {
    get_user(user_uuid, conn)?;

    let color: NewColor = NewColor {
        user_id: user_uuid,
        code_color: payload.code_color.unwrap(),
        name_color: payload.name_color.unwrap(),
    };

    let insert_action = diesel::insert_into(colors::table)
        .values(&color)
        .execute(conn)
        .is_ok();

    if !insert_action {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Unknow error".to_string(),
        });
    }

    Ok(ResponseSuccess {
        message: "The color had been created".to_string(),
        data: serde_json::to_value("").unwrap(),
    })
}

pub fn update_color(
    uuid_color: Uuid,
    payload: UpdateColorDTO,
    conn: &mut PgConnection,
) -> Result<ResponseSuccess, ResponseError> {
    let result_color: Color = get_color(uuid_color, conn)?;

    let data_color: UpdateColor = UpdateColor {
        code_color: payload.code_color,
        name_color: payload.name_color,
        updated_at: Some(Utc::now().naive_utc()),
    };

    let update_action: bool = diesel::update(&result_color)
        .set(&data_color)
        .execute(conn)
        .is_ok();

    if !update_action {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Unknown Error".to_string(),
        });
    }

    Ok(ResponseSuccess {
        message: "The color has been updated".to_string(),
        data: serde_json::to_value("").unwrap(),
    })
}

pub fn delete_color(
    uuid_color: Uuid,
    conn: &mut PgConnection,
) -> Result<ResponseSuccess, ResponseError> {
    let result_color: Color = get_color(uuid_color, conn)?;

    let delete_action: bool = diesel::delete(&result_color).execute(conn).is_ok();

    if !delete_action {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "The think hadn't been deleted".to_string(),
        });
    }

    Ok(ResponseSuccess {
        message: "The think had been deleted".to_string(),
        data: serde_json::to_value("").unwrap(),
    })
}
