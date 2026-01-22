This is a Rust solution to CodeCrafters'
["Build Your Own Shell" Challenge](https://app.codecrafters.io/courses/shell/overview).

> In this challenge, you'll build your own POSIX-compliant shell that's capable of
interpreting shell commands, running external programs and builtin commands like
cd, pwd, echo and more. Along the way, you'll learn about shell command parsing,
REPLs, builtin commands, and more.

Head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.

## Compiling locally

1. Ensure you have `cargo 1.92` installed by running `cargo --version` in shell.
1. Run `./your_program.sh`, provided by CodeCrafters, to run the program.
Cargo commands such as `cargo run` and `cargo check` work too.

## Project caveats

1. CodeCrafters provided tests.
1. Much of this implementation is building a command-line interface on top of Rust
functions on top of (I assume) certain POSIX implementations and binaries. It is not
an attempt to really build the shell from the bottom up. I've had a lot of help.
1. The base stages and first extension were checked into source control via the
`codecrafters` CLI, which does not use meaningful commit messages.
