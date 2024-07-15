use std::process::ExitCode;
use crate::builtins;

fn unknown_command() -> ExitCode {
    println!("Unknown command");
    ExitCode::FAILURE
}

pub fn process(input: String) {
    // Check if it's a builtin.

    let words = input.split_whitespace().collect::<Vec<_>>();
    dbg!(&words);

    match words[0] {
        "echo" => builtins::echo(words),
        "exit" => builtins::exit(words),
        &_ => unknown_command(),
    };
}

