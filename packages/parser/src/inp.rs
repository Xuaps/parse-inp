use serde::{Serialize, Deserialize};
use crate::{Sectionable, SectionError};
use crate::sections::{SOURCE, RESERVOIR, PIPE, UNKNOWN, ERROR};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct INP {
    title: String,
    sources: Vec<SOURCE>,
    reservoirs: Vec<RESERVOIR>,
    pipes: Vec<PIPE>,
    unknown_sections: Vec<UNKNOWN>,
    errors: Vec<ERROR>
}

impl INP {
    pub fn read(content: &str) -> Self {
        let mut inp = INP { 
            title: String::new(), 
            sources: Vec::new(), 
            reservoirs: Vec::new(), 
            pipes: Vec::new(),
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
                    section = INP::read_section(line.trim());
                }
                Some(';') => continue,
                _ => match section.as_deref() {
                        Some("TITLE") => inp.set_title_line(INP::read_title_line(line).as_str()),
                        Some("SOURCES") => inp.add_source(line, line_number),
                        Some("RESERVOIRS") => inp.add_reservoir(line, line_number),
                        Some("PIPES") => inp.add_pipe(line, line_number),
                        _ => inp.unknown_sections.push(UNKNOWN { text: line.to_string() })
                    }
            }
            line_number += 1;
        }
        inp
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
        let (properties, comment) = INP::get_properties_and_comment(line);

        T::from_section(properties, comment)
    }

    fn get_properties_and_comment<'a>(line: &'a str) -> (Vec<&str>, Option<String>) {
        let mut parts = line.split(";");
        let properties = parts.next().unwrap_or("").split_whitespace().collect::<Vec<&'a str>>();
        let comment = parts.next().map(|s| s.to_string());

        (properties, comment)
    }

    fn set_title_line(&mut self, s: &str) {
        if !self.title.is_empty() {
            self.title.push(' ');
        }
        self.title.push_str(s);
    }

    fn add_source(&mut self, line: &str, line_number: u32) {
        match INP::build_section::<SOURCE>(line) {
            Ok(source) => self.sources.push(source),
            Err(e) => self.errors.push(ERROR { message: e.to_string(), line: line.to_string(), line_number })
        }
    }

    fn add_reservoir(&mut self, line: &str, line_number: u32) {
        match INP::build_section::<RESERVOIR>(line) {
            Ok(reservoir) => self.reservoirs.push(reservoir),
            Err(e) => self.errors.push(ERROR { message: e.to_string(), line: line.to_string(), line_number })
        }
    }

    fn add_pipe(&mut self, line: &str, line_number: u32) {
        match INP::build_section::<PIPE>(line) {
            Ok(pipe) => self.pipes.push(pipe),
            Err(e) => self.errors.push(ERROR { message: e.to_string(), line: line.to_string(), line_number })
        }
    }
}


#[cfg(test)]
mod test {
    use super::INP;
    use super::SOURCE;
    use super::RESERVOIR;
    use super::PIPE;
    use super::UNKNOWN;
    use crate::Sectionable;
    use crate::sections::ERROR;

    #[test]
    fn read_inp() {
        let input =r#"
[TITLE]
Hello World
Line two
;comment

[RESERVOIRS]
;ID    Head      Pattern
;----------------------- 
R1     512               ;Head stays constant
R2     120       Pat1    ;Head varies with time

[SOURCES] 
;Node  Type    Strength  Pattern 
;-------------------------------- 
N1     CONCEN  1.2       Pat1    ;Concentration varies with time
N44    MASS    12                ;Constant mass injection

[PIPES]
;ID   Node1  Node2   Length   Diam.  Roughness  Mloss   Status
;-------------------------------------------------------------
P1    J1     J2     1200      12      120       0.2     OPEN
P2    J3     J2      600       6      110       0       CV
P3    J1     J10    1000      12      120

[TEST] 
;Node  Type    Strength  Pattern 
;-------------------------------- 
N1     CONCEN  1.2       Pat1    ;Concentration varies with time
N44    MASS    12                
        "#;
        let inp = INP::read(input);
        assert_eq!(inp.title, "Hello World Line two");
        assert_eq!(
            inp.reservoirs, 
            vec![
                RESERVOIR::from_section(
                    vec!["R1", "512"],
                    Some("Head stays constant".to_string())
                ).unwrap(),
                RESERVOIR::from_section(
                    vec!["R2", "120", "Pat1"],
                    Some("Head varies with time".to_string())
                ).unwrap()
            ]
        );
        assert_eq!(
            inp.sources, 
            vec![SOURCE::from_section(
                vec!["N1", "CONCEN", "1.2", "Pat1"],
                Some("Concentration varies with time".to_string())
            ).unwrap(),
            SOURCE::from_section(
                vec!["N44", "MASS", "12"],
                Some("Constant mass injection".to_string())
            ).unwrap()]
        );
        assert_eq!(
            inp.pipes, 
            vec![PIPE::from_section(
                vec!["P1", "J1", "J2", "1200", "12", "120", "0.2", "OPEN"],
                None
            ).unwrap(),
            PIPE::from_section(
                vec!["P2", "J3", "J2", "600", "6", "110", "0", "CV"],
                None
            ).unwrap(),
            PIPE::from_section(
                vec!["P3", "J1", "J10", "1000", "12", "120"],
                None
            ).unwrap()]
        );
        assert_eq!(
            inp.unknown_sections, 
            vec![
                UNKNOWN { 
                    text: "N1     CONCEN  1.2       Pat1    ;Concentration varies with time".to_string(), 
                },
                UNKNOWN { 
                    text: "N44    MASS    12                ".to_string(), 
                },
            ]
        );
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
        let inp = INP::read(input);
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
        let inp = INP::read(input);
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

