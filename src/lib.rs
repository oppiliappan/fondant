use std::path::{Path, PathBuf};

pub use serde::{de::DeserializeOwned, Serialize};
pub use directories::ProjectDirs;
pub use toml;
pub use serde_json;
pub use serde_yaml;

#[derive(Debug)]
pub enum FondantError {
    InvalidHomeDir,
    ConfigParseError,
    DirCreateErr(std::io::Error),
    LoadError,
    FileWriteError,
    FileReadError,
    FileOpenError,
}

pub trait Config: Serialize + DeserializeOwned + Default {
    fn load() -> Result<Self, FondantError>;
    fn store(&self) -> Result<(), FondantError>;
}
