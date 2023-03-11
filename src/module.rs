use crate::ctype::CType;
use crate::function::Function;

#[derive(Debug, Clone)]
pub struct Module {
    name: String,
    functions: Vec<Function>,
    global_includes: Vec<String>,
    local_includes: Vec<String>,
}

impl Module {
    pub fn new(name: &str) -> Module {
        Module {
            name: name.to_string(),
            functions: vec![],
            global_includes: vec![],
            local_includes: vec![],
        }
    }

    pub fn name(&self) -> &str { &self.name }

    pub fn include(&mut self, include: &str, local: bool) {
        if local {
            self.local_includes.push(include.to_string());
            return;
        }
        
        self.global_includes.push(include.to_string());
    }

    pub fn add_function(&mut self, name: &str, ret_type: &CType, args: Option<Vec<CType>>, is_variadic: bool) -> &mut Function {
        let func = Function::new(name, ret_type, args, is_variadic);
        self.functions.push(func);
        self.functions.last_mut().unwrap()
    }

    pub fn emit_c(&self, out: &mut impl std::io::Write) -> std::io::Result<()> {

        for include in &self.global_includes {
            write!(out, "#include <{}>\n", include)?;
        }

        for include in &self.local_includes {
            write!(out, "#include \"{}\"\n", include)?;
        }

        write!(out, "\n")?;

        for func in &self.functions {
            func.emit_c(out)?;
        }

        Ok(())
    }
}