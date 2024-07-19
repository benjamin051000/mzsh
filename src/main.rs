mod command;
mod builtins;
use std::io::Write;
use std::env;
use std::process::ExitCode;

fn print_ps1() {
    print!("$ ");
    std::io::stdout().flush().expect("Couldn't flush stdio");
}

fn get_input() -> String {
    let mut line = String::new();

    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    line
}


fn interactive_mode() {
    loop {
        print_ps1();
        let input = get_input();
        command::process(input);
    }
}

enum Mode {
    Interactive,
    SingleCommand(String),
    Error
}


fn parse_args() -> Mode {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 0 {
        return Mode::Interactive;
    }
    else {
       if args[0] == "-c" {
            return Mode::SingleCommand(args[1..].join(" "));
        } 
    }

    Mode::Error
}


fn main() -> ExitCode {
    match parse_args() {
        Mode::Interactive => interactive_mode(),
        Mode::SingleCommand(cmd) => command::process(cmd),
        Mode::Error => {
            println!("Error: Unknown argument.");
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}

