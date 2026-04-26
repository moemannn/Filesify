mod config;

use config::{DISTRO_PACKAGE_MANAGERS, USER_PACKAGE_MANAGERS};

const LIST_COMMANDS: bool = false;
const CHECKS_PACKAGE_MANAGERS: bool = true;

fn main() {
    if LIST_COMMANDS { list_commands(); }
    if CHECKS_PACKAGE_MANAGERS { checks_package_managers(); }
}

fn checks_package_managers(){
    for pm in DISTRO_PACKAGE_MANAGERS {

    }

    for pm in DISTRO_PACKAGE_MANAGERS {
        
    }
}

fn list_commands() {
    for pm in DISTRO_PACKAGE_MANAGERS {
        println!("Package manager: {}", pm.name);

        for cmd in pm.commands {
            println!(
                "  {} - {} ({})",
                cmd.name,
                cmd.cmd,
                cmd.description
            );
        }
    }

    for pm in USER_PACKAGE_MANAGERS {
        println!("Package manager: {}", pm.name);

        for cmd in pm.commands {
            println!(
                "  {} - {} ({})",
                cmd.name,
                cmd.cmd,
                cmd.description
            );
        }
    }
}