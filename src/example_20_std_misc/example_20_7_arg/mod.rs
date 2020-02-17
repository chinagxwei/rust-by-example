mod example_20_7_1_matching;

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn example_20_7_arg() {
        let args: Vec<String> = env::args().collect();

        // 第一个参数是调用本程序的路径
        println!("My path is {}.", args[0]);

        // 其余的参数是被传递给程序的命令行参数。
        // 请这样调用程序：
        //   $ ./args arg1 arg2
        println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
    }
}