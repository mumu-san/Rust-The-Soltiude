use std::error::Error;
use std::fs;
use std::path::PathBuf;
pub struct Vmtranslator {
    n_logi: i32,
    n_ret: i32,
    n_line: i32,
    content: String,
    file_name: String,
}
impl Vmtranslator {
    pub fn new() -> Vmtranslator {
        Vmtranslator {
            n_logi: 0,
            n_ret: 0,
            n_line: 0,
            file_name: String::new(),
            content: String::new(),
        }
    }
    pub fn run(&mut self, file_or_dir: &str) -> Result<(), Box<dyn Error>> {
        match fs::read_dir(file_or_dir) {
            Ok(dir) => {
                let initfile = PathBuf::from(&format!("{}\\Sys.vm", file_or_dir));
                if initfile.exists() {
                    println!("init");
                    self.process_file(&initfile)?;
                }
                for entry in dir {
                    let file = entry?;
                    if file.path().file_stem().unwrap() != "Sys" {
                        if file.path().extension().unwrap() == "vm" {
                            self.process_file(&file.path())?;
                        }
                    }
                }
                let outname = file_or_dir.rsplit("\\").next().unwrap();
                fs::write(format!("{}\\{}.asm", file_or_dir, outname), &self.content)?;
            }
            _ => {
                let file = PathBuf::from(file_or_dir);
                if file.extension().unwrap() == "vm" {
                    self.process_file(&file)?;
                    fs::write(format!("{}.asm", self.file_name), &self.content)?;
                }
            }
        }
        Ok(())
    }
    fn process_file(&mut self, file: &PathBuf) -> Result<(), Box<dyn Error>> {
        let mut vmcode = String::new();
        self.file_name = file.file_stem().unwrap().to_str().unwrap().to_string();
        if let Ok(s) = fs::read_to_string(file) {
            if file.file_stem().unwrap() == "Sys" {
                vmcode = String::from("call Sys.init 0\n");
                self.content.push_str("@256\nD=A\n@SP\nM=D\n");
            }
            vmcode.push_str(&s);
        }
        for i in vmcode.lines() {
            self.n_line += 1;
            let c: Vec<&str> = i
                .split("//")
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .collect();
            if !i.starts_with("//") {
                if c.len() > 0 {
                    if let Err(e) = self.line_to_asm(c) {
                        println!("line {}: \"{}\"", self.n_line, i);
                        return Err(e);
                    }
                }
            }
        }
        Ok(())
    }
    fn line_to_asm(&mut self, c: Vec<&str>) -> Result<(), Box<dyn Error>> {
        match c[0] {
            "push" => match c.len() {
                3 => match c[2].parse() {
                    Ok(addr) => self.push_second_match(c[1], addr),
                    Err(_) => Err("Third match needs a number".into()),
                },
                _ => Err("Third match error".into()),
            },
            "pop" => match c.len() {
                3 => match c[2].parse() {
                    Ok(addr) => self.pop_second_match(c[1], addr),
                    Err(_) => Err("Third match needs a number".into()),
                },
                _ => Err("Third match error".into()),
            },
            "add" => Ok(self.content.push_str("@SP\nAM=M-1\nD=M\nA=A-1\nM=M+D\n")),
            "sub" => Ok(self.content.push_str("@SP\nAM=M-1\nD=M\nA=A-1\nM=M-D\n")),
            "eq" => {
                self.n_logi += 1;
                Ok(self.content.push_str(&format!(
                "@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\nM=0\n@END_EQ{}\nD;JNE\n@SP\nA=M-1\nM=-1\n(END_EQ{})\n",self.n_logi,self.n_logi
            )))
            }
            "lt" => {
                self.n_logi += 1;
                Ok(self.content.push_str(&format!(
                "@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\nM=0\n@END_LT{}\nD;JGE\n@SP\nA=M-1\nM=-1\n(END_LT{})\n",self.n_logi,self.n_logi
            )))
            }
            "gt" => {
                self.n_logi += 1;
                Ok(self.content.push_str(&format!(
                "@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\nM=0\n@END_GT{}\nD;JLE\n@SP\nA=M-1\nM=-1\n(END_GT{})\n",self.n_logi,self.n_logi
            )))
            }
            "and" => Ok(self.content.push_str("@SP\nAM=M-1\nD=M\nA=A-1\nM=D&M\n")),
            "or" => Ok(self.content.push_str("@SP\nAM=M-1\nD=M\nA=A-1\nM=D|M\n")),
            "not" => Ok(self.content.push_str("@SP\nA=M-1\nM=!M\n")),
            "neg" => Ok(self.content.push_str("@SP\nA=M-1\nM=-M\n")),
            "label" => match c.len() {
                2 => Ok(self.content.push_str(&format!("({})\n", c[1]))),
                _ => Err("Second match needs a number".into()),
            },
            "goto" => match c.len() {
                2 => Ok(self.content.push_str(&format!("@{}\n0;JMP\n", c[1]))),
                _ => Err("Second match needs a number".into()),
            },
            "if-goto" => match c.len() {
                2 => Ok(self
                    .content
                    .push_str(&format!("@SP\nAM=M-1\nD=M\n@{}\nD;JNE\n", c[1]))),
                _ => Err("Second match needs a number".into()),
            },
            "function" => match c.len() {
                3 => {
                    self.content.push_str(&format!("({})\n", c[1]));
                    match c[2].parse() {
                        Ok(i) => {
                            for _ in 0..i {
                                self.content.push_str("@0\nD=A\n@SP\nAM=M+1\nA=A-1\nM=D\n");
                            }
                            Ok(())
                        }
                        Err(_) => Err("Third match needs a number".into()),
                    }
                }
                _ => Err("Third match error".into()),
            },
            "call" => match c.len() {
                3 => {
                    match c[2].parse::<i32>() {
                        Ok(i) => {
                            self.content.push_str(&format!(
                                "@RET_LABEL{}\nD=A\n@SP\nAM=M+1\nA=A-1\nM=D\n",
                                self.n_ret
                            ));
                            self.content
                                .push_str("@LCL\nD=M\n@SP\nAM=M+1\nA=A-1\nM=D\n");
                            self.content
                                .push_str("@ARG\nD=M\n@SP\nAM=M+1\nA=A-1\nM=D\n");
                            self.content
                                .push_str("@THIS\nD=M\n@SP\nAM=M+1\nA=A-1\nM=D\n");
                            self.content
                                .push_str("@THAT\nD=M\n@SP\nAM=M+1\nA=A-1\nM=D\n");
                            self.content.push_str("@SP\nD=M\n@LCL\nM=D\n");
                            self.content
                                .push_str(&format!("@5\nD=D-A\n@{}\nD=D-A\n@ARG\nM=D\n", i));
                            //ARG=sp-5-nArgs
                            self.content.push_str(&format!(
                                "@{}\n0;JMP\n(RET_LABEL{})\n",
                                c[1], self.n_ret
                            ));
                            Ok(self.n_ret += 1)
                        }
                        Err(_) => Err("Third match needs a number".into()),
                    }
                }
                _ => Err("Third match error".into()),
            },
            "return" => {
                self.content
                    .push_str("@LCL\nD=M\n@R11\nM=D\n@5\nA=D-A\nD=M\n@R12\nM=D\n");
                self.content.push_str("@SP\nAM=M-1\nD=M\n@ARG\nA=M\nM=D\n");
                self.content.push_str("@ARG\nD=M\n@SP\nM=D+1\n");
                self.content
                    .push_str("@R11\nD=M-1\nAM=D\nD=M\n@THAT\nM=D\n");
                self.content
                    .push_str("@R11\nD=M-1\nAM=D\nD=M\n@THIS\nM=D\n");
                self.content.push_str("@R11\nD=M-1\nAM=D\nD=M\n@ARG\nM=D\n");
                self.content.push_str("@R11\nD=M-1\nAM=D\nD=M\n@LCL\nM=D\n");
                Ok(self.content.push_str("@R12\nA=M\n0;JMP\n"))
            }
            _ => Err("First match error".into()),
        }
    }
    fn push_second_match(&mut self, second: &str, addr: i32) -> Result<(), Box<dyn Error>> {
        match second {
            "constant" => Ok(self
                .content
                .push_str(&format!("@{}\nD=A\n@SP\nAM=M+1\nA=A-1\nM=D\n", addr))),
            "static" => Ok(self.content.push_str(&format!(
                "@{}.{}\nD=M\n@SP\nAM=M+1\nA=A-1\nM=D\n",
                self.file_name, addr
            ))),
            "argument" => Ok(self.content.push_str(&format!(
                "@ARG\nD=M\n@{}\nA=D+A\nD=M\n@SP\nAM=M+1\nA=A-1\nM=D\n",
                addr
            ))),
            "local" => Ok(self.content.push_str(&format!(
                "@LCL\nD=M\n@{}\nA=D+A\nD=M\n@SP\nAM=M+1\nA=A-1\nM=D\n",
                addr
            ))),
            "this" => Ok(self.content.push_str(&format!(
                "@THIS\nD=M\n@{}\nA=D+A\nD=M\n@SP\nAM=M+1\nA=A-1\nM=D\n",
                addr
            ))),
            "that" => Ok(self.content.push_str(&format!(
                "@THAT\nD=M\n@{}\nA=D+A\nD=M\n@SP\nAM=M+1\nA=A-1\nM=D\n",
                addr
            ))),
            "pointer" => Ok(self
                .content
                .push_str(&format!("@{}\nD=M\n@SP\nAM=M+1\nA=A-1\nM=D\n", 3 + addr))),
            "temp" => Ok(self
                .content
                .push_str(&format!("@{}\nD=M\n@SP\nAM=M+1\nA=A-1\nM=D\n", 5 + addr))),
            _ => Err("Second match error".into()),
        }
    }
    fn pop_second_match(&mut self, second: &str, addr: i32) -> Result<(), Box<dyn Error>> {
        match second {
            "static" => Ok(self.content.push_str(&format!(
                "@SP\nAM=M-1\nD=M\n@{}.{}\nM=D\n",
                self.file_name, addr
            ))),
            "argument" => Ok(self.content.push_str(&format!(
                "@ARG\nD=M\n@{}\nD=D+A\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n",
                addr
            ))),
            "local" => Ok(self.content.push_str(&format!(
                "@LCL\nD=M\n@{}\nD=D+A\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n",
                addr
            ))),
            "this" => Ok(self.content.push_str(&format!(
                "@THIS\nD=M\n@{}\nD=D+A\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n",
                addr
            ))),
            "that" => Ok(self.content.push_str(&format!(
                "@THAT\nD=M\n@{}\nD=D+A\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n",
                addr
            ))),
            "pointer" => Ok(self
                .content
                .push_str(&format!("@SP\nAM=M-1\nD=M\n@{}\nM=D\n", 3 + addr))),
            "temp" => Ok(self
                .content
                .push_str(&format!("@SP\nAM=M-1\nD=M\n@{}\nM=D\n", 5 + addr))),
            _ => Err("Second match error".into()),
        }
    }
}
