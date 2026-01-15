use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数!");

    let secret_number = rand::thread_rng().gen_range(1, 101); //前闭后开
    println!("秘密数字是: {}", secret_number);

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("你猜的数是: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("太小了!"),
        Ordering::Greater => println!("太大了!"),
        Ordering::Equal => println!("你赢了!"),
    }
}
