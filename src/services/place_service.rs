use super::color_service::{create_color, get_color, get_color_by_code_and_user};
use super::users_service::get_user;

use align_mind_server::models::color_model::{Color, NewColorDTO};
use align_mind_server::models::place_model::*;
use align_mind_server::models::response_model::{ResponseError, ResponseSuccess};
use align_mind_server::models::think_model::{Think, TrashThink};
use align_mind_server::models::user_model::User;
use align_mind_server::schema::{places, thinks};

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

pub fn get_thinks_place(
    uuid_place: Uuid,
    conn: &mut PgConnection,
) -> Result<Vec<Think>, ResponseError> {
    let result_place: Place = get_place(uuid_place, conn)?;

    let result_thinks: Result<Vec<Think>, Error> = Think::belonging_to(&result_place)
        .filter(thinks::is_archive.eq(false))
        .load::<Think>(conn);

    if result_thinks.is_err() {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Unknown error".to_string(),
        });
    }

    Ok(result_thinks.unwrap())
}

pub fn get_thinks_archive_place(
    uuid_place: Uuid,
    conn: &mut PgConnection,
) -> Result<Vec<Think>, ResponseError> {
    let result_place: Place = get_place(uuid_place, conn)?;

    let result_thinks: Result<Vec<Think>, Error> = Think::belonging_to(&result_place)
        .filter(thinks::is_archive.eq(true))
        .load::<Think>(conn);

    if result_thinks.is_err() {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Unknown error".to_string(),
        });
    }

    Ok(result_thinks.unwrap())
}

pub fn get_thinks_trash_place(
    uuid_place: Uuid,
    conn: &mut PgConnection,
) -> Result<Vec<TrashThink>, ResponseError> {
    let result_place: Place = get_place(uuid_place, conn)?;

    let result_thinks: Result<Vec<TrashThink>, Error> =
        TrashThink::belonging_to(&result_place).load::<TrashThink>(conn);

    if result_thinks.is_err() {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Unknown error".to_string(),
        });
    }

    Ok(result_thinks.unwrap())
}

pub fn get_color_places_with_user_uuid(
    uuid_user: Uuid,
    conn: &mut PgConnection,
) -> Result<Vec<Color>, ResponseError> {
    get_user(uuid_user, conn)?;

    let result_places: Vec<Place> = get_places_with_user_uuid(uuid_user, conn)?;
    let mut colors_place: Vec<Color> = Vec::new();

    for place in result_places.iter() {
        let result_color: Color = get_color(place.color_id, conn)?;
        colors_place.push(result_color)
    }

    Ok(colors_place)
}

pub fn get_place_with_text(
    text: String,
    uuid_user: Uuid,
    conn: &mut PgConnection,
) -> Result<Place, ResponseError> {
    places::table
        .filter(places::user_id.eq(uuid_user))
        .filter(places::name_place.eq(text))
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
) -> Result<Place, ResponseError> {
    get_user(uuid_user, conn)?;

    if get_place_with_text(payload.name_place.to_owned().unwrap(), uuid_user, conn).is_ok() {
        return Err(ResponseError {
            code: Status::Conflict.code,
            message: "The place exist".to_string(),
        });
    }

    let result_color =
        get_color_by_code_and_user(uuid_user, payload.code_color.to_owned().unwrap(), conn);
        
    if result_color.is_err() {
        let _result = create_color(
            uuid_user,
            NewColorDTO {
                code_color: payload.code_color.to_owned(),
            },
            conn,
        )?;
    }
    let result_color: Color =
        get_color_by_code_and_user(uuid_user, payload.code_color.to_owned().unwrap(), conn)?;

    let place: NewPlace = NewPlace {
        name_place: payload.name_place.unwrap(),
        user_id: uuid_user,
        color_id: result_color.color_id,
    };

    let insert_action = diesel::insert_into(places::table)
        .values(&place)
        .get_result(conn);

    if insert_action.is_err() {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Unknown Error".to_string(),
        });
    }

    Ok(insert_action.unwrap())
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

    if let Some(code_color) = payload.code_color {
        let result_color = get_color_by_code_and_user(
            result_place.user_id.to_owned(),
            code_color.to_owned(),
            conn,
        );

        if result_color.is_err() {
            let _result = create_color(
                result_place.user_id.to_owned(),
                NewColorDTO {
                    code_color: Some(code_color.to_owned()),
                },
                conn,
            )?;
        }
        let result_color: Color =
            get_color_by_code_and_user(result_place.user_id, code_color.to_owned(), conn)?;

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
