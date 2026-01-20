use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input)?;
        input = input.trim().to_string();

        if input == "exit" {
            break;
        } else if input.starts_with("echo") {
            let content = &input[5..]; // todo: handle multiple whitespace separations
            println!("{}", content);
            continue;
        }

        println!("{}: command not found", input);
    }
    Ok(())
}
