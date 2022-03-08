#![allow(unused_imports)]
pub use anyhow::Result;
pub use log::{debug, info, warn};
pub use syn::File;

pub fn log_setting() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
}

pub fn parse_file(content: impl AsRef<str>) -> Result<File> {
    Ok(syn::parse_file(content.as_ref())?)
}

pub fn parse_str<T: syn::parse::Parse>(content: impl AsRef<str>) -> Result<T> {
    Ok(syn::parse_str(content.as_ref())?)
}
