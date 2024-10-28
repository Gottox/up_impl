pub mod fin;
pub mod up;

use crate::query::Query;
use async_trait::async_trait;

#[async_trait]
pub trait Container
where
    Self: Sized,
{
    type Value: Query;

    async fn create(
        user_data: <Self::Value as Query>::UserData,
        key: <Self::Value as Query>::Key,
    ) -> Result<Self, <Self::Value as Query>::Error>;
}

pub trait HasContainerType {
    type ContainerType: ?Sized;
}

/*impl<L, R, P> HasContainerType for (Either<L, R>, P)
where
    L: HasContainerType,
    R: HasContainerType,
{
    type ContainerType = Either<L::ContainerType, R::ContainerType>;
}*/
