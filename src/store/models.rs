#![allow(dead_code, unused_imports)]
pub enum EitherKeyType<'a> {
    Regular(&'a str),
    Number(usize),
}
