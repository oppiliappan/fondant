extern crate proc_macro;

use ::std::ffi::{OsStr, OsString};
use ::std::path::Path;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, Lit, Meta, MetaNameValue};

#[derive(Debug, Default)]
struct ConfigPath {
    parent: String,
    filename: Option<OsString>,
    extension: Option<OsString>,
}

#[proc_macro_derive(Configure, attributes(config_file))]
pub fn config_attribute(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(item as DeriveInput);
    let cfg_path = extract_attributes(&ast);

    gen_impl(&ast, cfg_path)
}

fn extract_attributes(ast: &DeriveInput) -> ConfigPath {
    for option in ast.attrs.iter() {
        let option = option.parse_meta().unwrap();
        match option {
            Meta::NameValue(MetaNameValue {
                ref path, ref lit, ..
            }) if path.is_ident("config_file") => {
                if let Lit::Str(f) = lit {
                    let f = f.value();
                    let fp = Path::new(&f);
                    let parent = fp.parent().unwrap_or(Path::new(""));
                    return ConfigPath {
                        parent: parent.to_str().unwrap().into(),
                        filename: fp.file_stem().map(OsStr::to_os_string),
                        extension: fp.extension().map(OsStr::to_os_string),
                    };
                }
            }
            _ => {}
        }
    }
    return Default::default();
}

fn pick_serializer(ext: &str) -> (Ident, Ident) {
    /* returns serializer and a corresponding function to
     * stringify with based on file extension
     * toml::to_string_pretty
     * serde_yaml::to_string
     * serde_json::to_string_pretty
     */
    match ext.as_ref() {
        "toml" => (
            Ident::new("toml", Span::call_site()),
            Ident::new("to_string_pretty", Span::call_site()),
        ),
        "yaml" => (
            Ident::new("serde_yaml", Span::call_site()),
            Ident::new("to_string", Span::call_site()),
        ),
        "json" => (
            Ident::new("serde_json", Span::call_site()),
            Ident::new("to_string_pretty", Span::call_site()),
        ),
        _ => panic!("Invalid extension!"),
    }
}

fn gen_impl(ast: &DeriveInput, cfg_path: ConfigPath) -> TokenStream {
    let struct_ident = &ast.ident;

    let filename = cfg_path
        .filename
        .unwrap_or(OsStr::new("config").to_os_string())
        .into_string()
        .unwrap();

    let filetype = cfg_path
        .extension
        .unwrap_or(OsStr::new("toml").to_os_string())
        .into_string()
        .unwrap();

    let parent = cfg_path.parent;

    let (ser, ser_fn) = pick_serializer(&filetype);

    let includes = quote! {
        use ::fondant::*;
        use ::std::option::Option;
        use ::std::fs::{self, File, OpenOptions};
        use ::std::io::prelude::*;
        use ::std::io::{ ErrorKind::NotFound, Write };
        use ::std::ffi::{OsStr, OsString};
        use ::std::path::{Path, PathBuf};
    };

    let load_paths = quote! {
        let pkg_name = env!("CARGO_PKG_NAME");
        let project = ProjectDirs::from("rs", "", pkg_name).unwrap();
        let default_dir: String = project.config_dir().to_str().unwrap().into();

        let d = if #parent != "" { #parent.into() } else { default_dir };
        let config_dir: String = expand_tilde(d)
            .as_path()
            .to_str()
            .unwrap()
            .into();

        let tip = Path::new(&#filename).with_extension(&#filetype);
        let mut config_file = PathBuf::from(&config_dir);
        config_file.push(tip);
    };

    let gen = quote! {
        #includes
        impl Configure for #struct_ident {
            fn load() -> Result<#struct_ident, FondantError> {
                #load_paths
                match File::open(&config_file) {
                    Ok(mut cfg) => {
                        let mut cfg_data = String::new();
                        cfg.read_to_string(&mut cfg_data);

                        let config: #struct_ident = #ser::from_str(&cfg_data[..])
                            .map_err(|_| FondantError::ConfigParseError)?;
                        return Ok(config);
                    },
                    Err(ref e) if e.kind() == NotFound => {
                        if !Path::new(&config_dir).is_dir() {
                            fs::create_dir_all(config_dir).map_err(FondantError::DirCreateErr)?;
                        }
                        let default_impl = #struct_ident::default();
                        Configure::store(&default_impl)?;
                        return Ok(default_impl);
                    },
                    Err(e) => return Err(FondantError::LoadError),
                };
            }
            fn store(&self) -> Result<(), FondantError> {
                #load_paths
                let mut f = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(config_file)
                    .map_err(|_| FondantError::FileOpenError)?;

                let s = #ser::#ser_fn(self).map_err(|_| FondantError::ConfigParseError)?;
                f.write_all(s.as_bytes()).map_err(|_| FondantError::FileWriteError);
                Ok(())
            }
        }
    };
    gen.into()
}
