#[derive(Clone)]
pub struct Location {
    line: usize,
    col: usize
}

impl Location {
    pub fn new(line: usize, col: usize) -> Location {
        Location { line, col }
    }
    
    pub fn empty() -> Location {
        Location { line: 0, col: 0 }
    }

    pub fn get_line(&self) -> usize {
        self.line
    }

    pub fn get_col(&self) -> usize {
        self.col
    }
    
    pub fn to_string(&self) -> String {
        format!("[{}|{}]", self.line, self.col)
    }
}