use crate::debug::Debug;

use crate::{query::Query, HasUp};
use async_trait::async_trait;

use super::{Container, HasContainerType};

impl<V> HasContainerType for V
where
    V: HasUp<Up: HasContainerType> + Debug,
    <V::Up as HasContainerType>::ContainerType: Container,
    <<V::Up as HasContainerType>::ContainerType as Container>::Output: Debug,
{
    type ContainerType = Up<V>;
}
#[derive(Debug)]
pub struct Up<V>
where
    V: HasUp<Up: HasContainerType> + Debug,
    <V::Up as HasContainerType>::ContainerType: Container,
    <<V::Up as HasContainerType>::ContainerType as Container>::Output: Debug,
{
    pub value: V,
    pub up: <<V::Up as HasContainerType>::ContainerType as Container>::Output,
}
#[async_trait]
impl<V> Container for Up<V>
where
    V: HasUp<Up: HasContainerType, UpKey = <V as Query>::Key>
        + Debug
        + HasContainerType
        + Query
        + Send
        + Sync,
    <V::Up as HasContainerType>::ContainerType: Debug
        + Container<
            UserData = <V as Query>::UserData,
            Key = <V as Query>::Key,
            Error = <V as Query>::Error,
        >,
    <<V::Up as HasContainerType>::ContainerType as Container>::Output: Debug,
    V::Up: Query<UserData = V::UserData, Key = V::UpKey, Error = V::Error>
        + Debug
        + HasContainerType,
    V::Error: Send + Sync,
    V::UserData: Send + Sync + Clone,
    V::Key: Send + Sync,
{
    type Error = <V as Query>::Error;
    type Key = <V as Query>::Key;
    type UserData = <V as Query>::UserData;
    type Output = Self;

    async fn create(
        user_data: Self::UserData,
        key: Self::Key,
    ) -> Result<Self, Self::Error> {
        let value = V::query(user_data.clone(), key).await?;
        let up = <<V as HasUp>::Up as HasContainerType>::ContainerType::create(
            user_data,
            value.key(),
        )
        .await?;

        Ok(Self { value, up })
    }
}
