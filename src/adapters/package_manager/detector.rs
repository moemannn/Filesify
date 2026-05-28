use crate::config::package_manager::*;
use super::validator::{PackageManagerResult, absolute_path, version_check, daemon_check};

pub fn detect_valid_package_managers() -> Vec<PackageManagerResult> {
    DISTRO_PACKAGE_MANAGERS
        .iter()
        .chain(USER_PACKAGE_MANAGERS.iter())
        .chain(LANGUAGE_PACKAGE_MANAGERS.iter())
        .map(|pm| package_manager_check(pm))
        .collect()
}

pub fn package_manager_check(pm: &'static PackageManager) -> PackageManagerResult {

    match pm.detection_method {
        DetectionMethod::AbsolutePath => absolute_path(pm),
        DetectionMethod::Version => version_check(pm),
        DetectionMethod::Daemon => daemon_check(pm),
    }
}