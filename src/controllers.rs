use http::headers::content_type::MediaType;
use http::status::{
    Found,
    NotFound,
};
use nickel::{
    Action,
    Continue,
    ErrorWithStatusCode,
    Halt,
    NickelError,
    Request,
    Response,
};
use time;
use url::{
    percent_encoding,
    Url,
};

use note::Note;
use settings;
use util;

pub fn post_note(request: &Request, response: &mut Response) {
    response.set_content_type("application/x-www-form-urlencoded");

    let body = request.origin.body.as_slice();
    // Replace '+' with ' ', as percent_encoding doesn't seem to be able to handle it.
    let body = body.replace("+", " ");
    let data = match body.as_slice()
                         .split('&')
                         .filter(|x| x.as_slice()
                                      .starts_with("data"))
                         .next() {
        Some(x) => match x.split('=').skip(1).next() {
            Some(value) => percent_encoding::lossy_utf8_percent_decode(value.as_bytes()),
            None        => {
                response.send(r#"{ "error": "failed to process form" }"#);
                return
            }
        },
        None    => {
            response.send(r#"{ "error": "failed to process form" }"#);
            return
        }
    };

    let code = util::random_string(settings::RANDOM_CODE_LENGTH);
    let mut note = Note::new(0, code.clone(), time::now_utc().to_timespec(), data.to_string());

    Note::insert(&mut note);

    response.origin.status = Found;
    response.origin.headers.location = Some(Url::parse(format!("{}/{}", settings::BASE_URL, code).as_slice()).unwrap());
}

pub fn get_note(request: &Request, response: &mut Response) {
    response.origin.headers.content_type = Some(MediaType {
        type_: "text".to_string(),
        subtype: "plain".to_string(),
        parameters: vec!((String::from_str("charset"), String::from_str("UTF-8")))
    });

    let code = request.params.index(&"code".to_string());

    let notes = Note::all();
    let mut notes = notes.iter().filter(|&x| &x.code == code);

    match notes.next() {
        Some(note) => {
            response.send(note.data.clone());
        },
        None => {
            response.set_content_type("html");
            response.origin.status = NotFound;
            response.send("<h1>404 - Paste Not Found</h1>");
        }
    };
}

pub fn custom_404(err: &NickelError, _req: &Request, response: &mut Response) -> Result<Action, NickelError> {
    match err.kind {
        ErrorWithStatusCode(NotFound) => {
            response.set_content_type("html");
            response.origin.status = NotFound;
            response.send("<h1>404 - Paste Not Found</h1>");
            Ok(Halt)
        },
        _ => Ok(Continue)
    }
}
