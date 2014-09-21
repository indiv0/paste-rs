pub static DATABASE_USERNAME: &'static str = "pasteuser";
pub static DATABASE_PASSWORD: &'static str = "pastepass";
pub static DATABASE_NAME: &'static str = "paste";

// Whether or not to attempt to create and setup the database.
pub static CREATE_DATABASE: bool = false;

// The length of the random string designating a link to a paste.
pub static RANDOM_CODE_LENGTH: uint = 6;

// The base url for the website (used for relative links and redirects).
pub static BASE_URL: &'static str = "http://127.0.0.1:3000";
