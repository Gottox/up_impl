use std::{fmt::Debug, marker::PhantomData};

use crate::{
    query::Query,
    up::{Container, HasUp},
};
use async_trait::async_trait;
use either::Either;

pub trait HasContainerType {
    type ContainerType;
}

impl<T, P> HasContainerType for (T, P)
where
    T: HasUp<Up: HasContainerType + Debug> + Debug,
    <T::Up as HasContainerType>::ContainerType: Debug,
{
    type ContainerType = Up<T, P>;
}

impl<L, R, P> HasContainerType for (Either<L, R>, P)
where
    L: HasContainerType,
    R: HasContainerType,
{
    type ContainerType = Either<L::ContainerType, R::ContainerType>;
}

impl<T, P> HasContainerType for Root<T, P> {
    type ContainerType = Root<T, P>;
}
#[async_trait]
impl<T, P> Query for Root<T, P>
where
    T: Query + Send + Sync,
    T::Error: Send + Sync,
    T::UserData: Send + Sync,
    T::Key: Send + Sync,
{
    type UserData = T::UserData;
    type Error = T::Error;
    type Key = T::Key;

    async fn query(
        user_data: Self::UserData,
        key: Self::Key,
    ) -> Result<Self, Self::Error> {
        T::query(user_data, key).await.map(|value| Self {
            value,
            phantom: PhantomData,
        })
    }
}

#[derive(Debug)]
pub struct Up<V, P>
where
    V: HasUp<Up: HasContainerType + Debug> + Debug,
    <V::Up as HasContainerType>::ContainerType: Debug,
{
    pub value: V,
    pub up: <V::Up as HasContainerType>::ContainerType,
    phantom: PhantomData<P>,
}

#[derive(Debug)]
pub struct Root<V, P> {
    phantom: PhantomData<P>,
    pub value: V,
}
#[async_trait]
impl<UD, V, K> Container for Root<V, (UD, K)>
where
    V: Query<UserData = UD, Key = K> + Send + Sync,
    V::Error: Send + Sync,
    UD: Send + Sync,
    K: Send + Sync,
{
    type Error = V::Error;
    type Key = K;
    type UserData = UD;

    // TODO: This should return V instead of Self
    async fn create(
        user_data: Self::UserData,
        key: Self::Key,
    ) -> Result<Self, Self::Error> {
        let value = V::query(user_data, key).await?;
        Ok(Self {
            value,
            phantom: PhantomData,
        })
    }
}
