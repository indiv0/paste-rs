use std::rand::{
    task_rng,
    Rng,
};

pub fn random_string(length: uint) -> String {
    let mut string = String::new();
    for n in task_rng().gen_ascii_chars().take(length) {
        string.push_char(n);
    }
    string
}
