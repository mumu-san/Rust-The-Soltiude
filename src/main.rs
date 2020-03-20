use std::env;
use std::io::{stdin, stdout, Read, Write};
use vmtranslator::Vmtranslator;
fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut v = Vmtranslator::new();
    if let Err(e) = v.run(&args[1]) {
        eprintln!("Error:{}", e);
    }
    pause();
}
