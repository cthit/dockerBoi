mod app;
mod page;
mod registry;

use seed::App;

fn main() {
    App::start("app", app::init, app::update, app::view);
}
