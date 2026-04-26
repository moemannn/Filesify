use which::which;
use crate::config::package_managers::*;

pub struct PackageManagerResult {
    pub manager: &'static PackageManager,
    pub status: PackageManagerStatus,
}

pub fn package_manager_check(pm: &'static PackageManager) -> PackageManagerResult {

    match pm.check_type {
        CheckType::AbsolutePath => absolute_path(pm),
        CheckType::Version => version_check(pm),
        CheckType::Daemon => daemon_check(pm),
    }
}
fn absolute_path(pm: &'static PackageManager) -> PackageManagerResult {
    let exists = pm.cmd.iter().any(|cmd| which(cmd).is_ok());

    PackageManagerResult {
        manager: pm,
        status: if exists {
            PackageManagerStatus::Installed
        } else {
            PackageManagerStatus::NotInstalled
        },
    }
}

fn version_check(pm: &'static PackageManager) -> PackageManagerResult {
    let version_ok = true; // TODO: real version check

    PackageManagerResult {
        manager: pm,
        status: if version_ok {
            PackageManagerStatus::Installed
        } else {
            PackageManagerStatus::Invalid
        },
    }
}

fn daemon_check(pm: &'static PackageManager) -> PackageManagerResult {
    let daemon_running = true; // TODO: real check

    PackageManagerResult {
        manager: pm,
        status: if daemon_running {
            PackageManagerStatus::Installed
        } else {
            PackageManagerStatus::Invalid
        },
    }
}
#[derive(PartialEq)]
pub enum PackageManagerStatus {
    NotInstalled,
    Installed,
    Invalid,
}