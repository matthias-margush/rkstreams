use crate::serde::Serde;
use serde_bytes::Bytes;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct JSON<T>(pub(crate) PhantomData<T>);

impl<'a, T: Default> Serde<'a> for JSON<T> {
    type Item = T;

    fn serialize(v: &Self::Item) -> &'a Bytes {
        unimplemented!()
    }

    fn deserialize(v: &Bytes) -> &Self::Item {
        unimplemented!()
    }
}
