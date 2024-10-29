use crate::async_trait;

#[async_trait]
pub trait HasQuery
where
    Self: Sized,
{
    type Query;
}

#[async_trait]
pub trait Query
where
    Self: Sized,
{
    type UserData;
    type Error;
    type Key;
    type Output;
    async fn query(
        key: Self::Key,
        user_data: &Self::UserData,
    ) -> Result<Self::Output, Self::Error>;
}
