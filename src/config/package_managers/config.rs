use super::types::*;

// ---------------- APT ----------------
pub const APT: PackageManager = PackageManager {
    name: "APT",
    cmd: &["apt", "apt-get"],
    detection_method: DetectionMethod::AbsolutePath,
    category: PackageManagerCategory::System,
    commands: &[
        Command {
            capability: Capability::Install,
            bin: "apt",
            args: &[
                Args::Static("install"),
                Args::PackageName,
            ],
            description: "Install packages",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Update,
            bin: "apt",
            args: &[Args::Static("update")],
            description: "Update package lists",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Upgrade,
            bin: "apt",
            args: &[Args::Static("upgrade")],
            description: "Upgrade packages",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Search,
            bin: "apt",
            args: &[
                Args::Static("search"),
                Args::PackageName,
            ],
            description: "Search packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::List,
            bin: "apt",
            args: &[
                Args::Static("list"),
                Args::Static("--installed"),
            ],
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
    category: PackageManagerCategory::System,
    commands: &[
        Command {
            capability: Capability::Install,
            bin: "dnf",
            args: &[
                Args::Static("install"),
                Args::PackageName,
            ],
            description: "Install packages",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Update,
            bin: "dnf",
            args: &[Args::Static("check-update")],
            description: "Check for updates",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Upgrade,
            bin: "dnf",
            args: &[Args::Static("upgrade")],
            description: "Upgrade packages",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Search,
            bin: "dnf",
            args: &[
                Args::Static("search"),
                Args::PackageName,
            ],
            description: "Search packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::List,
            bin: "dnf",
            args: &[
                Args::Static("list"),
                Args::Static("installed"),
            ],
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
    category: PackageManagerCategory::System,
    commands: &[
        Command {
            capability: Capability::Install,
            bin: "yum",
            args: &[
                Args::Static("install"),
                Args::PackageName,
            ],
            description: "Install packages",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Update,
            bin: "yum",
            args: &[Args::Static("check-update")],
            description: "Check for updates",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Upgrade,
            bin: "yum",
            args: &[Args::Static("update")],
            description: "Upgrade packages",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Search,
            bin: "yum",
            args: &[
                Args::Static("search"),
                Args::PackageName,
            ],
            description: "Search packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::List,
            bin: "yum",
            args: &[
                Args::Static("list"),
                Args::Static("installed"),
            ],
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
    category: PackageManagerCategory::System,
    commands: &[
        Command {
            capability: Capability::Install,
            bin: "pacman",
            args: &[
                Args::Static("-S"),
                Args::PackageName,
            ],
            description: "Install packages",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Update,
            bin: "pacman",
            args: &[Args::Static("-Sy")],
            description: "Refresh package database",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Upgrade,
            bin: "pacman",
            args: &[Args::Static("-Syu")],
            description: "Upgrade system",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Search,
            bin: "pacman",
            args: &[
                Args::Static("-Ss"),
                Args::PackageName,
            ],
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
            description: "Show Pacman version",
            requires_sudo: false,
        },
    ],
};

// ---------------- ZYPPER ----------------
pub const ZYPPER: PackageManager = PackageManager {
    name: "Zypper",
    cmd: &["zypper"],
    detection_method: DetectionMethod::AbsolutePath,
    category: PackageManagerCategory::System,
    commands: &[
        Command {
            capability: Capability::Install,
            bin: "zypper",
            args: &[
                Args::Static("install"),
                Args::PackageName,
            ],
            description: "Install packages",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Update,
            bin: "zypper",
            args: &[Args::Static("refresh")],
            description: "Refresh repositories",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Upgrade,
            bin: "zypper",
            args: &[Args::Static("update")],
            description: "Upgrade packages",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Search,
            bin: "zypper",
            args: &[
                Args::Static("search"),
                Args::PackageName,
            ],
            description: "Search packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::List,
            bin: "zypper",
            args: &[
                Args::Static("search"),
                Args::Static("--installed-only"),
            ],
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
    category: PackageManagerCategory::System,
    commands: &[
        Command {
            capability: Capability::Install,
            bin: "emerge",
            args: &[Args::Static("-a"), Args::PackageName],
            description: "Install packages",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Update,
            bin: "emerge",
            args: &[Args::Static("--sync")],
            description: "Sync repositories",
            requires_sudo: true,
        },
        Command {
            capability: Capability::Upgrade,
            bin: "emerge",
            args: &[Args::Static("-u"), Args::Static("world")],
            description: "Upgrade system",
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
    category: PackageManagerCategory::System,
    commands: &[
        Command {
            capability: Capability::Install,
            bin: "nix-env",
            args: &[Args::Static("-i"), Args::PackageName],
            description: "Install packages",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Update,
            bin: "nix-channel",
            args: &[Args::Static("--update")],
            description: "Update channels",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Upgrade,
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

// ---------------- SNAP ----------------
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
            capability: Capability::Update,
            bin: "snap",
            args: &[Args::Static("refresh")],
            description: "Update packages",
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
            capability: Capability::Version,
            bin: "snap",
            args: &[Args::Static("--version")],
            description: "Show version",
            requires_sudo: false,
        },
    ],
};

// ---------------- FLATPAK ----------------
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
            capability: Capability::Update,
            bin: "flatpak",
            args: &[Args::Static("update")],
            description: "Update apps",
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
    ],
};

// ---------------- CARGO ----------------
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
            capability: Capability::Update,
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
    ],
};

// ---------------- NPM ----------------
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
            capability: Capability::Update,
            bin: "npm",
            args: &[Args::Static("update")],
            description: "Update packages",
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
    ],
};

// ---------------- PIP ----------------
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
            capability: Capability::Update,
            bin: "pip3",
            args: &[Args::Static("install"), Args::Static("--upgrade"), Args::PackageName],
            description: "Upgrade packages",
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
    ],
};

// ---------------- RUSTUP ----------------

pub const RUSTUP: PackageManager = PackageManager {
    name: "Rustup",
    cmd: &["rustup"],
    detection_method: DetectionMethod::AbsolutePath,
    category: PackageManagerCategory::Toolchain,
    commands: &[
        Command {
            capability: Capability::Install,
            bin: "rustup",
            args: &[Args::Static("toolchain"), Args::Static("install"), Args::PackageName],
            description: "Install toolchain",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Update,
            bin: "rustup",
            args: &[Args::Static("update")],
            description: "Update toolchains",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Search,
            bin: "rustup",
            args: &[Args::Static("toolchain"), Args::Static("list")],
            description: "List toolchains",
            requires_sudo: false,
        },
        Command {
            capability: Capability::List,
            bin: "rustup",
            args: &[Args::Static("toolchain"), Args::Static("list")],
            description: "List installed toolchains",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Version,
            bin: "rustup",
            args: &[Args::Static("--version")],
            description: "Show version",
            requires_sudo: false,
        },
    ],
};

// ---------------- PYENV ----------------
pub const PYENV: PackageManager = PackageManager {
    name: "Pyenv",
    cmd: &["pyenv"],
    detection_method: DetectionMethod::AbsolutePath,
    category: PackageManagerCategory::Toolchain,
    commands: &[
        Command {
            capability: Capability::Install,
            bin: "pyenv",
            args: &[Args::Static("install"), Args::PackageName],
            description: "Install Python version",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Update,
            bin: "pyenv",
            args: &[Args::Static("update")],
            description: "Update pyenv",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Search,
            bin: "pyenv",
            args: &[Args::Static("install"), Args::Static("--list")],
            description: "List versions",
            requires_sudo: false,
        },
        Command {
            capability: Capability::List,
            bin: "pyenv",
            args: &[Args::Static("versions")],
            description: "List installed versions",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Version,
            bin: "pyenv",
            args: &[Args::Static("--version")],
            description: "Show version",
            requires_sudo: false,
        },
    ],
};

// ---------------- NVM ----------------
pub const NVM: PackageManager = PackageManager {
    name: "NVM",
    cmd: &["nvm"],
    detection_method: DetectionMethod::AbsolutePath,
    category: PackageManagerCategory::Toolchain,
    commands: &[
        Command {
            capability: Capability::Install,
            bin: "nvm",
            args: &[Args::Static("install"), Args::PackageName],
            description: "Install Node version",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Update,
            bin: "nvm",
            args: &[Args::Static("install"), Args::Static("node")],
            description: "Update Node",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Search,
            bin: "nvm",
            args: &[Args::Static("ls-remote")],
            description: "List versions",
            requires_sudo: false,
        },
        Command {
            capability: Capability::List,
            bin: "nvm",
            args: &[Args::Static("ls")],
            description: "List installed versions",
            requires_sudo: false,
        },
        Command {
            capability: Capability::Version,
            bin: "bash",
            args: &[Args::Static("-c"), Args::Static("nvm --version")],
            description: "Show version",
            requires_sudo: false,
        },
    ],
};