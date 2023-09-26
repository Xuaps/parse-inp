use super::sectionable::{Sectionable, SectionError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Emitter {
    pub junction_id: String,
    pub flow_coefficient: f64,
    pub comment: Option<String>,
}

impl Sectionable for Emitter {
    type SelfType = Emitter;

    fn from_section(properties: Vec<&str>, comment: Option<String>) -> Result<Self::SelfType, SectionError> {
        let junction_id = properties.first().unwrap_or(&"").to_string();
        let flow_coefficient = properties.get(1).unwrap_or(&"").parse::<f64>()?;

        Ok(Emitter {
            junction_id,
            flow_coefficient,
            comment
        })
    }
}

#[cfg(test)]
mod test {
    use super::Sectionable;
    use super::Emitter;

    #[test]
    fn craete_emitter_from_section() {
        let a_emitter = Emitter::from_section(
            vec!["J1", "0.5"],
            None,
        );

        assert_eq!(
            a_emitter,
            Ok(Emitter {
                junction_id: "J1".to_string(),
                flow_coefficient: 0.5,
                comment: None
            })
        );
    }

    #[test]
    fn all_the_properties_are_compulsory() {
        let a_emitter = Emitter::from_section(
            vec!["J1"],
            None,
        );

        assert!(a_emitter.is_err(), "Should be an error");
    }
}
