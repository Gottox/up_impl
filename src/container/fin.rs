use crate::{query::HasQuery, root::Root, Query};
use async_trait::async_trait;

use super::{Container, HasContainerType};

impl<T> HasContainerType for Root<T> {
    type ContainerType = Fin<T>;
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
    V: Send + Sync,
{
    type Error = <V::Query as Query>::Error;
    type Key = <V::Query as Query>::Key;
    type UserData = <V::Query as Query>::UserData;
    type Output = V;
    type Inner = V;

    async fn with_key<K: Into<Self::Key> + Send + Sync>(
        user_data: Self::UserData,
        key: K,
    ) -> Result<Self::Output, Self::Error> {
        <V::Query as Query>::query(key.into(), &user_data).await
    }

    async fn with(
        _user_data: Self::UserData,
        value: Self::Inner,
    ) -> Result<Self::Output, Self::Error> {
        Ok(value)
    }
}
