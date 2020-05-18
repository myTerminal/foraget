use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};

pub fn evaluate(expression: String) -> String {
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
}

pub fn does_exist(command: &'static str) -> bool {
    // Return true if the command exists in the environment
    evaluate(format!("which {}", command)) != ""
}

pub fn run_command(command: String) -> Result<(), Error> {
    // Run the command and capture the stdout
    let stdout = Command::new("/bin/bash")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

    // Print the stdout
    let reader = BufReader::new(stdout);
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));

    // Exit the function
    Ok(())
}
