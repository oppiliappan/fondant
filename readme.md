# fondant

[Documentation](https://docs.rs/fondant/) ·
[Architecture](#Architecture) · [Usage](#Usage) ·
[Customization](#Customization) · [Todo](#Todo)

`fondant` is a macro based library to take the boilerplate out of
configuration handling. All you need to do is derive the
`Configure` trait on your struct, and fondant will decide
where to store it and and how to do so safely.


Most of `fondant` is based off the `confy` crate.

`fondant` adds a couple of extra features:

 - support for json, yaml and toml
 - support for custom config paths
 - support for custom config file names

### Usage ([Full Documentation](https://docs.rs/fondant/))

Drop this in your `Cargo.toml` to gets started:

```
[dependencies]
fondant = "0.1.0"
```

Derive the macro:

```rust
// the struct has to derive Serialize, Deserialize and Default
use fondant::Configure;
use serde::{Serialize, Deserialize};

#[derive(Configure, Serialize, Deserialize, Default)]
#[config_file = "config.toml"]
struct AppConfig {
    port: u32,
    username: String,
}

fn main() {
    // use `load` to load the config file
    // loads in Default::default if it can't find one
    let mut conf = AppConfig::load().unwrap();

    // do stuff with conf
    conf.port = 7878;

    // call `store` to save changes
    conf.store().unwrap();
}
```

Find more examples and options at [docs.rs](https://docs.rs/fondant/).

### Architecture

`fondant` is split into 3 separate crates:

 - `fondant_deps`: external crates and utils that `fondant` requires
 - `fondant_derive`: core macro definitions
 - `fondant`: the user facing library that brings it all together

This slightly strange architecture arose because of some
limitations with proc-macro crates and strict cyclic
dependencies in cargo. All you need is the `fondant` crate.


### Todo

 - [ ] improve error types
 - [ ] use `syn::Error` and `syn::Result` to report macro errors
 - [x] write docs
 - [x] bundle and publish to crates.io

