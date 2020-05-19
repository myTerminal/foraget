use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};

pub fn evaluate_as_list(expression: String) -> Vec<String> {
    // Run the expression as a shell command and obtain the output
    let output = Command::new("/bin/bash")
        .arg("-c")
        .arg(expression)
        .output()
        .expect("Something went wrong!");

    // Return the stdout as a string
    std::str::from_utf8(&output.stdout)
        .unwrap()
        .trim()
        .to_string()
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

pub fn does_exist(command: &'static str) -> bool {
    // Return true if the command exists in the environment
    evaluate_as_list(format!("which {}", command))[0] != ""
}

pub fn run_command(command: String) -> Result<Vec<String>, Error> {
    // Run the command and capture the stdout
    let stdout = Command::new("/bin/bash")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

    // Read the stdout
    let reader = BufReader::new(stdout);

    // Return the stdout as a list
    Ok(
        reader
            .lines()
            .filter_map(|line| line.ok())
            .collect::<Vec<String>>()
    )
}

pub fn run_command_and_print_result(command: String) {
    run_command(command)
        .iter()
        .for_each(|line| println!("{:?}", line));
}
