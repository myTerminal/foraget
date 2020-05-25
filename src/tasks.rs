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

pub fn search(package_managers: &Vec<PackageManager>, package_to_search: &str) {
    println!("Searching {}...", Color::Yellow.paint(package_to_search));

    // Print search results from all available package managers
    print_list(&get_search_results(&package_managers, package_to_search));
}

fn get_search_results(
    package_managers: &Vec<PackageManager>,
    package_to_search: &str,
) -> Vec<String> {
    // Generate search results across package managers
    let mut list_of_search_results = package_managers
        .iter()
        .filter(|p| does_exist(p.command_name)) // Filter out package managers that don't exist
        .map(|p| {
            get_paired_search_results(
                // Run the search command and create a decorated list with package managers
                p.command_name,
                &run_command_and_get_list(&p.gen_search_command(package_to_search)),
            )
        })
        .collect::<Vec<Vec<String>>>();

    // Collect all search results in a single list
    let mut all_search_results = Vec::<String>::new();
    for results in &mut list_of_search_results {
        all_search_results.append(results);
    }

    // Return the flat list of search results
    all_search_results
}

fn get_paired_search_results(package_manager: &str, package_list: &Vec<String>) -> Vec<String> {
    package_list
        .iter()
        .filter(|p| p.len() > 0) // Filter out the search results with zero packages
        .map(|p| format!("{} -> {}", package_manager, p))
        .collect::<Vec<String>>()
}

pub fn install(package_managers: &Vec<PackageManager>, package_to_install: &str) {
    let search_results = get_search_results(package_managers, package_to_install);

    if search_results.len() == 1 {
        // When there's only a single package
        install_from_selected_pair(&package_managers, &search_results[0]);
    } else if search_results.len() == 0 {
        // When there's no package
        println!(
            "{} {}",
            Color::Red.paint("There were no results found for"),
            Color::Yellow.paint(package_to_install.to_string())
        );
    } else {
        // Let user choose one of the options
        let selected_pair = prompt_for_value_from_list(&search_results);

        install_from_selected_pair(&package_managers, &selected_pair);
    }
}

fn break_pair_from_search_result(result_item: &str) -> (String, String) {
    let pair = result_item.split(" -> ").collect::<Vec<&str>>();

    (pair[0].to_string(), pair[1].to_string())
}

fn install_from_selected_pair(package_managers: &Vec<PackageManager>, result_pair: &str) {
    let pair = break_pair_from_search_result(&result_pair);

    println!(
        "Installing {} via {}...",
        Color::Yellow.paint(&pair.1),
        pair.0
    );

    package_managers
        .iter()
        .filter(|p| p.command_name == pair.0)
        .for_each(|p| {
            let output = run_command_continuous(&p.gen_install_command(&pair.1));

            match output {
                Ok(_) => println!("{}", Color::Blue.paint("Operation complete!")),
                Err(_) => println!(
                    "{} {}",
                    Color::Red.paint("There was an error installing"),
                    Color::Yellow.paint(pair.1.to_string())
                ),
            }
        });
}

pub fn uninstall(_package_managers: &Vec<PackageManager>, package_to_uninstall: &str) {
    println!(
        "Uninstalling {}...",
        Color::Yellow.paint(package_to_uninstall)
    );

    // TODO: Implement uninstallation of packages
    println!(
        "{}",
        Color::Red.paint("Uninstalling packages is not yet implemented!")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_paired_search_results_some_values() {
        assert_eq!(
            vec![
                String::from("pack-man -> One"),
                String::from("pack-man -> Two"),
                String::from("pack-man -> Three")
            ],
            get_paired_search_results(
                "pack-man",
                &vec![
                    String::from("One"),
                    String::from("Two"),
                    String::from("Three")
                ]
            ),
            "Creates a string vector of pairs when supplied with a few values"
        );
    }

    #[test]
    fn get_paired_search_results_no_value() {
        assert_eq!(
            Vec::<String>::new(),
            get_paired_search_results("pack-man", &Vec::<String>::new()),
            "Creates a string vector of pairs when supplied with no value"
        );
    }

    #[test]
    fn break_pair_from_search_result_simple() {
        assert_eq!(
            (String::from("pack-man"), String::from("emacs")),
            break_pair_from_search_result("pack-man -> emacs"),
            "Breaks a search result into a pair"
        );
    }
}
