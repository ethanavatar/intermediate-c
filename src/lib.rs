use std::fmt::Display;


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

#[derive(Debug, Clone)]
pub struct Module {
    name: String,
    functions: Vec<Function>,
}

impl Module {
    pub fn new(name: &str) -> Module {
        Module {
            name: name.to_string(),
            functions: vec![],
        }
    }

    pub fn name(&self) -> &str { &self.name }

    pub fn add_function(&mut self, name: &str, ret_type: &CType, args: Vec<CType>, is_variadic: bool) -> &mut Function {
        let mut func = Function::new(name, ret_type, args, is_variadic);
        self.functions.push(func);
        self.functions.last_mut().unwrap()
    }

    pub fn emit_c(&self) -> String {
        
        let mut output = String::new();
        for func in &self.functions {
            output.push_str(&func.emit_c());
        }

        output
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    name: String,
    ret_type: CType,
    args: Vec<CType>,
    is_variadic: bool,

    block: Option<Block>,
}

impl Function {
    pub fn new(name: &str, ret_type: &CType, args: Vec<CType>, is_variadic: bool) -> Function {
        Function {
            name: name.to_string(),
            ret_type: ret_type.clone(),
            args: args,
            is_variadic: is_variadic,

            block: None,
        }
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn ret_type(&self) -> &CType { &self.ret_type }
    pub fn args(&self) -> &Vec<CType> { &self.args }
    pub fn is_variadic(&self) -> bool { self.is_variadic }

    pub fn add_block(&mut self) -> &mut Block {
        let block = Block::new(self);
        {
            self.block = Some(block);
        }
        self.block.as_mut().unwrap()
    }

    fn emit_c(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("{} {}({})", self.ret_type, self.name, self.args.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(", ")));
        
        if let Some(block) = &self.block {
            output.push_str(" ");
            output.push_str(&block.emit_c());
        } else {
            output.push_str(";");
        }

        output
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    code: Vec<String>,
}

impl Block {
    pub fn new(function: &Function) -> Block {
        Block {
            code: vec![],
        }
    }
    fn emit_c(&self) -> String {

        let mut output = String::new();
        output.push_str("{\n");

        for line in &self.code {
            output.push_str(&format!("    {}\n", line));
        }

        output.push_str("}\n");
        output
    }
}

#[derive(Debug, Clone)]
pub struct CallValue {
    value: CType,
    name: String,
}

#[derive(Debug)]
pub struct Builder<'a> {
    block: Option<&'a mut Block>,
}

impl<'a> Builder<'a> {
    pub fn new() -> Builder<'a> {
        Builder {
            block: None,
        }
    }

    pub fn position_at_end(&mut self, block: &'a mut Block) {
        self.block = Some(block);
    }

    fn add_line(&mut self, line: &str) -> () {
        if let Some(block) = &mut self.block {
            block.code.push(line.to_string());
        }
    }

    pub fn build_call(&mut self, function: &Function, args: Vec<CValue>, name: &str) -> CallValue {
        let line = format!("{} {} = {}({});", function.ret_type(), name, function.name(), args.iter().map(|a| a.to_string()).collect::<Vec<String>>().join(", "));
        self.add_line(&line);

        CallValue {
            value: function.ret_type().clone(),
            name: name.to_string(),
        }
    }

    pub fn build_return(&mut self, value: &CValue) -> () {
        let line = format!("return {};", value);
        self.add_line(&line);
    }
}

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


#[cfg(test)]
mod tests {
    use super::*;
}
