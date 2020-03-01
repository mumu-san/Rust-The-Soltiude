use std::env;
use std::error::Error;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    if let Err(e) = run(&args[1]) {
        eprintln!("error:{}", e);
    }
}

fn run(file: &str) -> Result<(), Box<dyn Error>> {
    let vmcode = fs::read_to_string(file)?;
    let mut asmstring = String::new();
    //let mut count = 1;
    for i in vmcode.lines() {
        let c: Vec<&str> = i.split_ascii_whitespace().collect();
        if !i.starts_with("//") {
            if c.len() > 0 {
                translate_to_asm(c, &mut asmstring)?;
            }
        }
        //count += 1;
    }
    let filename = match file.split(".").next() {
        Some(s) => s,
        _ => "default",
    };
    fs::write(format!("{}.asm", filename), asmstring)?;
    Ok(())
}
fn translate_to_asm(c: Vec<&str>, asmstring: &mut String) -> Result<(), Box<dyn Error>> {
    let mut addr: i32 = 0;
    if c.len() > 1 {
        addr = c[2].parse()?;
    }
    match c[0] {
        "push" => {
            //println!("push");
            match c[1] {
                "constant" => {
                    asmstring.push_str(&format!("@{}\nD=A\n@SP\nAM=M+1\nA=A-1\nM=D\n", c[2]))
                }
                "static" => {
                    asmstring.push_str(&format!("@{}\nD=A\n@SP\nAM=M+1\nA=A-1\nM=D\n", 16 + addr))
                }
                "argument" => asmstring.push_str(&format!(
                    "@ARG\nD=M\n@{}\nA=D+A\nD=M\n@SP\nAM=M+1\nA=A-1\nM=D\n",
                    addr
                )),
                "local" => asmstring.push_str(&format!(
                    "@LCL\nD=M\n@{}\nA=D+A\nD=M\n@SP\nAM=M+1\nA=A-1\nM=D\n",
                    addr
                )),
                "this" => asmstring.push_str(&format!(
                    "@THIS\nD=M\n@{}\nA=D+A\nD=M\n@SP\nAM=M+1\nA=A-1\nM=D\n",
                    addr
                )),
                "that" => asmstring.push_str(&format!(
                    "@THAT\nD=M\n@{}\nA=D+A\nD=M\n@SP\nAM=M+1\nA=A-1\nM=D\n",
                    addr
                )),
                "temp" => {
                    asmstring.push_str(&format!("@{}\nD=A\n@SP\nAM=M+1\nA=A-1\nM=D\n", 5 + addr))
                }
                _ => (),
            };
        }
        "pop" => {
            //println!("pop");
            match c[1] {
                "static" => asmstring.push_str(&format!(
                    "@{}\nD=M\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n",
                    16 + addr
                )),
                "argument" => asmstring.push_str(&format!(
                    "@ARG\nD=M\n@{}\nD=D+A\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n",
                    addr
                )),
                "local" => asmstring.push_str(&format!(
                    "@LCL\nD=M\n@{}\nD=D+A\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n",
                    addr
                )),
                "this" => asmstring.push_str(&format!(
                    "@THIS\nD=M\n@{}\nD=D+A\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n",
                    addr
                )),
                "that" => asmstring.push_str(&format!(
                    "@THAT\nD=M\n@{}\nD=D+A\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n",
                    addr
                )),
                "temp" => asmstring.push_str(&format!(
                    "@{}\nD=M\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n",
                    5 + addr
                )),
                _ => (),
            };
        }
        "add" => {
            //println!("add");
            asmstring.push_str(&format!("@SP\nAM=M-1\nD=M\nM=M+D\n"))
        }
        "sub" => {
            //println!("sub");
            asmstring.push_str(&format!("@SP\nAM=M-1\nD=M\nM=M-D\n"))
        }
        _ => (),
    };
    Ok(())
}
