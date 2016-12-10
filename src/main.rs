use std::io;
use std::io::Write;
use std::io::stdout;
use std::process::Command;
use std::env;

fn main() {
    // Load config files.

    // Run command loop
    let exit_code = rusty_shell_loop();

    // Perform shutdown/cleanup

    // Exit
    std::process::exit(exit_code);
}

fn rusty_shell_loop() -> i32 {
    let mut exit_code;

    loop {
        let mut line = String::new();
        let args: Vec<&str>;

        show_prompt();
        stdout().flush().ok().expect("Could not flush stdout");

        io::stdin().read_line(&mut line)
            .expect("Failed to read line");
        args = rsh_parse_line(&line);
        exit_code = rsh_execute(&args);

        if exit_code != 0 {
            break;
        }
    }

    exit_code
}

fn show_prompt() {
    let current_dir = env::current_dir().unwrap();
    print!("{} > ", current_dir.display());
}

fn rsh_parse_line(line: &str) -> Vec<&str> {
    // Split a line into command and args
    let args: Vec<&str> = line.split_whitespace().collect();
    args
}

fn rsh_execute(args: &Vec<&str>) -> i32 {

    let mut command = Command::new(args[0]);

    if args.len() > 1 {
        command.args(&args[1..args.len()-1]);
    }

    let status = command.status()
        .expect("command failed to start");
    
    // Pattern match to retrieve the value
    match status.code() {
        Some(x) => x,
        None    => -1,
    }
}