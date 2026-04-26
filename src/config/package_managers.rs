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

pub const TOOLCHAIN_PACKAGE_MANAGERS: &[PackageManager] = &[
    RUSTUP, 
    PYENV, 
    NVM,
];

pub struct Command {
    pub cmd_type: CommandType,
    pub bin: &'static str,
    pub args: &'static [&'static str],
    pub description: &'static str,
    pub requires_sudo: bool,
}

pub struct PackageManager {
    pub name: &'static str,
    pub cmd: &'static [&'static str],
    pub category: PackageManagerCategory,
    pub commands: &'static [Command],
}

pub enum PackageManagerCategory {
    System,
    User,
    Language,
    Toolchain,
}

pub enum CommandType {
    Install,
    Update,
    Upgrade,
    Search,
    List,
    Alias(&'static str),
}

// ---------------- HELPER FUNCTION ----------------
impl CommandType {
    pub fn as_str(&self) -> &'static str {
        match self {
            CommandType::Install => "install",
            CommandType::Update => "update",
            CommandType::Upgrade => "upgrade",
            CommandType::Search => "search",
            CommandType::List => "list",
            CommandType::Alias(s) => s,
        }
    }
}

// ---------------- APT ----------------
pub const APT: PackageManager = PackageManager {
    name: "APT",
    cmd: &["apt", "apt-get"],
    category: PackageManagerCategory::System,
    commands: &[
        Command { cmd_type: CommandType::Install, bin: "apt", args: &["install"], description: "Install packages", requires_sudo: true },
        Command { cmd_type: CommandType::Update, bin: "apt", args: &["update"], description: "Update package lists", requires_sudo: true },
        Command { cmd_type: CommandType::Upgrade, bin: "apt", args: &["upgrade"], description: "Upgrade packages", requires_sudo: true },
        Command { cmd_type: CommandType::Search, bin: "apt", args: &["search"], description: "Search packages", requires_sudo: false },
        Command { cmd_type: CommandType::List, bin: "apt", args: &["list", "--installed"], description: "List installed packages", requires_sudo: false },
    ],
};

// ---------------- DNF ----------------
pub const DNF: PackageManager = PackageManager {
    name: "DNF",
    cmd: &["dnf"],
    category: PackageManagerCategory::System,
    commands: &[
        Command { cmd_type: CommandType::Install, bin: "dnf", args: &["install"], description: "Install packages", requires_sudo: true },
        Command { cmd_type: CommandType::Update, bin: "dnf", args: &["check-update"], description: "Check for updates", requires_sudo: false },
        Command { cmd_type: CommandType::Upgrade, bin: "dnf", args: &["upgrade"], description: "Upgrade packages", requires_sudo: true },
        Command { cmd_type: CommandType::Search, bin: "dnf", args: &["search"], description: "Search packages", requires_sudo: false },
        Command { cmd_type: CommandType::List, bin: "dnf", args: &["list", "installed"], description: "List installed packages", requires_sudo: false },
    ],
};
// ---------------- YUM ----------------
pub const YUM: PackageManager = PackageManager {
    name: "YUM",
    cmd: &["yum"],
    category: PackageManagerCategory::System,
    commands: &[
        Command { cmd_type: CommandType::Install, bin: "yum", args: &["install"], description: "Install packages", requires_sudo: true },
        Command { cmd_type: CommandType::Update, bin: "yum", args: &["check-update"], description: "Check for updates", requires_sudo: false },
        Command { cmd_type: CommandType::Upgrade, bin: "yum", args: &["update"], description: "Upgrade packages", requires_sudo: true },
        Command { cmd_type: CommandType::Search, bin: "yum", args: &["search"], description: "Search packages", requires_sudo: false },
        Command { cmd_type: CommandType::List, bin: "yum", args: &["list", "installed"], description: "List installed packages", requires_sudo: false },
    ],
};

// ---------------- PACMAN ----------------
pub const PACMAN: PackageManager = PackageManager {
    name: "Pacman",
    cmd: &["pacman"],
    category: PackageManagerCategory::System,
    commands: &[
        Command { cmd_type: CommandType::Install, bin: "pacman", args: &["-S"], description: "Install packages", requires_sudo: true },
        Command { cmd_type: CommandType::Update, bin: "pacman", args: &["-Sy"], description: "Refresh package database", requires_sudo: true },
        Command { cmd_type: CommandType::Upgrade, bin: "pacman", args: &["-Syu"], description: "Upgrade system", requires_sudo: true },
        Command { cmd_type: CommandType::Search, bin: "pacman", args: &["-Ss"], description: "Search packages", requires_sudo: false },
        Command { cmd_type: CommandType::List, bin: "pacman", args: &["-Q"], description: "List installed packages", requires_sudo: false },
    ],
};

