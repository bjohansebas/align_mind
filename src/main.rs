#[macro_use]
extern crate rocket;
use rocket::http::Method;
use rocket_cors::AllowedHeaders;
use rocket_cors::{AllowedOrigins, CorsOptions};

use routes::auth_route::*;
use routes::color_route::*;
use routes::emotion_route::*;
use routes::place_route::*;
use routes::think_route::*;
use routes::trash_route::*;
use routes::users_route::*;

mod jwt;
mod routes;
mod services;
mod utils;

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept","Content-Type"]),
        allowed_methods: vec![Method::Get, Method::Post, Method::Delete, Method::Put]
            .into_iter()
            .map(From::from)
            .collect(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors();

    rocket::build()
        .mount("/auth", routes![sign_up, login])
        .mount(
            "/users",
            routes![
                getting_user,
                getting_profile,
                getting_places_of_user,
                getting_thinks_of_user,
                getting_trash_of_user,
                getting_colors_of_user,
                save_profile,
                updating_user,
                update_profile_user,
                delete_account,
            ],
        )
        .mount(
            "/colors",
            routes![getting_color, updating_color, deleting_color, save_color],
        )
        .mount(
            "/places",
            routes![getting_place, save_place, deleting_place, updating_place],
        )
        .mount(
            "/thinks",
            routes![
                getting_think,
                save_think,
                deleting_think,
                updating_think,
                move_to_trash
            ],
        )
        .mount("/trash", routes![getting_trash, restore_think])
        .mount(
            "/emotions",
            routes![
                getting_emotion,
                save_emotion,
                deleting_emotion,
                updating_emotion
            ],
        )
        .attach(cors.unwrap())
        .register("/", catchers![rocket_validation::validation_catcher])
}
