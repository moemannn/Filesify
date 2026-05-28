#[derive(Debug)]
pub struct Package {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Groups {
    pub name: String,
}