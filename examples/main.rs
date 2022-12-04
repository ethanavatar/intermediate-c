use immediacy::*;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

fn main() {
    let i32_t = CType::Int(32);
    let mut module = Module::new("main");
    let main_func = module.add_function("main", &i32_t, vec![], false);

    let main_block = main_func.add_block();
    let printf_func = Function::new("printf", &CType::Int(32), vec![CType::Ptr(Box::new(CType::Int(8)))], true);

    let mut builder = Builder::new();
    builder.position_at_end(main_block);

    let str = CValue::StringLiteral(r"Hello, Sailor!\n");
    let print_call = builder.build_call(&printf_func, vec![str], "print_call");

    builder.build_return(&i32_t.into_value(0));

    let out = module.emit_c();
    
    let file = File::create("./examples/main.c").unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(out.as_bytes()).unwrap();
}