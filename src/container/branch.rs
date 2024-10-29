use crate::{query::HasQuery, Query};
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
    L: HasContainerType + HasQuery,
    L::Query: Query,
    <L as HasContainerType>::ContainerType: Container<
        UserData = <L::Query as Query>::UserData,
        Key = <L::Query as Query>::Key,
        Error = <L::Query as Query>::Error,
    >,
    R: HasContainerType + HasQuery,
    R::Query: Query,
    <R as HasContainerType>::ContainerType: Container<
        UserData = <L::Query as Query>::UserData,
        Key = <R::Query as Query>::Key,
        Error = <L::Query as Query>::Error,
    >,
    <L::Query as Query>::Key: Send + Sync,
    <R::Query as Query>::Key: Send + Sync,
    <L::Query as Query>::UserData: Send + Sync,
{
    type Error = <L::Query as Query>::Error;
    type Key = Either<<L::Query as Query>::Key, <R::Query as Query>::Key>;
    type UserData = <L::Query as Query>::UserData;
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
