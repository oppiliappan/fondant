//! # fondant
//!
//! [Documentation](https://docs.rs/fondant/) ·
//! [Architecture](#Architecture) · [Usage](#Usage) ·
//! [Customization](#Customization) · [Todo](#Todo)
//!
//! `fondant` is a macro based library to take the boilerplate out of
//! configuration handling. All you need to do is derive the
//! `Configure` trait on your struct, and `fondant` will decide
//! where to store it and and how to do so safely.
//!
//! Most of `fondant` is based off the `confy` crate.
//! `fondant` adds a couple of extra features:
//!
//!  - support for json, yaml and toml
//!  - support for custom config paths
//!  - support for custom config file names
//!
//! ### Architecture
//!
//! `fondant` is split into 3 separate crates:
//!
//!  - `fondant_deps`: external crates and utils that `fondant` requires
//!  - `fondant_derive`: core macro definitions
//!  - `fondant`: the user facing library that brings it all together
//!
//! This slightly strange architecture arose because of some
//! limitations with proc-macro crates and strict cyclic
//! dependencies in cargo. All you need is the `fondant` crate.
//!
//! ### Usage
//!
//! ```rust
//! // the struct has to derive Serialize, Deserialize and Default
//! #[derive(Configure, Serialize, Deserialize, Default)]
//! #[config_file = "config.toml"]
//! // `config_file` attribute sets the file name to "config.toml"
//! // the file format to "toml"
//! // and the file path to "default" (read the notes below)
//! struct AppConfig {
//!     version: u32,
//!     port: u32,
//!     username: String,
//! }
//!
//! fn main() {
//!     // use `load` to load the config file
//!     // loads in Default::default if it can't find one
//!     let mut conf = AppConfig::load().unwrap();
//!
//!     // do stuff with conf
//!     conf.version = 2;
//!
//!     // call `store` to save changes
//!     conf.store().unwrap();
//! }
//! ```
//!
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
//! ```rust
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
//!
//! ```rust
//! #[derive(Configure, Serialize, Deserialize, Default)]
//! #[config_file = "config.yaml"]
//! struct AppConfig {
//!     // -- snip --
//! }
//! // effective path: $XDG_CONFIG_HOME/my_cool_crate/config.yaml
//! // effective format: yaml
//! ```
//!
//! Override the default config path (not recommended),
//! for ex.: the home directory:
//!
//! ```rust
//! #[derive(Configure, Serialize, Deserialize, Default)]
//! #[config_file = "~/.apprc.json"]
//! struct AppConfig {
//!     // -- snip --
//! }
//! // effective path: $HOME/.apprc.json
//! // effective format: json
//! ```
//!
//! Fondant meshes well with Serde, for ex.:
//! ```rust
//! #[derive(Configure, Serialize, Deserialize, Default)]
//! #[config_file = "config.toml"]
//! struct Madagascar {
//!     #[serde(skip)]
//!     rating: u32,
//!
//!     name: String,
//!     penguins: u32,
//! }
//! ```
//!
//! Above snippet produces this config file:
//! ```toml
//! name = 'Central Park Zoo'
//! penguins = 4
//! ```

pub use fondant_deps::fondant_exports;
pub use fondant_deps::Configure;
pub use fondant_deps::FondantError;
pub use fondant_derive::Configure;
