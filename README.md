# intermediate-c

A C code generator using an LLVM-like API

**NOTICE: At this point, this is a proof of concept. Please don't treat it like viable software.**

## Example

- - -

### examples/main.rs

```rust
use intermediate_c::*;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

fn main() {
    let i32_t = CType::Int(32);
    let mut module = Module::new("main");

    // Included for `printf`
    module.include("stdio.h");

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

    // The type signature of the `printf` function
    let printf_func = Function::new(
        "printf", // name
        &CType::Int(32), // return type
        Some(vec![CType::Ptr(Box::new(CType::Int(8)))]), // arguments
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

    // Write the module to a file
    let file = File::create("./examples/main.c").unwrap();
    let mut writer = BufWriter::new(file);

    let src = module.emit_c();
    writer.write_all(src.as_bytes()).unwrap();
}
```

The code above can be run using:

```bash
cargo run --release --example main
```

and will result in the following C code:


- - -

### examples/main.c

```c
#include <stdio.h>

int main(void) {
    printf("Hello, Sailor!\n");
    return 0;
}
```
