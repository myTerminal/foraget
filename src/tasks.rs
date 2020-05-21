use crate::environment::{
    does_exist, print_list, prompt_for_value_from_list, run_command_and_get_list,
    run_command_continuous,
};
use crate::package_managers::{Installer, PackageManager};
use ansi_term::Color;

pub fn init() {
    println!(
        "Initializing more package sources.. ({}).",
        Color::Red.paint("Not implemented!")
    );
}

pub fn search(relevant_package_managers: &Vec<PackageManager>, package_to_search: &str) {
    println!("Searching {}...", Color::Yellow.paint(package_to_search));

    // Print search results from all available package managers
    print_list(&get_search_results(
        &relevant_package_managers,
        package_to_search,
    ));
}

fn get_search_results(
    relevant_package_managers: &Vec<PackageManager>,
    package_to_search: &str,
) -> Vec<String> {
    // Generate search results across package managers
    let mut set_of_search_results = relevant_package_managers
        .iter()
        .filter(|p| does_exist(p.command_name)) // Filter out package managers that don't exist
        .map(|p| {
            get_decorated_search_results(
                // Run the search command and create a decorated list with package managers
                p.command_name,
                run_command_and_get_list(p.gen_search_command(package_to_search.to_string())),
            )
        })
        .collect::<Vec<Vec<String>>>();

    // Collect all search results in a single list
    let mut combined_search_results = Vec::<String>::new();
    for results in &mut set_of_search_results {
        combined_search_results.append(results);
    }

    // Return the flat list of search results
    combined_search_results
}

fn get_decorated_search_results(package_manager: &str, package_list: Vec<String>) -> Vec<String> {
    package_list
        .iter()
        .filter(|p| p.len() > 0) // Filter out the search results with zero packages
        .map(|p| format!("{} -> {}", package_manager.to_owned(), p))
        .collect::<Vec<String>>()
}

pub fn install(relevant_package_managers: &Vec<PackageManager>, package_to_install: &str) {
    let search_results = get_search_results(relevant_package_managers, package_to_install);

    if search_results.len() == 1 {
        // When there's only a single package
        install_selected_package(
            &relevant_package_managers,
            &search_results[0],
            package_to_install,
        );
    } else if search_results.len() == 0 {
        // When there's no package
        println!(
            "{} {}",
            Color::Red.paint("There were no results found for"),
            Color::Yellow.paint(package_to_install.to_string())
        );
    } else {
        // Let user choose one of the options
        let selected_package = prompt_for_value_from_list(&search_results);

        install_selected_package(
            &relevant_package_managers,
            &selected_package,
            package_to_install,
        );
    }
}

fn break_pair_from_search_result(search_result: &String) -> (String, String) {
    let pair = search_result.split(" -> ").collect::<Vec<&str>>();

    (pair[0].to_string(), pair[1].to_string())
}

fn install_selected_package(
    package_managers: &Vec<PackageManager>,
    decorated_result: &String,
    package_to_install: &str,
) {
    let pair = break_pair_from_search_result(&decorated_result);

    println!(
        "Installing {} via {}...",
        Color::Yellow.paint(&pair.1),
        pair.0
    );

    package_managers
        .iter()
        .filter(|p| p.command_name == pair.0)
        .for_each(|p| {
            let output =
                run_command_continuous(p.gen_install_command(package_to_install.to_string()));

            match output {
                Ok(_) => println!("{}", Color::Blue.paint("Operation complete!")),
                Err(_) => println!(
                    "{} {}",
                    Color::Red.paint("There was an error installing"),
                    Color::Yellow.paint(package_to_install)
                ),
            }
        });
}

pub fn uninstall(_relevant_package_managers: &Vec<PackageManager>, package_to_uninstall: &str) {
    println!(
        "Uninstalling {}...",
        Color::Yellow.paint(package_to_uninstall)
    );
}
