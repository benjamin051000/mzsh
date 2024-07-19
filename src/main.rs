mod command;
mod builtins;
use std::io::Write;
use std::env;

fn print_ps1(last: &Result<(), std::io::Error>) {
    if last.is_ok() {
        print!("$ ");
    }
    else {
        print!("x $ ");
    }

    std::io::stdout().flush().expect("Couldn't flush stdio");
}

fn get_input() -> String {
    let mut line = String::new();

    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    line
}


fn interactive_mode() -> ! {
    let mut last_cmd_err: Result<(), std::io::Error> = Ok(());
    loop {
        print_ps1(&last_cmd_err);
        let input = get_input();
        last_cmd_err = command::process(input);
    }
}

enum Mode {
    Interactive,
    SingleCommand(String),
}


fn parse_args() -> Result<Mode, ()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 0 {
        return Ok(Mode::Interactive);
    }
    else {
       if args[0] == "-c" {
            return Ok(Mode::SingleCommand(args[1..].join(" ")));
        } 
    }

    Err(())
}


fn main() -> Result<(), std::io::Error> {
    match parse_args() {
        Ok(Mode::Interactive) => interactive_mode(),
        Ok(Mode::SingleCommand(cmd)) => command::process(cmd),
        Err(..) => {
            println!("Error: Unknown argument.");
            todo!()
        }
    }

}

