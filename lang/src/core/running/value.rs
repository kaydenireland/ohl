#[derive(Debug, Clone)]
pub enum Value {
    INT(i32),
    FLOAT(f32),
    CHAR(char),
    STRING(String),
    BOOLEAN(bool),
    NULL
}

impl Value {
    pub fn as_int(&self) -> Result<i32, String> {
        match self {
            Value::INT(i) => Ok(*i),
            _ => Err(format!("Expected Int, found {:?}", self)),
        }
    }

    pub fn as_float(&self) -> Result<f32, String> {
        match self {
            Value::FLOAT(f) => Ok(*f),
            _ => Err(format!("Expected Int, found {:?}", self)),
        }
    }

    pub fn as_char(&self) -> Result<char, String> {
        match self {
            Value::CHAR(c) => Ok(*c),
            _ => Err(format!("Expected Int, found {:?}", self)),
        }
    }

    pub fn as_string(&self) -> Result<String, String> {
        match self {
            Value::STRING(s) => Ok(s.clone()),
            _ => Err(format!("Expected Int, found {:?}", self)),
        }
    }

    pub fn as_boolean(&self) -> Result<bool, String> {
        match self {
            Value::BOOLEAN(b) => Ok(*b),
            _ => Err(format!("Expected Boolean, found {:?}", self)),
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Value::NULL)
    }

    pub fn is_numeric(&self) -> bool {
        match self {
            Value::INT(..) | Value::FLOAT(..) => true,
            _ => false
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::NULL => false,
            Value::BOOLEAN(b) => *b,
            _ => true,
        }
    }
}

#[derive(Clone)]
pub struct Binding {
    pub value: Value,
    pub mutable: bool
}

impl Binding {
    pub fn new(value: Value, mutable: bool) -> Self {
        Self { value, mutable }
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn set(&mut self, value: Value) -> Result<(), String> {
        if !self.mutable {
            return Err("Cannot assign to immutable binding".to_string());
        }
        self.value = value;
        Ok(())
    }
}
