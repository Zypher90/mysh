mod tokens;

use std::error::Error;
use std::io::{self, stdin, Write};

fn main() -> Result<(), Box<dyn Error>>{
    let mut buffer = String::new();
    loop {
        print!("mysh>>");
        io::stdout().flush().expect("Failed to flush output stream");
        buffer.clear();
        match stdin().read_line(&mut buffer){
            Ok(0) => break,
            Ok(_) => {},
            Err(e) => {
                print!("Error reading command: {}", e);
                io::stdout().flush().expect("Failed to read command");
                continue;
            }
        }
        buffer = buffer.trim().to_string();
        if buffer.is_empty() {continue}
        match buffer.as_str() {
            "exit" => {
                println!("Exiting mysh...");
                break;
            },
            s => {println!("Would execute: {}", s)}
        }
    }
    Ok(())
}
