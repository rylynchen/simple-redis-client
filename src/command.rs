pub struct CommandWriter {
    pub buf: String,
}

impl CommandWriter {
    pub fn new() -> CommandWriter {
        CommandWriter {
            buf: "".to_string(),
        }
    }

    pub fn write_arrs(&mut self, n: usize) -> &mut Self {
        self.add_char('*');
        self.add_unit(n);
        self.add_crnl();
        self
    }

    pub fn write_bulk_string(&mut self, s: &str) -> &mut Self {
        if s.is_empty() {
            self.add_str("$-1");
        } else {
            self.add_char('$');
            self.add_unit(s.len());
            self.add_crnl();
            self.add_str(s);
        }
        self.add_crnl();
        self
    }

    pub fn add_char(&mut self, s: char) {
        self.buf.push(s);
    }
    pub fn add_str(&mut self, s: &str) {
        self.buf.push_str(s);
    }

    pub fn add_unit(&mut self, n: usize) {
        self.add_str(n.to_string().as_str());
    }

    pub fn add_crnl(&mut self) {
        self.add_char('\r');
        self.add_char('\n');
    }
}