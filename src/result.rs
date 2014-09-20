use std::str::SendStr;

pub type PasteResult<T> = Result<T, PasteError>;

pub struct PasteError {
    kind: PasteErrorKind,
    desc: SendStr
}

pub enum PasteErrorKind {
    ConnectionError,
    NoResultsError
}

impl PasteError {
    pub fn new<T: IntoMaybeOwned<'static>>(desc: T, kind: PasteErrorKind) -> PasteError {
        PasteError {
            kind: kind,
            desc: desc.into_maybe_owned()
        }
    }
}
