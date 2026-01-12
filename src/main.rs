use std::io;

fn main() {
    println!("猜数!");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("你猜的数是: {}", guess);
}
