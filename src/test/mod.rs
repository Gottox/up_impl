mod types;

mod construct_root;
mod construct_up;

pub use types::*;

#[macro_export]
macro_rules! assert_ty {
    ($e:ty, $a:expr) => {{
        fn assert_ty<E: 'static, A: 'static>(_a: &A) {
            assert_ty_eq!(E, A);
        }
        assert_ty::<$e, _>(&$a);
    }};
}

#[macro_export]
macro_rules! assert_ty_eq {
    ($e:ty, $a:ty) => {{
        fn assert_ty<E: 'static, A: 'static>() {
            assert_eq!(
                std::any::TypeId::of::<E>(),
                std::any::TypeId::of::<A>(),
                concat!(
                    "\n  left expr: {}",
                    "\n  left type: {}",
                    "\n right expr: {}",
                    "\n right type: {}",
                ),
                stringify!($e),
                std::any::type_name::<E>(),
                stringify!($a),
                std::any::type_name::<A>(),
            );
        }
        assert_ty::<$e, $a>();
    }};
}
pub(crate) use assert_ty;
pub(crate) use assert_ty_eq;
