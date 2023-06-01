use std::str::Chars;

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
        let mut chars = content.chars();
        while let Some(c) = chars.next() {
            match c {
                '[' => match inp.read_section(&mut chars).as_deref() {
                    Some("TITLE") => inp.read_title(&mut chars),
                    Some("RESERVOIRS") => inp.read_reservoirs(&mut chars),
                    _ => panic!("Invalid section")
                },
                _ => panic!("Invalid INP file")
            }
        }
        inp
    }

    fn read_section(&mut self, chars: &mut Chars) -> Option<String> {
        let mut section = String::new();
        let mut c = chars.next();
        while c != Some(']') {
            section.push(c.unwrap());
            c = chars.next();
        }
        self.skip_line(chars);
        Some(section)
    }

    fn read_title(&mut self, chars: &mut Chars) {
        let mut title = String::new();
        let mut c = chars.next();
        while c != Some('\n') {
            title.push(c.unwrap());
            c = chars.next();
        }
        self.title = title;
    }

    fn read_reservoirs(&mut self, chars: &mut Chars) {
        let mut reservoirs = Vec::new();
        let mut c = chars.next();
        while c != Some('[') {
            let mut name = String::new();
            while c != Some(' ') {
                name.push(c.unwrap());
                c = chars.next();
            }
            c = self.skip_spaces(chars);
            let mut head = String::new();
            while c != Some(' ') {
                head.push(c.unwrap());
                c = chars.next();
            }
            c = self.skip_spaces(chars);
            let mut pattern = String::new();
            while c != Some('\n') {
                pattern.push(c.unwrap());
                c = chars.next();
            }
            c = self.skip_spaces(chars);
            reservoirs.push(RESERVOIR { 
                id: name, 
                head: head.parse::<f64>().unwrap(), 
                pattern
            });
        }
        self.reservoirs = reservoirs;
    }


    fn skip_line(&mut self, chars: &mut Chars) {
        let mut c = chars.next();
        while c != Some('\n') {
            c = chars.next();
        }
    }

    pub fn skip_spaces(&mut self, chars: &mut Chars) -> Option<char> {
        let mut c = chars.next();
        while c == Some(' ') {
            c = chars.next();
        }
        c
    }
}


#[cfg(test)]
mod test {
    use super::INP;
    use super::RESERVOIR;

    #[test]
    fn read_inp_title() {
        let input = "[TITLE]\nHello World\n";
        let inp = INP::read(input);
        assert_eq!(inp.title, "Hello World");
    }

    #[test]
    fn read_inp_reservoirs() {
        let input = "[TITLE]\nHello World\n[RESERVOIRS]\nR2     120       Pat1\n[";
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

