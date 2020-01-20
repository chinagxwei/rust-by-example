
// rustc ./src/example_11_crates/example_11_2_link.rs --extern example_11_1_lib=libexample_11_1_lib.rlib && example_11_2_link
// 执行以上命令可链接一个库

extern crate example_11_1_lib;

fn main() {
    example_11_1_lib::public_function();

    example_11_1_lib::indirect_access();
}