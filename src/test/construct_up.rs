use super::*;
use crate::{
    container::{Container, HasContainerType},
    HasUp, Up,
};

#[tokio::test]
async fn construct_up() {
    type T1 = <Up<Mother> as Container>::Value;
    println!(
        "{:#?}",
        std::any::type_name::<
            <<Mother as HasUp>::Up as HasContainerType>::ContainerType,
        >()
    );
    let root = Up::<Mother>::create(UserData, Key).await;
    println!("{:#?}", std::any::type_name_of_val(&root));
    println!("{:#?}", root);
    //assert_ty_eq!(<<Mother as HasUp>::Up as HasContainerType>::ContainerType, String);
}
