use http::status::NotFound;
use nickel::{
    Action,
    Continue,
    ErrorWithStatusCode,
    Halt,
    JsonBody,
    NickelError,
    Request,
    Response,
};
use time;
use url::percent_encoding;

use note::{
    Note,
    NoteForm,
};
use settings;
use util;

pub fn post_note(request: &Request, response: &mut Response) {
    response.set_content_type("application/x-www-form-urlencoded");

    let body = request.origin.body.as_slice();
    println!("{}", body);
    let data = match body.split('&')
                         .filter(|x| x.as_slice()
                                      .starts_with("data"))
                         .next() {
        Some(x) => match x.split('=').skip(1).next() {
            Some(value) => percent_encoding::percent_decode(value.as_bytes()),
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

    println!("{}", data);

    let code = util::random_string(settings::RANDOM_CODE_LENGTH);
    let mut note = Note::new(0, code.clone(), time::now_utc().to_timespec(), data.to_string());

    Note::insert(&mut note);

    response.send(format!("/{}", code));
}

pub fn get_note(request: &Request, response: &mut Response) {
    response.set_content_type("text/plain");

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
