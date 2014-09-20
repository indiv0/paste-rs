use time::Timespec;

#[deriving(Clone, PartialEq, Show)]
pub struct Note {
    pub id: i32,
    pub code: String,
    pub time_created: Timespec,
    pub data: Option<Vec<u8>>
}

impl Note {
    pub fn new(id: i32, code: String, time_created: Timespec, data: Option<Vec<u8>>) -> Note {
        Note {
            id: id,
            code: code,
            time_created: time_created,
            data: data
        }
    }
}
