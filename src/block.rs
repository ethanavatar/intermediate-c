#[derive(Debug, Clone)]
pub struct Block {
    code: Vec<String>,
}

impl Block {
    pub fn new() -> Block {
        Block {
            code: vec![],
        }
    }

    pub fn code(&self) -> &Vec<String> { &self.code }

    pub fn add_line(&mut self, line: &str) -> () {
        self.code.push(line.to_string());
    }

    pub fn emit_c(&self, out: &mut impl std::io::Write) -> std::io::Result<()> {
        write!(out, "{{\n")?;

        for line in &self.code {
            write!(out, "\t{}\n", line)?;
        }

        write!(out, "}}\n")?;

        Ok(())
    }
}