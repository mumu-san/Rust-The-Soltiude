use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let secret = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("input your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed:{}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret) {
            Ordering::Less => println!("小了"),
            Ordering::Greater => println!("大了"),
            Ordering::Equal => {
                println!("好了");
                break;
            }
        }
    }
    //println!("The Secret number is:{}", secret);
}
