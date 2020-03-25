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

pub use fondant_deps::fondant_exports;
pub use fondant_deps::Configure;
pub use fondant_deps::FondantError;
pub use fondant_derive::Configure;
