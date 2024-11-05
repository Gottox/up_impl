use super::*;
use crate::container::{branch::Branch, up::Up, Container};
use either::Either;

#[tokio::test]
async fn construct_branch() {
    let key: Either<AnchestorKey, AnchestorKey> = Either::Left(AnchestorKey);
    let parents = Branch::<Mother, Father>::with_key(UserData, key)
        .await
        .unwrap();
    assert_ty!(Either<Up<Mother>, Up<Father>>, parents);
    assert!(parents.left().is_some());
}
