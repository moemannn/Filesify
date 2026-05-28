mod config;
mod adapters;
mod app;
mod presentation;

use presentation::app::*;

use adapters::package_manager::*;
use crate::app::{AppState, app_state};
use crate::config::package_manager::{PackageManager};
use crate::config::package_manager::package::r#type::*;

fn main() {
    let mut state = app_state().lock().unwrap();
    get_package_mangers(&mut state);
    
    presentation::app::app(&mut state).expect("TODO: panic message");
}

fn get_package_mangers(
    state: &mut AppState
) {
    updater::update_package_managers(state);

    get_packages_list(state);

}

fn get_packages_list(state: &mut AppState) {
    use config::package_manager::{DISTRO_PACKAGE_MANAGERS, types::Capability};

    // state.packaged_grouped_by_manager.clear();

    for manager in DISTRO_PACKAGE_MANAGERS {
        let detection = state
            .detected_package_managers
            .iter()
            .find(|d| d.manager.name == manager.name);

        let is_installed = detection
            .map(|d| d.status == PackageManagerStatus::Installed)
            .unwrap_or(false);

        if !is_installed {
            continue;
        }

        if let Ok(list) = run_command(manager, Capability::List, "") {
            for line in list.lines() {
                if is_library(line) {
                    continue;
                }

                assign_grouping(line, manager.name, state);
            }
        }
    }
}
fn assign_grouping(
    line: &str,
    manager_name: &str,
    state: &mut AppState,
) {
    let group_name = clean_pkg_name(line);

    let skip = ["Listing...", "", "No", "└──"];

    if skip.contains(&group_name) || group_name.is_empty() {
        return;
    }

    state
        .detected_package_grouped
        .entry(manager_name.to_string())
        .or_default()
        .entry(Groups {
            name: group_name.to_string(),
        })
        .or_default()
        .push(Package {
            name: line.to_string(),
        });
}

fn clean_pkg_name(
    pkg: &str
) -> &str {
    pkg.split([' ', '-', '/'])
        .next()
        .unwrap_or("")
}

fn is_library(
    pkg: &str
) -> bool {
    pkg.starts_with("lib")
        || pkg.contains("-dev")
        || pkg.contains("-devel")
}

pub fn run_command(
    pm: &PackageManager,
    capability: config::package_manager::types::Capability,
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

    Ok(if stdout.trim().is_empty() { stderr } else { stdout })
}