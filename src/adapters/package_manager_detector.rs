use crate::config::package_managers::*;
use crate::adapters::package_manager_validation::{PackageManagerResult, package_manager_check};
use crate::app::state::AppState;

fn detect_all_package_managers() -> Vec<PackageManagerResult> {
    DISTRO_PACKAGE_MANAGERS
        .iter()
        .chain(USER_PACKAGE_MANAGERS.iter())
        .chain(LANGUAGE_PACKAGE_MANAGERS.iter())
        .chain(TOOLCHAIN_PACKAGE_MANAGERS.iter())
        .map(|pm| package_manager_check(pm))
        .collect()
}

pub fn update_package_managers(state: &mut AppState) {
    let results = detect_all_package_managers();

    state.set_package_manager_result(results);
}