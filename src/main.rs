mod command;
mod builtins;
use std::io::Write;

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


fn main() {
    loop {
        print_ps1();
        let input = get_input();
        command::process(input);
    }
}
