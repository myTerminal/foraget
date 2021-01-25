//! Contains information and logic around package managers for various environments.

use crate::environment::does_exist;

/// An abtract representation of a typical package manager.
///
/// This struct defines basic properties of a package manager that includes searching, installing,
/// uninstalling and running a package.
pub struct PackageManager {
    pub command_name: &'static str,
    pub search_key: &'static str,
    pub install_key: &'static str,
    pub uninstall_key: &'static str,
    pub run_key: &'static str,
    pub no_confirm_key: &'static str,
    pub does_need_root: bool,
}

/// A trait for generating commands for a package manager.
pub trait Installer {
    fn gen_search_command(&self, package: &str) -> String;
    fn gen_install_command(&self, package: &str) -> String;
    fn gen_uninstall_command(&self, package: &str) -> String;
    fn gen_run_command(&self, package: &str) -> String;
}

/// Implementation of trait `Installer` for struct `PackageManager`.
impl Installer for PackageManager {
    fn gen_search_command(&self, package_name: &str) -> String {
        format!("{} {} {}", self.command_name, self.search_key, package_name)
    }

    fn gen_install_command(&self, package_name: &str) -> String {
        if self.does_need_root {
            format!(
                "sudo {} {} {} {}",
                self.command_name, self.install_key, package_name, self.no_confirm_key,
            )
        } else {
            format!(
                "{} {} {} {}",
                self.command_name, self.install_key, package_name, self.no_confirm_key,
            )
        }
    }

    fn gen_uninstall_command(&self, package_name: &str) -> String {
        if self.does_need_root {
            format!(
                "sudo {} {} {} {}",
                self.command_name, self.uninstall_key, package_name, self.no_confirm_key,
            )
        } else {
            format!(
                "{} {} {} {}",
                self.command_name, self.uninstall_key, package_name, self.no_confirm_key,
            )
        }
    }

    fn gen_run_command(&self, package_name: &str) -> String {
        if self.run_key != "" {
            format!("{} {} {}", self.command_name, self.run_key, package_name)
        } else {
            format!("{}", package_name)
        }
    }
}

/// Generates an instance of 'pacman'.
fn get_pacman() -> PackageManager {
    PackageManager {
        command_name: "pacman",
        search_key: "-Ssq",
        install_key: "-S",
        uninstall_key: "-R",
        run_key: "",
        no_confirm_key: "--noconfirm",
        does_need_root: true,
    }
}

/// Generates an instance of 'paru'.
fn get_paru() -> PackageManager {
    PackageManager {
        command_name: "paru",
        search_key: "-Ssq",
        install_key: "-S",
        uninstall_key: "-R",
        run_key: "",
        no_confirm_key: "--noconfirm",
        does_need_root: false,
    }
}

/// Generates an instance of 'dnf'.
fn get_dnf() -> PackageManager {
    PackageManager {
        command_name: "dnf",
        search_key: "search",
        install_key: "install",
        uninstall_key: "uninstall",
        run_key: "",
        no_confirm_key: "-y",
        does_need_root: true,
    }
}

/// Generates an instance of 'apt'.
fn get_apt() -> PackageManager {
    PackageManager {
        command_name: "apt",
        search_key: "search",
        install_key: "install",
        uninstall_key: "uninstall",
        run_key: "",
        no_confirm_key: "-y",
        does_need_root: true,
    }
}

/// Generates an instance of 'snap'.
fn get_snap() -> PackageManager {
    PackageManager {
        command_name: "snap",
        search_key: "find",
        install_key: "install",
        uninstall_key: "remove",
        run_key: "",
        no_confirm_key: "",
        does_need_root: false,
    }
}

/// Generates an instance of 'flatpak'.
fn get_flatpak() -> PackageManager {
    PackageManager {
        command_name: "flatpak",
        search_key: "search",
        install_key: "install",
        uninstall_key: "uninstall",
        run_key: "run",
        no_confirm_key: "",
        does_need_root: false,
    }
}

/// Generates an instance of 'brew'.
fn get_brew() -> PackageManager {
    PackageManager {
        command_name: "brew",
        search_key: "search",
        install_key: "install",
        uninstall_key: "uninstall",
        run_key: "",
        no_confirm_key: "",
        does_need_root: false,
    }
}

/// Generates a list of package managers for Arch
fn get_package_managers_for_arch() -> Vec<PackageManager> {
    vec![get_pacman(), get_paru(), get_snap(), get_flatpak()]
}

/// Generates a list of package managers for RedHat
fn get_package_managers_for_redhat() -> Vec<PackageManager> {
    vec![get_dnf(), get_snap(), get_flatpak()]
}

/// Generates a list of package managers for Debian
fn get_package_managers_for_debian() -> Vec<PackageManager> {
    vec![get_apt(), get_snap(), get_flatpak()]
}

/// Generates a list of package managers for Linux
pub fn get_package_managers_for_linux() -> Option<Vec<PackageManager>> {
    if does_exist("pacman") {
        Some(get_package_managers_for_arch())
    } else if does_exist("dnf") {
        Some(get_package_managers_for_redhat())
    } else if does_exist("apt") {
        Some(get_package_managers_for_debian())
    } else {
        None
    }
}

/// Generates a list of package managers for MacOS
pub fn get_package_managers_for_macos() -> Vec<PackageManager> {
    vec![get_brew()]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_package_manager_with_root() -> PackageManager {
        PackageManager {
            command_name: "pack-man",
            search_key: "find",
            install_key: "add",
            uninstall_key: "remove",
            run_key: "start",
            no_confirm_key: "-y",
            does_need_root: true,
        }
    }

    fn get_package_manager_without_root() -> PackageManager {
        PackageManager {
            command_name: "pack-man",
            search_key: "find",
            install_key: "add",
            uninstall_key: "remove",
            run_key: "",
            no_confirm_key: "-y",
            does_need_root: false,
        }
    }

    #[test]
    fn gen_search_command() {
        assert_eq!(
            String::from("pack-man find emacs"),
            get_package_manager_with_root().gen_search_command("emacs"),
            "Generates search command for a package manager"
        );
    }

    #[test]
    fn gen_install_command_with_root() {
        assert_eq!(
            String::from("sudo pack-man add emacs -y"),
            get_package_manager_with_root().gen_install_command("emacs"),
            "Generates install command for a package manager with root"
        );
    }

    #[test]
    fn gen_install_command_without_root() {
        assert_eq!(
            String::from("pack-man add emacs -y"),
            get_package_manager_without_root().gen_install_command("emacs"),
            "Generates install command for a package manager without root"
        );
    }

    #[test]
    fn gen_uninstall_command_with_root() {
        assert_eq!(
            String::from("sudo pack-man remove emacs -y"),
            get_package_manager_with_root().gen_uninstall_command("emacs"),
            "Generates uninstall command for a package manager with root"
        );
    }

    #[test]
    fn gen_uninstall_command_without_root() {
        assert_eq!(
            String::from("pack-man remove emacs -y"),
            get_package_manager_without_root().gen_uninstall_command("emacs"),
            "Generates uninstall command for a package manager without root"
        );
    }

    #[test]
    fn gen_run_command_with_run_key() {
        assert_eq!(
            String::from("pack-man start emacs"),
            get_package_manager_with_root().gen_run_command("emacs"),
            "Generates run command for a package manager with a run key"
        );
    }

    #[test]
    fn gen_run_command_without_run_key() {
        assert_eq!(
            String::from("emacs"),
            get_package_manager_without_root().gen_run_command("emacs"),
            "Generates run command for a package manager without a run key"
        );
    }
}
