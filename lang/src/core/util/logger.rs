use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref LOGGER: Mutex<Logger> = Mutex::new(Logger { 
        indent: 0,
        _debug: false
    });
}

#[derive(Debug, Clone)]
pub struct Logger{
    indent: usize,
    _debug: bool
}

impl Logger {
    const INDENT: usize = 2;

    pub fn new(_debug: bool) -> Logger {
        Logger { indent: 0, _debug }
    }

    pub fn info(&self, msg: &str) {
        if self._debug {
            println!("{:<indent$}{:}", "", msg, indent=self.indent);
        }
    }
    
    pub fn set_debug(&mut self, _debug: bool) {
        self._debug = _debug;
    }

    pub fn indent_inc(&mut self) {
        self.indent += Self::INDENT;
    }
    pub fn indent_dec(&mut self) {
        self.indent -= Self::INDENT;
    }

    pub fn reset_indent(&mut self) {
        self.indent = 0;
    }
}