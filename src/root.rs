use crate::query::HasQuery;
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

pub struct Root<T>(pub T);

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
