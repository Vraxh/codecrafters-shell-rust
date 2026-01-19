#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        match io::stdin().read_line(&mut command) {
            Ok(_) if command.trim().is_empty() => continue,
            Ok(_) if command.trim() == "exit" => return,
            Ok(_) if command.starts_with("echo ") => {
                let args = command.trim_start_matches("echo").trim();
                println!("{}", args);
            }
            Ok(_) if command.starts_with("type ") => {
                if command.contains("type type") {
                    println!("type is a shell builtin");
                } else if command.contains("exit") {
                    println!("exit is a shell builtin");
                } else if command.contains("echo") {
                    println!("echo is a shell builtin");
                } else {
                    let cmd_name = command.trim_start_matches("type").trim();
                    println!("{}: not found", cmd_name);
                }
            }
            Ok(_) => println!("{}: command not found", command.trim()),
            Err(error) => println!("error: {error}"),
        }
    }
}
