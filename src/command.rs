use std::process::ExitCode;
use std::process::Command;
use crate::builtins;

#[allow(dead_code)]
fn unknown_command() -> ExitCode {
    println!("Unknown command");
    ExitCode::FAILURE
}

fn exec(words: Vec<&str>) -> ExitCode {
    // TODO This doesn't run "interactively".
    let output = Command::new(words[0]).args(&words[1..]).output();

    if let Ok(output) = output {
        println!("{}", String::from_utf8(output.stdout).unwrap());
    }
    else {
        println!("Error: {}", output.err().unwrap());
    }

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
