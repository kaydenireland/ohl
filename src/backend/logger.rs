mod logger;

pub struct Logger{
    indent: usize
}

impl Logger {
    const INDENT: usize = 2;

    pub fn new() -> Log {
        Log { indent: 0 }
    }

    pub fn info(&self, msg: &str) {
        println!("{:<indent$}{:}", "", msg, indent=self.indent);
    }

    pub fn indent_inc(&mut self) {
        self.indent += Self::INDENT;
    }
    pub fn indent_dec(&mut self) {
        self.indent -= Self::INDENT;
    }
}