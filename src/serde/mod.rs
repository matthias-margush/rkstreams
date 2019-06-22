use crate::serde::json::JSON;
use serde_bytes::Bytes;
use std::marker::PhantomData;

pub trait Serde<'a> {
    type Item: Default;

    fn serialize(v: &Self::Item) -> &'a Bytes;
    fn deserialize(v: &Bytes) -> &Self::Item;
}

pub fn json<T>() -> JSON<T> {
    JSON(PhantomData)
}

pub mod json;
