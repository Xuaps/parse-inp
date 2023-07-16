use super::sectionable::Sectionable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RESERVOIR {
    id: String,
    head: f64,
    pattern: Option<String>,
    comment: Option<String>,
}

impl Sectionable for RESERVOIR {
    type SelfType = RESERVOIR;

    fn from_section(properties: Vec<&str>, comment: Option<String>) -> Result<RESERVOIR, String> {
        let id = properties[0].to_string();
        let head = properties[1].parse::<f64>().unwrap();
        let pattern = if properties.len() > 2 {
            Some(properties[2].to_string())
        } else {
            None
        };
        let comment = comment.map(|s| s.to_string());

        let reservoir = RESERVOIR {
            id,
            head,
            pattern,
            comment,
        };

        Ok(reservoir)
    }
}
