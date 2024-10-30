use super::*;
use crate::{container::Container, root::Root, Fin};

#[tokio::test]
async fn construct_root() {
    let root = Fin::<GrandParent>::with_key(UserData, GrandParentKey)
        .await
        .unwrap();

    assert_ty!(Root<GrandParent>, root);

    assert_ty_eq!(Root<GrandParent>, <Fin<GrandParent> as Container>::Output);
}
