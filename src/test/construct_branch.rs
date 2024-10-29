use super::*;
use crate::container::{branch::Branch, up::Up, Container};
use either::Either;

#[tokio::test]
async fn construct_branch() {
    let parents =
        Branch::<Mother, Father>::create(UserData, Either::Left(MotherKey))
            .await
            .unwrap();
    assert_ty!(Either<Up<Mother>, Up<Father>>, parents);
    assert!(parents.left().is_some());
}
