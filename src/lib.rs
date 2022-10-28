use std::marker::PhantomData;
use crate::link::Linkable;

pub mod async_connect;
pub mod connect;
pub mod link;
mod test;

pub type Error = link::Error;

pub fn begin<T>() -> Start<T> {
    return Start {
        _p: Default::default(),
    };
}

pub struct Start<T> {
    _p: PhantomData<T>,
}

impl<IN: Send + Sync> Linkable for Start<IN> {
    type OUT = IN;
}
