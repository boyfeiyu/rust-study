use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let ans = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut number = String::new();
        println!("请输入你猜测的数字：");
        io::stdin().read_line(&mut number).expect("Error input");

        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };
        
        println!("你猜测的数字是{}", number);

        match number.cmp(&ans) {
            Ordering::Equal => {
                println!("恭喜你猜对了！");
                break;
            }
            Ordering::Less => println!("你猜小了！"),
            Ordering::Greater => println!("你猜大了！")
        }


    }

    println!("答案是{}", ans);

}
