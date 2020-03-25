//! `fondant` is a macro based library to take the boilerplate out of
//! configuration handling. Most of `fondant` is based off the `confy` crate.
//!
//! `fondant` adds a couple of extra features:
//!
//!  - support for json, yaml and toml
//!  - support for custom config paths
//!  - support for custom config file names
//!
//!  ### Sample usage
//!
//! ```
//! // the struct has to derive Serialize, Deserialize and Default
//! #[derive(Configure, Serialize, Deserialize, Default)]
//! #[config_file = "config.toml"]
//! // |
//! // `-- this attribute sets the file name to "config.toml"
//! // `-- the file format to "toml"
//! // `-- the file path to "default" (read the notes below)
//! struct AppConfig {
//!     version: u32,
//!     port: u32,
//!     username: String,
//! }
//!
//! fn main() {
//!     // use `load` (associated method) to load the config file
//!     let mut conf = AppConfig::load().unwrap();
//!
//!     // do stuff with conf
//!     conf.version = 2;
//!
//!     // call `store` to save changes
//!     conf.store().unwrap();
//! }
//! ```
//! **Notes**:  
//!  - `load` returns `Default::default` if the config file is not present, and stores
//!  a serialized `Default::default` at the specified path
//!  - the "default" config path varies by platform:
//!     * GNU/Linux: `$XDG_CONFIG_HOME/my_cool_crate/config.toml` (follows xdg spec)
//!     * MacOS: `$HOME/Library/Preferences/my_cool_crate/config.toml`
//!     * Windows: `{FOLDERID_RoamingAppData}\_project_path_\config`
//!
//! ### Customization
//!
//! Set your own filename, for ex.: `apprc`
//!
//! ```
//! #[derive(Configure, Serialize, Deserialize, Default)]
//! #[config_file = "apprc.toml"]
//! struct AppConfig {
//!     // -- snip --
//! }
//! // effective path: $XDG_CONFIG_HOME/my_cool_crate/apprc.toml
//! // effective format: toml
//! ```
//!
//! Change file format to `yaml`, by changing the file extension.
//! Supported extensions are `yaml`, `toml`, `json`:
//! ```
//! #[derive(Configure, Serialize, Deserialize, Default)]
//! #[config_file = "config.yaml"]
//! struct AppConfig {
//!     // -- snip --
//! }
//! // effective path: $XDG_CONFIG_HOME/my_cool_crate/config.yaml
//! // effective format: yaml
//! ```
//!
//! Override the default config path, for ex.: the home directory
//! (it is not recommended to override config path):
//! ```
//! #[derive(Configure, Serialize, Deserialize, Default)]
//! #[config_file = "~/.apprc.json"]
//! struct AppConfig {
//!     // -- snip --
//! }
//! // effective path: $HOME/.apprc.json
//! // effective format: json
//! ```

/// Re-exporting crates that fondant_derive depends on.
///
///
/// Unfortunately, this seems to be the only way to bring crates into
/// scope from a proc-macro crate
/// This module should not bother you
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

pub use fondant_derive::Configure;
