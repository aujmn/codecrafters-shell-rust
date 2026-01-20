use std::{
    collections::HashSet,
    io::{self, Write},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    let builtin_keywords = HashSet::<&str>::from(["exit", "echo", "type"]);

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input)?;
        input = input.trim().to_string();

        if input == "exit" {
            break;
        } else if let Some((command, content)) = input.split_once(' ')
            && command == "echo"
        {
            if content.contains("  ") {
                todo!()
            }
            println!("{content}");
            continue;
        } else if let Some((command, query)) = input.split_once(' ')
            && command == "type"
        {
            if query.contains(' ') {
                todo!()
            }
            if builtin_keywords.contains(query) {
                println!("{query} is a shell builtin");
            } else {
                println!("{query}: not found");
            }
            continue;
        }

        println!("{input}: command not found");
    }
    Ok(())
}
