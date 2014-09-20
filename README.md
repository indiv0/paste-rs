# paste [![Build Status](https://travis-ci.org/Indiv0/paste.svg?branch=master)](https://travis-ci.org/Indiv0/paste)

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
