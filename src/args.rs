use std::io::{Error, Result};

/// Kinds of "scopes" where characters within may be escaped.
enum Scope {
    SingleQuotes,
    DoubleQuotes,
    Backslash(BackslashScope),
    None,
}

/// Kinds of backslash "scopes" where the character following the backslash may
/// be escaped.
enum BackslashScope {
    DoubleQuotes,
    OutsideQuotes,
    // backslashes within single quotes are literals themselves
    // and do not require special treatment
}

pub(crate) fn parser(input: &str) -> Result<Vec<String>> {
    use Scope::*;

    let mut args = vec![];
    let mut arg = String::with_capacity(64);
    let mut scope = None;

    for c in input.chars() {
        match &scope {
            SingleQuotes if c == '\'' => scope = None,
            SingleQuotes => arg.push(c),

            DoubleQuotes if c == '\"' => scope = None,
            DoubleQuotes if c == '\\' => {
                scope = Backslash(BackslashScope::DoubleQuotes);
            }
            DoubleQuotes => arg.push(c),

            Backslash(BackslashScope::DoubleQuotes) if c == '\"' || c == '\\' => {
                arg.push(c);
                scope = DoubleQuotes;
            }
            Backslash(BackslashScope::DoubleQuotes) => {
                /*
                CodeCrafters: "Within double quotes, a backslash only escapes
                certain special characters: ", \, $, `, and newline. For all
                other characters, the backslash is treated literally."
                This differs from actual shell behavior.
                */
                arg.push('\\');
                arg.push(c);
                scope = DoubleQuotes;
            }
            Backslash(BackslashScope::OutsideQuotes) => {
                arg.push(c);
                scope = None;
            }

            None if c.is_whitespace() => {
                // the only time a word is finished (the only time to push `arg`
                // into `args`) is when it's outside quote scoping and it's
                // followed by whitespace.
                if !arg.is_empty() {
                    args.push(arg.clone());
                    arg.clear();
                }
            }
            None if c == '\\' => scope = Backslash(BackslashScope::OutsideQuotes),
            None if c == '\'' => scope = SingleQuotes,
            None if c == '"' => scope = DoubleQuotes,
            None => arg.push(c),
        }
    }

    match scope {
        SingleQuotes => Err(Error::other("Input contains dangling single quote")),
        DoubleQuotes => Err(Error::other("Input contains dangling double quote")),
        Backslash(BackslashScope::DoubleQuotes) => Err(Error::other(
            "Input contains dangling double quote before backslash",
        )),
        Backslash(BackslashScope::OutsideQuotes) => {
            /*
            this parser currently does not handle cases where backslash is
            used before newline to indicate further input, e.g.
            ```
            $ echo 1\
            > 2
            12
            ```
            */
            Err(Error::other("Unsupported input: backslash at line end"))
        }
        None => {
            if !arg.is_empty() {
                args.push(arg);
            }
            Ok(args)
        }
    }
}

#[cfg(test)]
pub(crate) mod test {
    mod parser {
        use crate::args::parser;

        use rstest::*;

        mod single_quotes {
            use super::*;

            #[rstest]
            #[case("hello    world", vec!["hello", "world"])]
            #[case("'hello    world'", vec!["hello    world"])]
            #[case("'hello''world'", vec!["helloworld"])]
            #[case("hello''world", vec!["helloworld"])]
            #[case("'' '''' ''", vec![])]
            fn success(#[case] input: &str, #[case] expected: Vec<&str>) {
                assert!(parser(&String::from(input)).is_ok());
                assert_eq!(parser(&String::from(input)).unwrap(), expected);
            }

            #[rstest]
            #[case("'")]
            #[case("''a'")]
            #[case("'''")]
            #[case("'  'a' ")]
            fn error(#[case] input: &str) {
                assert!(
                    parser(&String::from(input))
                        .is_err_and(|e| e.to_string() == "Input contains dangling single quote")
                );
            }
        }

        mod double_quotes {
            use super::*;

