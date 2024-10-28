use crate::{async_trait};

#[async_trait]
pub trait Container
where
    Self: Sized,
{
    type Error;
    type Key;
    type UserData;

    async fn create(
        user_data: Self::UserData,
        key: Self::Key,
    ) -> Result<Self, Self::Error>;
}

pub trait HasUp {
    type Up;
    type QueryKey;
    type UserData;

    fn key(&self) -> Self::QueryKey;
}
