use super::sectionable::{Sectionable, SectionError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EMITTER {
    pub junction_id: String,
    pub flow_coefficient: f64,
    pub comment: Option<String>,
}

impl Sectionable for EMITTER {
    type SelfType = EMITTER;

    fn from_section(properties: Vec<&str>, comment: Option<String>) -> Result<Self::SelfType, SectionError> {
        let junction_id = properties.get(0).unwrap_or(&"").to_string();
        let flow_coefficient = properties.get(1).unwrap_or(&"").parse::<f64>()?;

        Ok(EMITTER {
            junction_id,
            flow_coefficient,
            comment
        })
    }
}

#[cfg(test)]
mod test {
    use super::Sectionable;
    use super::EMITTER;

    #[test]
    fn craete_emitter_from_section() {
        let a_emitter = EMITTER::from_section(
            vec!["J1", "0.5"],
            None,
        );

        assert_eq!(
            a_emitter,
            Ok(EMITTER {
                junction_id: "J1".to_string(),
                flow_coefficient: 0.5,
                comment: None
            })
        );
    }

    #[test]
    fn all_the_properties_are_compulsory() {
        let a_emitter = EMITTER::from_section(
            vec!["J1"],
            None,
        );

        assert!(a_emitter.is_err(), "Should be an error");
    }
}
