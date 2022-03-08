use crate::comm::*;

pub fn struct_content() -> String {
    r#"
pub struct A {
    pub a: i32,
    b: B,
    c: C,
    d: tokio::syn::mpsc::Sender<D>,
}
pub enum B {
    B1(i32),
}
struct C(i32);
    "#
    .to_string()
}

pub fn singe_struct_content() -> String {
    r#"
use crate::common::B;
use super::C;
use d::D;
pub struct A {
    pub a: i32,
    b: B,
    c: C,
    d: tokio::syn::mpsc::Sender<D>,
}
    "#
    .to_string()
}
