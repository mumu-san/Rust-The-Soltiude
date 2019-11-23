use std::io;
fn main() {
    let n: u32 = input_inumber();
    fbn(n);
}
fn fbn(u: u32) {
    if u <= 0 {
        println!("Wrong Number");
    } else if u > 47 {
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
fn input_inumber() -> u32 {
    println!("Input a Number:");
    let mut a = String::new();
    match io::stdin().read_line(&mut a) {
        Ok(_) => {
            match a.trim().parse() {
                Ok(n) => return n,
                Err(_) => {
                    println!("Not a number");
                    return 0;
                }
            };
        }
        Err(_) => {
            println!("Input Error");
            return 0;
        }
    };
}