// ---------------- ZYPPER ----------------
pub const ZYPPER: PackageManager = PackageManager {
    name: "Zypper",
    cmd: &["zypper"],
    category: PackageManagerCategory::System,
    commands: &[
        Command { cmd_type: CommandType::Install, bin: "zypper", args: &["install"], description: "Install packages", requires_sudo: true },
        Command { cmd_type: CommandType::Update, bin: "zypper", args: &["refresh"], description: "Refresh repositories", requires_sudo: true },
        Command { cmd_type: CommandType::Upgrade, bin: "zypper", args: &["update"], description: "Upgrade packages", requires_sudo: true },
        Command { cmd_type: CommandType::Search, bin: "zypper", args: &["search"], description: "Search packages", requires_sudo: false },
        Command { cmd_type: CommandType::List, bin: "zypper", args: &["search", "--installed-only"], description: "List installed packages", requires_sudo: false },
    ],
};

// ---------------- EMERGE ----------------
pub const EMERGE: PackageManager = PackageManager {
    name: "Portage (emerge)",
    cmd: &["emerge"],
    category: PackageManagerCategory::System,
    commands: &[
        Command { cmd_type: CommandType::Install, bin: "emerge", args: &["-a"], description: "Install packages", requires_sudo: true },
        Command { cmd_type: CommandType::Update, bin: "emerge", args: &["--sync"], description: "Sync repositories", requires_sudo: true },
        Command { cmd_type: CommandType::Upgrade, bin: "emerge", args: &["-u", "world"], description: "Upgrade system", requires_sudo: true },
        Command { cmd_type: CommandType::Search, bin: "emerge", args: &["--search"], description: "Search packages", requires_sudo: false },
        Command { cmd_type: CommandType::List, bin: "emerge", args: &["--search"], description: "List packages (via search)", requires_sudo: false },
    ],
};

// ---------------- NIX ----------------
pub const NIX: PackageManager = PackageManager {
    name: "Nix",
    cmd: &["nix", "nix-env", "nix-channel"],
    category: PackageManagerCategory::System,
    commands: &[
        Command { cmd_type: CommandType::Install, bin: "nix-env", args: &["-i"], description: "Install packages", requires_sudo: false },
        Command { cmd_type: CommandType::Update, bin: "nix-channel", args: &["--update"], description: "Update channels", requires_sudo: false },
        Command { cmd_type: CommandType::Upgrade, bin: "nix-env", args: &["-u"], description: "Upgrade packages", requires_sudo: false },
        Command { cmd_type: CommandType::Search, bin: "nix-env", args: &["-qa"], description: "Search packages", requires_sudo: false },
        Command { cmd_type: CommandType::List, bin: "nix-env", args: &["-q"], description: "List installed packages", requires_sudo: false },
    ],
};

// ---------------- SNAP ----------------
pub const SNAP: PackageManager = PackageManager {
    name: "Snap",
    cmd: &["snap"],
    category: PackageManagerCategory::User,
    commands: &[
        Command { cmd_type: CommandType::Install, bin: "snap", args: &["install"], description: "Install packages", requires_sudo: true },
        Command { cmd_type: CommandType::Update, bin: "snap", args: &["refresh"], description: "Update packages", requires_sudo: true },
        Command { cmd_type: CommandType::Search, bin: "snap", args: &["find"], description: "Search packages", requires_sudo: false },
        Command { cmd_type: CommandType::List, bin: "snap", args: &["list"], description: "List installed packages", requires_sudo: false },
    ],
};

// ---------------- FLATPAK ----------------
pub const FLATPAK: PackageManager = PackageManager {
    name: "Flatpak",
    cmd: &["flatpak"],
    category: PackageManagerCategory::User,
    commands: &[
        Command { cmd_type: CommandType::Install, bin: "flatpak", args: &["install"], description: "Install packages", requires_sudo: false },
        Command { cmd_type: CommandType::Update, bin: "flatpak", args: &["update"], description: "Update packages", requires_sudo: false },
        Command { cmd_type: CommandType::Search, bin: "flatpak", args: &["search"], description: "Search packages", requires_sudo: false },
        Command { cmd_type: CommandType::List, bin: "flatpak", args: &["list"], description: "List installed packages", requires_sudo: false },
    ],
};

