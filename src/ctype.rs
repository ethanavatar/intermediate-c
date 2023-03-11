use std::fmt::Display;
use crate::cvalue::CValue;

#[derive(Debug, Clone)]
pub enum CType {
    Int(u8),
    Float(u8),
    Double,
    Void,
    Char,
    Bool,
    Array(Box<CType>, u64),
    Struct(String),
    Union(String),
    Enum(String),
    Func(Vec<CType>, Box<CType>),
    Ptr(Box<CType>),
}

impl CType {
    pub fn as_ptr(&self) -> CType {
        CType::Ptr(Box::new(self.clone()))
    }

    pub fn into_value(&self, value: i64) -> CValue {
        CValue::from_type(self.clone(), value)
    }
}

impl Display for CType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CType::Int(8) => write!(f, "i8"),
            CType::Int(16) => write!(f, "i16"),
            CType::Int(32) => write!(f, "int"),
            CType::Int(64) => write!(f, "i64"),
            CType::Float(32) => write!(f, "float"),
            CType::Double => write!(f, "double"),
            CType::Void => write!(f, "void"),
            CType::Char => write!(f, "char"),
            CType::Bool => write!(f, "bool"),
            CType::Array(t, size) => write!(f, "[{} x {}]", size, t),
            CType::Struct(name) => write!(f, "%{}", name),
            CType::Union(name) => write!(f, "%{}", name),
            CType::Enum(name) => write!(f, "%{}", name),
            CType::Func(args, ret) => {
                write!(f, "({}) -> {}", args.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(", "), ret)
            }
            CType::Ptr(t) => write!(f, "*{}", t),

            _ => write!(f, "unimplemented"),
        }
    }
}