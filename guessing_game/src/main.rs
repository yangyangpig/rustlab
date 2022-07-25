use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    println!("猜一个数据");

    // 定义一个不可变的变量
    let secret_number = rand::thread_rng().gen_range(1..101);
    // 这是一个注释
    //println!("神秘数字是: {}", secret_number);
    loop {
        println!("猜测一个数字");
        // 声明一个可变变量
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");
        
        // shadow
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜的数字是:{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

//