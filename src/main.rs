mod builtin;
mod env;

use std::{
    io::{self, Write},
    path::PathBuf,
};

use crate::builtin::{TypeResult::*, cd_handler, type_handler};

fn main() -> io::Result<()> {
    let mut input = String::with_capacity(4096);
    let mut current_dir = std::env::current_dir()?.canonicalize()?; // shouldn't start up

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input)?;
        input = input.trim().to_string();
        let input_parsed = match parser(&input) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        };

        match switcher(&mut input_parsed.iter(), &mut current_dir) {
            Ok(Some(())) => {}
            Ok(None) => break Ok(()),
            Err(e) => eprintln!("{e}"),
        }
    }
}

fn switcher<'a>(
    args: &mut core::slice::Iter<'a, String>,
    current_dir: &mut PathBuf,
) -> Result<Option<()>, SwitcherError<'a>> {
    let Some(command) = args.next() else {
        return Ok(Some(())); // ignore empty input
    };
    match command.as_str() {
        "exit" => {
            return if args.next().is_some() {
                Err(SwitcherError::Args { command, count: 0 })
            } else {
                Ok(None) // exit signal
            };
        }
        "echo" => {
            println!(
                "{}",
                args.map(|arg| arg.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            );
        }
        "type" => {
            let arg = args.next();
            if arg.is_none() || args.next().is_some() {
                return Err(SwitcherError::Args { command, count: 1 });
            }
            let result = type_handler(arg.unwrap())?;
            println!("{result}");
        }
        "pwd" => {
            println!("{}", current_dir.display());
        }
        "cd" => {
            let arg = args.next();
            if arg.is_none() || args.next().is_some() {
                return Err(SwitcherError::Args { command, count: 1 });
            }
            cd_handler(arg.unwrap(), current_dir)?;
        }
        _ =>
        // "exec"; todo: extract into a handler?
        {
            match type_handler(command)? {
                Builtin(_) => unreachable!(),
                Executable {
                    command,
                    path_to_command: _,
                } => {
                    let args = args.map(|arg| arg.to_string()).collect::<Vec<_>>();
                    std::process::Command::new(command)
                        .args(args)
                        .spawn()?
                        .wait()?;
                }
                Unknown(_) => println!("{command}: command not found"),
            };
        }
    };
    Ok(Some(()))
}

enum SwitcherError<'a> {
    Args { command: &'a str, count: u8 },
    Io(io::Error),
}

impl<'a> From<io::Error> for SwitcherError<'a> {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl<'a> std::fmt::Display for SwitcherError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SwitcherError::Args { command, count } => {
                write!(
                    f,
                    "{}: requires {} {}",
                    command,
                    count,
                    if *count == 1 { "argument" } else { "arguments" }
                )
            }
            SwitcherError::Io(e) => e.fmt(f),
        }
    }
}

fn parser(input: &str) -> io::Result<Vec<String>> {
    let mut args = vec![];
    let mut arg = String::with_capacity(64);
    let mut between_single_quotes = false;
    let mut between_double_quotes = false;
    for c in input.chars()
    // `bytes()` or `chars()`?
    {
        if between_single_quotes {
            if c == '\'' {
                between_single_quotes = false;
            } else {
                arg.push(c);
            }
        } else if between_double_quotes {
            if c == '\"' {
                between_double_quotes = false;
            } else {
                arg.push(c);
            }
        } else if c.is_whitespace() {
            // the only time a word is finished (the only time to push `arg`
            // into `args`) is when it's outside quote scoping and it's
            // followed by whitespace.
            if !arg.is_empty() {
                args.push(arg.clone());
                arg.clear();
            }
        } else if c == '\'' {
            between_single_quotes = true;
        } else if c == '"' {
            between_double_quotes = true;
        } else {
            arg.push(c);
        }
    }
    if between_single_quotes {
        Err(io::Error::other("Input contains dangling single quotes"))
    } else if between_double_quotes {
        Err(io::Error::other("Input contains dangling double quotes"))
    } else {
        if !arg.is_empty() {
            args.push(arg);
        }
        Ok(args)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("hello    world", vec!["hello", "world"])]
    #[case("'hello    world'", vec!["hello    world"])]
    #[case("'hello''world'", vec!["helloworld"])]
    #[case("hello''world", vec!["helloworld"])]
    #[case("'' '''' ''", vec![])]
    fn test_parser_single_quotes(#[case] input: &str, #[case] expected: Vec<&str>) {
        assert!(parser(&String::from(input)).is_ok());
        assert_eq!(parser(&String::from(input)).unwrap(), expected);
    }

    #[rstest]
    #[case("\"hello    world\"", vec!["hello    world"])]
    #[case("\"hello\"\"world\"", vec!["helloworld"])]
    #[case("\"hello\" \"world\"", vec!["hello", "world"])]
    #[case("\"shell's test\"", vec!["shell's test"])]
    #[case("\"\" \"\"\"\" \"\"", vec![])]
    fn test_parser_double_quotes(#[case] input: &str, #[case] expected: Vec<&str>) {
        assert!(parser(&String::from(input)).is_ok());
        assert_eq!(parser(&String::from(input)).unwrap(), expected);
    }

    #[rstest]
    #[case("'\"'", vec!["\""])]
    #[case("\"'\"", vec!["'"])]
    #[case("'\"a'\"b\"", vec!["\"ab"])]
    #[case("\"'a\"'b'", vec!["'ab"])]
    #[case("\"a\"'b'", vec!["ab"])]
    fn test_parser_mixed_quotes(#[case] input: &str, #[case] expected: Vec<&str>) {
        assert!(parser(&String::from(input)).is_ok());
        assert_eq!(parser(&String::from(input)).unwrap(), expected);
    }
}
