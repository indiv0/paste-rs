use http::headers::content_type::MediaType;
use http::status::{
    Found,
    InternalServerError,
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

    let mut code: Option<String> = None;
    for i in range(0u, 50u) {
        // If the code generator collides 50 times in a row, quit.
        if i == 49 {
            response.origin.status = InternalServerError;
            response.send("Dem collisions doe");
            return
        }

        let generated_code = util::random_string(settings::RANDOM_CODE_LENGTH);

        // Verify the code is not already taken.
        match Note::find_by_code(generated_code.as_slice()) {
            Some(_) => {},
            None => {
                code = Some(generated_code);
                break
            }
        }
    }
    let code = code.unwrap();
    let mut note = Note::new(0, code.clone().as_slice(), time::now_utc().to_timespec(), data.as_slice());

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

    let note = Note::find_by_code(code.as_slice());

    match note {
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
