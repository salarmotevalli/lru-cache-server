// use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),

    Int(isize),

    Str(String),
    // Arr(Vec<Value>),

    // TODO: add as a feature
    // Map(HashMap<String, Value>),
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Self::Bool(v) => format!("{}", v),
            Self::Int(v) => format!("{}", v),
            Self::Str(v) => v.clone(),
        }
    }
}
