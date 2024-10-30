use crate::{query::HasQuery, Query};
use async_trait::async_trait;
use either::Either;

use super::{Container, HasContainer};
impl<L, R> HasContainer for Either<L, R>
where
    L: HasContainer,
    L::Container: Container,

    R: HasContainer,
    R::Container: Container,
{
    type Container = Branch<L, R>;
}

pub struct Branch<L, R>(L, R)
where
    L: HasContainer,
    <L as HasContainer>::Container: Container,
    R: HasContainer,
    <R as HasContainer>::Container: Container;

#[async_trait]
impl<L, R> Container for Branch<L, R>
where
    L: HasContainer + HasQuery,
    L::Query: Query,
    <L as HasContainer>::Container: Container<
        UserData = <L::Query as Query>::UserData,
        Key = <L::Query as Query>::Key,
        Error = <L::Query as Query>::Error,
    >,
    R: HasContainer + HasQuery,
    R::Query: Query,
    <R as HasContainer>::Container: Container<
        UserData = <L::Query as Query>::UserData,
        Key = <R::Query as Query>::Key,
        Error = <L::Query as Query>::Error,
    >,
    <L::Query as Query>::Key: Send + Sync,
    <R::Query as Query>::Key: Send + Sync,
    <L::Container as Container>::Inner: Send + Sync,
    <R::Container as Container>::Inner: Send + Sync,
    <L::Query as Query>::UserData: Send + Sync,
{
    type Error = <L::Query as Query>::Error;
    type Key = Either<<L::Query as Query>::Key, <R::Query as Query>::Key>;
    type UserData = <L::Query as Query>::UserData;
    type Output = Either<
        <L::Container as Container>::Output,
        <R::Container as Container>::Output,
    >;
    type Inner = Either<
        <L::Container as Container>::Inner,
        <R::Container as Container>::Inner,
    >;

    async fn with_key<K: Into<Self::Key> + Send + Sync>(
        user_data: Self::UserData,
        key: K,
    ) -> Result<Self::Output, Self::Error> {
        use Either::*;
        match key.into() {
            Left(l) => L::Container::with_key(user_data, l).await.map(Left),
            Right(r) => R::Container::with_key(user_data, r).await.map(Right),
        }
    }

    async fn with(
        user_data: Self::UserData,
        value: Self::Inner,
    ) -> Result<Self::Output, Self::Error> {
        use Either::*;
        match value {
            Left(l) => L::Container::with(user_data, l).await.map(Left),
            Right(r) => R::Container::with(user_data, r).await.map(Right),
        }
    }
}
