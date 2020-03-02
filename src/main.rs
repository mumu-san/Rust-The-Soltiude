use std::env;
use std::error::Error;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    if let Err(e) = run(&args[1]) {
        eprintln!("error:{}", e);
    }
}

fn run(file: &str) -> Result<(), Box<dyn Error>> {
    let vmcode = fs::read_to_string(file)?;
    let mut asmstring = String::new();
    let mut count = 0;
    for i in vmcode.lines() {
        let c: Vec<&str> = i.split_ascii_whitespace().collect();
        if !i.starts_with("//") {
            if c.len() > 0 {
                translate_to_asm(c, &mut asmstring, &mut count)?;
            }
        }
    }
    let filename = match file.split(".").next() {
        Some(s) => s,
        _ => "default",
    };
    fs::write(format!("{}.asm", filename), asmstring)?;
    Ok(())
}
fn translate_to_asm(
    c: Vec<&str>,
    asmstring: &mut String,
    count: &mut i32,
) -> Result<(), Box<dyn Error>> {
    let mut addr: i32 = 0;
    if c.len() > 1 {
        addr = c[2].parse()?;
    }
    match c[0] {
        "push" => {
            match c[1] {
                "constant" => {
                    asmstring.push_str(&format!("@{}\nD=A\n@SP\nAM=M+1\nA=A-1\nM=D\n", addr))
                }
                "static" => {
                    asmstring.push_str(&format!("@{}\nD=M\n@SP\nAM=M+1\nA=A-1\nM=D\n", 16 + addr))
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
                "pointer" => {
                    asmstring.push_str(&format!("@{}\nD=M\n@SP\nAM=M+1\nA=A-1\nM=D\n", 3 + addr))
                }
                "temp" => {
                    asmstring.push_str(&format!("@{}\nD=M\n@SP\nAM=M+1\nA=A-1\nM=D\n", 5 + addr))
                }
                _ => (),
            };
        }
        "pop" => {
            match c[1] {
                "static" => asmstring.push_str(&format!("@SP\nAM=M-1\nD=M\n@{}\nM=D\n", 16 + addr)),
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
                "pointer" => asmstring.push_str(&format!("@SP\nAM=M-1\nD=M\n@{}\nM=D\n", 3 + addr)),
                "temp" => asmstring.push_str(&format!("@SP\nAM=M-1\nD=M\n@{}\nM=D\n", 5 + addr)),
                _ => (),
            };
        }
        "add" => asmstring.push_str(&format!("@SP\nAM=M-1\nD=M\nA=A-1\nM=M+D\n")),
        "sub" => asmstring.push_str(&format!("@SP\nAM=M-1\nD=M\nA=A-1\nM=M-D\n")),
        "eq" => {
            *count += 1;
            asmstring.push_str(&format!(
            "@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\nM=0\n@END_EQ{}\nD;JNE\n@SP\nA=M-1\nM=-1\n(END_EQ{})\n",count,count
        ));
        }
        "lt" => {
            *count += 1;
            asmstring.push_str(&format!(
            "@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\nM=0\n@END_LT{}\nD;JGE\n@SP\nA=M-1\nM=-1\n(END_LT{})\n",count,count
        ));
        }
        "gt" => {
            *count += 1;
            asmstring.push_str(&format!(
            "@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\nM=0\n@END_GT{}\nD;JLE\n@SP\nA=M-1\nM=-1\n(END_GT{})\n",count,count
        ));
        }
        "and" => asmstring.push_str(&format!("@SP\nAM=M-1\nD=M\nA=A-1\nM=D&M\n")),
        "or" => asmstring.push_str(&format!("@SP\nAM=M-1\nD=M\nA=A-1\nM=D|M\n")),
        "not" => asmstring.push_str(&format!("@SP\nA=M-1\nM=!M\n")),
        "neg" => asmstring.push_str(&format!("@SP\nA=M-1\nM=-M\n")),
        _ => panic!("首位错误"),
    };
    Ok(())
}
