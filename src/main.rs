use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the Number!!!");
    //随机生成一个1到100的数
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret_number is: {secret_number}");
    //加入循环
    loop {
        println!("Please input your guess!!!");

        //mut可以使一个变量可变
        let mut guess = String::new();

        //读取控制台打印的数据是多少
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line !");

        //优化：处理无效的输入
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed:{guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println! {"Too small!"},
            Ordering::Greater => println! {"Too large!"},
            //如果猜中了就跳出循环
            Ordering::Equal => {
                println!("You Win!!!");
                break;
            }
        }
    }
}
