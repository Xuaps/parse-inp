use serde::{Serialize, Deserialize};
use crate::{Sectionable, SectionError};
use crate::sections::{Source, Reservoir, Pipe, Unknown, Error, Junction, Tank, Pump, Valve, Emitter, Quality};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct INP {
    title: String,
    junctions: Vec<Junction>,
    reservoirs: Vec<Reservoir>,
    tanks: Vec<Tank>,
    pipes: Vec<Pipe>,
    pumps: Vec<Pump>,
    valves: Vec<Valve>,
    emitters: Vec<Emitter>,
    
    quality: Vec<Quality>,
    sources: Vec<Source>,
    
    unknown_sections: Vec<Unknown>,
    errors: Vec<Error>
}

struct LineData {
    content: String,
    number: i32
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
    while c.is_some() {
        title.push(c.unwrap());
        c = chars.next();
    }
    title
}

fn build_section<T: Sectionable<SelfType=T>>(line: &str) -> Result<T, SectionError> {
    let (properties, comment) = get_properties_and_comment(line);

    T::from_section(properties, comment)
}

fn add<T: Sectionable<SelfType=T>>(line: LineData, items: &mut Vec<T>, errors: &mut Vec<Error>)
{
    match build_section::<T>(line.content.as_str()) {
        Ok(source) => items.push(source),
        Err(e) => errors.push(Error { message: e.to_string(), line: line.content.to_string(), line_number: line.number })
    }
    
}

fn get_properties_and_comment<'a>(line: &'a str) -> (Vec<&str>, Option<String>) {
    let mut parts = line.split(';');
    let properties = parts.next().unwrap_or("").split_whitespace().collect::<Vec<&'a str>>();
    let comment = parts.next().map(|s| s.to_string());

    (properties, comment)
}

impl INP {
    pub fn read(content: String) -> Self {
        let mut inp = INP { 
            title: String::new(), 
            junctions: Vec::new(),
            tanks: Vec::new(),
            reservoirs: Vec::new(), 
            pipes: Vec::new(),
            pumps: Vec::new(),
            valves: Vec::new(),
            emitters: Vec::new(),
            quality: Vec::new(),
            sources: Vec::new(), 
            unknown_sections: Vec::new(),
            errors: Vec::new(),
        };
        let mut line_number = 1;
        let lines = content.lines();
        let mut section = None;
        for line in lines {
            let data = LineData { content: line.to_string() , number: line_number };
            match line.trim().chars().next() {
                None => continue,
                Some('[') => {
                    section = read_section(line.trim());
                }
                Some(';') => continue,
                _ => match section.as_deref() {
                        Some("TITLE") => inp.set_title_line(read_title_line(line).as_str()),
                        Some("JUNCTIONS") => add::<Junction>(data, &mut inp.junctions, &mut inp.errors),
                        Some("RESERVOIRS") => add::<Reservoir>(data, &mut inp.reservoirs, &mut inp.errors),
                        Some("TANKS") => add::<Tank>(data, &mut inp.tanks, &mut inp.errors),
                        Some("PIPES") => add::<Pipe>(data, &mut inp.pipes, &mut inp.errors),
                        Some("PUMPS") => add::<Pump>(data, &mut inp.pumps, &mut inp.errors),
                        Some("VALVES") => add::<Valve>(data, &mut inp.valves, &mut inp.errors),
                        Some("EMITTERS") => add::<Emitter>(data, &mut inp.emitters, &mut inp.errors),
                        Some("SOURCES") => add::<Source>(data, &mut inp.sources, &mut inp.errors),
                        Some("QUALITY") => add::<Quality>(data, &mut inp.quality, &mut inp.errors),
                        _ => inp.unknown_sections.push(Unknown { text: line.to_string() })
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
}


#[cfg(test)]
mod test {
    use std::fs;
    use super::INP;
    use super::Unknown;
    use crate::sections::Error;

    #[test]
    fn read_inp() {
        let file_path = "tests/MagneticIslandEnhanced.inp";

        let input = fs::read_to_string(file_path).unwrap();

        let inp = INP::read(input);
        assert!(inp.title.contains("Magnetic Island"));
        assert_eq!(inp.reservoirs.len(), 2);
        assert_eq!(inp.pipes.len(), 1648);
        assert_eq!(inp.junctions.len(), 2050);
        assert_eq!(inp.tanks.len(), 4);
        assert_eq!(inp.pumps.len(), 5);
        assert_eq!(inp.valves.len(), 507);
        assert_eq!(inp.emitters.len(), 2020);

        assert_eq!(inp.quality.len(), 1);
        assert_eq!(inp.sources.len(), 0);
        
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
        assert_eq!(inp.errors[0], Error {
                message: "invalid float literal".to_string(), 
                line: "R1     Pat1               ;Head stays constant".to_string(),
                line_number: 2
        });
        assert_eq!(inp.errors[1], Error {
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
                Unknown { 
                    text: "R1     Test               ;Head stays constant".to_string(), 
                },
                Unknown { 
                    text: "R2     120       Pat1    ;Head varies with time".to_string(), 
                },
            ]
        );
    }
}

