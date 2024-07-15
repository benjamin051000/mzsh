use std::process::ExitCode;
use crate::builtins;
use nix::{sys::wait::waitpid, unistd::{fork, ForkResult, write}};
use nix::libc;

fn unknown_command() -> ExitCode {
    println!("Unknown command");
    ExitCode::FAILURE
}

fn exec(_words: Vec<&str>) -> ExitCode {
    match unsafe{fork()} {
        Ok(ForkResult::Parent {child}) => {
            waitpid(child, None).unwrap();
        },
        Ok(ForkResult::Child) => {
            write(std::io::stdout(), "I'm the child!\n".as_bytes()).ok();
            // TODO exec command
            unsafe {libc::_exit(0)};
        }
        Err(_) => {
            eprintln!("Fork failed");
            return ExitCode::FAILURE;
        }
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
