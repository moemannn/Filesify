use crate::config::package_managers::PackageManager;

pub struct AppState {
    pub available_package_managers: Vec<&'static PackageManager>,
    pub selected_package_manager: Option<&'static PackageManager>,
    // pub selected_package: Option<&'static PackageManager>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            available_package_managers: Vec::new(),
            selected_package_manager: None,
            // selected_package: None,
        }
    }

    pub fn set_available(&mut self, managers: Vec<&'static PackageManager>) {
        self.available_package_managers.clear();
        self.available_package_managers.extend(managers);
    }

    pub fn select_package_manager(&mut self, pm: &'static PackageManager) {
        self.selected_package_manager = Some(pm);
    }

    pub fn clear_selected_package_manager(&mut self) {
        self.selected_package_manager = None;
    }
}