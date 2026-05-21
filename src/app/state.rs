use std::collections::HashMap;
use crate::adapters::PackageManagerResult;
use crate::config::package_managers::*;
use crate::config::package_managers::config::{APT, CARGO, DNF, EMERGE, FLATPAK, NIX, NPM, PACMAN, PIP, SNAP, YUM, ZYPPER};

#[derive(Debug)]
pub struct AppState {
    pub detection_package_managers: Vec<PackageManagerResult>,
    pub selected_package_manager: Option<PackageManager>,
    pub selected_package: Option<Package>,
    pub packaged_grouped_by_manager: HashMap<String, HashMap<Groups, Vec<Package>>>
}

#[derive(Debug)]
pub struct Package {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Groups {
    pub name: String,
}


impl AppState {
    pub fn new() -> Self {
        Self {
            detection_package_managers: Vec::new(),
            selected_package_manager: None,
            selected_package: None,
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

    pub fn select_package(&mut self, pm: Package) {
        self.selected_package = Some(pm);
    }

    pub fn clear_selected(&mut self) {
        self.selected_package_manager = None;
        self.selected_package = None;
    }
}