// ---------------- CARGO ----------------
pub const CARGO: PackageManager = PackageManager {
    name: "Cargo",
    cmd: &["cargo"],
    category: PackageManagerCategory::Language,
    commands: &[
        Command {cmd_type: CommandType::Install, bin: "cargo", args: &["install"], description: "Install Rust binaries", requires_sudo: false, },
        Command {cmd_type: CommandType::Update, bin: "cargo", args: &["update"], description: "Update dependencies", requires_sudo: false, }, 
        Command {cmd_type: CommandType::Search, bin: "cargo", args: &["search"], description: "Search crates", requires_sudo: false, }, 
        Command {cmd_type: CommandType::List, bin: "cargo", args: &["install", "--list"], description: "List installed binaries", requires_sudo: false, },
    ],
};

// ---------------- NPM ----------------
pub const NPM: PackageManager = PackageManager {
    name: "NPM",
    cmd: &["npm"],
    category: PackageManagerCategory::Language,
    commands: &[
        Command {cmd_type: CommandType::Install, bin: "npm", args: &["install"], description: "Install dependencies", requires_sudo: false, }, 
        Command {cmd_type: CommandType::Update, bin: "npm", args: &["update"], description: "Update dependencies", requires_sudo: false, }, 
        Command {cmd_type: CommandType::Search, bin: "npm", args: &["search"], description: "Search packages", requires_sudo: false, }, 
        Command {cmd_type: CommandType::List, bin: "npm", args: &["list", "--depth=0"], description: "List installed packages", requires_sudo: false, },
    ],
};

// ---------------- PIP ----------------
pub const PIP: PackageManager = PackageManager {
    name: "PIP",
    cmd: &["pip", "pip3"],
    category: PackageManagerCategory::Language,
    commands: &[
        Command {cmd_type: CommandType::Install, bin: "pip3", args: &["install"], description: "Install Python packages", requires_sudo: false, }, 
        Command {cmd_type: CommandType::Update, bin: "pip3", args: &["install", "--upgrade"], description: "Update Python packages", requires_sudo: false, }, 
        Command {cmd_type: CommandType::Search, bin: "pip3", args: &["index", "versions"], description: "Show available package versions (modern replacement for search)", requires_sudo: false, }, 
        Command {cmd_type: CommandType::List, bin: "pip3", args: &["list"], description: "List installed packages", requires_sudo: false, },
    ],
};

// ---------------- RUSTUP ----------------
pub const RUSTUP: PackageManager = PackageManager {
    name: "Rustup",
    cmd: &["rustup"],
    category: PackageManagerCategory::Toolchain,
    commands: &[
        Command {cmd_type: CommandType::Install, bin: "rustup", args: &["toolchain", "install"], description: "Install Rust toolchain", requires_sudo: false, }, 
        Command {cmd_type: CommandType::Update, bin: "rustup", args: &["update"], description: "Update Rust toolchains", requires_sudo: false, }, 
        Command {cmd_type: CommandType::Search, bin: "rustup", args: &["toolchain", "list"], description: "List toolchains", requires_sudo: false, }, 
        Command {cmd_type: CommandType::List, bin: "rustup", args: &["toolchain", "list"], description: "List installed toolchains", requires_sudo: false, },
    ],
};

// ---------------- PYENV ----------------
pub const PYENV: PackageManager = PackageManager {
    name: "Pyenv",
    cmd: &["pyenv"],
    category: PackageManagerCategory::Toolchain,
    commands: &[
        Command {cmd_type: CommandType::Install, bin: "pyenv", args: &["install"], description: "Install Python versions", requires_sudo: false, }, 
        Command {cmd_type: CommandType::Update, bin: "pyenv", args: &["update"], description: "Update pyenv", requires_sudo: false, },
        Command {cmd_type: CommandType::Search, bin: "pyenv", args: &["install", "--list"], description: "List available Python versions", requires_sudo: false, },
        Command {cmd_type: CommandType::List, bin: "pyenv", args: &["versions"], description: "List installed Python versions", requires_sudo: false, },
    ],
};

pub const NVM: PackageManager = PackageManager {
    name: "NVM",
    cmd: &["nvm"],
    category: PackageManagerCategory::Toolchain,
    commands: &[
        Command {cmd_type: CommandType::Install, bin: "nvm", args: &["install"], description: "Install Node.js versions", requires_sudo: false, }, 
        Command {cmd_type: CommandType::Update, bin: "nvm", args: &["install", "node"], description: "Update Node.js", requires_sudo: false, }, 
        Command {cmd_type: CommandType::Search, bin: "nvm", args: &["ls-remote"], description: "List available Node.js versions", requires_sudo: false, }, 
        Command {cmd_type: CommandType::List, bin: "nvm", args: &["ls"], description: "List installed Node versions", requires_sudo: false, },
    ],
};