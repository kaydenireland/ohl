#[derive(Debug, Clone, PartialEq)]
pub enum FunctionType {
    PUBLIC,
    PROTECTED,
    PRIVATE
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableType {
    INT,
    FLOAT,
    BOOLEAN,
    CHAR,
    STRING,
    NULL,
}

impl VariableType {
    pub fn is_numeric(&self) -> bool {
        matches!(self, VariableType::INT | VariableType::FLOAT)
    }

    pub fn is_comparable(&self) -> bool {
        matches!(
            self,
            VariableType::INT
                | VariableType::FLOAT
                | VariableType::CHAR
                | VariableType::STRING
                | VariableType::BOOLEAN
        )
    }
}
