use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};
use ansi_term::Color;

mod tasks;

fn main() {
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
        tasks::init();
    } else if let Some(matches) = matches.subcommand_matches("search") {
        let package_to_search = matches.value_of("PACKAGE").unwrap();

        tasks::search(package_to_search);
    } else if let Some(matches) = matches.subcommand_matches("install") {
        let package_to_install = matches.value_of("PACKAGE").unwrap();

        tasks::install(package_to_install);
    } else if let Some(matches) = matches.subcommand_matches("uninstall") {
        let package_to_uninstall = matches.value_of("PACKAGE").unwrap();

        tasks::uninstall(package_to_uninstall);
    } else {
        println!("{}", Color::Red.paint("Please run foraget with a command!"));
    }
}
