mod config;
mod adapters;
mod app;

use adapters::*;
use app::*;

fn main() {
    let mut state = AppState::new();
    update_package_managers(&mut state);

    print_group("Installed package managers:", &state, PackageManagerStatus::Installed);
    print_group("Not installed package managers:", &state, PackageManagerStatus::NotInstalled);
    print_group("Invalid package managers:", &state, PackageManagerStatus::Invalid);
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