use super::*;
use crate::{container::up::Up, container::Container, root::Root};

#[tokio::test]
async fn construct_up() {
    let child = Up::<Child>::create(UserData, ChildKey).await.unwrap();
    assert_ty!(Up<Child>, child);
    assert_ty_eq!(Up<Child>, <Up<types::Child> as Container>::Output);
    //assert_ty_eq!(String, <Up<types::Child> as Container>::Output);
    //assert_ty_eq!(String, <Up<types::Child> as Container>::Output);
    //assert_ty_eq!(Root<GrandParent>, mother.up);
}
