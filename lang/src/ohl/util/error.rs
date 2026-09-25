
use crate::ohl::util::location::Location;

pub struct OhlError {
    location: Location,
    message: String,
    show_location: bool
}

impl OhlError {

    pub fn new(line: usize, col: usize, message: String) -> OhlError {
        OhlError {
            location: Location::new(line, col),
            message,
            show_location: true,
        }
    }

    pub fn from(message: String) -> OhlError {
        OhlError {
            location: Location::empty(),
            message,
            show_location: false,
        }
    }

    pub fn report(&self) {
        eprintln!("{}", self.to_string());()
    }
    
    pub fn disable_location(&mut self) {
        self.show_location = false;
    }
    
    pub fn to_string(&self) -> String {
        if self.show_location {
            format!("{} {}", self.location.to_string(), self.message)
        } else {
            format!("{}", self.message)
        }
    }

}