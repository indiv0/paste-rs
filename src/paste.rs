extern crate nickel;
extern crate mysql;
extern crate time;

use database::Database;

mod database;
mod note;
mod settings;
mod result;

fn main() {
    let database = Database::new(settings::DATABASE_USERNAME.to_string());
}
