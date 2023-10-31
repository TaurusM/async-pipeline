use crate::link::Linkable;
use std::marker::PhantomData;

pub mod async_connect;
pub mod connect;
pub mod link;
mod test;

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
