use std::fmt::Display;
use crate::core::converter::operator::Operator;

#[derive(Debug, Clone)]
pub enum Value {
    INT(i32),
    VAR(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::INT(x) => write!(f, "{}", x),
            Value::VAR(x) => write!(f, "{}", x),
        }
    }
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    NEGATE,
    NOT
}

impl UnaryOperator {
    pub fn from(typ: Operator) -> UnaryOperator {
        match typ {
            Operator::NEGATE => UnaryOperator::NEGATE,
            Operator::COMPLEMENT => UnaryOperator::NOT,

            _ => panic!("Unknown unary operator {:?}", typ)
        }
    }
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperator::NOT => write!(f, "not"),
            UnaryOperator::NEGATE => write!(f, "neg"),
        }
    }
}

