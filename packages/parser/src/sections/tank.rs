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
        let id = properties.get(0).unwrap_or(&"").to_string();
        let elevation = properties.get(1).unwrap_or(&"").parse::<f64>()?;
        let init_level = properties.get(2).unwrap_or(&"").parse::<f64>()?;
        let min_level = properties.get(3).unwrap_or(&"").parse::<f64>()?;
        let max_level = properties.get(4).unwrap_or(&"").parse::<f64>()?;
        let diameter = properties.get(5).unwrap_or(&"").parse::<f64>()?;
        let min_volume = properties.get(6).unwrap_or(&"").parse::<f64>()?;
        let volume_curve_id = properties.get(7).map(|s| s.to_string());
        let overflow = properties.get(8).map(|s| s.to_string().to_lowercase()).unwrap_or("no".to_string()) == "yes";

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
            comment
        })
    }
}

#[cfg(test)]
mod test {
    use super::TANK;
    use super::Sectionable;
    use super::SectionError;

    #[test]
    fn create_tank_from_section() {
        let tank = TANK::from_section(
            vec!["TANK1", "10.0", "20.0", "30.0", "40.0", "50.0", "60.0", "VOLUME_CURVE", "YES"],
            None).unwrap();    
        
        assert_eq!(tank.id, "TANK1");
        assert_eq!(tank.elevation, 10.0);
        assert_eq!(tank.init_level, 20.0);
        assert_eq!(tank.min_level, 30.0);
        assert_eq!(tank.max_level, 40.0);
        assert_eq!(tank.diameter, 50.0);
        assert_eq!(tank.min_volume, 60.0);
        assert_eq!(tank.volume_curve_id, Some("VOLUME_CURVE".to_string()));
        assert_eq!(tank.overflow, true);
    }
    
    #[test]
    fn create_tank_from_section_without_optional_fields() {
        let tank = TANK::from_section(
            vec!["TANK1", "10.0", "20.0", "30.0", "40.0", "50.0", "60.0"],
            None).unwrap();    
        
        assert_eq!(tank.id, "TANK1");
        assert_eq!(tank.elevation, 10.0);
        assert_eq!(tank.init_level, 20.0);
        assert_eq!(tank.min_level, 30.0);
        assert_eq!(tank.max_level, 40.0);
        assert_eq!(tank.diameter, 50.0);
        assert_eq!(tank.min_volume, 60.0);
        assert_eq!(tank.volume_curve_id, None);
        assert_eq!(tank.overflow, false);
    }

    #[test]
    fn create_tank_from_section_with_wrong_properties() {
        let tank = TANK::from_section(
            vec!["TANK1", "10.0", "20.0", "30.0", "40.0", "50.0", "WRONG"],
            None);   

        assert_eq!(
            tank,
            Err(SectionError { message: "invalid float literal".to_string() })
        );
    }
}
