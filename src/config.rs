use std::{path::PathBuf, sync::LazyLock};

pub static CARGO_TOML: &str = "Cargo.toml";

pub static BASE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    if std::env::args().len() == 2 {
        let path: String = std::env::args().nth(1).unwrap();
        std::path::PathBuf::from(path)
    } else {
        std::env::current_dir().unwrap()
    }
});
