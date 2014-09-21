use mysql::value::{
    from_value,
    Value,
};
use time::Timespec;

use database::Database;

#[deriving(Clone, PartialEq, Show)]
pub struct Note {
    pub id: i32,
    pub code: String,
    pub time_created: Timespec,
    pub data: String
}

impl Note {
    pub fn new(id: i32, code: &str, time_created: Timespec, data: &str) -> Note {
        Note {
            id: id,
            code: code.to_string(),
            time_created: time_created,
            data: data.to_string()
        }
    }

    pub fn from_row(row: &Vec<Value>) -> Note {
        Note {
            id: from_value(&row[0]),
            code: from_value(&row[1]),
            time_created: from_value(&row[2]),
            data: from_value(&row[3])
        }
    }

    pub fn find_by_code(code: &str) -> Option<Note> {
        let conn = Database::new().connect();
        let mut stmt = match conn.prepare("SELECT * FROM paste \
                                       WHERE code = ? LIMIT 1") {
            Ok(stmt) => stmt,
            Err(e) => {
                println!("Error, failed to find Note: {}", e);
                return None
            }
        };

        for row in stmt.execute(&[&code.to_string()]).unwrap() {
            let row = row.unwrap();
            return Some(Note::from_row(&row))
        }

        None
    }

    pub fn insert(note: &mut Note) {
        let conn = Database::new().connect();
        let mut stmt = conn.prepare("
            INSERT INTO paste (
                code,
                time_created,
                data
            )
            VALUES (?, ?, ?);").unwrap();

        for row in stmt.execute(&[&note.code, &note.time_created, &note.data]).unwrap() {
            let row = row.unwrap();
            let id = from_value(&row[0]);
            note.id = id;
            break;
        }
    }

    pub fn _all() -> Vec<Note> {
        let mut notes: Vec<Note> = Vec::new();
        let conn = Database::new().connect();
        let mut stmt = conn.prepare("SELECT id, code, time_created, data FROM paste;").unwrap();

        for row in stmt.execute([]).unwrap() {
            let row = row.unwrap();
            let note = Note::from_row(&row);
            notes.push(note);
        }
        notes
    }
}
