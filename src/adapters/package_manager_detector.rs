use which::which;
use crate::config::package_managers::*;

pub fn detect_available_package_managers() -> Vec<&'static PackageManager> {
    let mut available = Vec::new();

    for pm in DISTRO_PACKAGE_MANAGERS
        .iter()
        .chain(USER_PACKAGE_MANAGERS.iter())
        .chain(LANGUAGE_PACKAGE_MANAGERS.iter())
        .chain(TOOLCHAIN_PACKAGE_MANAGERS.iter())
    {
        if pm.cmd.iter().any(|cmd| which(cmd).is_ok()) {
            available.push(pm);
        }
    }
    available
}