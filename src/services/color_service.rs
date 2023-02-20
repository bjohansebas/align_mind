use align_mind_server::models::color_model::*;
use align_mind_server::models::response_model::{ResponseMessage, ResponseValue};
use align_mind_server::models::user_model::User;
use align_mind_server::schema::colors;

use chrono::Utc;
use diesel::prelude::*;
use rocket::http::Status;
use uuid::Uuid;

use super::users_service::get_user_account;

pub fn get_color(
    uuid_color: Uuid,
    conn: &mut PgConnection,
) -> Result<ResponseValue<Color>, ResponseMessage> {
    colors::table
        .filter(colors::color_id.eq(uuid_color))
        .first::<Color>(conn)
        .map(|color| ResponseValue {
            code: Status::Accepted.code,
            value: color,
        })
        .map_err(|_| ResponseMessage {
            code: Some(Status::NotFound.code),
            message: "The color not found".to_string(),
        })
}

pub fn get_colors_with_user_uuid(
    uuid_user: Uuid,
    conn: &mut PgConnection,
) -> Result<ResponseValue<Vec<Color>>, ResponseMessage> {
    let result_user: User = get_user_account(uuid_user, conn)?.value;

    Color::belonging_to(&result_user)
        .load::<Color>(conn)
        .map(|color| ResponseValue {
            code: Status::Accepted.code,
            value: color,
        })
        .map_err(|_| ResponseMessage {
            code: Some(Status::BadRequest.code),
            message: "Unknown error".to_string(),
        })
}

pub fn get_color_by_code_and_user(
    uuid_user: Uuid,
    code: String,
    conn: &mut PgConnection,
) -> Result<ResponseValue<Color>, ResponseMessage> {
    colors::table
        .filter(colors::user_id.eq(uuid_user))
        .filter(colors::code_color.eq(code))
        .first::<Color>(conn)
        .map(|color| ResponseValue {
            code: Status::Accepted.code,
            value: color,
        })
        .map_err(|_| ResponseMessage {
            code: Some(Status::NotFound.code),
            message: "The color not found".to_string(),
        })
}

pub fn create_color(
    uuid_user: Uuid,
    payload: NewColorDTO,
    conn: &mut PgConnection,
) -> ResponseMessage {
    if let Err(e) = get_user_account(uuid_user, conn) {
        return e;
    }

    let color: NewColor = NewColor {
        user_id: uuid_user,
        code_color: payload.code_color.unwrap(),
    };

    let insert_action = diesel::insert_into(colors::table)
        .values(&color)
        .execute(conn)
        .is_ok();

    if !insert_action {
        return ResponseMessage {
            code: Some(Status::BadRequest.code),
            message: "Unknow error".to_string(),
        };
    }

    ResponseMessage {
        code: Some(Status::Accepted.code),
        message: "The color had been created".to_string(),
    }
}

pub fn update_color(
    uuid_color: Uuid,
    payload: UpdateColorDTO,
    conn: &mut PgConnection,
) -> ResponseMessage {
    let result_color: Result<ResponseValue<Color>, ResponseMessage> = get_color(uuid_color, conn);

    if let Err(e) = result_color {
        return e;
    }

    let data_color: UpdateColor = UpdateColor {
        code_color: payload.code_color,
        updated_at: Some(Utc::now().naive_utc()),
    };

    let color: Color = result_color.unwrap().value;

    let update_action: bool = diesel::update(&color)
        .set(&data_color)
        .execute(conn)
        .is_ok();

    if !update_action {
        return ResponseMessage {
            code: Some(Status::BadRequest.code),
            message: "Unknown Error".to_string(),
        };
    }

    ResponseMessage {
        code: Some(Status::Ok.code),
        message: "The color has been updated".to_string(),
    }
}

pub fn delete_color(uuid_color: Uuid, conn: &mut PgConnection) -> ResponseMessage {
    let result_color: Result<ResponseValue<Color>, ResponseMessage> = get_color(uuid_color, conn);

    if let Err(e) = result_color {
        return e;
    }

    let color: Color = result_color.unwrap().value;

    let delete_action: bool = diesel::delete(&color).execute(conn).is_ok();

    if !delete_action {
        return ResponseMessage {
            code: Some(Status::BadRequest.code),
            message: "The color hadn't been deleted".to_string(),
        };
    }

    ResponseMessage {
        code: Some(Status::Ok.code),
        message: "The color had been deleted".to_string(),
    }
}
