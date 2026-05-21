use super::types::*;

// ===================== Distro PACKAGE MANAGERS =====================

// ---------------- APT ----------------
pub const APT: PackageManager = PackageManager {
    name: "APT",
    cmd: &["apt", "apt-get"],
    detection_method: DetectionMethod::AbsolutePath,
    category: PackageManagerCategory::Distro,
    commands: &[
        Command {
            capability: Capability::Install,
            bin: "apt",
            args: &[Args::Static("install"), Args::PackageName],
            description: "Install packages",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Info,
            bin: "apt",
            args: &[Args::Static("show"), Args::PackageName],
            description: "Show package info",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Refresh,
            bin: "apt",
            args: &[Args::Static("update")],
            description: "Refresh package lists",
            requires_sudo: true,
        },
        Command {
            capability: Capability::UpgradeAll,
            bin: "apt",
            args: &[Args::Static("upgrade"), Args::Static("-y")],
            description: "Upgrade all packages",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Search,
            bin: "apt",
            args: &[Args::Static("search"), Args::PackageName],
            description: "Search packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::List,
            bin: "apt",
            args: &[Args::Static("list"), Args::Static("--installed")],
            description: "List installed packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Version,
            bin: "apt",
            args: &[Args::Static("--version")],
            description: "Show APT version",
            requires_sudo: false,
        },
    ],
};

// ---------------- DNF ----------------
pub const DNF: PackageManager = PackageManager {
    name: "DNF",
    cmd: &["dnf"],
    detection_method: DetectionMethod::AbsolutePath,
    category: PackageManagerCategory::Distro,
    commands: &[
        Command {
            capability: Capability::Install,
            bin: "dnf",
            args: &[Args::Static("install"), Args::PackageName],
            description: "Install packages",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Info,
            bin: "dnf",
            args: &[Args::Static("info"), Args::PackageName],
            description: "Show packages info",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Refresh,
            bin: "dnf",
            args: &[Args::Static("makecache")],
            description: "Refresh metadata cache",
            requires_sudo: true,
        },
        Command {
            capability: Capability::UpgradeAll,
            bin: "dnf",
            args: &[Args::Static("upgrade"), Args::Static("-y")],
            description: "Upgrade all packages",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Search,
            bin: "dnf",
            args: &[Args::Static("search"), Args::PackageName],
            description: "Search packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::List,
            bin: "dnf",
            args: &[Args::Static("list"), Args::Static("installed")],
            description: "List installed packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Version,
            bin: "dnf",
            args: &[Args::Static("--version")],
            description: "Show DNF version",
            requires_sudo: false,
        },
    ],
};

// ---------------- YUM ----------------
pub const YUM: PackageManager = PackageManager {
    name: "YUM",
    cmd: &["yum"],
    detection_method: DetectionMethod::AbsolutePath,
    category: PackageManagerCategory::Distro,
    commands: &[
        Command {
            capability: Capability::Install,
            bin: "yum",
            args: &[Args::Static("install"), Args::PackageName],
            description: "Install packages",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Info,
            bin: "yum",
            args: &[Args::Static("info"), Args::PackageName],
            description: "Show packages info",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Refresh,
            bin: "yum",
            args: &[Args::Static("makecache")],
            description: "Refresh metadata",
            requires_sudo: true,
        },
        Command {
            capability: Capability::UpgradeAll,
            bin: "yum",
            args: &[Args::Static("update"), Args::Static("-y")],
            description: "Upgrade all packages",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Search,
            bin: "yum",
            args: &[Args::Static("search"), Args::PackageName],
            description: "Search packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::List,
            bin: "yum",
            args: &[Args::Static("list"), Args::Static("installed")],
            description: "List installed packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Version,
            bin: "yum",
            args: &[Args::Static("--version")],
            description: "Show YUM version",
            requires_sudo: false,
        },
    ],
};

// ---------------- PACMAN ----------------
pub const PACMAN: PackageManager = PackageManager {
    name: "Pacman",
    cmd: &["pacman"],
    detection_method: DetectionMethod::AbsolutePath,
    category: PackageManagerCategory::Distro,
    commands: &[
        Command {
            capability: Capability::Install,
            bin: "pacman",
            args: &[Args::Static("-S"), Args::PackageName],
            description: "Install packages",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Info,
            bin: "pacman",
            args: &[Args::Static("-Si"), Args::PackageName],
            description: "Show package information",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Refresh,
            bin: "pacman",
            args: &[Args::Static("-Sy")],
            description: "Refresh package database",
            requires_sudo: true,
        },
        Command {
            capability: Capability::UpgradeAll,
            bin: "pacman",
            args: &[Args::Static("-Syu"), Args::Static("--noconfirm")],
            description: "Upgrade Distro",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Search,
            bin: "pacman",
            args: &[Args::Static("-Ss"), Args::PackageName],
            description: "Search packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::List,
            bin: "pacman",
            args: &[Args::Static("-Q")],
            description: "List installed packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Version,
            bin: "pacman",
            args: &[Args::Static("--version")],
            description: "Show version",
            requires_sudo: false,
        },
    ],
};

