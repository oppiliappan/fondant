pub use directories::{ProjectDirs, UserDirs};
pub use serde::{de::DeserializeOwned, Serialize};
pub use serde_json;
pub use serde_yaml;
pub use toml;

use std::path::{Path, PathBuf};

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

pub trait Configure: Serialize + DeserializeOwned + Default {
    fn load() -> Result<Self, FondantError>;
    fn store(&self) -> Result<(), FondantError>;
}
