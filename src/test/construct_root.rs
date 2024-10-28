use super::*;
use crate::{container::Container, root::Root, Fin};

#[tokio::test]
async fn construct_root() {
    let root = Fin::<Root<GrandParent>>::create(UserData, Key)
        .await
        .unwrap();

    assert_ty!(Root<GrandParent>, root);

    assert_ty_eq!(
        Root<GrandParent>,
        <Fin<Root<GrandParent>> as Container>::Value
    );

    assert_ty_eq!(
        Root<GrandParent>,
        <Fin<Root<GrandParent>> as Container>::Value
    );
}
