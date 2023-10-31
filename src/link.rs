use crate::async_connect::AsyncConnect;
use crate::connect::Connect;
use std::future::Future;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub trait Linkable {
    type OUT: Send + Sync;
    fn then_async<F, FUT, NXT>(self: Self, f: F) -> AsyncConnect<Self, F>
    where
        F: Fn(Self::OUT) -> FUT,
        FUT: Future<Output = NXT>,
        Self: Sized,
    {
        AsyncConnect {
            prev: Box::new(self),
            next: f,
        }
    }

    fn then_async_result<F, FUT, NXT>(self: Self, f: F) -> AsyncConnect<Self, ErrorFuc<F>>
    where
        F: Fn(Self::OUT) -> FUT,
        FUT: Future<Output = Result<NXT, Error>>,
        Self: Sized,
    {
        AsyncConnect {
            prev: Box::new(self),
            next: ErrorFuc::new(f),
        }
    }

    fn then<F, NXT>(self: Self, f: F) -> Connect<Self, F>
    where
        F: Fn(Self::OUT) -> NXT,
        Self: Sized,
    {
        Connect {
            prev: Box::new(self),
            next: f,
        }
    }

    fn then_result<F, NXT>(self: Self, f: F) -> Connect<Self, ErrorFuc<F>>
    where
        F: Fn(Self::OUT) -> Result<NXT, Error>,
        Self: Sized,
    {
        Connect {
            prev: Box::new(self),
            next: ErrorFuc::new(f),
        }
    }
}

#[async_trait::async_trait]
pub trait Pipeline: Linkable {
    type IN: Send + Sync;
    // todo return Result
    async fn process(self: &Self, input: Self::IN) -> Result<Self::OUT, Error>;
}

pub struct ErrorFuc<F> {
    pub f: F,
}

impl<F> ErrorFuc<F> {
    fn new(f: F) -> Self {
        ErrorFuc { f }
    }
}
