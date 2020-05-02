pub mod fondant_exports {
    pub use directories::{ProjectDirs, UserDirs};
    pub use serde::{de::DeserializeOwned, Serialize};
    pub use serde_json;
    pub use serde_yaml;
    use std::path::{Path, PathBuf};
    pub use toml;
    pub fn expand_tilde<P: AsRef<Path>>(path: P) -> PathBuf {
        let p = path.as_ref();
        if p.starts_with("~") {
            if p == Path::new("~") {
                return UserDirs::new().unwrap().home_dir().to_path_buf();
            } else {
                let mut h = UserDirs::new().unwrap().home_dir().to_path_buf();
                h.push(p.strip_prefix("~/").unwrap());
                return h;
            }
        }
        return p.to_path_buf();
    }
}

use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;
use std::fmt;
use std::path::Path;

#[derive(Debug)]
/// Errors that `load` and `store` can result in
pub enum FondantError {
    /// Occurs when the home dir is not accessible.
    /// You should probably `panic!` when this is thrown.
    InvalidHomeDir,

    /// Invalid toml/yaml/json config.
    ConfigParseError,

    /// Invalid permissions to create config dir.
    /// Might occur when you set config dir to, say, `/etc/config.toml` and run without superuser.
    DirCreateErr(std::io::Error),
    LoadError,
    FileWriteError,
    FileReadError,
    FileOpenError,
}

impl fmt::Display for FondantError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let suggestion_text =
            "HELP: You might have insufficient permissions to perform this action.";
        match self {
            FondantError::InvalidHomeDir => write!(f, "Failed to find home directory!"),
            FondantError::ConfigParseError => write!(f, "Invalid configuration file!"),
            FondantError::DirCreateErr(_) => {
                write!(f, "Failed to write to configuration directory!")
            }
            FondantError::LoadError => write!(f, "Failed to load configuration file!"),
            FondantError::FileWriteError => {
                write!(f, "Failed to write configuration file! {}", suggestion_text)
            }
            FondantError::FileReadError => {
                write!(f, "Failed to read configuration file! {}", suggestion_text)
            }
            FondantError::FileOpenError => {
                write!(f, "Failed to open configuration file! {}", suggestion_text)
            }
        }
    }
}

impl Error for FondantError {}

/// Derive this trait on a struct to mark it as a 'configuration' struct.
pub trait Configure: Serialize + DeserializeOwned + Default {
    fn load_file<P: AsRef<Path>>(config_file: P) -> Result<Self, FondantError>;
    fn load() -> Result<Self, FondantError>;
    fn store(&self) -> Result<(), FondantError>;
    fn store_file<P: AsRef<Path>>(&self, config_file: P) -> Result<(), FondantError>;
}
