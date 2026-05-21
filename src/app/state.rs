use std::collections::HashMap;
use crate::adapters::PackageManagerResult;
use crate::config::package_managers::*;
use crate::config::package_managers::config::{APT, CARGO, DNF, EMERGE, FLATPAK, NIX, NPM, PACMAN, PIP, SNAP, YUM, ZYPPER};

#[derive(Debug)]
pub struct AppState {
    pub detection_package_managers: Vec<PackageManagerResult>,
    pub selected_package_manager: Option<PackageManager>,
    pub packaged_grouped_by_manager: HashMap<String, Vec<Package>>,
}

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub group: String,
}


impl AppState {
    pub fn new() -> Self {
        Self {
            detection_package_managers: Vec::new(),
            selected_package_manager: None,
            packaged_grouped_by_manager: HashMap::new(),
        }
    }

    pub fn set_package_manager_result(&mut self, managers: Vec<PackageManagerResult>) {
        self.detection_package_managers.clear();
        self.detection_package_managers.extend(managers);
    }

    pub fn select_package_manager(&mut self, pm: PackageManager) {
        self.selected_package_manager = Some(pm);
    }

    pub fn clear_selected_package_manager(&mut self) {
        self.selected_package_manager = None;
    }
}