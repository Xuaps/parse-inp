use super::sectionable::{Sectionable, SectionError};
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

    fn from_section(properties: Vec<&str>, comment: Option<String>) -> Result<RESERVOIR, SectionError> {
        if properties.len() < 2 {
            return Err(SectionError { message: "Not enough properties to create RESERVOIR section".to_string()});
        }

        let id = properties.get(0).unwrap_or(&"").to_string();
        let head = properties.get(1).unwrap_or(&"0.0").parse::<f64>()?;
        let pattern = properties.get(2).map(|s| s.to_string());
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

#[cfg(test)]
mod test {
    use super::Sectionable;
    use super::RESERVOIR;

    #[test]
    fn create_reservoir_from_section() {
        let a_reservoir = RESERVOIR::from_section(
            vec![
                "R2",
                "120",
                "Pat1",
            ],
            Some("Head varies with time".to_string()),
        );

        assert_eq!(
            a_reservoir,
            Ok(RESERVOIR {
                id: "R2".to_string(),
                head: 120.0,
                pattern: Some("Pat1".to_string()),
                comment: Some("Head varies with time".to_string()),
            })
        );
    }

    #[test]
    fn create_reservoir_from_section_without_optional_value() {
        let a_reservoir = RESERVOIR::from_section(
            vec!["R1", "512"],
            Some("Head stays constant".to_string()),
        );

        assert_eq!(
            a_reservoir,
            Ok(RESERVOIR {
                id: "R1".to_string(),
                head: 512.0,
                pattern: None,
                comment: Some("Head stays constant".to_string()),
            })
        );
    }

    #[test]
    fn return_error_with_not_enough_properties() {
        let a_reservoir = RESERVOIR::from_section(
            vec!["R1"],
            Some("Head stays constant".to_string()),
        );

        assert!(
            a_reservoir.is_err(),
        );
        assert!(
            a_reservoir
                .unwrap_err()
                .message == "Not enough properties to create RESERVOIR section",
        );
    }

    #[test]
    fn return_error_wrong_type() {
        let a_reservoir = RESERVOIR::from_section(
            vec!["R1", "Test"],
            Some("Head stays constant".to_string()),
        );

        assert!(
            a_reservoir.is_err(),
            "Expected error to be returned",
        );
        assert!(
            a_reservoir
                .unwrap_err()
                .message == "invalid float literal",
            "Expected error message to be 'invalid float literal'",
        )
    }
}