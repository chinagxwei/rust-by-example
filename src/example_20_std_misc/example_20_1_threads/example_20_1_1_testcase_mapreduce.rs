use std::thread;

pub fn testcase_mapreduce() {

    // 这是我们要处理的数据。
    // 我们会通过线程实现 map-reduce 算法，从而计算每一位的和
    // 每个用空白符隔开的块都会分配给单独的线程来处理
    //
    // 试一试：插入空格，看看输出会怎样变化！
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

    // 创建一个向量，用于储存将要创建的子线程
    let mut children = vec![];

    /*************************************************************************
     * "Map" 阶段
     *
     * 把数据分段，并进行初始化处理
     ************************************************************************/

    // 把数据分段，每段将会单独计算
    // 每段都是完整数据的一个引用（&str）
    let chunked_data = data.split_whitespace();

    // 对分段的数据进行迭代。
    // .enumerate() 会把当前的迭代计数与被迭代的元素以元组 (index, element)
    // 的形式返回。接着立即使用 “解构赋值” 将该元组解构成两个变量，
    // `i` 和 `data_segment`。
    for (i, data_segment) in chunked_data.enumerate() {

        // 用单独的线程处理每一段数据
        //
        // spawn() 返回新线程的句柄（handle），我们必须拥有句柄，
        // 才能获取线程的返回值。
        //
        // 'move || -> u32' 语法表示该闭包：
        // * 没有参数（'||'）
        // * 会获取所捕获变量的所有权（'move'）
        // * 返回无符号 32 位整数（'-> u32'）
        //
        children.push(
            thread::spawn(move || -> u32 {
                // 计算该段的每一位的和：
                let result = data_segment.chars()
                    .map(|x| {
                        x.to_digit(10)
                            .expect("should be a digit")
                    }).sum();

                println!("processed segment: {}, result = {}", i, result);

                result
            })
        )
    }

    /*************************************************************************
     * "Reduce" 阶段
     *
     * 收集中间结果，得出最终结果
     ************************************************************************/

    // 把每个线程产生的中间结果收入一个新的向量中
    let mut intermediate_sums = vec![];

    for child in children {
        // 收集每个子线程的返回值
        let intermediate_sum = child.join().unwrap();
        intermediate_sums.push(intermediate_sum);
    }

    // 把所有中间结果加起来，得到最终结果
    //
    // 我们用 “涡轮鱼” 写法 ::<> 来为 sum() 提供类型提示。
    //
    // 试一试：不使用涡轮鱼写法，而是显式地指定 intermediate_sums 的类型
    let final_result = intermediate_sums.iter().sum::<u32>();

    println!("Final sum result: {}", final_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_20_1_1_testcase_mapreduce() {
        testcase_mapreduce();
    }
}