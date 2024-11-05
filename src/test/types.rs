//! Constructs Directed Acyclic Graphs (DAGs) of a family tree.
//!
//! ```text
//!       GrandParent
//!        /      \
//!       /        \
//!      /          \
//!    Father     Mother
//!      \          /
//!       \        /
//!        \      /
//!         Child
//! ```
use async_trait::async_trait;
use either::Either;

use crate::{query::Query, HasQuery, HasUp, OneOf, Root};

#[derive(Debug)]
pub struct UserData;

#[derive(Debug, Default, Clone)]
pub struct AnchestorKey;

#[derive(Debug, Default, Clone)]
pub struct MotherKey;

impl From<AnchestorKey> for MotherKey {
    fn from(_: AnchestorKey) -> Self {
        MotherKey
    }
}

#[derive(Debug, Default, Clone)]
pub struct FatherKey;

impl From<AnchestorKey> for FatherKey {
    fn from(_: AnchestorKey) -> Self {
        FatherKey
    }
}

#[derive(Debug, Default, Clone)]
pub struct GrandParentKey;

impl From<AnchestorKey> for GrandParentKey {
    fn from(_: AnchestorKey) -> Self {
        GrandParentKey
    }
}

#[derive(Debug, Default, Clone)]
pub struct ChildKey;

impl From<AnchestorKey> for ChildKey {
    fn from(_: AnchestorKey) -> Self {
        ChildKey
    }
}

#[derive(Debug, Default, Clone)]
pub struct GrandParent(GrandParentKey);
impl HasQuery for GrandParent {
    type Query = FamilyTreeQuery<GrandParentKey, Self>;
}

#[derive(Debug, Default)]
pub struct Father(FatherKey);
impl HasUp for Father {
    type Up = Root<GrandParent>;
    type UpKey = GrandParentKey;

    fn key(&self) -> Self::UpKey {
        GrandParentKey
    }
}
impl HasQuery for Father {
    type Query = FamilyTreeQuery<FatherKey, Self>;
}

#[derive(Debug, Default)]
pub struct Mother(GrandParentKey);
impl HasUp for Mother {
    type Up = Root<GrandParent>;
    type UpKey = GrandParentKey;

    fn key(&self) -> Self::UpKey {
        self.0.clone()
    }
}
impl HasQuery for Mother {
    type Query = FamilyTreeQuery<MotherKey, Self>;
}

#[derive(Debug)]
pub struct Child(Either<FatherKey, MotherKey>);
impl HasUp for Child {
    type Up = Either<Father, Mother>;
    type UpKey = OneOf<FatherKey, MotherKey>;

    fn key(&self) -> Self::UpKey {
        self.0.clone().into()
    }
}
impl Default for Child {
    fn default() -> Self {
        Child(Either::Right(MotherKey))
    }
}
impl HasQuery for Child {
    type Query = FamilyTreeQuery<ChildKey, Self>;
}

pub struct FamilyTreeQuery<K, O>(K, O);
#[async_trait]
impl<K, O> Query for FamilyTreeQuery<K, O>
where
    K: Send + Sync,
    O: Default,
{
    type UserData = UserData;
    type Error = std::io::Error;
    type Key = K;
    type Output = O;

    async fn query(
        _: Self::Key,
        _: &Self::UserData,
    ) -> Result<Self::Output, std::io::Error> {
        Ok(O::default())
    }
}
