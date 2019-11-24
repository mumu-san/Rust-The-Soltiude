use std::io;
fn main() {
    let a: f64 = input_fnumber();
    println!("{}℉={}℃", a, f_to_c(a));
    //println!("The Secret number is:{}", secret);
}
fn f_to_c(x: f64) -> f64 {
    5.0 / 9.0 * (x - 32.0)
}
fn input_fnumber() -> f64 {
    let mut a = String::new();
    match io::stdin().read_line(&mut a) {
        Ok(_) => {
            let _a: f64 = match a.trim().parse() {
                Ok(n) => return n,
                Err(_) => {
                    println!("Not a number");
                    return 0.0;
                }
            };
        }
        Err(_) => {
            println!("Input Error");
            return 0.0;
        }
    };
}
