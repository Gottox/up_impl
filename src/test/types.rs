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

use crate::{query::Query, root::Root, HasQuery, HasUp};

#[derive(Debug)]
pub struct UserData;

#[derive(Debug)]
pub struct MotherKey;

#[derive(Debug)]
pub struct FatherKey;

#[derive(Debug)]
pub struct GrandParentKey;

#[derive(Debug)]
pub struct ChildKey;

#[derive(Debug, Default)]
pub struct GrandParent;
impl HasQuery for GrandParent {
    type Query = FamilyTreeQuery<GrandParentKey, Self>;
}

#[derive(Debug, Default)]
pub struct Father;
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
pub struct Mother;
impl HasUp for Mother {
    type Up = Root<GrandParent>;
    type UpKey = GrandParentKey;

    fn key(&self) -> Self::UpKey {
        GrandParentKey
    }
}
impl HasQuery for Mother {
    type Query = FamilyTreeQuery<MotherKey, Self>;
}

#[derive(Debug, Default)]
pub struct Child;
impl HasUp for Child {
    type Up = Either<Father, Mother>;
    type UpKey = Either<FatherKey, MotherKey>;

    fn key(&self) -> Self::UpKey {
        Either::Right(MotherKey)
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
