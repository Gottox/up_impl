pub mod container;
pub mod query;
pub mod up;

pub use async_trait::async_trait;

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use either::Either;

    use crate::{
        container::Root,
        query::Query,
        up::{Container, HasUp},
    };

    #[derive(Debug)]
    struct Key;
    #[derive(Debug)]
    struct UserData;

    #[derive(Debug)]
    struct GrandParent;
    #[async_trait]
    impl Query for GrandParent {
        type UserData = UserData;
        type Error = std::io::Error;
        type Key = Key;

        async fn query(
            _user_data: Self::UserData,
            _key: Self::Key,
        ) -> Result<Self, std::io::Error> {
            Ok(Self)
        }
    }

    #[derive(Debug)]
    struct Father;
    impl HasUp for Father {
        type Up = GrandParent;
        type QueryKey = Key;
        type UserData = UserData;

        fn key(&self) -> Self::QueryKey {
            Key
        }
    }

    #[derive(Debug)]
    struct Mother;
    impl HasUp for Mother {
        type Up = GrandParent;
        type QueryKey = Key;
        type UserData = UserData;

        fn key(&self) -> Self::QueryKey {
            Key
        }
    }

    #[derive(Debug)]
    struct Child;
    impl HasUp for Child {
        type Up = Either<Father, Mother>;
        type QueryKey = Key;
        type UserData = UserData;

        fn key(&self) -> Self::QueryKey {
            Key
        }
    }

    #[tokio::test]
    async fn test_container() {
        let root = Root::<
            Root<GrandParent, ()>,
            _>::create(UserData, Key).await.unwrap();
        println!("{:#?}", std::any::type_name_of_val(&root));
        println!("{:#?}", root);
    }
}
