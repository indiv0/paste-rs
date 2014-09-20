use mysql::conn::MyOpts;
use mysql::conn::pool::MyPool;
use mysql::value::from_value;
use std::default::Default;
use time::Timespec;

use note::Note;
use result::{
    PasteError,
    NoResultsError,
};

pub struct Database {
    pool: MyPool
}

impl Database {
    pub fn new(user: String) -> Database {
        let opts = MyOpts {
            user: Some(user),
            ..Default::default()
        };

        let pool = MyPool::new(opts).unwrap();

        pool.query("CREATE DATABASE IF NOT EXISTS paste");
        pool.query("USE paste");
        pool.query("CREATE TABLE person(
                        id INT NOT NULL AUTO_INCREMENT PRIMARY_KEY,
                        code TEXT,
                        time_created TIMESTAMP,
                        data BLOB
                    );");

        Database {
            pool: pool
        }
    }

    // TODO: Replace with Result value.
    pub fn insert_note(&self, note: Note) -> () {
        self.pool.prepare("INSERT INTO person (code, time_created, data)
                           VALUES (?, ?, ?);")
            .and_then(|mut stmt| {
                stmt.execute(&[&note.code, &note.time_created, &note.data]).and(Ok(()))
            });
    }

    // TODO: Replace with Result value.
    pub fn get_note(&self, code: String) -> Result<Note, PasteError> {
        self.pool.prepare("SELECT id, code, time_created, data FROM paste WHERE code = ?")
            .and_then(|mut stmt| {
                let row = stmt.execute(&[&code]).unwrap().next().unwrap().unwrap();
                Ok(Note::new(from_value(&row[0]),
                              from_value(&row[1]),
                              from_value(&row[2]),
                              from_value(&row[3])))
            }
        );

        return Err(PasteError::new("Failed to find note", NoResultsError));
    }
}
