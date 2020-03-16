pub use directories::ProjectDirs;
pub use serde::{de::DeserializeOwned, Serialize};
pub use serde_json;
pub use serde_yaml;
pub use toml;

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

pub trait Configure: Serialize + DeserializeOwned + Default {
    fn load() -> Result<Self, FondantError>;
    fn store(&self) -> Result<(), FondantError>;
}
