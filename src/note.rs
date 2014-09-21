use mysql::value::from_value;
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
    pub fn new(id: i32, code: String, time_created: Timespec, data: String) -> Note {
        Note {
            id: id,
            code: code,
            time_created: time_created,
            data: data
        }
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

    pub fn delete(note: &Note) {
        let conn = Database::new().connect();
        let mut stmt = conn.prepare("
            DELETE FROM paste
            WHERE id = ?").unwrap();

        stmt.execute(&[&note.id]).unwrap();
    }

    pub fn all() -> Vec<Note> {
        let mut notes: Vec<Note> = Vec::new();
        let conn = Database::new().connect();
        let mut stmt = conn.prepare("SELECT id, code, time_created, data FROM paste;").unwrap();

        for row in stmt.execute([]).unwrap() {
            let row = row.unwrap();
            let note = Note::new(
                from_value(&row[0]),
                from_value(&row[1]),
                from_value(&row[2]),
                from_value(&row[3])
            );
            notes.push(note);
        }
        notes
    }
}
