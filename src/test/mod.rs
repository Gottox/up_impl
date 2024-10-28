mod types;

mod construct_branch;
mod construct_root;
mod construct_up;

pub use types::*;

#[macro_export]
macro_rules! assert_ty_eq {
    ($e:ty, $a:ty) => {{
        __assert_ty_eq!($e, $a, stringify!($e), stringify!($a));
    }};
}

#[macro_export]
macro_rules! assert_ty {
    ($e:ty, $a:expr) => {{
        fn assert_ty<E: 'static, A: 'static>(_a: &A) {
            __assert_ty_eq!(E, A, stringify!($e), stringify!($a));
        }
        assert_ty::<$e, _>(&$a);
    }};
}

#[macro_export]
macro_rules! __assert_ty_eq {
    ($e:ty, $a:ty, $ee:expr, $ae:expr) => {{
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
                $ee,
                std::any::type_name::<E>(),
                $ae,
                std::any::type_name::<A>(),
            );
        }
        assert_ty::<$e, $a>();
    }};
}
pub(crate) use __assert_ty_eq;
pub(crate) use assert_ty;
pub(crate) use assert_ty_eq;
