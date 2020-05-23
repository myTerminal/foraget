use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};

// Runs a shell command with no stdin and returns the output as a list
pub fn run_command_and_get_list(expression: &str) -> Vec<String> {
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

// Returns whether a command exists
pub fn does_exist(command: &str) -> bool {
    // Return true if the command exists in the environment
    run_command_and_get_list(&format!("which {}", command))[0] != ""
}

// Runs a shell command with stdin and returns the stdout at the end
pub fn run_command_and_get_result(command: &str) -> Result<Vec<String>, Error> {
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
    Ok(reader
        .lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<String>>())
}

// Runs a shell command with stdin and prints stdout at the end
pub fn run_command_and_print_result(command: &str) {
    run_command_and_get_result(&command)
        .iter()
        .for_each(|line| println!("{:?}", line));
}

// Runs a shell command with full stdio only returns the status
pub fn run_command_continuous(command: &str) -> Result<(), Error> {
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

pub fn print_list(list: &Vec<String>) {
    list.iter().for_each(|l| println!("{}", l));
}

fn get_multiline_string(items: &Vec<String>) -> String {
    if items.len() == 0 {
        return String::new();
    }

    let mut command = String::from(&items[0]);

    for i in 1..(items.len()) {
        command.push_str(&format!("\\n{}", items[i]));
    }

    command
}

pub fn prompt_for_value_from_list(options: &Vec<String>) -> String {
    let result = run_command_and_get_result(&format!(
        "echo -e \"{}\" | fzf",
        get_multiline_string(&options)
    ));

    result.unwrap_or(vec![String::from("")])[0].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_multiline_string_some_values() {
        assert_eq!(
            "One\\nTwo\\nThree",
            get_multiline_string(&vec![
                String::from("One"),
                String::from("Two"),
                String::from("Three")
            ]),
            "Creates a multiline string out of a string vector with a few strings"
        );
    }

    #[test]
    fn get_multiline_string_one_value() {
        assert_eq!(
            "One",
            get_multiline_string(&vec![String::from("One"),]),
            "Creates a multiline string out of a string vector with a single string"
        );
    }

    #[test]
    fn get_multiline_string_no_value() {
        assert_eq!(
            "",
            get_multiline_string(&Vec::<String>::new()),
            "Creates a multiline string out of an empty string vector"
        );
    }
}
