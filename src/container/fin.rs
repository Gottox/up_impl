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

    async fn create<K: Into<Self::Key> + Send + Sync>(
        user_data: Self::UserData,
        key: K,
    ) -> Result<Self::Output, Self::Error> {
        V::query(key.into(), &user_data).await
    }
}
