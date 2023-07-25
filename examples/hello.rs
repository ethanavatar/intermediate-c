use intermediate_c::{
    builder::Builder,
    function::Function,
    module::Module,
    ctype::CType,
    cvalue::CValue
};
use std::fs::File;
use std::io::BufWriter;

/*
fn build_add_func(module: Module) -> () {
    let add_func = module.add_function(
        "add", // name
        &i32_t, // return type
        Some(vec![i32_t, i32_t]), // arguments
        false, // is_variadic
        false // is_static
    );

    let add_block = add_func.add_block();
    let mut builder = Builder::new();
    builder.position_at_end(add_block);

    
    let x = add_func.get_param(0);
    let y = add_func.get_param(1);

    let sum: RuntimeValue = builder.build_add(x, y);

    builder.build_return(sum);
}
*/

fn main() {
    let i32_t = CType::Int(32);
    let charptr_t = CType::Ptr(Box::new(CType::Char));
    let mut module = Module::new("hello");

    // included for `printf`.
    // `local = false` means it's an include from the standard library
    module.include("stdio.h", false);

    // declare the `main` function
    let main_func = module.add_function(
        "main", // name
        &i32_t, // return type
        None, // arguments
        false, // is_variadic
        false // is_static
    );

    // create a builder for the `main` function
    let mut builder = Builder::new();

    // define a block for the `main` function
    let main_block = main_func.add_block();
    builder.position_at_end(main_block);

    // the type signature of the `printf` function
    let printf_func = Function::new(
        "printf", // name
        &i32_t, // return type
        Some(vec![charptr_t]), // arguments
        true, // is_variadic
        false // is_static
    );

    let str = CValue::StringLiteral(r"Hello, Sailor!\n");
    let _print_call = builder.build_call(
        &printf_func, // function
        vec![str], // arguments
        None // name of the return value variable
    );

    // return from `main`
    builder.build_return(&i32_t.into_value(0));

    // write the module's header to a file
    let file = File::create("./examples/hello.h").unwrap();
    let mut writer = BufWriter::new(file);

    module.emit_h(&mut writer).unwrap();

    // write the module's source to a file
    let file = File::create("./examples/hello.c").unwrap();
    let mut writer = BufWriter::new(file);

    module.emit_c(&mut writer).unwrap();
}

