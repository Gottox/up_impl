mod container;
mod query;

#[cfg(any(test, doc))]
mod test;

pub use async_trait::async_trait;
pub use container::root::*;
pub use container::up::*;
pub use container::*;
pub use query::*;

pub trait HasUp {
    type Up;
    type UpKey;

    fn key(&self) -> Self::UpKey;
}
