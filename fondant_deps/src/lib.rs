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

/// Derive this trait on a struct to mark it as a 'configuration' struct.
pub trait Configure: Serialize + DeserializeOwned + Default {
    fn load() -> Result<Self, FondantError>;
    fn store(&self) -> Result<(), FondantError>;
}
