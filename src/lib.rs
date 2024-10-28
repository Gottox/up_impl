mod container;
mod query;
mod root;

#[cfg(any(test, doc))]
mod test;

pub use async_trait::async_trait;
pub use container::fin::*;
pub use container::up::*;

pub trait HasUp {
    type Up;
    type UpKey;

    fn key(&self) -> Self::UpKey;
}
