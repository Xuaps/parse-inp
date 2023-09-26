use super::sectionable::{Sectionable, SectionError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Pipe {
    id: String,
    node1: String,
    node2: String,
    length: f64,
    diameter: f64,
    roughness: f64,
    minor_loss: f64,
    status: String,
    comment: Option<String>,
}

impl Sectionable for Pipe {
    type SelfType = Pipe;

    fn from_section(properties: Vec<&str>, comment: Option<String>) -> Result<Pipe, SectionError> {
        if properties.len() < 6 {
            return Err(SectionError { message: "Not enough properties to create PIPE section".to_string() });
        }

        let id = properties.first().unwrap_or(&"").to_string();
        let node1 = properties.get(1).unwrap_or(&"").to_string();
        let node2 = properties.get(2).unwrap_or(&"").to_string();
        let length = properties.get(3).unwrap_or(&"0.0").parse::<f64>()?;
        let diameter = properties.get(4).unwrap_or(&"0.0").parse::<f64>()?;
        let roughness = properties.get(5).unwrap_or(&"0.0").parse::<f64>()?;
        let minor_loss = properties.get(6).unwrap_or(&"0.0").parse::<f64>()?;
        let status = properties.get(7).unwrap_or(&"OPEN").to_string();

        let pipe = Pipe {
            id,
            node1,
            node2,
            length,
            diameter,
            roughness,
            minor_loss,
            status,
            comment,
        };

        Ok(pipe)
    }
}

#[cfg(test)]
mod test {
    use super::Sectionable;
    use super::Pipe;

    #[test]
    fn craete_pipe_from_section() {
        let a_pipe = Pipe::from_section(
            vec!["P1", "J1", "J2", "1200", "12", "120", "0.2", "OPEN"],
            None,
        );

        assert_eq!(
            a_pipe,
            Ok(Pipe {
                id: "P1".to_string(),
                node1: "J1".to_string(),
                node2: "J2".to_string(),
                length: 1200.0,
                diameter: 12.0,
                roughness: 120.0,
                minor_loss: 0.2,
                status: "OPEN".to_string(),
                comment: None,
            })
        );
    }

    #[test]
    fn create_pipe_from_section_without_optional_value() {
        let a_pipe = Pipe::from_section(
            vec!["P3", "J1", "J10", "1000", "12", "120"],
            Some("Description".to_string()),
        );

        assert_eq!(
            a_pipe,
            Ok(Pipe {
                id: "P3".to_string(),
                node1: "J1".to_string(),
                node2: "J10".to_string(),
                length: 1000.0,
                diameter: 12.0,
                roughness: 120.0,
                minor_loss: 0.0,
                status: "OPEN".to_string(),
                comment: Some("Description".to_string()),
            })
        );
    }

    #[test]
    fn return_error_with_not_enough_properties() {
        let a_pipe = Pipe::from_section(
            vec!["P3", "J1", "J10", "1000"],
            Some("Description".to_string()),
        );

        assert!(a_pipe.is_err(), "Should return error");
        assert!(a_pipe.err().unwrap().message == "Not enough properties to create PIPE section",
            "Should return error with message 'Not enough properties to create PIPE section'"); 
    }

    #[test]
    fn return_error_wrong_type() {
        let a_pipe = Pipe::from_section(
            vec!["P3", "J1", "J10", "1000", "12", "Test"],
            Some("Head stays constant".to_string()),
        );

        assert!(a_pipe.is_err(), "Should return error");
        assert!(a_pipe.err().unwrap().message == "invalid float literal", 
            "Should return error with message 'invalid float literal'");
    }
}
