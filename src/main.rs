use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;
use std::process::{exit, Command};

fn check_command(command: &str, path_dirs: &[&str]) -> (bool, String) {
    for dir in path_dirs {
        let path = format!("{}/{}", dir, command);
        if Path::new(&path).exists() {
            return (true, path);
        }
    }
    (false, format!("{}: not found", command))
}

fn main() {
    let builtin_commands: Vec<&str> = vec!["echo", "exit", "type", "pwd", "cd"];
    let path: String = env::var("PATH").unwrap();
    let path_dirs: Vec<&str> = path.split(":").collect();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let words: Vec<&str> = input.trim_start().splitn(2, " ").collect();

        if words[0].trim() == "exit" {
            if words.len() > 2 {
                println!("Error. Please give an integer exit code");
                exit(1);
            }
            let exitcode: i32 = words[1].trim().parse().unwrap();
            exit(exitcode);
        } else if words[0].trim() == "echo" {
            print!("{}", words[1]);
            continue;
        } else if words[0].trim() == "cd" {
            let dir_path = Path::new(words[1].trim());
            if !dir_path.exists() {
                println!("cd: {}: No such file or directory", words[1].trim());
                continue;
            }
            let _ = std::env::set_current_dir(&dir_path);
        } else if words[0].trim() == "pwd" {
            let pwd = env::current_dir().unwrap();
            println!("{}", pwd.display());
            continue;
        } else if words[0].trim() == "type" {
            if builtin_commands.contains(&words[1].trim()) {
                println!("{} is a shell builtin", words[1].trim())
            } else {
                let result = check_command(words[1].trim(), &path_dirs);
                println!("{}", &result.1);
            }
            continue;
        }

        let result = check_command(words[0].trim(), &path_dirs);

        if !result.0 {
            println!("{}: not found", input.trim());
        } else {
            if words.len() == 1 && words[0] == "" {
                exit(1);
            }
            let args: Vec<&str> = if words.len() > 1 {
                words[1].trim().split_ascii_whitespace().collect()
            } else {
                Vec::new()
            };
            let status = Command::new(result.1)
                .args(args)
                .spawn()
                .expect("Something went wrong")
                .wait()
                .expect("Something went wrong");

            if !status.success() {
                println!("Process failed");
            }
        }
    }
}
