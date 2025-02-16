use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::config::{BASE_PATH, CARGO_TOML};

#[derive(serde::Deserialize, Debug)]
pub struct Package {
    #[allow(dead_code)]
    pub name: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum Dependency {
    #[allow(dead_code)]
    Path(String),
    Table(toml::Table),
}

#[derive(serde::Deserialize, Debug)]
pub struct Workspace {
    pub members: Vec<String>,
    pub dependencies: Option<HashMap<String, Dependency>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct CargoToml {
    pub workspace: Option<Workspace>,
    #[allow(dead_code)]
    pub package: Option<Package>,
    pub dependencies: Option<HashMap<String, Dependency>>,
}

impl Workspace {
    /// Returns a HashSet of all workspace dependency names
    pub fn get_workspace_dependency_names(&self) -> HashSet<String> {
        if let Some(dependencies) = &self.dependencies {
            return dependencies
                .iter()
                .map(|(key, _)| key.to_string())
                .collect();
        }
        HashSet::new()
    }

    /// Goes through all members of the workspace and returns a HashMap<member_name, HashMap<dependency_name, Dependency>>
    pub fn get_member_dependencies(
        &self,
    ) -> anyhow::Result<HashMap<String, HashMap<String, Dependency>>> {
        let cargo_base_path = &*BASE_PATH;
        let mut member_dependencies: HashMap<String, HashMap<String, Dependency>> = HashMap::new();

        for member in self.members.iter() {
            let member_path = cargo_base_path.join(member).join(CARGO_TOML);
            let member_toml = load_cargo_toml(member_path)?;
            if let Some(dependencies) = member_toml.dependencies {
                member_dependencies.insert(member.to_string(), dependencies);
            }
        }

        Ok(member_dependencies)
    }

    /// Finds all duplicate dependencies in the workspace that are not workspace dependencies or paths
    pub fn find_duplicate_dependencies(&self) -> anyhow::Result<Vec<String>> {
        let workspace_dependency_names = self.get_workspace_dependency_names();
        let member_dependencies = self.get_member_dependencies()?;

        let mut duplicate_dependencies: HashMap<String, u8> = HashMap::new();

        // Go through all workspace dependencies to initialize the duplicate_dependencies HashMap
        // This way, if we only have 1 crate that is not using the workspace dependency the count
        // will become 2
        workspace_dependency_names
            .iter()
            .for_each(|dependency_name| {
                duplicate_dependencies
                    .entry(dependency_name.to_string())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            });

        // Go through all members
        for (_, member_dependencies) in member_dependencies.iter() {
            // Go through all dependencies
            for (dependency_name, dependency) in member_dependencies.iter() {
                match dependency {
                    Dependency::Table(table) => {
                        // If the dependency has a path field ignore
                        if table.get("path").is_some() || table.get("workspace").is_some() {
                            continue;
                        }

                        // add the dependency to the duplicate_dependencies HashMap
                        duplicate_dependencies
                            .entry(dependency_name.to_string())
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                    // Basic versioned dependency
                    Dependency::Path(_) => {
                        duplicate_dependencies
                            .entry(dependency_name.to_string())
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                }
            }
        }

        Ok(duplicate_dependencies
            .into_iter()
            .filter(|(_, count)| *count > 1)
            .map(|(dependency_name, _)| dependency_name)
            .collect())
    }
}

#[tracing::instrument(level = "trace")]
pub fn load_cargo_toml(file_path: PathBuf) -> anyhow::Result<CargoToml> {
    let file_path = file_path.to_str();

    if let Some(file_path) = file_path {
        tracing::trace!(file_path, "loaded file path from current working directory");
        let content = std::fs::read_to_string(file_path)?;

        let content = toml::from_str::<CargoToml>(&content)?;
        return Ok(content);
    }

    Err(anyhow::anyhow!("could not find file path {file_path:?}"))
}
