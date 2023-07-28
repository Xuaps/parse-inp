use super::sectionable::{Sectionable, SectionError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct VALVE {
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
    PRV,
    PSV,
    PBV,
    FCV,
    TCV,
    GPV,
}

impl Sectionable for VALVE {
    type SelfType = VALVE;

    fn from_section(properties: Vec<&str>, comment: Option<String>) -> Result<Self::SelfType, SectionError> {
        let id = properties.get(0).unwrap_or(&"").to_string();
        let start_node = properties.get(1).unwrap_or(&"").to_string();
        let end_node = properties.get(2).unwrap_or(&"").to_string();
        let diameter = properties.get(3).unwrap_or(&"").parse::<f64>()?;
        let valve_type = match properties.get(4) {
            Some(&"PRV") => ValveType::PRV,
            Some(&"PSV") => ValveType::PSV,
            Some(&"PBV") => ValveType::PBV,
            Some(&"FCV") => ValveType::FCV,
            Some(&"TCV") => ValveType::TCV,
            Some(&"GPV") => ValveType::GPV,
            _ => return Err(SectionError { message: "Invalid section".to_string() })
        };
        let valve_setting = properties.get(5).unwrap_or(&"").parse::<f64>()?;
        let minor_loss_coefficient = properties.get(6).unwrap_or(&"").parse::<f64>()?;

        Ok(VALVE {
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
    use super::VALVE;
    use super::ValveType;
    use super::Sectionable;

    #[test]
    fn create_a_valve() {
        let a_valve = VALVE::from_section(
            vec!["V1", "J1", "J2", "12", "PRV", "120", "0.2"],
            None,
        );

        assert_eq!(
            a_valve,
            Ok(VALVE {
                id: "V1".to_string(),
                start_node: "J1".to_string(),
                end_node: "J2".to_string(),
                diameter: 12.0,
                valve_type: ValveType::PRV,
                valve_setting: 120.0,
                minor_loss_coefficient: 0.2,
                comment: None,
            })
        ); 
    }

    #[test]
    fn all_the_properties_are_compulsory() {
        let a_valve = VALVE::from_section(
            vec!["V1", "J1", "J2", "12", "PRV", "0.2"],
            None,
        );

        assert!(a_valve.is_err());
    }
}
