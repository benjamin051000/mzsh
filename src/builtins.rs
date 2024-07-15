use std::process::ExitCode;

pub fn exit(args: Vec<&str>) -> ! {
    let exit_code = args
        .get(1)
        .copied()
        .unwrap_or("0")
        .parse::<i32>()
        .unwrap();

    std::process::exit(exit_code);
}

pub fn echo(args: Vec<&str>) -> ExitCode {
    // TODO handle flags

    let result = args.into_iter().skip(1).collect::<Vec<_>>().join(" ");
    println!("{}", result);
    
    ExitCode::SUCCESS
}
