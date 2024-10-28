use async_trait::async_trait;
use either::Either;

use crate::{query::Query, root::Root, HasUp};

#[derive(Debug)]
pub struct Key;
#[derive(Debug, Clone)]
pub struct UserData;

#[derive(Debug)]
pub struct GrandParent;
#[async_trait]
impl Query for GrandParent {
    type UserData = UserData;
    type Error = std::io::Error;
    type Key = Key;

    async fn query(
        _: Self::UserData,
        _: Self::Key,
    ) -> Result<Self, std::io::Error> {
        Ok(Self)
    }
}

#[derive(Debug)]
pub struct Father;
impl HasUp for Father {
    type Up = Root<GrandParent>;
    type UpKey = Key;

    fn key(&self) -> Self::UpKey {
        Key
    }
}
#[async_trait]
impl Query for Father {
    type UserData = UserData;
    type Error = std::io::Error;
    type Key = Key;

    async fn query(
        _: Self::UserData,
        _: Self::Key,
    ) -> Result<Self, std::io::Error> {
        Ok(Self)
    }
}

#[derive(Debug)]
pub struct Mother;
impl HasUp for Mother {
    type Up = Root<GrandParent>;
    type UpKey = Key;

    fn key(&self) -> Self::UpKey {
        Key
    }
}
#[async_trait]
impl Query for Mother {
    type UserData = UserData;
    type Error = std::io::Error;
    type Key = Key;

    async fn query(
        _: Self::UserData,
        _: Self::Key,
    ) -> Result<Self, std::io::Error> {
        Ok(Self)
    }
}

#[derive(Debug)]
pub struct Child;
impl HasUp for Child {
    type Up = Either<Father, Mother>;
    type UpKey = Either<Key, Key>;

    fn key(&self) -> Self::UpKey {
        Either::Right(Key)
    }
}
#[async_trait]
impl Query for Child {
    type UserData = UserData;
    type Error = std::io::Error;
    type Key = Key;

    async fn query(
        _: Self::UserData,
        _: Self::Key,
    ) -> Result<Self, std::io::Error> {
        Ok(Self)
    }
}
