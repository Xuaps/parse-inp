use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Error {
    pub message: String,
    pub line: String,
    pub line_number: i32,
}
