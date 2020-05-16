pub fn init () {
    println!("Initializing more package sources...");
}

pub fn search (package_to_search: &str) {
    println!("Searching {}...", package_to_search);
}

pub fn install (package_to_install: &str) {
    println!("Installing {}...", package_to_install);
}

pub fn uninstall (package_to_uninstall: &str) {
    println!("Uninstalling {}...", package_to_uninstall);
}
