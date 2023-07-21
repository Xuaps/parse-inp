use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ERROR {
    pub message: String,
    pub line: String,
    pub line_number: usize,
}
