pub struct Logger{
    indent: usize,
    debug: bool
}

impl Logger {
    const INDENT: usize = 2;

    pub fn new(debug: bool) -> Logger {
        Logger { indent: 0, debug }
    }

    pub fn info(&self, msg: &str) {
        if self.debug {
            println!("{:<indent$}{:}", "", msg, indent=self.indent);
        }
    }

    pub fn indent_inc(&mut self) {
        self.indent += Self::INDENT;
    }
    pub fn indent_dec(&mut self) {
        self.indent -= Self::INDENT;
    }
}