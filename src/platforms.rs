use crate::package_managers::{
    get_package_managers_for_linux, get_package_managers_for_macos, PackageManager,
};

pub enum Platform {
    Linux,
    MacOS,
    // Windows,
    Unknown,
}

pub fn get_operating_platform() -> Platform {
    if cfg!(target_os = "linux") {
        Platform::Linux
    } else if cfg!(target_os = "macos") {
        Platform::MacOS
    // } else if cfg!(target_os = "windows") {
    //     Platform::Windows
    } else {
        Platform::Unknown
    }
}

pub fn get_relevant_package_managers() -> Option<Vec<PackageManager>> {
    match get_operating_platform() {
        Platform::Linux => get_package_managers_for_linux(),
        Platform::MacOS => Some(get_package_managers_for_macos()),
        // Platform::Windows => (),
        _ => None,
    }
}
