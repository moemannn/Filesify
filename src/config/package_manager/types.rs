use super::config::*;

// ---------------- REGISTRY ----------------
pub const DISTRO_PACKAGE_MANAGERS: &[PackageManager] = &[
    APT,
    DNF,
    YUM,
    PACMAN,
    ZYPPER,
    EMERGE,
    NIX,
];

pub const USER_PACKAGE_MANAGERS: &[PackageManager] = &[
    SNAP,
    FLATPAK,
];

pub const LANGUAGE_PACKAGE_MANAGERS: &[PackageManager] = &[
    CARGO,
    NPM,
    PIP,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageManagerCategory {
    Distro,
    User,
    Language,
}

#[derive(Debug, Clone)]
pub struct PackageManager {
    pub name: &'static str,
    pub cmd: &'static [&'static str],
    pub category: PackageManagerCategory,
    pub commands: &'static [Command],
    pub detection_method: DetectionMethod,
}

#[derive(Debug)]
pub struct Command {
    pub capability: Capability,
    pub bin: &'static str,
    pub args: &'static [Args],
    pub description: &'static str,
    pub requires_sudo: bool,
}
#[derive(Debug, PartialEq)]
pub enum Capability {
    Install,
    Update,
    Refresh,
    Upgrade,
    UpgradeAll,
    Info,
    Search,
    List,
    Version,
    Alias(&'static str),
}

#[derive(Debug, Clone)]
pub enum DetectionMethod {
    AbsolutePath,
    Version,
    Daemon,
}

#[derive(Debug)]
pub enum Args {
    Static(&'static str),
    PackageName,
    Version,
    Input(&'static str),
    Flag(&'static str, &'static str),
    Many(&'static [Args]),
}