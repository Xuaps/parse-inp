use std::error::Error;
use std::fmt;
use std::num::ParseFloatError;

#[derive(Debug, PartialEq)]
pub struct SectionError {
    pub message: String,
}

impl fmt::Display for SectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<ParseFloatError> for SectionError {
    fn from(error: ParseFloatError) -> Self {
        SectionError {
            message: error.to_string(),
        }
    }
}

impl Error for SectionError {}

pub trait Sectionable {
    type SelfType;

    fn from_section(properties: Vec<&str>, comment: Option<String>) -> Result<Self::SelfType, SectionError>
    where
    Self: Sized;
}
