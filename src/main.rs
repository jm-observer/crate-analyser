#![allow(unused_imports)]

pub mod attrs;
pub mod comm;
pub mod fields;
pub mod files;
pub mod structs;

use crate::comm::log_setting;
use crate::comm::*;
use crate::files::{add_file_header, only_attrs};
use proc_macro2::TokenStream;
use quote::quote;
use std::fs::ReadDir;

fn main() {
    log_setting();
    test_parse_file().unwrap();
}

fn test_parse_file() -> Result<()> {
    let res: File = parse_file(init_file())?;
    debug!("{:#?}", res);
    for attr in &res.attrs {
        debug!("attr: {:#?}", attr);
    }
    Ok(())
}

fn init_file() -> String {
    add_file_header("pub async fn init(c: i32) {\n let a = 3;\n let b = 4; \n let _c = a + b;}")
}
