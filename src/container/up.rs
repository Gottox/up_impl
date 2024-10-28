use crate::debug::Debug;

use crate::{query::Query, HasUp};
use async_trait::async_trait;

use super::{Container, HasContainerType};

impl<V> HasContainerType for V
where
    V: HasUp<Up: HasContainerType> + Debug,
    <V::Up as HasContainerType>::ContainerType: Container + Debug,
{
    type ContainerType = Up<V>;
}

#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Up<V>
where
    V: HasUp<Up: HasContainerType> + Debug,
    <V::Up as HasContainerType>::ContainerType: Container + Debug,
{
    pub value: V,
    pub up: <V::Up as HasContainerType>::ContainerType,
}

#[async_trait]
impl<V> Container for Up<V>
where
    V: HasUp + Debug + HasContainerType + Query + Send + Sync,
    <V::Up as HasContainerType>::ContainerType:
        Debug + Container<Value = V::Up>,
    V::Up: Query<UserData = V::UserData, Key = V::UpKey, Error = V::Error>
        + Debug
        + HasContainerType,
    V::Error: Send + Sync,
    V::UserData: Send + Sync + Clone,
    V::Key: Send + Sync,
{
    type Value = V;

    async fn create(
        user_data: <Self::Value as Query>::UserData,
        key: <Self::Value as Query>::Key,
    ) -> Result<Self, <Self::Value as Query>::Error> {
        let value = V::query(user_data.clone(), key).await?;
        let up = <<V as HasUp>::Up as HasContainerType>::ContainerType::create(
            user_data,
            value.key(),
        )
        .await?;

        Ok(Self { value, up })
    }
}
