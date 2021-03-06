use std::io;
enum InputNum<T> {
    Num(T),
    Null,
}
fn main() {
    println!("input a number:");
    let n: InputNum<u32> = input_number();
    if let InputNum::Num(i) = n {
        fbn(i);
    } else {
        println!("wrong number");
    }
}
fn fbn(u: u32) {
    if u > 47 {
        println!("May OverFlow");
    } else {
        println!("fbn below:");
        let mut p1: u32 = 1;
        let mut p2: u32 = 0;
        for _i in 0..u {
            p2 = p1 + p2;
            p1 = p2 - p1;
            print!("{} ", p2);
        }
    }
}
fn input_number() -> InputNum<u32> {
    let mut a = String::new();
    match io::stdin().read_line(&mut a) {
        Ok(_) => match a.trim().parse() {
            Ok(n) => InputNum::Num(n),
            Err(_) => InputNum::Null,
        },
        Err(_) => InputNum::Null,
    }
}
