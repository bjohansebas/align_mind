use super::color_service::{create_color, get_color, get_color_by_code_and_user};
use super::users_service::get_user;

use align_mind_server::models::color_model::{Color, NewColorDTO};
use align_mind_server::models::place_model::*;
use align_mind_server::models::response_model::{ResponseError, ResponseSuccess};
use align_mind_server::models::user_model::User;
use align_mind_server::schema::places;

use chrono::Utc;
use diesel::prelude::*;
use diesel::result::Error;
use rocket::http::Status;
use uuid::Uuid;

pub fn get_places_with_user_uuid(
    uuid_user: Uuid,
    conn: &mut PgConnection,
) -> Result<Vec<Place>, ResponseError> {
    let result_user: User = get_user(uuid_user, conn)?;

    let result_places: Result<Vec<Place>, Error> =
        Place::belonging_to(&result_user).load::<Place>(conn);

    if result_places.is_err() {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Unknown error".to_string(),
        });
    }

    Ok(result_places.unwrap())
}

pub fn get_place(uuid_place: Uuid, conn: &mut PgConnection) -> Result<Place, ResponseError> {
    places::table
        .filter(places::place_id.eq(uuid_place))
        .first::<Place>(conn)
        .map_err(|_| ResponseError {
            code: Status::NotFound.code,
            message: "The place not found".to_string(),
        })
}

pub fn create_place(
    uuid_user: Uuid,
    payload: NewPlaceDTO,
    conn: &mut PgConnection,
) -> Result<ResponseSuccess, ResponseError> {
    get_user(uuid_user, conn)?;

    let result_color =
        get_color_by_code_and_user(uuid_user, payload.code_color.to_owned().unwrap(), conn);
    if result_color.is_err() {
        if let Some(name_color) = payload.name_color {
            let _result = create_color(
                uuid_user,
                NewColorDTO {
                    code_color: payload.code_color.to_owned(),
                    name_color: Some(name_color),
                },
                conn,
            )?;
        } else {
            return Err(ResponseError {
                code: Status::BadRequest.code,
                message: "Name color is required for created new color".to_string(),
            });
        }
    }
    let result_color: Color =
        get_color_by_code_and_user(uuid_user, payload.code_color.to_owned().unwrap(), conn)?;

    let place: NewPlace = NewPlace {
        name_place: payload.name_place.unwrap(),
        user_id: uuid_user,
        color_id: result_color.color_id,
    };

    let insert_action: bool = diesel::insert_into(places::table)
        .values(&place)
        .execute(conn)
        .is_ok();

    if !insert_action {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Unknown Error".to_string(),
        });
    }

    Ok(ResponseSuccess {
        message: "The place has been created".to_string(),
        data: serde_json::to_value("").unwrap(),
    })
}

pub fn update_place(
    uuid_place: Uuid,
    payload: UpdatePlaceDTO,
    conn: &mut PgConnection,
) -> Result<ResponseSuccess, ResponseError> {
    let result_place: Place = get_place(uuid_place, conn)?;

    let mut data_place: UpdatePlace = UpdatePlace {
        name_place: payload.name_place,
        color_id: None,
        updated_at: Some(Utc::now().naive_utc()),
    };

    if let Some(color_id) = payload.color_id {
        let uuid_color: Result<Uuid, uuid::Error> = Uuid::parse_str(color_id.as_str());

        if uuid_color.is_err() {
            return Err(ResponseError {
                code: Status::NotFound.code,
                message: "The color not found".to_string(),
            });
        }

        let result_color: Color = get_color(uuid_color.unwrap(), conn)?;
        if !result_color.user_id.eq(&Some(result_place.user_id)) {
            return Err(ResponseError {
                code: Status::BadRequest.code,
                message: "The color not own of user".to_string(),
            });
        }
        data_place.color_id = Some(result_color.color_id);
    }

    let update_action = diesel::update(&result_place)
        .set(&data_place)
        .execute(conn)
        .is_ok();

    if !update_action {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Unknown Error".to_string(),
        });
    }

    Ok(ResponseSuccess {
        message: "The place has been created".to_string(),
        data: serde_json::to_value("").unwrap(),
    })
}

pub fn delete_place(
    uuid_place: Uuid,
    conn: &mut PgConnection,
) -> Result<ResponseSuccess, ResponseError> {
    let result_place: Place = get_place(uuid_place, conn)?;

    let delete_action: bool = diesel::delete(&result_place).execute(conn).is_ok();
    if !delete_action {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "The place hadn't been deleted".to_string(),
        });
    }

    Ok(ResponseSuccess {
        message: "The place had been deleted".to_string(),
        data: serde_json::to_value("").unwrap(),
    })
}
