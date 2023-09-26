use super::sectionable::{Sectionable, SectionError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Valve {
    id: String,
    start_node: String,
    end_node: String,
    diameter: f64,
    valve_type: ValveType,
    valve_setting: f64,
    minor_loss_coefficient: f64,
    comment: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ValveType {
    Prv,
    Psv,
    Pbv,
    Fcv,
    Tcv,
    Gpv,
}

impl Sectionable for Valve {
    type SelfType = Valve;

    fn from_section(properties: Vec<&str>, comment: Option<String>) -> Result<Self::SelfType, SectionError> {
        let id = properties.first().unwrap_or(&"").to_string();
        let start_node = properties.get(1).unwrap_or(&"").to_string();
        let end_node = properties.get(2).unwrap_or(&"").to_string();
        let diameter = properties.get(3).unwrap_or(&"").parse::<f64>()?;
        let valve_type = match properties.get(4) {
            Some(&"PRV") => ValveType::Prv,
            Some(&"PSV") => ValveType::Psv,
            Some(&"PBV") => ValveType::Pbv,
            Some(&"FCV") => ValveType::Fcv,
            Some(&"TCV") => ValveType::Tcv,
            Some(&"GPV") => ValveType::Gpv,
            _ => return Err(SectionError { message: "Invalid section".to_string() })
        };
        let valve_setting = properties.get(5).unwrap_or(&"").parse::<f64>()?;
        let minor_loss_coefficient = properties.get(6).unwrap_or(&"").parse::<f64>()?;

        Ok(Valve {
            id,
            start_node,
            end_node,
            diameter,
            valve_type,
            valve_setting,
            minor_loss_coefficient,
            comment
        })
    }
}

#[cfg(test)]
mod test {
    use super::Valve;
    use super::ValveType;
    use super::Sectionable;

    #[test]
    fn create_a_valve() {
        let a_valve = Valve::from_section(
            vec!["V1", "J1", "J2", "12", "PRV", "120", "0.2"],
            None,
        );

        assert_eq!(
            a_valve,
            Ok(Valve {
                id: "V1".to_string(),
                start_node: "J1".to_string(),
                end_node: "J2".to_string(),
                diameter: 12.0,
                valve_type: ValveType::Prv,
                valve_setting: 120.0,
                minor_loss_coefficient: 0.2,
                comment: None,
            })
        ); 
    }

    #[test]
    fn all_the_properties_are_compulsory() {
        let a_valve = Valve::from_section(
            vec!["V1", "J1", "J2", "12", "PRV", "0.2"],
            None,
        );

        assert!(a_valve.is_err());
    }
}
