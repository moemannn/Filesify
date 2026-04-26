pub const DISTRO_PACKAGE_MANAGERS: &[PackageManager] = &[
    APT,
    PACMAN,
];

pub const USER_PACKAGE_MANAGERS: &[PackageManager] = &[
    NPM,
    FLAT,
    SNAP,
];

pub struct PackageManager {
    pub name: &'static str,
    pub commands: &'static [Command],
}

pub struct Command {
    pub name: &'static str,
    pub cmd: &'static str,
    pub description: &'static str,
}

pub const FLAT: PackageManager = PackageManager {
    name: "FlatPak",
    commands: &[
        Command {
            name: "install",
            cmd: "flatpak install",
            description: "Install a package",
        },
        Command {
            name: "update",
            cmd: "flatpak update",
            description: "Update packages",
        },
        Command {
            name: "run",
            cmd: "flatpak run",
            description: "Run an application",
        },
    ],
};

pub const SNAP: PackageManager = PackageManager {
    name: "Snap",
    commands: &[
        Command {
            name: "install",
            cmd: "snap install",
            description: "Install a package",
        },
        Command {
            name: "update",
            cmd: "snap refresh",
            description: "Update packages",
        },
        Command {
            name: "run",
            cmd: "snap run",
            description: "Run an application",
        },
        Command {
            name: "remove",
            cmd: "snap remove",
            description: "Remove a package",
        },
        Command {
            name: "list",
            cmd: "snap list",
            description: "List installed packages",
        },
    ],
};

pub const NPM: PackageManager = PackageManager {
    name: "npm",
    commands: &[
        Command {
            name: "install",
            cmd: "npm install",
            description: "Install dependencies",
        },
        Command {
            name: "update",
            cmd: "npm update",
            description: "Update dependencies",
        },
        Command {
            name: "run",
            cmd: "npm run",
            description: "Run a script",
        },
    ],
};

pub const APT: PackageManager = PackageManager {
    name: "Advanced Packaging Tool",
    commands: &[
        Command {
            name: "install",
            cmd: "apt install",
            description: "Install dependencies",
        },
        Command {
            name: "update",
            cmd: "apt update",
            description: "Update dependencies",
        },
        Command {
            name: "upgrade",
            cmd: "apt upgrade",
            description: "upgrade dependencies",
        },
        Command {
            name: "search",
            cmd: "apt search",
            description: "Search available packages",
        },
        Command {
            name: "list",
            cmd: "apt list --installed",
            description: "List installed packages",
        },
    ],
};

pub const PACMAN: PackageManager = PackageManager {
    name: "Pacman",
    commands: &[
        Command {
            name: "install",
            cmd: "pacman -S",
            description: "Install packages",
        },
        Command {
            name: "update",
            cmd: "pacman -Sy",
            description: "Refresh package databases",
        },
        Command {
            name: "upgrade",
            cmd: "pacman -Syu",
            description: "Upgrade the system",
        },
        Command {
            name: "remove",
            cmd: "pacman -R",
            description: "Remove packages",
        },
        Command {
            name: "search",
            cmd: "pacman -Ss",
            description: "Search for packages",
        },
        Command {
            name: "list",
            cmd: "pacman -Q",
            description: "List installed packages",
        },
    ],
};