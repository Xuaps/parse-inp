
#[derive(Debug, PartialEq)]
pub struct INP {
    title: String,
    reservoirs: Vec<RESERVOIR>
}

#[derive(Debug, PartialEq)]
pub struct RESERVOIR {
    id: String,
    head: f64,
    pattern: String
}

impl INP {
    pub fn read(content: &str) -> Self {
        let mut inp = INP { title: String::new(), reservoirs: Vec::new() };
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
        let properties = line.split_whitespace().collect::<Vec<&str>>();

        self.reservoirs.push(RESERVOIR  {
            id: properties[0].to_string(),
            head: properties[1].parse::<f64>().unwrap(),
            pattern: properties[2].to_string()
        });
    }
}


#[cfg(test)]
mod test {
    use super::INP;
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
            [TITLE]
            Hello World
            [RESERVOIRS]
            R2     120       Pat1
        "#;
        let inp = INP::read(input);
        assert_eq!(
            inp.reservoirs, 
            vec![RESERVOIR { 
                id: "R2".to_string(), 
                head: 120.0, 
                pattern: "Pat1".to_string() 
            }]
        );
    }
}

