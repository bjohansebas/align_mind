#[macro_use]
extern crate rocket;
use rocket::launch;
use routes::users_route::*;

mod routes;
mod services;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/users",
        routes![
            getting_user,
            getting_profile,
            save_profile_user,
            delete_profile_with_user,
            updating_user,
            update_profile_user
        ],
    )
}
