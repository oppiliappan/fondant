extern crate proc_macro;

use std::path::{Path, PathBuf};

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttrStyle, Attribute, DeriveInput, Lit, Meta, MetaNameValue};

use serde::{de::DeserializeOwned, Serialize};

trait Config<T>
where
    T: Serialize + DeserializeOwned + Default,
{
    fn load() -> Result<T, String>;
    fn store() -> Result<(), String>;
}

fn is_outer_attribute(a: &Attribute) -> bool {
    match a.style {
        AttrStyle::Outer => true,
        _ => false,
    }
}

#[proc_macro_derive(Config, attributes(filename, filetype))]
pub fn config_attribute(item: TokenStream) -> TokenStream {
    let mut ast: DeriveInput = syn::parse(item).unwrap();

    let mut filename: Option<PathBuf> = None;
    let mut filetype: Option<PathBuf> = None;

    for option in ast.attrs.into_iter() {
        let option = option.parse_meta().unwrap();
        match option {
            Meta::NameValue(MetaNameValue {
                ref path, ref lit, ..
            }) if path.is_ident("filename") => {
                if let Lit::Str(f) = lit {
                    filename = Some(PathBuf::from(f.value()));
                }
            }
            Meta::NameValue(MetaNameValue {
                ref path, ref lit, ..
            }) if path.is_ident("filetype") => {
                if let Lit::Str(f) = lit {
                    filetype = Some(PathBuf::from(f.value()));
                }
            }
            _ => {}
        }
    }

    println!("{:?} {:?}", filename, filetype);

    TokenStream::new()
}