// ---------------- ZYPPER ----------------
pub const ZYPPER: PackageManager = PackageManager {
    name: "Zypper",
    cmd: &["zypper"],
    detection_method: DetectionMethod::AbsolutePath,
    category: PackageManagerCategory::Distro,
    commands: &[
        Command {
            capability: Capability::Install,
            bin: "zypper",
            args: &[Args::Static("install"), Args::PackageName],
            description: "Install packages",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Info,
            bin: "zypper",
            args: &[Args::Static("info"), Args::PackageName],
            description: "Show package information",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Refresh,
            bin: "zypper",
            args: &[Args::Static("refresh")],
            description: "Refresh repositories",
            requires_sudo: true,
        },
        Command {
            capability: Capability::UpgradeAll,
            bin: "zypper",
            args: &[Args::Static("update"), Args::Static("-y")],
            description: "Upgrade all packages",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Search,
            bin: "zypper",
            args: &[Args::Static("search"), Args::PackageName],
            description: "Search packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::List,
            bin: "zypper",
            args: &[Args::Static("search"), Args::Static("--installed-only")],
            description: "List installed packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Version,
            bin: "zypper",
            args: &[Args::Static("--version")],
            description: "Show Zypper version",
            requires_sudo: false,
        },
    ],
};

// ---------------- EMERGE ----------------
pub const EMERGE: PackageManager = PackageManager {
    name: "Portage (emerge)",
    cmd: &["emerge"],
    detection_method: DetectionMethod::AbsolutePath,
    category: PackageManagerCategory::Distro,
    commands: &[
        Command {
            capability: Capability::Install,
            bin: "emerge",
            args: &[Args::Static("-a"), Args::PackageName],
            description: "Install packages",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Info,
            bin: "emerge",
            args: &[Args::Static("--info"), Args::PackageName],
            description: "Show package information",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Refresh,
            bin: "emerge",
            args: &[Args::Static("--sync")],
            description: "Sync repositories",
            requires_sudo: true,
        },
        Command {
            capability: Capability::UpgradeAll,
            bin: "emerge",
            args: &[Args::Static("-u"), Args::Static("world")],
            description: "Upgrade Distro",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Search,
            bin: "emerge",
            args: &[Args::Static("--search"), Args::PackageName],
            description: "Search packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::List,
            bin: "emerge",
            args: &[Args::Static("-qa")],
            description: "List installed packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Version,
            bin: "emerge",
            args: &[Args::Static("--version")],
            description: "Show version",
            requires_sudo: false,
        },
    ],
};

// ---------------- NIX ----------------
pub const NIX: PackageManager = PackageManager {
    name: "Nix",
    cmd: &["nix", "nix-env", "nix-channel"],
    detection_method: DetectionMethod::AbsolutePath,
    category: PackageManagerCategory::Distro,
    commands: &[
        Command {
            capability: Capability::Install,
            bin: "nix-env",
            args: &[Args::Static("-i"), Args::PackageName],
            description: "Install packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Info,
            bin: "nix-env",
            args: &[Args::Static("-qa"), Args::PackageName],
            description: "Show package information",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Refresh,
            bin: "nix-channel",
            args: &[Args::Static("--update")],
            description: "Update channels",
            requires_sudo: false,
        },
        Command {
            capability: Capability::UpgradeAll,
            bin: "nix-env",
            args: &[Args::Static("-u")],
            description: "Upgrade packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Search,
            bin: "nix-env",
            args: &[Args::Static("-qa"), Args::PackageName],
            description: "Search packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::List,
            bin: "nix-env",
            args: &[Args::Static("-q")],
            description: "List installed packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Version,
            bin: "nix",
            args: &[Args::Static("--version")],
            description: "Show version",
            requires_sudo: false,
        },
    ],
};

// ===================== USER PACKAGE MANAGERS =====================

pub const SNAP: PackageManager = PackageManager {
    name: "Snap",
    cmd: &["snap"],
    detection_method: DetectionMethod::Version,
    category: PackageManagerCategory::User,
    commands: &[
        Command {
            capability: Capability::Install,
            bin: "snap",
            args: &[Args::Static("install"), Args::PackageName],
            description: "Install packages",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Refresh,
            bin: "snap",
            args: &[Args::Static("refresh")],
            description: "Refresh packages",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Search,
            bin: "snap",
            args: &[Args::Static("find"), Args::PackageName],
            description: "Search packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::List,
            bin: "snap",
            args: &[Args::Static("list")],
            description: "List installed packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Info,
            bin: "snap",
            args: &[Args::Static("info"), Args::PackageName],
            description: "Show package information",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Version,
            bin: "snap",
            args: &[Args::Static("--version")],
            description: "Show version",
            requires_sudo: false,
        },
        Command {
            capability: Capability::UpgradeAll,
            bin: "snap",
            args: &[Args::Static("refresh")],
            description: "Upgrade all snaps",
            requires_sudo: true,
        },
    ],
};

