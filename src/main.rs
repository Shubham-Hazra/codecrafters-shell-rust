#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
    
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let words: Vec<&str> = input.split_ascii_whitespace().collect();

        if words[0] == "exit" {
            let exitcode: i32 = words[1].parse().unwrap();
            process::exit(exitcode);
        }

        println!("{}: not found", input.trim());
    }

}
