use super::detector::detect_valid_package_managers;
use crate::app::state::AppState;
pub fn update_package_managers(state: &mut AppState) {
    let results = detect_valid_package_managers();

    state.set_package_manager_result(results);
}