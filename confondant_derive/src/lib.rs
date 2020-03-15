extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Error, Ident, Lit, Meta, MetaNameValue, Result};

#[proc_macro_derive(Config, attributes(filename, extension))]
pub fn config_attribute(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(item as DeriveInput);
    let (filename, filetype) = extract_attributes(&ast);

    gen_impl(
        &ast,
        filename.unwrap_or("config".into()),
        filetype.unwrap_or("toml".into()),
    )
}

fn extract_attributes(ast: &DeriveInput) -> (Option<String>, Option<String>) {
    let mut filename: Option<String> = None;
    let mut filetype: Option<String> = None;
    for option in ast.attrs.iter() {
        let option = option.parse_meta().unwrap();
        match option {
            Meta::NameValue(MetaNameValue {
                ref path, ref lit, ..
            }) if path.is_ident("filename") => {
                if let Lit::Str(f) = lit {
                    filename = Some(f.value());
                }
            }
            Meta::NameValue(MetaNameValue {
                ref path, ref lit, ..
            }) if path.is_ident("extension") => {
                if let Lit::Str(f) = lit {
                    filetype = Some(f.value());
                }
            }
            _ => {}
        }
    }
    return (filename, filetype);
}

fn map_ser(ext: &str) -> (Ident, Ident) {
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

fn gen_impl(ast: &DeriveInput, filename: String, filetype: String) -> TokenStream {
    let struct_name = &ast.ident.to_string();
    let struct_ident = &ast.ident;
    let (ser, ser_fn) = map_ser(&filetype);

    let gen = quote! {
        use ::confondant::{ ProjectDirs, toml, serde_json, serde_yaml, FondantError };
        use ::std::path::{ Path, PathBuf };
        use ::std::option::Option;
        use ::std::fs::{self, File, OpenOptions};
        use ::std::io::prelude::*;
        use ::std::io::{ ErrorKind::NotFound, Write };

        impl Config for #struct_ident {
            fn load() -> Result<#struct_ident, FondantError> {
                let project = ProjectDirs::from("rs", "", #struct_name).ok_or(FondantError::InvalidHomeDir)?;
                let config_dir = project.config_dir();

                let tip = (Path::new(#filename)).with_extension(&#filetype);
                let config_file: PathBuf = [config_dir, &tip].iter().collect();

                match File::open(&config_file) {
                    Ok(mut cfg) => {
                        // the file exists, parse the toml and return the struct
                        let mut cfg_data = String::new();
                        cfg.read_to_string(&mut cfg_data);

                        let config: #struct_ident = #ser::from_str(&cfg_data[..])
                            .map_err(|_| FondantError::ConfigParseError)?;
                        return Ok(config);
                    },
                    Err(ref e) if e.kind() == NotFound => {
                        fs::create_dir_all(project.config_dir());
                        let default_impl = #struct_ident::default();
                        Config::store(&default_impl)?;
                        return Ok(default_impl);
                    },
                    Err(e) => return Err(FondantError::LoadError),
                };
            }
            fn store(&self) -> Result<(), FondantError> {
                let project = ProjectDirs::from("rs", "", #struct_name).ok_or(FondantError::InvalidHomeDir)?;
                let config_dir = project.config_dir();

                let tip = (Path::new(#filename)).with_extension(&#filetype);
                let config_file: PathBuf = [config_dir, &tip].iter().collect();

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
