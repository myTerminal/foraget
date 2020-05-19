use crate::environment::{does_exist, run_command};
use crate::package_managers::PackageManager;
use ansi_term::Color;

pub fn init() {
    println!(
        "Initializing more package sources.. ({}).",
        Color::Red.paint("Not implemented!")
    );
}

pub fn search(relevant_package_managers: Vec<PackageManager>, package_to_search: &str) {
    println!("Searching {}...", Color::Yellow.paint(package_to_search));

    // Iterate through the relevant package managers
    for p in &relevant_package_managers {
        // Check if the package manager exists
        if does_exist(p.command_name) {
            // Compose a command for the current package manager and run itc
            run_command(format!(
                "{} {} {}",
                p.command_name, p.search_command, package_to_search,
            ))
            .expect("There was an error searching for the package!");
        }
    }
}

pub fn install(_relevant_package_managers: Vec<PackageManager>, package_to_install: &str) {
    println!("Installing {}...", Color::Yellow.paint(package_to_install));
}

pub fn uninstall(_relevant_package_managers: Vec<PackageManager>, package_to_uninstall: &str) {
    println!(
        "Uninstalling {}...",
        Color::Yellow.paint(package_to_uninstall)
    );
}
