use crate::{query::Query, root::Root};
use async_trait::async_trait;

use super::{Container, HasContainerType};

impl<T> HasContainerType for Root<T> {
    type ContainerType = Fin<Root<T>>;
}
pub struct Fin<V>(V);
#[async_trait]
impl<V> Container for Fin<V>
where
    V: Query,
    V::Error: Send + Sync,
    V::UserData: Send + Sync,
    V::Key: Send + Sync,
{
    type Error = V::Error;
    type Key = V::Key;
    type UserData = V::UserData;

    type Output = V;

    async fn create(
        user_data: Self::UserData,
        key: Self::Key,
    ) -> Result<Self::Output, Self::Error> {
        V::query(user_data, key).await
    }
}
