use std::str::Chars;

pub struct INP {
    title: String
}

impl INP {
    pub fn read(content: &str) -> Self {
        let mut inp = INP { title: String::new() };
        let mut chars = content.chars();
        match chars.next() {
            Some('[') => match inp.read_section(&mut chars).as_deref() {
                Some("TITLE") => {
                    inp.skip_line(&mut chars);
                    inp.read_title(&mut chars)
                },
                _ => panic!("Invalid section")
            },
            _ => panic!("Invalid INP file")
        }
        inp
    }

    pub fn read_section(&mut self, chars: &mut Chars) -> Option<String> {
        let mut section = String::new();
        let mut c = chars.next();
        while c != Some(']') {
            section.push(c.unwrap());
            c = chars.next();
        }
        Some(section)
    }

    pub fn read_title(&mut self, chars: &mut Chars) {
        let mut title = String::new();
        let mut c = chars.next();
        while c != Some('\n') {
            title.push(c.unwrap());
            c = chars.next();
        }
        self.title = title;
    }

    pub fn skip_line(&mut self, chars: &mut Chars) {
        let mut c = chars.next();
        while c != Some('\n') {
            c = chars.next();
        }
    }
}


#[cfg(test)]
mod test {
    use super::INP;

    #[test]
    fn read_inp_title() {
        let input = "[TITLE]\nHello World\n";
        let inp = INP::read(input);
        assert_eq!(inp.title, "Hello World");
    }
}

