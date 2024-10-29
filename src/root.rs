use crate::query::Query;
use async_trait::async_trait;
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

pub struct Root<T>(pub T);

#[async_trait]
impl<T> Query for Root<T>
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
        key: Self::Key,
        user_data: &Self::UserData,
    ) -> Result<Self, Self::Error> {
        T::query(key, user_data).await.map(Root)
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
