use mysql::conn::MyOpts;
use mysql::conn::pool::MyPool;
use mysql::error::MyError;
use std::default::Default;

use settings;

pub struct Database {
    opts: MyOpts
}

impl Database {
    pub fn new() -> Database {
        let opts = MyOpts {
            user: Some(settings::DATABASE_USERNAME.to_string()),
            pass: Some(settings::DATABASE_PASSWORD.to_string()),
            db_name: Some(settings::DATABASE_NAME.to_string()),
            ..Default::default()
        };

        Database {
            opts: opts
        }
    }

    pub fn connect(&self) -> MyPool {
        MyPool::new(self.opts.clone()).unwrap()
    }

    /// Sets up the database and table required.
    /// For this to work, priveleged user and password must be specified in settings.
    /// Additionally, no database name must be specified.
    pub fn create(&self) -> Result<(), MyError> {
        let conn = self.connect();

        try!(conn.query("CREATE DATABASE IF NOT EXISTS paste"));
        try!(conn.query("USE paste"));
        try!(conn.query("CREATE TABLE IF NOT EXISTS paste (
            id           INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
            code         TEXT,
            time_created TIMESTAMP,
            data         BLOB)"));
        Ok(())
    }
}
