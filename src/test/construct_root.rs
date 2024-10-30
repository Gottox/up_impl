use super::*;
use crate::{container::Container, Root};

#[tokio::test]
async fn construct_root() {
    let root = Root::<GrandParent>::with_key(UserData, GrandParentKey)
        .await
        .unwrap();

    assert_ty!(Root<GrandParent>, root);

    assert_ty_eq!(Root<GrandParent>, <Root<GrandParent> as Container>::Output);
}
