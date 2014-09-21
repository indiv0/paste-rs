# paste-rs [![Build Status](https://travis-ci.org/Indiv0/paste-rs.svg?branch=master)](https://travis-ci.org/Indiv0/paste-rs)

## Database

Setup the database with something like the following:

```SQL
CREATE USER 'pasteuser'@'localhost' IDENTIFIED BY 'pastepass';

CREATE DATABASE IF NOT EXISTS paste;
USE paste;
CREATE TABLE IF NOT EXISTS paste (
    id           INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    code         TEXT,
    time_created TIMESTAMP,
    data         BLOB
);

GRANT SELECT, INSERT, UPDATE, DELETE ON paste.paste TO 'pasteuser'@'localhost';
FLUSH PRIVILEGES;
```

## Credits

This project was inspired by [pxqz](https://github.com/Uiri/pxqz) by my friend [Uiri](https://github.com/Uiri).

Additional inspiration (particularly for the nickel.rs routing and layout) from [superlogical/rusty](https://github.com/superlogical/rusty).

Dependencies used:

* [nickel.rs](https://github.com/nickel-org/nickel.rs)
* [rust-url](https://github.com/servo/rust-url)
* my fork of [rust-mysql-simple](https://github.com/blackbeam/rust-mysql-simple)
