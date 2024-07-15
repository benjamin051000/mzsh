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

    match words.get(0).copied().unwrap_or("") {
        "echo" => builtins::echo(words),
        "exit" => builtins::exit(words),
        "" => ExitCode::SUCCESS, // empty string shouldn't print an error
        &_ => unknown_command(),
    };
}

