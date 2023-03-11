use crate::ctype::CType;

#[derive(Debug, Clone)]
pub struct CallValue {
    value: CType,
}

impl CallValue {
    pub fn new(value: &CType) -> CallValue {
        CallValue {
            value: value.clone(),
        }
    }

    pub fn value(&self) -> &CType { &self.value }
}