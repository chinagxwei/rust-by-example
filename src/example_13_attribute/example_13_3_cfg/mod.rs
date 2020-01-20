mod example_13_3_1_custom;

#[cfg(target_os = "linux")]
fn are_you_on_linux(){
    println!("You are running linux!");
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux(){
    println!("You are *not* running linux!");
}

fn example_13_3_cfg(){
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os="linux") {
        println!("Yes. It's definitely linux!");
    }else {
        println!("Yes. It's definitely *not* linux!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_13_3_cfg() {
        example_13_3_cfg();
    }
}