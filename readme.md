# fondant

> an experimental, macro-only, boilerplate free, configuration management library

### example

```rust
// derive the `Configure` trait, and specify the
// `config_file` attribute
#[derive(Configure, Default, Serialize, Deserialize)]
#[config_file = "config.toml"]
struct AppConfig {
    -- snip --
}

// `Configure` exposes `load` and `store`
fn main() {
    let mut config = AppConfig::load().unwrap();
    config.version = 2;
    config.path = "/home/np".to_string();
    config.store();
}
```

You can specify a path to be used, unless you want to stick
with the defaults:

```rust
// load from and store in user's home directory
#[config_file = "~/config.toml"]
```

`fondant` supports `yaml`, `toml` and `json`, defaults to
`toml`:

```rust
#[config_file = "config.toml"]
#[config_file = "my_app_config.yaml"]
#[config_file = "~/.app.conf.json"]
```

### todo

 - [ ] improve error from trait impl
 - [ ] use `syn::Error` and `syn::Result` to report macro errors
 - [ ] write docs
 - [ ] bundle and publish to crates.io
