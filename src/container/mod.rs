pub mod branch;
pub mod fin;
pub mod up;

use async_trait::async_trait;

#[async_trait]
pub trait Container
where
    Self: Sized,
{
    type Error;
    type Key;
    type UserData;
    type Output;

    async fn create<K: Into<Self::Key> + Send + Sync>(
        user_data: Self::UserData,
        key: K,
    ) -> Result<Self::Output, Self::Error>;
}

pub trait HasContainerType {
    type ContainerType: ?Sized;
}
