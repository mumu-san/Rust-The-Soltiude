use std::error::Error;
use std::fs;
fn main() {
    if let Err(e) = run("1.txt") {
        eprintln!("error:{}", e);
    }
}

fn run(file: &str) -> Result<(), Box<dyn Error>> {
    let vmcode = fs::read_to_string(file)?;
    let mut asmstring = String::new();
    for i in vmcode.lines() {
        let c: Vec<&str> = i.split_ascii_whitespace().collect();
        if c.len() > 0 {
            translate_to_asm(c, &mut asmstring)?;
        }
    }
    println!("{}", asmstring);
    Ok(())
}
fn translate_to_asm(c: Vec<&str>, asmstring: &mut String) -> Result<(), &'static str> {
    match c[0] {
        "push" => {
            //println!("push");
            match c[1] {
                "constant" => asmstring.push_str(&format!("@{}\nD=A\n@SP\nAM=M+1\nM=D\n", c[2])),
                "static" | "argument" | "local" | "this" | "that" | "temp" => {
                    asmstring.push_str(&format!("@{}\nD=A\n@SP\nAM=M+1\nM=D\n", c[2]))
                }
                _ => (),
            };
        }
        "pop" => {
            //println!("pop");
            match c[1] {
                "static" | "argument" | "local" | "this" | "that" | "temp" => {
                    asmstring.push_str(&format!("@{}\nD=A\n@SP\nAM=M+1\nM=D\n", c[2]))
                }
                _ => (),
            };
        }
        "add" => {
            //println!("add");
        }
        "sub" => {
            //println!("sub");
        }
        _ => (),
    };
    Ok(())
}
