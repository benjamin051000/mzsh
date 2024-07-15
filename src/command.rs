use std::process::ExitCode;
use std::process::{Command, Stdio};
use crate::builtins;

fn unknown_command() -> ExitCode {
    println!("Unknown command");
    ExitCode::FAILURE
}

fn exec(words: Vec<&str>) -> ExitCode {
    dbg!(&words);
    assert!(words.len() >= 1);
    let output = Command::new(words[0])
        .args(&words[1..])
        // .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    println!("{}", String::from_utf8(output.stdout).unwrap());

    ExitCode::SUCCESS
}

pub fn process(input: String) {
    // Check if it's a builtin.

    let words = input.split_whitespace().collect::<Vec<_>>();
    dbg!(&words);

    match words.get(0).copied().unwrap_or("") {
        // TODO see Tsoding Daily "Forbidden Rust" to use the command pattern here?
        "echo" => builtins::echo(words),
        "exit" => builtins::exit(words),
        "" => ExitCode::SUCCESS, // empty string shouldn't print an error
        &_ => exec(words),
    };
}
