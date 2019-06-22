use crate::serde::Serde;

#[derive(Debug)]
pub struct Topic<'a, S>(pub &'a str, pub S);
