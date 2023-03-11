use std::fmt::Display;
use crate::ctype::CType;

#[derive(Debug, Clone)]
pub enum CValue {
    Int(u8, i64),
    Float(u8, f64),
    Double(f64),
    Void,
    Char(char),
    Bool(bool),
    Array(Vec<CValue>),
    Struct(String, Vec<CValue>),
    Union(String, Vec<CValue>),
    Enum(String, i64),
    Func(Vec<CValue>, Box<CValue>),
    Ptr(Box<CValue>),

    StringLiteral(&'static str),
}

impl CValue {
    pub fn from_type(ty: CType, value: i64) -> CValue {
        match ty {
            CType::Int(size) => CValue::Int(size, value),
            CType::Float(size) => CValue::Float(size, value as f64),
            CType::Double => CValue::Double(value as f64),
            CType::Void => CValue::Void,
            CType::Char => CValue::Char(value as u8 as char),
            CType::Bool => CValue::Bool(value != 0),
            CType::Array(ty, size) => CValue::Array(vec![CValue::from_type(*ty, value); size as usize]),
            CType::Struct(name) => CValue::Struct(name, vec![]),
            CType::Union(name) => CValue::Union(name, vec![]),
            CType::Enum(name) => CValue::Enum(name, value),
            CType::Func(args, ret) => todo!("CValue::from_type: CType::Func"),
            CType::Ptr(ty) => CValue::Ptr(Box::new(CValue::from_type(*ty, value))),

            _ => todo!("CValue::from_type: {:?}", ty),
        }
    }

    pub fn as_ptr(&self) -> CValue {
        CValue::Ptr(Box::new(self.clone()))
    }
}

impl Display for CValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CValue::Int(size, value) => write!(f, "{}", value),
            CValue::Float(size, value) => write!(f, "{}", value),
            CValue::Double(value) => write!(f, "{}", value),
            CValue::Void => write!(f, "void"),
            CValue::Char(value) => write!(f, "{}", value),
            CValue::Bool(value) => write!(f, "{}", value),
            CValue::Array(values) => write!(f, "[{}]", values.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(", ")),
            CValue::Struct(name, values) => write!(f, "{} {{ {} }}", name, values.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(", ")),
            CValue::Union(name, values) => write!(f, "{} {{ {} }}", name, values.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(", ")),
            CValue::Enum(name, value) => write!(f, "{}", value),
            CValue::Func(args, ret) => write!(f, "({}) -> {}", args.iter().map(|a| a.to_string()).collect::<Vec<String>>().join(", "), ret),
            CValue::Ptr(value) => write!(f, "*{}", value),
            CValue::StringLiteral(value) => write!(f, "\"{}\"", value),
        }
    }
}