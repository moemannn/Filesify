use super::types::*;

// impl Capability {
//     pub fn as_str(&self) -> &'static str {
//         match self {
//             Capability::Install => "install",
//             Capability::Update => "update",
//             Capability::Upgrade => "upgrade",
//             Capability::Info => "info",
//             Capability::Search => "search",
//             Capability::List => "list",
//             Capability::Version => "version",
//             Capability::Alias(s) => s,
//         }
//     }
// }

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