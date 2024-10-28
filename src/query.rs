use crate::async_trait;

#[async_trait]
pub trait Query
where
    Self: Sized,
{
    type UserData;
    type Error;
    type Key;
    async fn query(
        user_data: Self::UserData,
        key: Self::Key,
    ) -> Result<Self, Self::Error>;
}
