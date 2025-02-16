mod cargo_toml;
use std::path::PathBuf;

use cargo_toml::CargoToml;

pub static CARGO_TOML: &str = "Cargo.toml";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_toml_file_path = if std::env::args().len() == 2 {
        let path: String = std::env::args().nth(1).unwrap();
        std::path::PathBuf::from(path).join(CARGO_TOML)
    } else {
        std::env::current_dir()?.join(CARGO_TOML)
    };

    let root_toml = load_cargo_toml(root_toml_file_path)?;
    println!("Cargo.toml found");

    match root_toml.workspace {
        Some(workspace) => {
            for member in workspace.members {
                println!("Member {member}");
            }
            for dependency in workspace.dependencies {
                println!("{dependency:?}");
            }
        }
        None => {
            println!("No workspace found");
        }
    }

    Ok(())
}

/// Load the config from the current working directory
#[tracing::instrument(level = "trace")]
fn load_cargo_toml(file_path: PathBuf) -> Result<CargoToml, Box<dyn std::error::Error>> {
    let file_path = file_path.to_str();

    if let Some(file_path) = file_path {
        tracing::trace!(file_path, "loaded file path from current working directory");
        let content = std::fs::read_to_string(file_path)?;

        let content = toml::from_str::<CargoToml>(&content)?;
        return Ok(content);
    }
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!("could not find file path {file_path:?}"),
    )))
}
