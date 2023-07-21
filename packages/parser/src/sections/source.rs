use super::sectionable::{Sectionable, SectionError};
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

    fn from_section(properties: Vec<&str>, comment: Option<String>) -> Result<SOURCE, SectionError> {
        if properties.len() < 3 {
            return Err(SectionError { message: "Not enough properties to create SOURCE section".to_string()});
        }

        let node = properties[0].to_string();
        let source_type = properties[1].to_string();
        let strength = properties[2].parse::<f64>()?;
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

#[cfg(test)]
mod test {
    use super::Sectionable;
    use super::SOURCE;

    #[test]
    fn create_reservoir_from_section() {
        let a_source = SOURCE::from_section(
            vec!["N1", "CONCEN", "1.2", "Pat1"],
            Some("Concentration varies with time".to_string())
        );

        assert_eq!(
            a_source,
            Ok(SOURCE {
                node: "N1".to_string(),
                source_type: "CONCEN".to_string(),
                strength: 1.2,
                pattern: Some("Pat1".to_string()),
                comment: Some("Concentration varies with time".to_string()),
            })
        );
    }

    #[test]
    fn create_reservoir_from_section_without_optional_value() {
        let a_source = SOURCE::from_section(
            vec!["N44", "MASS", "12"],
            Some("Constant mass injection".to_string())
        );

        assert_eq!(
            a_source,
            Ok(SOURCE {
                node: "N44".to_string(),
                source_type: "MASS".to_string(),
                strength: 12.0,
                pattern: None,
                comment: Some("Constant mass injection".to_string()),
            })
        );
    }

    #[test]
    fn return_error_with_not_enough_properties() {
        let a_source = SOURCE::from_section(
            vec!["N44"],
            Some("Constant mass injection".to_string())
        );

        assert!(
            a_source.is_err(),
            "Expected error to be returned",
        );
        assert!(
            a_source
                .unwrap_err()
                .message == "Not enough properties to create SOURCE section",
        );
    }

    #[test]
    fn return_error_wrong_type() {
        let a_source = SOURCE::from_section(
            vec!["N44", "MASS", "TEST"],
            Some("Constant mass injection".to_string())
        );

        assert!(
            a_source.is_err(),
            "Expected error to be returned",
        );
        assert!(
            a_source
                .unwrap_err()
                .message == "invalid float literal",
            "Expected error message to be 'invalid float literal'",
        )
    }
}
