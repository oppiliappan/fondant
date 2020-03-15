# fondant

> an experimental, macro-only, boilerplate free, configuration management library

### example

```rust
#[derive(Config, Default, Serialize, Deserialize)]
#[filename = "config"] // ~/.config/appconfig/config.toml (XDG spec)
#[extension = "toml"]  // possible values: yaml, toml, json
struct AppConfig {
    version: u8,
    path: String,
}

fn main() {
    let mut config = AppConfig::load().unwrap();
    config.version = 3;
    config.path = "/home/np".to_string();
    config.store();
}
```
