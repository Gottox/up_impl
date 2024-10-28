#[cfg(feature = "debug")]
pub use std::fmt::Debug;

#[cfg(not(feature = "debug"))]
use std::any::Any;
#[cfg(not(feature = "debug"))]
pub trait Debug {}
#[cfg(not(feature = "debug"))]
impl<T> Debug for T {}
