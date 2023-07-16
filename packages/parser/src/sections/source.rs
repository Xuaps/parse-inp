use super::sectionable::Sectionable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SOURCE {
    node: String,
    source_type: String,
    strength: f64,
    pattern: Option<String>,
    comment: Option<String>,
}

impl Sectionable for SOURCE {
    type SelfType = SOURCE;

    fn from_section(properties: Vec<&str>, comment: Option<String>) -> Result<SOURCE, String> {
        let node = properties[0].to_string();
        let source_type = properties[1].to_string();
        let strength = properties[2].parse::<f64>().unwrap();
        let pattern = if properties.len() > 3 {
            Some(properties[3].to_string())
        } else {
            None
        };
        let comment = comment.map(|s| s.to_string());

        let source = SOURCE {
            node,
            source_type,
            strength,
            pattern,
            comment,
        };

        Ok(source)
    }
}
