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

use note::{
    Note,
    NoteForm,
};
use settings;
use util;

pub fn get_home(_request: &Request, response: &mut Response) {
    response.send("hello world");
}

pub fn post_note(request: &Request, response: &mut Response) {
    response.set_content_type("application/json");

    let form = match request.json_as::<NoteForm>() {
        Some(form) => form,
        None       => {
            response.send(r#"{ "error": "failed to process form" }"#);
            return
        }
    };

    let data = match form.data {
        Some(data) => data,
        None       => {
            response.send(r#"{ "error": "data is required" }"#);
            return
        }
    };
    let code = util::random_string(settings::RANDOM_CODE_LENGTH);
    let mut note = Note::new(0, code.clone(), time::now_utc().to_timespec(), data);

    Note::insert(&mut note);

    response.send(format!("/{}", code));
}

pub fn get_note(request: &Request, response: &mut Response) {
    let code = request.params.get(&"code".to_string());

    let notes = Note::all();
    let mut notes = notes.iter().filter(|&x| &x.code == code);

    let note = match notes.next() {
        Some(note) => note,
        None       => {
            response.send("Failed to find note in database");
            return
        }
    };

    response.send(note.data.clone());
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
