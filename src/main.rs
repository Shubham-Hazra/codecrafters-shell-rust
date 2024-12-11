use core::str;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, path::Path, process};
fn main() {
    let builtins = vec!["exit", "echo", "type", "pwd"];
    let path_env = env::var("PATH").unwrap_or_else(|_| "PATH not found".to_string());
    let paths: Vec<&str> = path_env.split(':').collect();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();
        let tokens = input.split_whitespace().collect::<Vec<&str>>();
        if tokens.is_empty() {
            continue;
        }
        match tokens[0] {
            "exit" if tokens.len() == 2 => {
                let code = tokens[1].parse::<i32>().unwrap_or_else(|_| {
                    println!("exit: invalid exit code");
                    1
                });
                process::exit(code);
            }
            "echo" => {
                let args = &tokens[1..].join(" ");
                println!("{}", args)
            }
            "type" if tokens.len() == 2 => {
                let command = tokens[1];
                if builtins.contains(&command) {
                    println!("{} is a shell builtin", command);
                } else {
                    let mut is_found = false;
                    for path in &paths {
                        let full_path = Path::new(path).join(command);
                        if full_path.exists() {
                            println!("{} is {}", command, full_path.display());
                            is_found = true;
                            break;
                        }
                    }
                    if !is_found {
                        println!("{} not found", command);
                    }
                }
            }
            "pwd" if tokens.len() == 1 => {
                let path = env::current_dir().unwrap();
                println!("{}", path.display());
            }
            _ => {
                let command = tokens[0];
                let mut is_found = false;
                for path in &paths {
                    let full_path = Path::new(path).join(command);
                    if full_path.exists() {
                        is_found = true;
                        let args = &tokens[1..];
                        let status = process::Command::new(full_path)
                            .args(args)
                            .status()
                            .expect("failed to execute process");
                        if !status.success() {
                            eprintln!("{}: command failed", command);
                        }
                        break;
                    }
                }
                if !is_found {
                    println!("{}: command not found", input)
                }
            }
        }
    }
}
