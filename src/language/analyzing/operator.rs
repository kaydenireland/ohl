#![allow(warnings)]

#[derive(Debug, Clone)]
pub enum Operator {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDER,
    REMAINDER,
    POWER,
    ROOT,

    NEGATIVE,
    RECIPRICOL,

    INCREMENT,
    DECREMENT,
    SQUARE,

    NOT,
    AND,
    OR,
    XOR,

    EQUAL,
    LESS_THAN,
    NOT_GREATER_THAN,
    GREATER_THAN,
    NOT_LESS_THAN,
    NOT_EQUAL,
}