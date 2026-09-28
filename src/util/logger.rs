use std::fmt;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::new(false));
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

    pub fn info(&self, msg: fmt::Arguments) {
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
        self.indent -= self.indent.saturating_sub(Self::INDENT);
    }

    pub fn reset_indent(&mut self) {
        self.indent = 0;
    }
}


#[macro_export]
macro_rules! info {
    () => ($crate::LOGGER.lock().info(format_args!("")));
    ($($arg:tt)*) => ($crate::LOGGER.lock().info(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! indent_inc {
    () => ($crate::LOGGER.lock().indent_inc());
}

#[macro_export]
macro_rules! indent_dec {
    () => ($crate::LOGGER.lock().indent_dec());
}

#[macro_export]
macro_rules! indent_reset {
    () => ($crate::LOGGER.lock().reset_indent());
}

#[macro_export]
macro_rules! log_debug {
    ($debug:expr) => ($crate::LOGGER.lock().set_debug($debug));
}