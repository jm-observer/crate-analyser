use crate::comm::*;
use syn::File;

static FILE_HEADER: &str = "\u{feff}";

pub fn add_file_header(content: impl AsRef<str>) -> String {
    format!("{}\n\t{}", FILE_HEADER, content.as_ref())
}

pub fn only_attrs() -> String {
    let content = r#"
#![allow(unused_imports)]
#![deny(unused)]
    "#;
    add_file_header(content)
}
pub fn only_variate() -> String {
    let content = r#"
pub struct A {
    pub a: i32
}
pub struct A(tokio::syn::mps::Sender<A>);
    "#;
    add_file_header(content)
}

#[test]
fn test_only_variate() -> Result<()> {
    log_setting();
    let res: File = syn::parse_file(only_variate().as_str())?;
    for ref item in res.items {
        debug!("{:#?}", item);
    }
    Ok(())
}

#[test]
fn test_only_attrs() -> Result<()> {
    log_setting();
    let res: File = syn::parse_file(only_attrs().as_str())?;
    debug!("{:#?}", res);
    Ok(())
}