            #[rstest]
            #[case(r#" "hello    world" "#, vec!["hello    world"])]
            #[case(r#" "hello""world" "#, vec!["helloworld"])]
            #[case(r#" "hello" "world" "#, vec!["hello", "world"])]
            #[case(r#" "shell's test" "#, vec!["shell's test"])]
            #[case(r#" "" """" "" "#, vec![])]
            fn success(#[case] input: &str, #[case] expected: Vec<&str>) {
                assert!(parser(&String::from(input)).is_ok());
                assert_eq!(parser(&String::from(input)).unwrap(), expected);
            }

            #[rstest]
            #[case(r#" " "#)]
            #[case(r#" " "a" "#)]
            #[case(r#" """ "#)]
            #[case(r#" "  " a" "#)]
            #[case(r#" "\ "#)]
            #[case(r#" "\\"#)]
            #[case(r#" "\""#)]
            #[case(r#" "\a"#)]
            fn error(#[case] input: &str) {
                assert!(
                    parser(&String::from(input))
                        .is_err_and(|e| e.to_string() == "Input contains dangling double quote")
                );
            }
        }

        mod mixed_quotes {
            use super::*;

            #[rstest]
            #[case(r#" '"' "#, vec!["\""])]
            #[case(r#" "'" "#, vec!["'"])]
            #[case(r#" '"a'"b" "#, vec!["\"ab"])]
            #[case(r#" "'a"'b' "#, vec!["'ab"])]
            #[case(r#" "a"'b' "#, vec!["ab"])]
            fn success(#[case] input: &str, #[case] expected: Vec<&str>) {
                assert!(parser(&String::from(input)).is_ok());
                assert_eq!(parser(&String::from(input)).unwrap(), expected);
            }

            #[rstest]
            #[case(r#" ' " ' " "#)]
            #[case(r#" " ' " ' "#)]
            fn error(#[case] input: &str) {
                assert!(parser(&String::from(input)).is_err());
            }
        }

        mod backslash {
            use super::*;

            mod outside_quotes {
                use super::*;

                #[rstest]
                #[case(r"three\ \ \ spaces", vec!["three   spaces"])]
                #[case(r"before\     after", vec!["before ", "after"])]
                #[case(r"test\nexample", vec!["testnexample"])]
                #[case(r"hello\\world", vec![r"hello\world"])]
                #[case(r"\'hello\'", vec!["'hello'"])]
                #[case(r" '\\' \\ ", vec![r"\\", r"\"])]
                fn success(#[case] input: &str, #[case] expected: Vec<&str>) {
                    assert!(parser(&String::from(input)).is_ok());
                    assert_eq!(parser(&String::from(input)).unwrap(), expected);
                }

                #[rstest]
                #[case(r"a''b c\")]
                fn error(#[case] input: &str) {
                    assert!(parser(&String::from(input)).is_err_and(|e| {
                        e.to_string() == "Unsupported input: backslash at line end"
                    }));
                }
            }

            mod double_quotes {
                use super::*;

                #[rstest]
                #[case(r#" "A \\ escapes itself" "#, vec![r"A \ escapes itself"])]
                #[case(r#" "A \" inside double quotes" "#, vec![r#"A " inside double quotes"#])]
                #[case(r#" "just'one'\\n'backslash" "#, vec![r#"just'one'\n'backslash"#])]
                #[case(r#" "inside\"literal_quote."outside\" "#, vec![r#"inside"literal_quote.outside""#])]
                fn success(#[case] input: &str, #[case] expected: Vec<&str>) {
                    assert!(parser(&String::from(input)).is_ok());
                    assert_eq!(parser(&String::from(input)).unwrap(), expected);
                }

                #[rstest]
                #[case(r#" "a"'b'"\"#)]
                fn error(#[case] input: &str) {
                    assert!(parser(&String::from(input)).is_err_and(|e| {
                        e.to_string() == "Input contains dangling double quote before backslash"
                    }));
                }
            }
        }
    }
}
