use super::*;
use crate::{container::up::Up, container::Container, root::Root};

#[tokio::test]
async fn construct_up() {
    //let mother = Up::<Mother>::create(UserData, MotherKey).await.unwrap();
    //assert_ty!(Up<Mother>, mother);
    //assert_ty!(Root<GrandParent>, mother.up);
    assert_ty_eq!(<Up<Mother> as Container>::Error, std::io::Error);
}
