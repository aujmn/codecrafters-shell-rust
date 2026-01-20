use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("$ ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input = input.trim().to_string();
    println!("{}: command not found", input);
    Ok(())
}
