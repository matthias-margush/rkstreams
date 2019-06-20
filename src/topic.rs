use std::marker::PhantomData;


#[derive(Debug)]
pub struct Topic<'a, Ser, De>(&'a str, PhantomData<Ser>, PhantomData<De>)
    where Ser: serde::Serialize,
          De: serde::Deserialize<'a>;

pub fn topic<'a, Ser, De>(name: &'a str) -> Topic<'a, Ser, De>
    where Ser: serde::Serialize,
          De: serde::Deserialize<'a> {
    Topic(name, PhantomData, PhantomData)
}

