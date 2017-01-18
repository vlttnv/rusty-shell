use std::io;
use std::io::Write;
use std::io::stdout;
use std::process::Command;
use std::env;

const BUILT_IN_COMMANDS: &'static [&'static str] = &["cd", "exit"];

fn main() {
    // Load config files.

    // Run command loop
    // let exit_code = rusty_shell_loop();
    rusty_shell_loop();

    // Perform shutdown/cleanup

    // Not handling this yet
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

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        args = rsh_parse_line(&line);

        // TODO: save exitcode?
        exit_code = rsh_run(&args)
    }

    // Not handling this yet
    // exit_code
}

/// Render the command line prompt
fn show_prompt() {
    let current_dir = env::current_dir().unwrap();
    print!("{} > ", current_dir.display());
    stdout().flush().ok().expect("Could not flush stdout");
}

/// Buildint for changing directories
pub fn rsh_cd(args: &Vec<&str>) -> i32 {
    if args.len() == 1 {
        println!("rsh: expected arguments to \"cd\"");
        -1
    } else {
        match env::set_current_dir(args[1]) {
            Ok(_) => 0,
            Err(err) => { println!("{}", err); -1},
        }
    }
}

#[allow(unreachable_code)]
fn rsh_exit() -> i32 {
    std::process::exit(0);
    0
}

/// Split a line into command and args
pub fn rsh_parse_line(line: &str) -> Vec<&str> {
    let args: Vec<&str> = line.split_whitespace().collect();
    args
}

pub fn rsh_run(args: &Vec<&str>) -> i32 {
    if args.len() == 0 {
        return 0;
    }

    // D(r)aft way of handling built in commands
    if BUILT_IN_COMMANDS.contains(&args[0]) {
        let exit_code = match args[0] {
            "cd" => rsh_cd(&args),
            "exit" => rsh_exit(),
            _ => -1,
        };
        return exit_code;
    }

    let mut command = Command::new(args[0]);

    if args.len() > 1 {
        command.args(&args[1..args.len()]);
    }

    // Try to get the code and return it
    // Print the error otherwise
    match command.status() {
        Ok(result) => result.code().unwrap_or(-1),
        Err(err) => {
            println!("{}: {}", args[0], err);
            err.raw_os_error().unwrap_or(-1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsh_parse_line() {
        let command = "ls -l -a";
        let args: Vec<&str>;
        args = rsh_parse_line(&command);
        assert_eq!(args.len(), 3);
        assert_eq!(args[0], "ls");
        assert_eq!(args[1], "-l");
        assert_eq!(args[2], "-a");
    }

    #[test]
    fn test_rsh_run() {
        let parsed_command = vec!["pwd"];
        let exit_code = rsh_run(&parsed_command);
        assert_eq!(exit_code, 0);
    }

    #[test]
    fn test_rsh_cd() {
        let parsed_command_single = vec!["cd"];
        let exit_code = rsh_cd(&parsed_command_single);
        assert_eq!(exit_code, -1);

        let parsed_command_single = vec!["cd", ".."];
        let exit_code = rsh_cd(&parsed_command_single);
        assert_eq!(exit_code, 0);
    }
}