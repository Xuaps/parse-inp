//One line for each junction containing:

// ID label
// Bottom elevation, ft (m)
// Initial water level, ft (m)
// Minimum water level, ft (m)
// Maximum water level, ft (m)
// Nominal diameter, ft (m)
// Minimum volume, cubic ft (cubic meters)
// Volume curve ID (optional)
// Overflow indicator (YES / NO) (optional)
//
use super::sectionable::{Sectionable, SectionError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TANK {
    pub id: String,
    pub elevation: f64,
    pub init_level: f64,
    pub min_level: f64,
    pub max_level: f64,
    pub diameter: f64,
    pub min_volume: f64,
    pub volume_curve_id: Option<String>,
    pub overflow: bool,
    pub comment: Option<String>,
}

impl Sectionable for TANK {
    type SelfType = TANK;

    fn from_section(properties: Vec<&str>, comment: Option<String>) -> Result<Self::SelfType, SectionError> {
        let id = properties[0].to_string();
        let elevation = properties.get(1).parse::<f64>().unwrap();
        let init_level = properties.get(2).parse::<f64>().unwrap();
        let min_level = properties[3].parse::<f64>().unwrap();
        let max_level = properties[4].parse::<f64>().unwrap();
        let diameter = properties[5].parse::<f64>().unwrap();
        let min_volume = properties[6].parse::<f64>().unwrap();
        let volume_curve_id = if properties.len() > 7 {
            Some(properties[7].to_string())
        } else {
            None
        };
        let overflow = if properties.len() > 8 {
            properties[8] == "YES"
        } else {
            false
        };

        Ok(TANK {
            id,
            elevation,
            init_level,
            min_level,
            max_level,
            diameter,
            min_volume,
            volume_curve_id,
            overflow,
            comment,
        })
    }
}
