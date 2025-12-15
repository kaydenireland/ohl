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