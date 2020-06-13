//! Contains logic around platform detection and package manager mappings.

use crate::package_managers::{
    get_package_managers_for_linux, get_package_managers_for_macos, PackageManager,
};


/// An enumeration of supported platforms.
pub enum Platform {
    Linux,
    MacOS,
    // Windows,
    Unknown,
}


/// Detects and returns current operating platform.
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

/// Get relevant package managers for current operating system.
pub fn get_relevant_package_managers() -> Option<Vec<PackageManager>> {
    match get_operating_platform() {
        Platform::Linux => get_package_managers_for_linux(),
        Platform::MacOS => Some(get_package_managers_for_macos()),
        // Platform::Windows => (),
        _ => None,
    }
}
