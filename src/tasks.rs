use ansi_term::Color;

pub fn init() {
    println!("Initializing more package sources...");
}

pub fn search(package_to_search: &str) {
    println!("Searching {}...", Color::Yellow.paint(package_to_search));
}

pub fn install(package_to_install: &str) {
    println!("Installing {}...", Color::Yellow.paint(package_to_install));
}

pub fn uninstall(package_to_uninstall: &str) {
    println!(
        "Uninstalling {}...",
        Color::Yellow.paint(package_to_uninstall)
    );
}
