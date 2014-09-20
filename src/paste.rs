extern crate http;
extern crate nickel;
extern crate mysql;
extern crate serialize;
extern crate time;
extern crate url;

use std::io::net::ip::Ipv4Addr;
use nickel::{
    IntoErrorHandler,
    Nickel,
};

use database::Database;

mod controllers;
mod database;
mod note;
mod settings;
mod util;

fn main() {
    // Initialize database connection.
    let database = Database::new();
    if settings::CREATE_DATABASE {
        println!("Creating database");
        match database.create() {
            Ok(_) => println!("Initialized database"),
            Err(e) => fail!("Failed to initialize database: {}", e)
        }
    }

    // Create web server.
    let mut server = Nickel::new();
    let mut router = Nickel::router();

    // Routes.
    router.get("/:code", controllers::get_note);
    router.post("/",     controllers::post_note);

    // Middleware.
    server.utilize(Nickel::static_files("assets/"));
    server.utilize(router);

    // Error handling.
    server.handle_error(IntoErrorHandler::from_fn(controllers::custom_404));

    // Start web server.
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
