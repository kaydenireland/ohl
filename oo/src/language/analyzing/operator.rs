#![allow(warnings)]

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    REMAINDER,
    POWER,
    ROOT,

    NEGATIVE,
    RECIPROCAL,

    INCREMENT,
    DECREMENT,
    SQUARE,

    NOT,
    AND,
    OR,
    XOR,

    NULL_COAL,

    EQUAL,
    LESS_THAN,
    NOT_GREATER_THAN,
    GREATER_THAN,
    NOT_LESS_THAN,
    NOT_EQUAL,
}