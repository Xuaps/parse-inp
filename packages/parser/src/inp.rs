use serde::{Serialize, Deserialize};
use crate::{Sectionable, SectionError};
use crate::sections::{SOURCE, RESERVOIR, PIPE, UNKNOWN, ERROR, JUNCTION, TANK, PUMP, VALVE};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct INP {
    title: String,
    junctions: Vec<JUNCTION>,
    reservoirs: Vec<RESERVOIR>,
    tanks: Vec<TANK>,
    pipes: Vec<PIPE>,
    pumps: Vec<PUMP>,
    valves: Vec<VALVE>,
    sources: Vec<SOURCE>,
    unknown_sections: Vec<UNKNOWN>,
    errors: Vec<ERROR>
}

fn read_section(line: &str) -> Option<String> {
    let mut section = String::new();
    let mut chars = line.chars().skip(1);
    let mut c = chars.next();
    while c != Some(']') {
        section.push(c.unwrap());
        c = chars.next();
    }

    Some(section)
}

fn read_title_line(line: &str) -> String {
    let mut title = String::new();
    let mut chars = line.chars();
    let mut c = chars.next();
    while c != None {
        title.push(c.unwrap());
        c = chars.next();
    }
    title
}

fn build_section<T: Sectionable<SelfType=T>>(line: &str) -> Result<T, SectionError> {
    let (properties, comment) = get_properties_and_comment(line);

    T::from_section(properties, comment)
}

fn get_properties_and_comment<'a>(line: &'a str) -> (Vec<&str>, Option<String>) {
    let mut parts = line.split(";");
    let properties = parts.next().unwrap_or("").split_whitespace().collect::<Vec<&'a str>>();
    let comment = parts.next().map(|s| s.to_string());

    (properties, comment)
}

impl INP {
    pub fn read(content: String) -> Self {
        let mut inp = INP { 
            title: String::new(), 
            junctions: Vec::new(),
            sources: Vec::new(), 
            tanks: Vec::new(),
            reservoirs: Vec::new(), 
            pipes: Vec::new(),
            pumps: Vec::new(),
            valves: Vec::new(),
            unknown_sections: Vec::new(),
            errors: Vec::new(),
        };
        let mut line_number = 1;
        let mut lines = content.lines();
        let mut section = None;
        while let Some(line) = lines.next() {
            match line.trim().chars().next() {
                None => continue,
                Some('[') => {
                    section = read_section(line.trim());
                }
                Some(';') => continue,
                _ => match section.as_deref() {
                        Some("TITLE") => inp.set_title_line(read_title_line(line).as_str()),
                        Some("JUNCTIONS") => inp.add_junction(line, line_number),
                        Some("TANKS") => inp.add_tank(line, line_number),
                        Some("SOURCES") => inp.add_source(line, line_number),
                        Some("RESERVOIRS") => inp.add_reservoir(line, line_number),
                        Some("PIPES") => inp.add_pipe(line, line_number),
                        Some("PUMPS") => inp.add_pump(line, line_number),
                        Some("VALVES") => inp.add_valve(line, line_number),
                        _ => inp.unknown_sections.push(UNKNOWN { text: line.to_string() })
                    }
            }
            line_number += 1;
        }
        inp
    }


    fn set_title_line(&mut self, s: &str) {
        if !self.title.is_empty() {
            self.title.push(' ');
        }
        self.title.push_str(s);
    }

    fn add_source(&mut self, line: &str, line_number: u32) {
        match build_section::<SOURCE>(line) {
            Ok(source) => self.sources.push(source),
            Err(e) => self.errors.push(ERROR { message: e.to_string(), line: line.to_string(), line_number })
        }
    }

    fn add_reservoir(&mut self, line: &str, line_number: u32) {
        match build_section::<RESERVOIR>(line) {
            Ok(reservoir) => self.reservoirs.push(reservoir),
            Err(e) => self.errors.push(ERROR { message: e.to_string(), line: line.to_string(), line_number })
        }
    }

    fn add_pipe(&mut self, line: &str, line_number: u32) {
        match build_section::<PIPE>(line) {
            Ok(pipe) => self.pipes.push(pipe),
            Err(e) => self.errors.push(ERROR { message: e.to_string(), line: line.to_string(), line_number })
        }
    }

    fn add_junction(&mut self, line: &str, line_number: u32) {
        match build_section::<JUNCTION>(line) {
            Ok(junction) => self.junctions.push(junction),
            Err(e) => self.errors.push(ERROR { message: e.to_string(), line: line.to_string(), line_number })
        }
    }

    fn add_tank(&mut self, line: &str, line_number: u32) {
        match build_section::<TANK>(line) {
            Ok(tank) => self.tanks.push(tank),
            Err(e) => self.errors.push(ERROR { message: e.to_string(), line: line.to_string(), line_number })
        }
    }

    fn add_pump(&mut self, line: &str, line_number: u32) {
        match build_section::<PUMP>(line) {
            Ok(pump) => self.pumps.push(pump),
            Err(e) => self.errors.push(ERROR { message: e.to_string(), line: line.to_string(), line_number })
        }
    }

    fn add_valve(&mut self, line: &str, line_number: u32) {
        match build_section::<VALVE>(line) {
            Ok(valve) => self.valves.push(valve),
            Err(e) => self.errors.push(ERROR { message: e.to_string(), line: line.to_string(), line_number })
        }
    }
}


#[cfg(test)]
mod test {
    use std::fs;
    use super::INP;
    use super::UNKNOWN;
    use crate::sections::ERROR;

    #[test]
    fn read_inp() {
        let file_path = "tests/MagneticIslandEnhanced.inp";

        let input = fs::read_to_string(file_path).unwrap();

        let inp = INP::read(input);
        assert!(inp.title.contains("Magnetic Island"));
        assert_eq!(inp.reservoirs.len(), 2);
        assert_eq!(inp.sources.len(), 0);
        assert_eq!(inp.pipes.len(), 1648);
        assert_eq!(inp.junctions.len(), 2050);
        assert_eq!(inp.tanks.len(), 4);
        assert_eq!(inp.pumps.len(), 5);
        assert_eq!(inp.valves.len(), 507);
        assert!(inp.unknown_sections.len() > 0);
    }

    #[test]
    fn read_inp_with_section_error() {
        let input =r#"
[RESERVOIRS]
;ID    Head      Pattern
;----------------------- 
R1     Pat1               ;Head stays constant
R2         ;Head varies with time

        "#;
        let inp = INP::read(input.to_string());
        assert_eq!(inp.errors[0], ERROR {
                message: "invalid float literal".to_string(), 
                line: "R1     Pat1               ;Head stays constant".to_string(),
                line_number: 2
        });
        assert_eq!(inp.errors[1], ERROR {
                message: "Not enough properties to create RESERVOIR section".to_string(), 
                line: "R2         ;Head varies with time".to_string(),
                line_number: 3
        });
    }

    #[test]
    fn read_inp_with_section_format_error() {
        let input =r#"
[[RESERVOIRS]
R1     Test               ;Head stays constant
R2     120       Pat1    ;Head varies with time

        "#;
        let inp = INP::read(input.to_string());
        assert_eq!(
            inp.unknown_sections, 
            vec![
                UNKNOWN { 
                    text: "R1     Test               ;Head stays constant".to_string(), 
                },
                UNKNOWN { 
                    text: "R2     120       Pat1    ;Head varies with time".to_string(), 
                },
            ]
        );
    }
}

