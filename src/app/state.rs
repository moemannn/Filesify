use std::collections::HashMap;
use crate::adapters::PackageManagerResult;
use crate::config::package_manager::*;
use crate::config::package_manager::package::r#type::*;
use crate::config::package_manager::PackageManagerCategory::Distro;

#[derive(Debug)]
pub struct AppState {
    pub selected_package_manager: Option<PackageManager>,
    pub selected_package: Option<Package>,
    pub selected_category: Option<PackageManagerCategory>,

    pub detected_package_managers: Vec<PackageManagerResult>,
    pub detected_package_grouped: HashMap<String, HashMap<Groups, Vec<Package>>>
}

impl AppState {
    pub fn new() -> Self {
        Self {
            detected_package_managers: Vec::new(),
            selected_package_manager: None,
            selected_package: None,
            selected_category: Option::from(Distro),
            detected_package_grouped: HashMap::new(),
        }
    }

    pub fn set_package_manager_result(&mut self, managers: Vec<PackageManagerResult>) {
        self.detected_package_managers.clear();
        self.selected_package_manager = managers.iter()
            .find(|pm| pm.manager.category == PackageManagerCategory::Distro)
            .map(|x| x.manager.clone());;
        self.detected_package_managers.extend(managers);
    }

    pub fn set_package_manager(&mut self, pm: PackageManager) {
        self.selected_package_manager = Some(pm);
    }

    pub fn set_category(&mut self, c: PackageManagerCategory) {
        self.selected_category = Some(c);
    }

    pub fn set_package(&mut self, pm: Package) {
        self.selected_package = Some(pm);
    }

    pub fn clear_selected(&mut self) {
        self.selected_package_manager = None;
        self.selected_package = None;
        self.selected_category = None;
    }
}