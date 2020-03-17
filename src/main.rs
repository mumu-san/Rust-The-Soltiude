use std::env;
use std::error::Error;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    if let Err(e) = run(&args[1]) {
        eprintln!("Error:{}", e);
    }
}

fn run(file: &str) -> Result<(), Box<dyn Error>> {
    let vmcode = fs::read_to_string(file)?;
    let mut asmstring = String::new();
    let mut line_count = 0;
    let mut logi_count = 0;
    for i in vmcode.lines() {
        line_count += 1;
        let c: Vec<&str> = i.split_ascii_whitespace().collect();
        if !i.starts_with("//") {
            if c.len() > 0 {
                if let Err(e) = translate_to_asm(c, &mut asmstring, &mut logi_count) {
                    println!("line {}: \"{}\"", line_count, i);
                    return Err(e);
                }
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
    logi_count: &mut i32,
) -> Result<(), Box<dyn Error>> {
    match c[0] {
        "push" => match c.len() {
            3 => match c[2].parse() {
                Ok(i) => {
                    let addr = i;
                    push_second_match(asmstring, c[1], addr)
                }
                Err(_) => Err("Third match needs a number".into()),
            },
            _ => Err("Third match error".into()),
        },
        "pop" => match c.len() {
            3 => match c[2].parse() {
                Ok(i) => {
                    let addr = i;
                    pop_second_match(asmstring, c[1], addr)
                }
                Err(_) => Err("Third match needs a number".into()),
            },
            _ => Err("Third match error".into()),
        },
        "add" => Ok(asmstring.push_str(&format!("@SP\nAM=M-1\nD=M\nA=A-1\nM=M+D\n"))),
        "sub" => Ok(asmstring.push_str(&format!("@SP\nAM=M-1\nD=M\nA=A-1\nM=M-D\n"))),
        "eq" => {
            *logi_count += 1;
            Ok(asmstring.push_str(&format!(
            "@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\nM=0\n@END_EQ{}\nD;JNE\n@SP\nA=M-1\nM=-1\n(END_EQ{})\n",logi_count,logi_count
        )))
        }
        "lt" => {
            *logi_count += 1;
            Ok(asmstring.push_str(&format!(
            "@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\nM=0\n@END_LT{}\nD;JGE\n@SP\nA=M-1\nM=-1\n(END_LT{})\n",logi_count,logi_count
        )))
        }
        "gt" => {
            *logi_count += 1;
            Ok(asmstring.push_str(&format!(
            "@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\nM=0\n@END_GT{}\nD;JLE\n@SP\nA=M-1\nM=-1\n(END_GT{})\n",logi_count,logi_count
        )))
        }
        "and" => Ok(asmstring.push_str(&format!("@SP\nAM=M-1\nD=M\nA=A-1\nM=D&M\n"))),
        "or" => Ok(asmstring.push_str(&format!("@SP\nAM=M-1\nD=M\nA=A-1\nM=D|M\n"))),
        "not" => Ok(asmstring.push_str(&format!("@SP\nA=M-1\nM=!M\n"))),
        "neg" => Ok(asmstring.push_str(&format!("@SP\nA=M-1\nM=-M\n"))),
        "label" => Ok(asmstring.push_str(&format!("({})\n", c[1]))),
        "goto" => Ok(asmstring.push_str(&format!("@{}\n0;JMP", c[1]))),
        "if-goto" => Ok(asmstring.push_str(&format!("@{}\nM;JNE", c[1]))),
        "function" => Ok(asmstring.push_str(&format!("({})\n", c[1]))),
        "call" => Ok(asmstring.push_str(&format!("@SP\nA=M-1\nM=-M\n"))),
        "return" => Ok(asmstring.push_str(&format!("@SP\nA=M-1\nM=-M\n"))),
        _ => Err("First match error".into()),
    }
}
fn push_second_match(
    asmstring: &mut String,
    second: &str,
    addr: i32,
) -> Result<(), Box<dyn Error>> {
    match second {
        "constant" => Ok(asmstring.push_str(&format!("@{}\nD=A\n@SP\nAM=M+1\nA=A-1\nM=D\n", addr))),
        "static" => {
            Ok(asmstring.push_str(&format!("@{}\nD=M\n@SP\nAM=M+1\nA=A-1\nM=D\n", 16 + addr)))
        }
        "argument" => Ok(asmstring.push_str(&format!(
            "@ARG\nD=M\n@{}\nA=D+A\nD=M\n@SP\nAM=M+1\nA=A-1\nM=D\n",
            addr
        ))),
        "local" => Ok(asmstring.push_str(&format!(
            "@LCL\nD=M\n@{}\nA=D+A\nD=M\n@SP\nAM=M+1\nA=A-1\nM=D\n",
            addr
        ))),
        "this" => Ok(asmstring.push_str(&format!(
            "@THIS\nD=M\n@{}\nA=D+A\nD=M\n@SP\nAM=M+1\nA=A-1\nM=D\n",
            addr
        ))),
        "that" => Ok(asmstring.push_str(&format!(
            "@THAT\nD=M\n@{}\nA=D+A\nD=M\n@SP\nAM=M+1\nA=A-1\nM=D\n",
            addr
        ))),
        "pointer" => {
            Ok(asmstring.push_str(&format!("@{}\nD=M\n@SP\nAM=M+1\nA=A-1\nM=D\n", 3 + addr)))
        }
        "temp" => Ok(asmstring.push_str(&format!("@{}\nD=M\n@SP\nAM=M+1\nA=A-1\nM=D\n", 5 + addr))),
        _ => Err("Second match error".into()),
    }
}
fn pop_second_match(asmstring: &mut String, second: &str, addr: i32) -> Result<(), Box<dyn Error>> {
    match second {
        "static" => Ok(asmstring.push_str(&format!("@SP\nAM=M-1\nD=M\n@{}\nM=D\n", 16 + addr))),
        "argument" => Ok(asmstring.push_str(&format!(
            "@ARG\nD=M\n@{}\nD=D+A\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n",
            addr
        ))),
        "local" => Ok(asmstring.push_str(&format!(
            "@LCL\nD=M\n@{}\nD=D+A\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n",
            addr
        ))),
        "this" => Ok(asmstring.push_str(&format!(
            "@THIS\nD=M\n@{}\nD=D+A\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n",
            addr
        ))),
        "that" => Ok(asmstring.push_str(&format!(
            "@THAT\nD=M\n@{}\nD=D+A\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n",
            addr
        ))),
        "pointer" => Ok(asmstring.push_str(&format!("@SP\nAM=M-1\nD=M\n@{}\nM=D\n", 3 + addr))),
        "temp" => Ok(asmstring.push_str(&format!("@SP\nAM=M-1\nD=M\n@{}\nM=D\n", 5 + addr))),
        _ => Err("Second match error".into()),
    }
}
