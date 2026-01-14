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

            Ok(_) if &command[0..4] == "echo" => {

                println!("{}", &command[4..].trim());
            }

            Ok(_) => println!("{}: command not found", command.trim()),

            Err(error) => println!("error: {error}"),
            
        }
    }
}
