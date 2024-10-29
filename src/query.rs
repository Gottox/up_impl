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
        key: Self::Key,
        user_data: &Self::UserData,
    ) -> Result<Self, Self::Error>;
}
