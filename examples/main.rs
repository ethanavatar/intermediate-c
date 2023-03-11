use intermediate_c::{
    builder::Builder,
    function::Function,
    module::Module,
    ctype::CType,
    cvalue::CValue
};
use std::fs::File;
use std::io::BufWriter;

fn main() {
    let i32_t = CType::Int(32);
    let charptr_t = CType::Ptr(Box::new(CType::Char));
    let mut module = Module::new("main");

    // included for `printf`.
    // `local = false` means it's an include from the standard library
    module.include("stdio.h", false);

    // declare the `main` function
    let main_func = module.add_function(
        "main", // name
        &i32_t, // return type
        None, // arguments
        false // is_variadic
    );

    // define a block for the `main` function
    let main_block = main_func.add_block();

    let mut builder = Builder::new();
    builder.position_at_end(main_block);

    // the type signature of the `printf` function
    let printf_func = Function::new(
        "printf", // name
        &i32_t, // return type
        Some(vec![charptr_t]), // arguments
        true // is_variadic
    );

    let str = CValue::StringLiteral(r"Hello, Sailor!\n");
    let _print_call = builder.build_call(
        &printf_func, // function
        vec![str], // arguments
        None // name of the return value variable
    );

    // return from `main`
    builder.build_return(&i32_t.into_value(0));

    // write the module to a file
    let file = File::create("./examples/main.c").unwrap();
    let mut writer = BufWriter::new(file);

    module.emit_c(&mut writer).unwrap();

}
