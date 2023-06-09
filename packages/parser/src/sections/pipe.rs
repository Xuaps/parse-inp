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
    fn from_section(properties: Vec<&str>, comment: Option<String>) -> Self {
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

        PIPE {
            id,
            node1,
            node2,
            length,
            diameter,
            roughness,
            minor_loss,
            status,
            comment,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Sectionable;
    use super::PIPE;

    #[test]
    fn build_pipes() {
        let a_pipe = PIPE::from_section(
            vec!["P1", "J1", "J2", "1200", "12", "120", "0.2", "OPEN"],
            None,
        );
        let another_pipe = PIPE::from_section(
            vec!["P3", "J1", "J10", "1000", "12", "120"],
            Some("Description".to_string()),
        );

        assert_eq!(
            a_pipe,
            PIPE {
                id: "P1".to_string(),
                node1: "J1".to_string(),
                node2: "J2".to_string(),
                length: 1200.0,
                diameter: 12.0,
                roughness: 120.0,
                minor_loss: 0.2,
                status: "OPEN".to_string(),
                comment: None,
            }
        );
        assert_eq!(
            another_pipe,
            PIPE {
                id: "P3".to_string(),
                node1: "J1".to_string(),
                node2: "J10".to_string(),
                length: 1000.0,
                diameter: 12.0,
                roughness: 120.0,
                minor_loss: 0.0,
                status: "OPEN".to_string(),
                comment: Some("Description".to_string()),
            }
        );
    }
}
