use super::sectionable::{Sectionable, SectionError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Quality {
    nodeid: String,
    initqual: f64,
    comment: Option<String>,
}

impl Sectionable for Quality {
    type SelfType = Quality;

    fn from_section(properties: Vec<&str>, comment: Option<String>) -> Result<Quality, SectionError> {
        if properties.len() < 2 {
            return Err(SectionError { 
                message: "Quality section must have at least 2 parameter; nodeid and initqual".to_string()
            });
        }
        let nodeid = properties[0].to_string();
        let initqual = properties[1].parse::<f64>()?;
        Ok(Quality {
            nodeid,
            initqual,
            comment,
        })
    }
}

#[cfg(test)]
mod test {
    use super::Quality;
    use super::Sectionable;

    #[test]
    fn create_quality_from_section() {
        let a_quality = Quality::from_section(vec!["1", "1"], Some("comment".to_string()));
        assert_eq!(a_quality, Ok(
            Quality {
                nodeid: "1".to_string(),
                initqual: 1.0,
                comment: Some("comment".to_string()),
            }));
    }

    #[test]
    fn create_quality_from_section_with_wrong_number_of_parameters() {
        let a_quality = Quality::from_section(vec!["1"], Some("comment".to_string()));
        assert!(a_quality.is_err(), "Expected error to be returned");
    } 

    #[test]
    fn create_quality_from_section_with_wrong_type_of_parameters() {
        let a_quality = Quality::from_section(vec!["1", "a"], Some("comment".to_string()));
        assert!(a_quality.is_err(), "Expected error to be returned");
    }
}