pub const FLATPAK: PackageManager = PackageManager {
    name: "Flatpak",
    cmd: &["flatpak"],
    detection_method: DetectionMethod::Version,
    category: PackageManagerCategory::User,
    commands: &[
        Command {
            capability: Capability::Install,
            bin: "flatpak",
            args: &[Args::Static("install"), Args::PackageName],
            description: "Install apps",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Info,
            bin: "flatpak",
            args: &[Args::Static("info"), Args::PackageName],
            description: "Show package information",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Refresh,
            bin: "flatpak",
            args: &[Args::Static("update")],
            description: "Refresh apps",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Search,
            bin: "flatpak",
            args: &[Args::Static("search"), Args::PackageName],
            description: "Search apps",
            requires_sudo: false,
        },
        Command {
            capability: Capability::List,
            bin: "flatpak",
            args: &[Args::Static("list")],
            description: "List installed apps",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Version,
            bin: "flatpak",
            args: &[Args::Static("--version")],
            description: "Show version",
            requires_sudo: false,
        },
        Command {
            capability: Capability::UpgradeAll,
            bin: "flatpak",
            args: &[Args::Static("update")],
            description: "Upgrade all apps",
            requires_sudo: false,
        },
    ],
};

// ===================== LANGUAGE PACKAGE MANAGERS =====================

pub const CARGO: PackageManager = PackageManager {
    name: "Cargo",
    cmd: &["cargo"],
    detection_method: DetectionMethod::AbsolutePath,
    category: PackageManagerCategory::Language,
    commands: &[
        Command {
            capability: Capability::Install,
            bin: "cargo",
            args: &[Args::Static("install"), Args::PackageName],
            description: "Install crates",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Info,
            bin: "cargo",
            args: &[Args::Static("search"), Args::PackageName],
            description: "Show crate info",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Refresh,
            bin: "cargo",
            args: &[Args::Static("update")],
            description: "Update dependencies",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Search,
            bin: "cargo",
            args: &[Args::Static("search"), Args::PackageName],
            description: "Search crates",
            requires_sudo: false,
        },
        Command {
            capability: Capability::List,
            bin: "cargo",
            args: &[Args::Static("install"), Args::Static("--list")],
            description: "List installed crates",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Version,
            bin: "cargo",
            args: &[Args::Static("--version")],
            description: "Show version",
            requires_sudo: false,
        },
        Command {
            capability: Capability::UpgradeAll,
            bin: "cargo",
            args: &[Args::Static("update")],
            description: "Upgrade all crates",
            requires_sudo: false,
        },
    ],
};

pub const NPM: PackageManager = PackageManager {
    name: "NPM",
    cmd: &["npm"],
    detection_method: DetectionMethod::Version,
    category: PackageManagerCategory::Language,
    commands: &[
        Command {
            capability: Capability::Install,
            bin: "npm",
            args: &[Args::Static("install"), Args::PackageName],
            description: "Install packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Info,
            bin: "npm",
            args: &[Args::Static("view"), Args::PackageName],
            description: "Show package info",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Refresh,
            bin: "npm",
            args: &[Args::Static("update")],
            description: "Refresh packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Search,
            bin: "npm",
            args: &[Args::Static("search"), Args::PackageName],
            description: "Search packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::List,
            bin: "npm",
            args: &[Args::Static("list"), Args::Static("--depth=0")],
            description: "List installed packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Version,
            bin: "npm",
            args: &[Args::Static("--version")],
            description: "Show version",
            requires_sudo: false,
        },
        Command {
            capability: Capability::UpgradeAll,
            bin: "npm",
            args: &[Args::Static("update"), Args::Static("-g")],
            description: "Upgrade global packages",
            requires_sudo: false,
        },
    ],
};

pub const PIP: PackageManager = PackageManager {
    name: "PIP",
    cmd: &["pip", "pip3"],
    detection_method: DetectionMethod::Version,
    category: PackageManagerCategory::Language,
    commands: &[
        Command {
            capability: Capability::Install,
            bin: "pip3",
            args: &[Args::Static("install"), Args::PackageName],
            description: "Install packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Info,
            bin: "pip3",
            args: &[Args::Static("show"), Args::PackageName],
            description: "Show package info",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Refresh,
            bin: "pip3",
            args: &[Args::Static("list"), Args::Static("--outdated")],
            description: "Check outdated packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Search,
            bin: "pip3",
            args: &[Args::Static("index"), Args::Static("versions"), Args::PackageName],
            description: "Search packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::List,
            bin: "pip3",
            args: &[Args::Static("list")],
            description: "List installed packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Version,
            bin: "pip3",
            args: &[Args::Static("--version")],
            description: "Show version",
            requires_sudo: false,
        },
        Command {
            capability: Capability::UpgradeAll,
            bin: "pip3",
            args: &[Args::Static("list"), Args::Static("--outdated")],
            description: "List outdated packages (manual upgrade needed)",
            requires_sudo: false,
        },
    ],
};