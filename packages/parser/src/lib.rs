
#[derive(Debug, PartialEq)]
pub struct INP {
    title: String,
    sources: Vec<SOURCE>,
    reservoirs: Vec<RESERVOIR>
}

#[derive(Debug, PartialEq)]
pub struct RESERVOIR {
    id: String,
    head: f64,
    pattern: Option<String>,
    comment: Option<String>
}

impl RESERVOIR {
    fn from(properties: Vec<&str>, comment: Option<String>) -> Self {
        let id = properties[0].to_string();
        let head = properties[1].parse::<f64>().unwrap();
        let pattern = if properties.len() > 2 { Some(properties[2].to_string()) } else { None };
        let comment = comment.map(|s| s.to_string());

        RESERVOIR { id, head, pattern, comment }
    }
}

#[derive(Debug, PartialEq)]
pub struct SOURCE {
    node: String,
    source_type: String,
    strength: f64,
    pattern: Option<String>,
    comment: Option<String>
}

impl SOURCE {
    fn from(properties: Vec<&str>, comment: Option<String>) -> Self {
        let node = properties[0].to_string();
        let source_type= properties[1].to_string();
        let strength = properties[2].parse::<f64>().unwrap();
        let pattern = if properties.len() > 3 { Some(properties[3].to_string()) } else { None };
        let comment = comment.map(|s| s.to_string());

        SOURCE { node, source_type, strength, pattern, comment }
    }
}

impl INP {
    pub fn read(content: &str) -> Self {
        let mut inp = INP { title: String::new(), sources: Vec::new(), reservoirs: Vec::new() };
        let mut lines = content.lines();
        let mut section = None;
        while let Some(line) = lines.next() {
            match line.trim().chars().next() {
                None => continue,
                Some('[') => {
                    section = inp.read_section(line.trim());
                }
                Some(';') => continue,
                _ => match section.as_deref() {
                        Some("TITLE") => inp.read_title_line(line),
                        Some("SOURCES") => inp.read_sources(line),
                        Some("RESERVOIRS") => inp.read_reservoir(line),
                        other => panic!("Invalid section {}", other.unwrap_or(""))
                    }
            }
        }
        inp
    }

    fn read_section(&mut self, line: &str) -> Option<String> {
        let mut section = String::new();
        let mut chars = line.chars().skip(1);
        let mut c = chars.next();
        while c != Some(']') {
            section.push(c.unwrap());
            c = chars.next();
        }

        Some(section)
    }

    fn read_title_line(&mut self, line: &str) {
        let mut title = String::new();
        let mut chars = line.chars();
        let mut c = chars.next();
        while c != None {
            title.push(c.unwrap());
            c = chars.next();
        }
        if self.title.len() > 0 {
            self.title.push(' ');
        }
        self.title.push_str(title.as_str());
    }

    fn read_reservoir(&mut self, line: &str) {
        let (properties, comment) = self.get_properties_and_comment(line);

        self.reservoirs.push(RESERVOIR::from(properties, comment));
    }

    fn read_sources(&mut self, line: &str) {
        let (properties, comment) = self.get_properties_and_comment(line);

        self.sources.push(SOURCE::from(properties, comment));
    }

    fn get_properties_and_comment<'a>(&'a self, line: &'a str) -> (Vec<&str>, Option<String>) {
        let mut parts = line.split(";");
        let properties = parts.next().unwrap_or("").split_whitespace().collect::<Vec<&'a str>>();
        let comment = parts.next().map(|s| s.to_string());

        (properties, comment)
    }
}


#[cfg(test)]
mod test {
    use super::INP;
    use super::SOURCE;
    use super::RESERVOIR;

    #[test]
    fn read_inp_title() {
        let input = "[TITLE]\nHello World\nLine two\n;comment";
        let inp = INP::read(input);
        assert_eq!(inp.title, "Hello World Line two");
    }

    #[test]
    fn read_inp_reservoirs() {
        let input = r#"
            [RESERVOIRS]
            ;ID    Head      Pattern
            ;----------------------- 
            R1     512               ;Head stays constant
            R2     120       Pat1    ;Head varies with time
        "#;
        let inp = INP::read(input);
        assert_eq!(
            inp.reservoirs, 
            vec![
                RESERVOIR { 
                    id: "R1".to_string(), 
                    head: 512.0, 
                    pattern: None,
                    comment: Some("Head stays constant".to_string()) },
                RESERVOIR { 
                    id: "R2".to_string(), 
                    head: 120.0, 
                    pattern: Some("Pat1".to_string()),
                    comment: Some("Head varies with time".to_string())},
            ]
        );
    }

    #[test]
    fn read_sources() {
        let input = r#"
            [SOURCES] 
            ;Node  Type    Strength  Pattern 
            ;-------------------------------- 
            N1     CONCEN  1.2       Pat1    ;Concentration varies with time
            N44    MASS    12                ;Constant mass injection
        "#;
        let inp = INP::read(input);
        assert_eq!(
            inp.sources, 
            vec![
                SOURCE { 
                    node: "N1".to_string(), 
                    source_type: "CONCEN".to_string(), 
                    strength: 1.2,
                    pattern: Some("Pat1".to_string()),
                    comment: Some("Concentration varies with time".to_string()) },
                SOURCE { 
                    node: "N44".to_string(), 
                    source_type: "MASS".to_string(), 
                    strength: 12.0,
                    pattern: None,
                    comment: Some("Constant mass injection".to_string()) },
            ]
        );
    }
}

