use crate::query::Query;
use async_trait::async_trait;
use either::Either;

use super::{Container, HasContainerType};
impl<L, R> HasContainerType for Either<L, R>
where
    L: HasContainerType,
    L::ContainerType: Container,

    R: HasContainerType,
    R::ContainerType: Container,
{
    type ContainerType = Branch<L, R>;
}

pub struct Branch<L, R>(L, R)
where
    L: HasContainerType,
    <L as HasContainerType>::ContainerType: Container,
    R: HasContainerType,
    <R as HasContainerType>::ContainerType: Container;

#[async_trait]
impl<L, R> Container for Branch<L, R>
where
    L: HasContainerType + Query,
    <L as HasContainerType>::ContainerType:
        Container<UserData = L::UserData, Key = L::Key, Error = L::Error>,
    R: HasContainerType + Query,
    <R as HasContainerType>::ContainerType:
        Container<UserData = L::UserData, Key = R::Key, Error = L::Error>,
    L::Key: Send + Sync,
    R::Key: Send + Sync,
    L::UserData: Send + Sync,
{
    type Error = <L as Query>::Error;
    type Key = Either<<L as Query>::Key, <R as Query>::Key>;
    type UserData = <L as Query>::UserData;
    type Output = Either<
        <L::ContainerType as Container>::Output,
        <R::ContainerType as Container>::Output,
    >;

    async fn create<K: Into<Self::Key> + Send + Sync>(
        user_data: Self::UserData,
        key: K,
    ) -> Result<Self::Output, Self::Error> {
        use Either::*;
        match key.into() {
            Left(l) => L::ContainerType::create(user_data, l).await.map(Left),
            Right(r) => R::ContainerType::create(user_data, r).await.map(Right),
        }
    }
}
