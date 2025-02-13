use std::collections::HashMap;

#[derive(serde::Deserialize, Debug)]
pub struct Package {
    pub name: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum Dependency {
    Path(String),
    Table(toml::Table),
}

#[derive(serde::Deserialize, Debug)]
pub struct Workspace {
    pub members: Vec<String>,
    pub dependencies: HashMap<String, Dependency>,
}

#[derive(serde::Deserialize, Debug)]
pub struct CargoToml {
    pub workspace: Option<Workspace>,
    pub package: Option<Package>,
    pub dependencies: Option<HashMap<String, Dependency>>,
}
