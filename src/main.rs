//! # foraget
//!
//! `foraget` is a simple universal package manager for Unix-like systems.

use std::process;

use ansi_term::Color;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};

mod environment;
mod package_managers;
mod platforms;
mod tasks;

use environment::does_exist;
use package_managers::PackageManager;
use platforms::get_relevant_package_managers;

/// The entry point to foraget.
///
/// Gathers information about relevant package managers for the current environment and calls `run`
/// passing them in. If there are no known package managers for the currently detected environment,
/// ends the program with an appropriate message.
fn main() {
    // Check for dependencies
    check_for_dependencies();

    // Get relevant package managers
    if let Some(package_managers) = get_relevant_package_managers() {
        // Run foraget for the relevant package managers
        run(&package_managers);
    } else {
        // Print error message about non-implementation for the platform
        println!(
            "{}",
            Color::Red.paint("This platform is not yet supported!")
        );

        // Exit foraget
        process::exit(0);
    }
}

fn check_for_dependencies() {
    let dependencies = vec![("fzf", "A command-line fuzzy finder")];

    dependencies.iter().for_each(|d| {
        if !does_exist(d.0) {
            println!("{}", Color::Red.paint("The below dependency is required:"));
            println!("{}  -  {}", d.0, d.1);
            process::exit(0);
        }
    })
}

/// Runs foraget with the supplied package managers.
fn run(package_managers: &Vec<PackageManager>) {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(SubCommand::with_name("init").about("Install additional package sources"))
        .subcommand(
            SubCommand::with_name("search")
                .about("Search for a package across sources")
                .arg(
                    Arg::with_name("PACKAGE")
                        .help("The package to search")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("install")
                .about("Install a package")
                .arg(
                    Arg::with_name("PACKAGE")
                        .help("The package to install")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("uninstall")
                .about("Uninstall a package if installed")
                .arg(
                    Arg::with_name("PACKAGE")
                        .help("The package to uninstall")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    if let Some(_) = matches.subcommand_matches("init") {
        // Init package sources
        tasks::init();
    } else if let Some(matches) = matches.subcommand_matches("search") {
        // Search for the package across relevant package managers
        tasks::search(&package_managers, matches.value_of("PACKAGE").unwrap());
    } else if let Some(matches) = matches.subcommand_matches("install") {
        // Prompt to install the package from one of the relevant package managers
        tasks::install(&package_managers, matches.value_of("PACKAGE").unwrap());
    } else if let Some(matches) = matches.subcommand_matches("uninstall") {
        // Try uninstalling the package using one of the relevant package managers
        tasks::uninstall(&package_managers, matches.value_of("PACKAGE").unwrap());
    } else {
        // Ask to be run with a command
        println!("{}", Color::Red.paint("Please run foraget with a command!"));
    }
}
