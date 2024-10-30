use crate::{
    query::{HasQuery, Query},
    HasUp,
};
use async_trait::async_trait;
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use super::{Container, HasContainerType};

impl<V> HasContainerType for V
where
    V: HasUp<Up: HasContainerType>,
    <V::Up as HasContainerType>::ContainerType: Container,
{
    type ContainerType = Up<V>;
}
pub struct Up<V>
where
    V: HasUp<Up: HasContainerType>,
    <V::Up as HasContainerType>::ContainerType: Container,
{
    pub value: V,
    pub up: <<V::Up as HasContainerType>::ContainerType as Container>::Output,
}
#[async_trait]
impl<V> Container for Up<V>
where
    V: HasUp + HasQuery + HasContainerType + HasQuery + Send + Sync,
    V::Query: Query<Output = V> + Send + Sync,
    V::Up: HasContainerType,
    <V::Up as HasContainerType>::ContainerType: Container<
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
        let up =
            <<V as HasUp>::Up as HasContainerType>::ContainerType::with_key(
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
        let up =
            <<V as HasUp>::Up as HasContainerType>::ContainerType::with_key(
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
    V: HasUp<Up: HasContainerType<ContainerType: Container>>,
    <<V::Up as HasContainerType>::ContainerType as Container>::Output: Debug,
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
    V: HasUp<Up: HasContainerType<ContainerType: Container>>,
{
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<V> DerefMut for Up<V>
where
    V: HasUp<Up: HasContainerType<ContainerType: Container>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
