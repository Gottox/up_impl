use super::*;
use crate::{container::Container, root::Root, Fin};

#[tokio::test]
async fn construct_root() {
    let root = Fin::<Root<GrandParent>>::create(UserData, Key)
        .await
        .unwrap();
    println!("{:#?}", std::any::type_name_of_val(&root));
    println!("{:#?}", root);
}
