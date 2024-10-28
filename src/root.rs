use crate::query::Query;
use async_trait::async_trait;
use std::fmt::Debug;

pub struct Root<T>(T);

#[async_trait]
impl<T> Query for Root<T>
where
    T: Query + Send + Sync,
    T::Error: Send + Sync,
    T::UserData: Send + Sync,
    T::Key: Send + Sync,
{
    type UserData = T::UserData;
    type Error = T::Error;
    type Key = T::Key;

    async fn query(
        user_data: Self::UserData,
        key: Self::Key,
    ) -> Result<Self, Self::Error> {
        T::query(user_data, key).await.map(Root)
    }
}

impl<T> Debug for Root<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Root").field(&self.0).finish()
    }
}
