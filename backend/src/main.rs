mod api;
mod registry;

use rocket::{launch, routes};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![api::get_repositories])
}
