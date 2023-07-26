use super::sectionable::{Sectionable, SectionError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct JUNCTION {
    id: String,
    elevation: f64,
    base_demand_flow: Option<f64>,
    demand_pattern_id: Option<String>,
    comment: Option<String>,
}

impl Sectionable for JUNCTION {
    type SelfType = JUNCTION;

    fn from_section(properties: Vec<&str>, comment: Option<String>) -> Result<JUNCTION, SectionError> {
        if properties.len() < 2 {
            return Err(SectionError { message: "Not enough properties to create JUNCTION section".to_string()});
        }

        let id = properties.get(0).unwrap_or(&"").to_string();
        let elevation = properties.get(1).unwrap_or(&"0.0").parse::<f64>()?;
        let base_demand_flow = properties.get(2).map(|s| s.parse::<f64>().unwrap());
        let demand_pattern_id = properties.get(3).map(|s| s.to_string());
        let comment = comment.map(|s| s.to_string());

        let junction = JUNCTION {
            id,
            elevation,
            base_demand_flow,
            demand_pattern_id,
            comment
        };

        Ok(junction)
    }
}

#[cfg(test)]
mod test {
    use super::Sectionable;
    use super::SectionError;
    use super::JUNCTION;

    #[test]
    fn create_junction_from_section() {
        let a_junction = JUNCTION::from_section(
            vec![
                "J1",
                "100",
                "0.0",
                "Pat1",
            ],
            Some("Junction 1".to_string()),
        );

        assert_eq!(
            a_junction,
            Ok(JUNCTION {
                id: "J1".to_string(),
                elevation: 100.0,
                base_demand_flow: Some(0.0),
                demand_pattern_id: Some("Pat1".to_string()),
                comment: Some("Junction 1".to_string()),
            })
        );
    }

    #[test]
    fn create_junction_from_section_without_optional_fields() {
        let a_junction = JUNCTION::from_section(
            vec![
                "J1",
                "100",
            ],
            Some("Junction 1".to_string()),
        );

        assert_eq!(
            a_junction,
            Ok(JUNCTION {
                id: "J1".to_string(),
                elevation: 100.0,
                base_demand_flow: None,
                demand_pattern_id: None,
                comment: Some("Junction 1".to_string()),
            })
        );
    }

    #[test]
    fn create_junction_from_section_with_invalid_elevation() {
        let a_junction = JUNCTION::from_section(
            vec![
                "J1",
                "100a",
            ],
            Some("Junction 1".to_string()),
        );

        assert_eq!(
            a_junction,
            Err(SectionError { message: "invalid float literal".to_string() })
        );
    }
}
