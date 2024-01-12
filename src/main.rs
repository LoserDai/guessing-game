use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the Number!!!");
    //随机生成一个1到100的数
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret_number is: {secret_number}");
    println!("Please input your guess!!!");

    //mut可以使一个变量可变
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line !");

    let guess: u32 = guess.trim().parse().expect("please type a number!");

    println!("You guessed:{guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println! {"Too small!"},
        Ordering::Equal => println! {"You Win!"},
        Ordering::Greater => println! {"Too large!"},
    }

    // let x = 2.00;
    // let y: f32 = 4.00;
    // println!("x:{x} and y:{y}");


    // let tup: (i32, f64, u8) = (58, 6.3, 1);
    // let (x,y,z) = tup;
    // println!("The value of y is :{y}");
    // let first = tup.0;
    // let sec = tup.1;
    // let third = tup.2;
    // println!("{first},{sec},{third}");
}
