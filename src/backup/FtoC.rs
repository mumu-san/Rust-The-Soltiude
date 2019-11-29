use std::io;
enum InputNum {
    Num(f64),
    Null,
}
fn main() {
    println!("input a number:");
    let a: InputNum = input_fnumber();
    if let InputNum::Num(i) = a {
        println!("{}℉ = {}℃", i, f_to_c(i));
    } else {
        println!("not a number");
    }
}
fn f_to_c(x: f64) -> f64 {
    5.0 / 9.0 * (x - 32.0)
}
fn input_fnumber() -> InputNum {
    let mut a = String::new();
    match io::stdin().read_line(&mut a) {
        Ok(_) => match a.trim().parse() {
            Ok(n) => InputNum::Num(n),
            Err(_) => InputNum::Null,
        },
        Err(_) => InputNum::Null,
    }
}
