use std::process;

use ansi_term::Color;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};

mod environment;
mod package_managers;
mod platforms;
mod tasks;

use package_managers::PackageManager;
use platforms::get_relevant_package_managers;

fn main() {
    // Get relevant package managers
    if let Some(relevant_package_managers) = get_relevant_package_managers() {
        // Run foraget for the relevant package managers
        run(relevant_package_managers);
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

fn run(relevant_package_managers: Vec<PackageManager>) {
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
        tasks::search(
            &relevant_package_managers,
            matches.value_of("PACKAGE").unwrap(),
        );
    } else if let Some(matches) = matches.subcommand_matches("install") {
        // Prompt to install the package from one of the relevant package managers
        tasks::install(
            &relevant_package_managers,
            matches.value_of("PACKAGE").unwrap(),
        );
    } else if let Some(matches) = matches.subcommand_matches("uninstall") {
        // Try uninstalling the package using one of the relevant package managers
        tasks::uninstall(
            &relevant_package_managers,
            matches.value_of("PACKAGE").unwrap(),
        );
    } else {
        // Print an error message about non-implementation for the current platform
        println!("{}", Color::Red.paint("Please run foraget with a command!"));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
