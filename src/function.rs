use crate::block::Block;
use crate::ctype::CType;



#[derive(Debug, Clone)]
pub struct Function {
    name: String,
    ret_type: CType,
    args: Option<Vec<CType>>,
    is_variadic: bool,

    block: Option<Block>,
}

impl Function {
    pub fn new(name: &str, ret_type: &CType, args: Option<Vec<CType>>, is_variadic: bool) -> Function {
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
    pub fn args(&self) -> &Option<Vec<CType>> { &self.args }
    pub fn is_variadic(&self) -> bool { self.is_variadic }

    pub fn add_block(&mut self) -> &mut Block {
        let block = Block::new();
        {
            self.block = Some(block);
        }

        self.block.as_mut().unwrap()
    }

    pub fn emit_c(&self, out: &mut impl std::io::Write) -> std::io::Result<()> {
        let decl = if let Some(a) = &self.args {
            let mut args = a.iter()
                .map(|t| t.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            if self.is_variadic {
                args.push_str(", ...");
            }

            format!("{} {}({})", self.ret_type, self.name, args)
        } else {
            format!("{} {}(void)", self.ret_type, self.name)
        };

        write!(out, "{}", decl)?;
        
        if let Some(block) = &self.block {
            write!(out, " ")?;
            block.emit_c(out)?;
        } else {
            write!(out, ";")?;
        }

        Ok(())
    }
}