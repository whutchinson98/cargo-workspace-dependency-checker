mod cargo_toml;
mod config;

use cargo_toml::load_cargo_toml;
use config::{BASE_PATH, CARGO_TOML};

fn main() -> anyhow::Result<()> {
    let root_toml_file_path = &*BASE_PATH;
    let root_toml_file_path = root_toml_file_path.join(CARGO_TOML);

    let root_toml = load_cargo_toml(root_toml_file_path)?;

    println!("Cargo.toml found");

    match root_toml.workspace {
        Some(workspace) => {
            let duplicate_dependencies = workspace.find_duplicate_dependencies()?;
            if !duplicate_dependencies.is_empty() {
                println!("Duplicate dependencies found");
                for dependency in duplicate_dependencies {
                    println!("{dependency}");
                }
                return Err(anyhow::anyhow!("Duplicate dependencies found"));
            }
        }
        None => {
            println!("No workspace found");
        }
    }

    Ok(())
}
