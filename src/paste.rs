extern crate nickel;
extern crate mysql;
extern crate time;

use std::io::net::ip::Ipv4Addr;
use nickel::{
    Nickel,
    Request,
    Response,
};

use database::Database;

mod database;
mod note;
mod settings;
mod result;

fn main() {
    // Initialize database connection.
    let database = Database::new(settings::DATABASE_USERNAME.to_string());

    // Initialize webserver.
    let mut server = Nickel::new();

    fn a_handler(_request: &Request, response: &mut Response) {
        response.send("hello world");
    }

    fn get_note(request: &Request, response: &mut Response) {
        let noteid = request.params.get(&"noteid".to_string());

        let note = database.get_note(noteid);

        response.send(noteid.as_slice());
        response.send("test");
    }

    server.get("/", a_handler);
    server.get("/note/:noteid", get_note);
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
    println!("Server listening on 3000.");
}
