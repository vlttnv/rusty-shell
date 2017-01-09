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


/// Infinite loop for main shell logic
///
/// First it renders the prompt then it reads and parses input.
/// Finally it executes the command and returns to loop again.
fn rusty_shell_loop() -> i32 {
    let mut exit_code;

    loop {
        let mut line = String::new();
        let args: Vec<&str>;

        show_prompt();

        io::stdin().read_line(&mut line)
            .expect("Failed to read line");
        args = rsh_parse_line(&line);
        exit_code = rsh_run(&args);

        if exit_code != 0 {
            break;
        }
    }

    exit_code
}

fn show_prompt() {
    let current_dir = env::current_dir().unwrap();
    print!("{} > ", current_dir.display());
    stdout().flush().ok().expect("Could not flush stdout");
}

fn rsh_cd(args: &Vec<&str>) {
    if args.len() == 1 {
        print!("rsh: expected arguments to \"cd\"");
    }

}

/// Split a line into command and args
fn rsh_parse_line(line: &str) -> Vec<&str> {
    let args: Vec<&str> = line.split_whitespace().collect();
    args
}

fn rsh_run(args: &Vec<&str>) -> i32 {
    if args.len() == 0 {
        return 0;
    }

    let mut command = Command::new(args[0]);

    if args.len() > 1 {
        command.args(&args[1..args.len()]);
    }


    // let status = command.status().unwrap_or_else(|e| { println!("{:?}", e); None });
    
    // Pattern match to retrieve the value
    // match status.code() {
    //     Some(x) => x,
    //     None    => -1,
    // }
    match command.status() {
        Ok(result) => match result.code() {
            Some(code)  => code,
            None        => -1,
        },
        Err(err) => { println!("{:?}", err); 0 }
    }
}