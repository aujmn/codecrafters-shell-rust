use std::io::{Error, Result};

pub(crate) fn parser(input: &str) -> Result<Vec<String>> {
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
        Err(Error::other("Input contains dangling single quotes"))
    } else if between_double_quotes {
        Err(Error::other("Input contains dangling double quotes"))
    } else {
        if !arg.is_empty() {
            args.push(arg);
        }
        Ok(args)
    }
}

#[cfg(test)]
pub(crate) mod test {
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
