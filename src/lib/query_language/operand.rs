
pub enum OperandEnum {
    Number(f32),
    String(String),
    // List(Vec<T>),
}

pub struct Operand(pub OperandEnum);

impl From<Operand> for f32 {
    fn from(operand: Operand) -> Self {
        match operand.0 {
            OperandEnum::Number(n) => n,
            OperandEnum::String(s) => s.parse::<f32>().unwrap_or_default(),
        }
    }
}

impl From<&Operand> for f32 {
    fn from(operand: &Operand) -> Self {
        match &operand.0 {
            OperandEnum::Number(n) => *n,
            OperandEnum::String(s) => s.parse::<f32>().unwrap_or_default(),
        }
    }
}

impl From<&Operand> for String {
    fn from(operand: &Operand) -> Self {
        match &operand.0 {
            OperandEnum::Number(n) => n.to_string(),
            OperandEnum::String(s) => s.clone(),
        }
    }
}

impl From<Operand> for String {
    fn from(operand: Operand) -> Self {
        match operand.0 {
            OperandEnum::Number(n) => n.to_string(),
            OperandEnum::String(s) => s,
        }
    }
}
