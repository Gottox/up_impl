use super::{Container, HasContainer};
use crate::query::{HasQuery, Query};
use async_trait::async_trait;
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

pub trait HasUp {
    type Up;
    type UpKey;

    fn key(&self) -> Self::UpKey;
}
impl<V> HasContainer for V
where
    V: HasUp<Up: HasContainer>,
    <V::Up as HasContainer>::Container: Container,
{
    type Container = Up<V>;
}
pub struct Up<V>
where
    V: HasUp<Up: HasContainer>,
    <V::Up as HasContainer>::Container: Container,
{
    pub value: V,
    pub up: <<V::Up as HasContainer>::Container as Container>::Output,
}
#[async_trait]
impl<V> Container for Up<V>
where
    V: HasUp + HasQuery + HasContainer + HasQuery + Send + Sync,
    V::Query: Query<Output = V> + Send + Sync,
    V::Up: HasContainer,
    <V::Up as HasContainer>::Container: Container<
        UserData = <V::Query as Query>::UserData,
        Key = <V as HasUp>::UpKey,
        Error = <V::Query as Query>::Error,
    >,
    <V::Query as Query>::Error: Send + Sync,
    <V::Query as Query>::UserData: Send + Sync,
    <V::Query as Query>::Key: Send + Sync,
    V::UpKey: Send + Sync,
{
    type Error = <V::Query as Query>::Error;
    type Key = <V::Query as Query>::Key;
    type UserData = <V::Query as Query>::UserData;
    type Output = Self;
    type Inner = V;

    async fn with_key<K: Into<Self::Key> + Send + Sync>(
        user_data: Self::UserData,
        key: K,
    ) -> Result<Self, Self::Error> {
        let value = <V::Query as Query>::query(key.into(), &user_data).await?;
        let up = <<V as HasUp>::Up as HasContainer>::Container::with_key(
            user_data,
            value.key(),
        )
        .await?;

        Ok(Self { value, up })
    }

    async fn with(
        user_data: Self::UserData,
        value: Self::Inner,
    ) -> Result<Self::Output, Self::Error> {
        let up = <<V as HasUp>::Up as HasContainer>::Container::with_key(
            user_data,
            value.key(),
        )
        .await?;

        Ok(Self { value, up })
    }
}

impl<V> Debug for Up<V>
where
    V: Debug,
    V: HasUp<Up: HasContainer<Container: Container>>,
    <<V::Up as HasContainer>::Container as Container>::Output: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Up")
            .field("value", &self.value)
            .field("up", &self.up)
            .finish()
    }
}

impl<V> Deref for Up<V>
where
    V: HasUp<Up: HasContainer<Container: Container>>,
{
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<V> DerefMut for Up<V>
where
    V: HasUp<Up: HasContainer<Container: Container>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
