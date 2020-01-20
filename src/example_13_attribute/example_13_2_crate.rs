// 这个 crate 是一个库文件
#![crate_type = "lib"]
// 库的名称为 “rary”
#![crate_name = "example_13_2_crate"]

// rustc ./src/example_13_attribute/example_13_2_crate.rs
// 直接执行就可以生成库了

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}