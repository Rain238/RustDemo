use std::cmp::Ordering;

use rand::Rng;

/***
 * Author LightRain
 * day01 demo
 * 猜数字游戏
 * 随机生成一个数字
 * 让用户输入数字
 * 比较用户输入的数字和随机生成的数字的大小
 * 直到用户猜对为止
 * 猜对后提示用户猜对了
 */
fn main() {
    println!("----------------猜字游戏----------------");
    //生成随机数
    let secret_number = rand::thread_rng().gen_range(1, 101);println!("神秘数字：{}", secret_number);
    //无限循环块
    loop {
        //初始化一个接收用户输入的变量
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Error reading");
        //将用户输入的解析为数字
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入一个数字");
                continue;
            }
        };
        //将用户输入的数字解析来和生成的随机数进行匹配
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small！"),
            Ordering::Greater => println!("Too big！"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
