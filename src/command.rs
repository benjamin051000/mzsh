use std::process::Command;
use crate::builtins;

fn exec(words: Vec<&str>) -> Result<(), std::io::Error> {
    // TODO This doesn't run "interactively".
    let result = Command::new(words[0]).args(&words[1..]).output();

    if let Ok(output) = result {
        println!("{}", String::from_utf8(output.stdout).unwrap());
    }
    else {
        println!("Error: {}", result.err().unwrap());
        todo!("return an Err here")
    }

    Ok(())
}

pub fn process(input: String) -> Result<(), std::io::Error> {
    // Check if it's a builtin.

    let words = input.split_whitespace().collect::<Vec<_>>();
    dbg!(&words);

    match words.get(0).copied().unwrap_or("") {
        // TODO see Tsoding Daily "Forbidden Rust" to use the command pattern here?
        "echo" => builtins::echo(words),
        "exit" => builtins::exit(words),
        "cd" => builtins::cd(words),
        "" => Ok(()), // empty string shouldn't print an error
        &_ => exec(words),
    }
}
