use crate::{query::Query, root::Root};
use async_trait::async_trait;

use super::{Container, HasContainerType};

impl<T> HasContainerType for Root<T> {
    type ContainerType = Fin<Root<T>>;
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Fin<V>(V);
#[async_trait]
impl<V> Container for Fin<V>
where
    V: Query,
    V::Error: Send + Sync,
    V::UserData: Send + Sync,
    V::Key: Send + Sync,
{
    type Value = V;
    type Output = V;

    async fn create(
        user_data: <Self::Value as Query>::UserData,
        key: <Self::Value as Query>::Key,
    ) -> Result<Self::Output, <Self::Value as Query>::Error> {
        V::query(user_data, key).await
    }
}
