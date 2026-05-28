use which::which;
use crate::config::package_manager::*;

#[derive(Debug, Clone, Copy)]
pub struct PackageManagerResult {
    pub manager: &'static PackageManager,
    pub status: PackageManagerStatus,
}

pub fn absolute_path(pm: &'static PackageManager) -> PackageManagerResult {
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

pub fn version_check(pm: &'static PackageManager) -> PackageManagerResult {
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

pub fn daemon_check(pm: &'static PackageManager) -> PackageManagerResult {
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
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PackageManagerStatus {
    NotInstalled,
    Installed,
    Invalid,
}