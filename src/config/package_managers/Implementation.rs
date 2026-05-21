use super::types::*;

use std::fmt;

impl fmt::Display for PackageManagerCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            PackageManagerCategory::Distro => "Distro",
            PackageManagerCategory::User => "User",
            PackageManagerCategory::Language => "Language",
        };

        write!(f, "{}", name)
    }
}

impl Args {
    pub fn resolve(&self, package: &str) -> Vec<String> {
        match self {
            Args::Static(s) => vec![s.to_string()],
            Args::PackageName => vec![package.to_string()],
            Args::Version => vec![],
            Args::Input(s) => vec![s.to_string()],
            Args::Flag(k, v) => vec![format!("--{}={}", k, v)],
            Args::Many(list) => {
                list.iter()
                    .flat_map(|a| a.resolve(package))
                    .collect()
            }
        }
    }
}