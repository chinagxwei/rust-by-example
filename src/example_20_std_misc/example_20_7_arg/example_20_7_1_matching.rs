fn increase(number: i32) {
    println!("{}", number + 1);
}

fn decrease(number: i32) {
    println!("{}", number - 1);
}

fn help(){
    println!("usage: match_args<string>\n
    Check whether given string is the answer.\n
    match_args {{increase|decrease}} <integer>\n
    Increase or decrease given integer by one.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn example_20_7_1_matching() {
        let args: Vec<String> = env::args().collect();

        match args.len() {
            // 没有传入参数
            1 => {
                println!("My name is 'match_args'. Try passing some arguments!");
            },
            // 一个传入参数
            2 => {
                match args[1].parse() {
                    Ok(42) => println!("This is the answer!"),
                    _ => println!("This is not the answer."),
                }
            },
            // 传入一条命令和一个参数
            3 => {
                let cmd = &args[1];
                let num = &args[2];
                // 解析数字
                let number: i32 = match num.parse() {
                    Ok(n) => {
                        n
                    },
                    Err(_) => {
                        println!("error: second argument not an integer");
                        help();
                        return;
                    },
                };
                // 解析命令
                match &cmd[..] {
                    "increase" => increase(number),
                    "decrease" => decrease(number),
                    _ => {
                        println!("error: invalid command");
                        help();
                    },
                }
            },
            // 所有其他情况
            _ => {
                // 显示帮助信息
                help();
            }
        }
    }
}