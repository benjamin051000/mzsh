use std::path::Path;
use std::env::set_current_dir;

pub fn exit(args: Vec<&str>) -> ! {
    let exit_code = args
        .get(1)
        .copied()
        .unwrap_or("0")
        .parse::<i32>()
        .unwrap();

    std::process::exit(exit_code);
}

pub fn echo(args: Vec<&str>) -> Result<(), std::io::Error> {
    // TODO handle flags

    let result = args
        .into_iter()
        .skip(1)
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", result);
    
    Ok(())
}

pub fn cd(args: Vec<&str>) -> Result<(), std::io::Error> {
    set_current_dir(Path::new(&args[1]))
}

