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

pub enum OneOf<L, R> {
    Left(L),
    Right(R),
}
impl<L, R> std::fmt::Debug for OneOf<L, R>
where
    L: std::fmt::Debug,
    R: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OneOf::Left(l) => write!(f, "Left({:?})", l),
            OneOf::Right(r) => write!(f, "Right({:?})", r),
        }
    }
}

impl<L, R> Clone for OneOf<L, R>
where
    L: Clone,
    R: Clone,
{
    fn clone(&self) -> Self {
        match self {
            OneOf::Left(l) => OneOf::Left(l.clone()),
            OneOf::Right(r) => OneOf::Right(r.clone()),
        }
    }
}

impl<EL, ER, OL, OR> From<Either<EL, ER>> for OneOf<OL, OR>
where
    EL: Into<OL>,
    ER: Into<OR>,
{
    fn from(v: Either<EL, ER>) -> Self {
        match v {
            Either::Left(l) => OneOf::Left(l.into()),
            Either::Right(r) => OneOf::Right(r.into()),
        }
    }
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
    type Key = OneOf<<L::Query as Query>::Key, <R::Query as Query>::Key>;
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
            OneOf::Left(l) => {
                L::Container::with_key(user_data, l).await.map(Left)
            }
            OneOf::Right(r) => {
                R::Container::with_key(user_data, r).await.map(Right)
            }
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
