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

    pub fn add_function(&mut self, name: &str, ret_type: &CType, args: Option<Vec<CType>>, is_variadic: bool, is_static: bool) -> &mut Function {
        let func = Function::new(name, ret_type, args, is_variadic, is_static);
        self.functions.push(func);
        self.functions.last_mut().unwrap()
    }

    fn emit_includes(&self, out: &mut impl std::io::Write) -> std::io::Result<()> {
        for include in &self.global_includes {
            write!(out, "#include <{}>\n", include)?;
        }

        for include in &self.local_includes {
            write!(out, "#include \"{}\"\n", include)?;
        }

        Ok(())
    }

    pub fn emit_h(&self, out: &mut impl std::io::Write) -> std::io::Result<()> {
        let name = format!("{}_H", self.name.to_uppercase());
        write!(out, "#ifndef {}\n", name)?;
        write!(out, "#define {}\n", name)?;
        write!(out, "\n")?;

        self.emit_includes(out)?;
        write!(out, "\n")?;
        for func in &self.functions {
            if func.is_static() || func.name() == "main" {
                continue;
            }
            func.emit_h(out)?;
        }
        write!(out, "\n")?;
        write!(out, "#endif // {}\n", name)?;
        Ok(())
    }

    pub fn emit_c(&self, out: &mut impl std::io::Write) -> std::io::Result<()> {
        self.emit_includes(out)?;
        write!(out, "\n")?;
        for func in &self.functions {
            func.emit_c(out)?;
        }
        Ok(())
    }
}
