use crate::{query::HasQuery, Container, HasContainer, Query};
use async_trait::async_trait;
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

impl<T> HasContainer for Root<T> {
    type Container = Root<T>;
}
pub struct Root<V>(V);

#[async_trait]
impl<V> Container for Root<V>
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
    type Output = Root<V>;
    type Inner = V;

    async fn with_key<K: Into<Self::Key> + Send + Sync>(
        user_data: Self::UserData,
        key: K,
    ) -> Result<Self::Output, Self::Error> {
        <V::Query as Query>::query(key.into(), &user_data)
            .await
            .map(Root)
    }

    async fn with(
        _user_data: Self::UserData,
        value: Self::Inner,
    ) -> Result<Self::Output, Self::Error> {
        Ok(Root(value))
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

impl<T> Deref for Root<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Root<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
