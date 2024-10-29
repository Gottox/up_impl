use crate::query::{HasQuery, Query};
use async_trait::async_trait;
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

pub struct Root<T>(pub T);

impl<T> HasQuery for Root<T> {
    type Query = Self;
}
#[async_trait]
impl<T> Query for Root<T>
where
    T: HasQuery,
    T::Query: Query<Output = T> + Send + Sync,
    <T::Query as Query>::Error: Send + Sync,
    <T::Query as Query>::UserData: Send + Sync,
    <T::Query as Query>::Key: Send + Sync,
{
    type UserData = <T::Query as Query>::UserData;
    type Error = <T::Query as Query>::Error;
    type Key = <T::Query as Query>::Key;
    type Output = Self;

    async fn query(
        key: Self::Key,
        user_data: &Self::UserData,
    ) -> Result<Self, Self::Error> {
        T::Query::query(key, user_data).await.map(Root)
    }
}

impl<T> Debug for Root<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Root").field(&self.0).finish()
    }
}

impl<T> Deref for Root<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Root<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
