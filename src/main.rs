mod config;
mod adapters;
mod app;

use adapters::*;
use config::{DISTRO_PACKAGE_MANAGERS, USER_PACKAGE_MANAGERS, LANGUAGE_PACKAGE_MANAGERS, TOOLCHAIN_PACKAGE_MANAGERS};
use app::*;

const LIST_COMMANDS: bool = false;
const CHECKS_PACKAGE_MANAGERS: bool = true;

fn main() {
    if LIST_COMMANDS { list_commands(); }
    if CHECKS_PACKAGE_MANAGERS { checks_package_managers();}
}

fn checks_package_managers(){
    let available = detect_available_package_managers();
    for pm in available {
        println!("Available: {}", pm.name);
    }
}

fn list_commands() {
    for pm in DISTRO_PACKAGE_MANAGERS.iter().chain(USER_PACKAGE_MANAGERS.iter()).chain(LANGUAGE_PACKAGE_MANAGERS.iter()).chain(TOOLCHAIN_PACKAGE_MANAGERS.iter()) {
        println!("Package manager: {}", pm.name);

        for cmd in pm.commands {
            let full_cmd = format!(
                "{}{} {}",
                if cmd.requires_sudo { "sudo " } else { "" },
                cmd.bin,
                cmd.args.join(" ")
            );

        println!(
            "  {:<8} → {} ({})",
            cmd.cmd_type.as_str(),
            full_cmd.trim(),
            cmd.description
        );
        }
    }
}