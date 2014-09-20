pub static DATABASE_USERNAME: &'static str = "pasteuser";
pub static DATABASE_PASSWORD: &'static str = "pastepass";
pub static DATABASE_NAME: &'static str = "paste";

// Whether or not to attempt to create and setup the database.
pub static CREATE_DATABASE: bool = false;

// The length of the random string designating a link to a paste.
pub static RANDOM_CODE_LENGTH: uint = 6;
