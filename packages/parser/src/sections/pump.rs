use super::sectionable::{Sectionable, SectionError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Pump {
    pub id: String,
    pub start_node: String,
    pub end_node: String,
    pub power: Option<f32>,
    pub head: Option<String>,
    pub speed: Option<f32>,
    pub pattern: Option<String>,
    pub comment: Option<String>,
}

impl Sectionable for Pump {
    type SelfType = Pump;

    fn from_section(properties: Vec<&str>, comment: Option<String>) -> Result<Self::SelfType, SectionError> {
        let id = properties.first().unwrap_or(&"").to_string();
        let start_node = properties.get(1).unwrap_or(&"").to_string();
        let end_node = properties.get(2).unwrap_or(&"").to_string();
        let mut power = None;
        let mut head = None;
        let mut speed = None;
        let mut pattern = None;

        for i in (3..properties.len()).step_by(2) {
            match properties.get(i) {
                Some(&"POWER") => power = Some(properties.get(i + 1).unwrap_or(&"").parse::<f32>()?),
                Some(&"HEAD") => head = Some(properties.get(i + 1).unwrap_or(&"").to_string()),
                Some(&"SPEED") => speed = Some(properties.get(i + 1).unwrap_or(&"").parse::<f32>()?),
                Some(&"PATTERN") => pattern = Some(properties.get(i + 1).unwrap_or(&"").to_string()),
                _ => return Err(SectionError { message: "Invalid section".to_string() })
            }
        }

        if power.is_none() && head.is_none() {
            return Err(SectionError { message: "Invalid section".to_string() });
        }

        Ok(Pump {
            id,
            start_node,
            end_node,
            power,
            head,
            speed,
            pattern,
            comment
        })
    }
}

#[cfg(test)]
mod test {
    use super::Pump;
    use super::Sectionable;

    #[test]
    fn create_pump_from_section() {
        let pump = Pump::from_section(
            vec!["PUMP1", "NODE1", "NODE2", "POWER", "10.0", "PATTERN", "PATTERN1"],
            None).unwrap();    
        
        assert_eq!(pump.id, "PUMP1");
        assert_eq!(pump.start_node, "NODE1");
        assert_eq!(pump.end_node, "NODE2");
        assert_eq!(pump.power, Some(10.0));
        assert_eq!(pump.head, None);
        assert_eq!(pump.speed, None);
        assert_eq!(pump.pattern, Some("PATTERN1".to_string()));
    }

    #[test]
    fn either_power_or_head_must_be_supplied_for_each_pump() {
        let pump = Pump::from_section(
            vec!["PUMP1", "NODE1", "NODE2", "SPEED", "10.0", "PATTERN", "PATTERN1"],
            None);

        assert!(pump.is_err());
    }

    #[test]
    fn id_start_node_and_end_node_are_compulsory () {
        let pump = Pump::from_section(
            vec!["PUMP1", "NODE1", "POWER", "10.0", "PATTERN", "PATTERN1"],
            None);

        assert!(pump.is_err());
    }
}
