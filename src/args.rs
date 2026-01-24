use std::io::{Error, Result};

pub(crate) fn parser(input: &str) -> Result<Vec<String>> {
    let mut args = vec![];
    let mut arg = String::with_capacity(64);
    let mut between_single_quotes = false;
    let mut between_double_quotes = false;
    let mut backslash_escaped = false;
    // let mut literal_scope: Option<char> = None; // todo: merge related logic?

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
        } else if backslash_escaped {
            arg.push(c);
            backslash_escaped = false;
        } else if c.is_whitespace() {
            // the only time a word is finished (the only time to push `arg`
            // into `args`) is when it's outside quote scoping and it's
            // followed by whitespace.
            if !arg.is_empty() {
                args.push(arg.clone());
                arg.clear();
            }
        } else if c == '\\' {
            backslash_escaped = true;
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
    #[case("'")]
    #[case("''a'")]
    #[case("'''")]
    #[case("'  'a' ")]
    fn test_parser_single_quotes_error(#[case] input: &str) {
        assert!(
            parser(&String::from(input))
                .is_err_and(|e| e.to_string() == "Input contains dangling single quotes")
        );
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
    #[case("\"")]
    #[case("\" \"a\" ")]
    #[case("\"\"\"")]
    #[case("\"  \" a\" ")]
    fn test_parser_double_quotes_error(#[case] input: &str) {
        assert!(
            parser(&String::from(input))
                .is_err_and(|e| e.to_string() == "Input contains dangling double quotes")
        );
    }

    #[rstest]
    #[case(r#" '"' "#, vec!["\""])]
    #[case(r#" "'" "#, vec!["'"])]
    #[case("'\"a'\"b\"", vec!["\"ab"])]
    #[case("\"'a\"'b'", vec!["'ab"])]
    #[case("\"a\"'b'", vec!["ab"])]
    fn test_parser_mixed_quotes(#[case] input: &str, #[case] expected: Vec<&str>) {
        assert!(parser(&String::from(input)).is_ok());
        assert_eq!(parser(&String::from(input)).unwrap(), expected);
    }

    #[rstest]
    #[case(r#" ' " ' " "#)]
    #[case(r#" " ' " ' "#)]
    fn test_parser_mixed_quotes_error(#[case] input: &str) {
        assert!(parser(&String::from(input)).is_err());
    }

    #[rstest]
    #[case(r"three\ \ \ spaces", vec!["three   spaces"])]
    #[case(r"before\     after", vec!["before ", "after"])]
    #[case(r"test\nexample", vec!["testnexample"])]
    #[case(r"hello\\world", vec![r"hello\world"])]
    #[case(r"\'hello\'", vec!["'hello'"])]
    #[case(r" '\\' \\ ", vec![r"\\", r"\"])]
    fn test_parser_backslash_outside_quotes(#[case] input: &str, #[case] expected: Vec<&str>) {
        assert!(parser(&String::from(input)).is_ok());
        assert_eq!(parser(&String::from(input)).unwrap(), expected);
    }
}
