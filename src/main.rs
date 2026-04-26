mod config;
mod adapters;
mod app;

use adapters::package_manager::*;
use crate::app::{AppState, app_state};
use crate::config::package_managers::PackageManager;

fn main() {
    let mut state = app_state().lock().unwrap();

    get_package_mangers(&mut state);
}

fn get_package_mangers(state: &mut AppState) {
    updater::update_package_managers(state);

    print_group("Installed package managers:", state, PackageManagerStatus::Installed);
    print_group("Not installed package managers:", state, PackageManagerStatus::NotInstalled);
    print_group("Invalid package managers:", state, PackageManagerStatus::Invalid);

    get_packages_list(state);

}

fn get_packages_list(state: &mut AppState) {
    use std::io;
    use config::package_managers::types::Capability;
    for pkg in &state.detection_result {
        if pkg.status == PackageManagerStatus::Installed {
            dbg!(pkg.manager.name.clone());
            
            if let Ok(list) = run_command(pkg.manager, Capability::List, "") {

                for i in list.lines() {
                    println!("{}", i);

                    if let Ok(output) = run_command(pkg.manager, Capability::Search, i) {
                        println!("{}", output);
                    }

                    let mut pause = String::new();
                    io::stdin().read_line(&mut pause).expect("failed to read input");
                }
            }

        }
    }
}

pub fn run_command(
    pm: &PackageManager,
    capability: config::package_managers::types::Capability,
    package: &str,
) -> Result<String, String> {
    use std::process::Command as SysCommand;
    let cmd = pm
        .commands
        .iter()
        .find(|c| c.capability == capability)
        .ok_or("Command not found")?;

    let args: Vec<String> = cmd
        .args
        .iter()
        .flat_map(|a| a.resolve(package))
        .collect();

    let output = SysCommand::new(cmd.bin)
        .args(&args)
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !output.status.success() {
        return Err(stderr);
    }

    Ok(stdout)
}

fn print_group(title: &str, state: &AppState, status: PackageManagerStatus) {
    println!("{title}");

    for pm in &state.detection_result {
        if pm.status == status {
            println!("- {}", pm.manager.name);
        }
    }

    println!();
}