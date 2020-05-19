use crate::environment::does_exist;

pub struct PackageManager {
    pub command_name: &'static str,
    pub search_command: &'static str,
    pub install_command: &'static str,
    pub uninstall_command: &'static str,
    pub run_command: &'static str,
}

pub trait Installer {
    fn gen_search_command(&self, package: String) -> String;
    fn gen_install_command(&self, package: String) -> String;
    fn gen_uninstall_command(&self, package: String) -> String;
    fn gen_run_command(&self, package: String) -> String;
}

impl Installer for PackageManager {
    fn gen_search_command(&self, package_name: String) -> String {
        format!(
            "{} {} {}",
            self.command_name, self.search_command, package_name,
        )
    }

    fn gen_install_command(&self, package_name: String) -> String {
        format!(
            "{} {} {}",
            self.command_name, self.install_command, package_name,
        )
    }

    fn gen_uninstall_command(&self, package_name: String) -> String {
        format!(
            "{} {} {}",
            self.command_name, self.uninstall_command, package_name,
        )
    }

    fn gen_run_command(&self, package_name: String) -> String {
        if self.run_command != "" {
            format!(
                "{} {} {}",
                self.command_name, self.search_command, package_name,
            )
        } else {
            format!("{}", package_name)
        }
    }
}

fn get_pacman() -> PackageManager {
    PackageManager {
        command_name: "pacman",
        search_command: "-Ssq",
        install_command: "-S",
        uninstall_command: "-R",
        run_command: "",
    }
}

fn get_yay() -> PackageManager {
    PackageManager {
        command_name: "yay",
        search_command: "-Ssq",
        install_command: "-S",
        uninstall_command: "-R",
        run_command: "",
    }
}

fn get_dnf() -> PackageManager {
    PackageManager {
        command_name: "dnf",
        search_command: "search",
        install_command: "install",
        uninstall_command: "uninstall",
        run_command: "",
    }
}

fn get_apt() -> PackageManager {
    PackageManager {
        command_name: "apt",
        search_command: "search",
        install_command: "install",
        uninstall_command: "uninstall",
        run_command: "",
    }
}

fn get_snap() -> PackageManager {
    PackageManager {
        command_name: "snap",
        search_command: "find",
        install_command: "install",
        uninstall_command: "remove",
        run_command: "",
    }
}

fn get_flatpak() -> PackageManager {
    PackageManager {
        command_name: "flatpak",
        search_command: "search",
        install_command: "install",
        uninstall_command: "uninstall",
        run_command: "run",
    }
}

fn get_brew() -> PackageManager {
    PackageManager {
        command_name: "brew",
        search_command: "search",
        install_command: "install",
        uninstall_command: "uninstall",
        run_command: "",
    }
}

fn get_package_managers_for_arch() -> Vec<PackageManager> {
    vec![get_pacman(), get_yay(), get_snap(), get_flatpak()]
}

fn get_package_managers_for_redhat() -> Vec<PackageManager> {
    vec![get_dnf(), get_snap(), get_flatpak()]
}

fn get_package_managers_for_debian() -> Vec<PackageManager> {
    vec![get_apt(), get_snap(), get_flatpak()]
}

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

pub fn get_package_managers_for_macos() -> Vec<PackageManager> {
    vec![get_brew()]
}
