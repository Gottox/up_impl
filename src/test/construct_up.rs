use super::*;
use crate::{container::Container, root::Root, Up};

#[tokio::test]
async fn construct_up() {
    let mother = Up::<Mother>::create(UserData, Key).await.unwrap();
    assert_ty!(Up<Mother>, mother);
    assert_ty!(Root<GrandParent>, mother.up);
}
