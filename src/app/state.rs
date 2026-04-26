use crate::adapters::PackageManagerResult;
use crate::config::package_managers::*;
use crate::config::*;

pub struct AppState {
    pub detection_result: Vec<PackageManagerResult>,
    pub selected_package_manager: Option<&'static PackageManager>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            detection_result: Vec::new(),
            selected_package_manager: None,
        }
    }

    pub fn set_package_manager_result(&mut self, managers: Vec<PackageManagerResult>) {
        self.detection_result.clear();
        self.detection_result.extend(managers);
    }

    pub fn select_package_manager(&mut self, pm: &'static PackageManager) {
        self.selected_package_manager = Some(pm);
    }

    pub fn clear_selected_package_manager(&mut self) {
        self.selected_package_manager = None;
    }
}