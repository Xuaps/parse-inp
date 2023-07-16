use super::sectionable::Sectionable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PIPE {
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

impl Sectionable for PIPE {
    type SelfType = PIPE;

    fn from_section(properties: Vec<&str>, comment: Option<String>) -> Result<PIPE, String> {
        if properties.len() < 6 {
            return Err("Not enough properties to create PIPE section".to_string());
        }

        let id = properties[0].to_string();
        let node1 = properties[1].to_string();
        let node2 = properties[2].to_string();
        let length = properties[3].parse::<f64>().unwrap();
        let diameter = properties[4].parse::<f64>().unwrap();
        let roughness = properties[5].parse::<f64>().unwrap();
        let minor_loss = if properties.len() > 6 {
            properties[6].parse::<f64>().unwrap()
        } else {
            0.0
        };
        let status = if properties.len() > 7 {
            properties[7].to_string()
        } else {
            "OPEN".to_string()
        };
        let comment = comment.map(|s| s.to_string());

        let pipe = PIPE {
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
    use super::PIPE;

    #[test]
    fn craete_pipe_from_section() {
        let a_pipe = PIPE::from_section(
            vec!["P1", "J1", "J2", "1200", "12", "120", "0.2", "OPEN"],
            None,
        );

        assert_eq!(
            a_pipe,
            Ok(PIPE {
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
        let a_pipe = PIPE::from_section(
            vec!["P3", "J1", "J10", "1000", "12", "120"],
            Some("Description".to_string()),
        );

        assert_eq!(
            a_pipe,
            Ok(PIPE {
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
        let a_pipe = PIPE::from_section(
            vec!["P3", "J1", "J10", "1000"],
            Some("Description".to_string()),
        );

        assert_eq!(
            a_pipe,
            Err("Not enough properties to create PIPE section".to_string())
        );
    }
}
