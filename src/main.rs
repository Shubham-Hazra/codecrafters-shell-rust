#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    let builtin_commands: Vec<&str> = vec!["echo", "exit", "type"];

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let words: Vec<&str> = input.splitn(2, " ").collect();

        if words[0] == "exit" {
            if words.len() > 2 {
                println!("Error. Please give an integer exit code");
                process::exit(1);
            }

            let exitcode: i32 = words[1].trim().parse().unwrap();
            process::exit(exitcode);
        } else if words[0] == "echo" {
            print!("{}", words[1]);
        } else if words[0] == "type" {
            if builtin_commands.contains(&words[1].trim()) {
                println!("{} is a shell builtin", words[1].trim())
            } else {
                println!("{}: not found", words[1].trim());
            }
        } else {
            println!("{}: not found", input.trim());
        }
    }
}
