use crate::block::Block;
use crate::callvalue::CallValue;
use crate::cvalue::CValue;
use crate::function::Function;

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
            block.add_line(line);
        }
    }

    pub fn build_call(&mut self, function: &Function, args: Vec<CValue>, name: Option<&str>) -> CallValue {
        let line = if let Some(name_str) = name {
            format!("{} {} = {}({});", function.ret_type(), name_str, function.name(), args.iter().map(|a| a.to_string()).collect::<Vec<String>>().join(", "))
        } else {
            format!("{}({});", function.name(), args.iter().map(|a| a.to_string()).collect::<Vec<String>>().join(", "))
        };
        self.add_line(&line);

        let value = function.ret_type();
        CallValue::new(value)
    }

    pub fn build_return(&mut self, value: &CValue) -> () {
        let line = format!("return {};", value);
        self.add_line(&line);
    }
}