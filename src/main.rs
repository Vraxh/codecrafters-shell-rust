#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        // Handle invalid commands
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let str_command = command.trim();
        match str_command{
            "exit" => break,
            _ => {
                println!("{}: command not found", command.trim());
            }
        }
    }
}
