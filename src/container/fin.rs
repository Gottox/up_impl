use crate::{query::HasQuery, root::Root, Query};
use async_trait::async_trait;

use super::{Container, HasContainerType};

impl<T> HasContainerType for Root<T> {
    type ContainerType = Fin<Root<T>>;
}
pub struct Fin<V>(V);
#[async_trait]
impl<V> Container for Fin<V>
where
    V: HasQuery,
    V::Query: Query<Output = V>,
    <V::Query as Query>::Error: Send + Sync,
    <V::Query as Query>::UserData: Send + Sync,
    <V::Query as Query>::Key: Send + Sync,
{
    type Error = <V::Query as Query>::Error;
    type Key = <V::Query as Query>::Key;
    type UserData = <V::Query as Query>::UserData;

    type Output = V;

    async fn create<K: Into<Self::Key> + Send + Sync>(
        user_data: Self::UserData,
        key: K,
    ) -> Result<Self::Output, Self::Error> {
        <V::Query as Query>::query(key.into(), &user_data).await
    }
}
