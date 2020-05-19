use crate::environment::{does_exist, evaluate_as_list};
use crate::package_managers::{Installer, PackageManager};
use ansi_term::Color;

pub fn init() {
    println!(
        "Initializing more package sources.. ({}).",
        Color::Red.paint("Not implemented!")
    );
}

pub fn search(relevant_package_managers: Vec<PackageManager>, package_to_search: &str) {
    println!("Searching {}...", Color::Yellow.paint(package_to_search));

    // Print search results from all available package managers
    get_search_results(relevant_package_managers, package_to_search)
        .iter()
        .for_each(|r| print_list(r));
}

fn get_search_results(
    relevant_package_managers: Vec<PackageManager>,
    package_to_search: &str,
) -> Vec<Vec<String>> {
    relevant_package_managers
        .iter()
        .filter(|p| does_exist(p.command_name)) // Filter out package managers that don't exist
        .map(|p| {
            get_decorated_search_results(
                // Run the search command and create a decorated list with package managers
                p.command_name,
                evaluate_as_list(p.gen_search_command(package_to_search.to_string())),
            )
        })
        .collect::<Vec<Vec<String>>>()
}

fn get_decorated_search_results(package_manager: &str, package_list: Vec<String>) -> Vec<String> {
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

fn print_list(list: &Vec<String>) {
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
