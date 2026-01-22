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

## License

### Material from CodeCrafters

Per [Terms and Conditions](https://codecrafters.io/terms) (Effective Date: January 24, 2020):

> Subject to your payment of any applicable fees, CodeCrafters, Inc. grants you a
non-transferable, royalty-free license, without the right to sublicense, to access
and use any CodeCrafters content (“Content”) you purchase through the Services. The
Content is owned by CodeCrafters, Inc.. Unless the Content has a separate license
that grants you other rights, you may only use the Content for your own personal,
noncommercial purposes, and you may not (a) reproduce, modify, translate or create
any derivative work of the Content; (b) sell, share, rent, lease, loan, provide,
distribute or otherwise transfer the Content; (c) circumvent any security measures
or attempt to gain access to Content that you have not paid for; or (d) permit or
encourage any third party to do any of the foregoing.
>
> While CodeCrafters, Inc. strives to offer only content of the highest caliber, we
do not guarantee that the Content is accurate or complete. Your use of or reliance
on any Content is at your own risk.

This repository was not made available with the intention to sublicense any such material.

### The rest

This work is licensed under GPLv3 to the greatest extent possible.
