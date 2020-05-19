use crate::environment::{does_exist, evaluate_as_list};
use crate::package_managers::{PackageManager, Installer};
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
            // Run the search command and create a decorated list with package managers
            let decorated_package_list =
                get_decorated_package_list(p.command_name, evaluate_as_list(p.search(package_to_search.to_string())));

            // Print the decorated list to stdout
            print_list(decorated_package_list);
        }
    }
}

fn get_decorated_package_list(package_manager: &str, package_list: Vec<String>) -> Vec<String> {
    package_list
        .iter()
        .map(|p| {
            format!(
                "{} -> {}",
                Color::Green.paint(package_manager.to_owned()),
                p
            )
        })
        .collect::<Vec<String>>()
}

fn print_list(list: Vec<String>) {
    list.iter().for_each(|l| println!("{}", l));
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
