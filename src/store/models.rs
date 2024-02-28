pub enum EitherKeyType<'a> {
    Regular(&'a str),
    Number(usize),
}
