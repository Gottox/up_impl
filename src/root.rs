use crate::debug::Debug;

use crate::query::Query;
use async_trait::async_trait;

#[cfg_attr(feature = "debug", derive(Debug))]
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
