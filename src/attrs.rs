use crate::add_file_header;
use anyhow::Result;
use log::debug;
use syn::{Attribute, File};

/// Attribute没有实现parse
#[test]
fn parse_attrs() -> Result<()> {
    let res: File = syn::parse_file(add_file_header("#![allow(unused_imports)]").as_str())?;
    debug!("{:#?}", res);
    Ok(())
}
