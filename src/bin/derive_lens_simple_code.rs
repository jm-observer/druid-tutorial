#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use druid::Lens;
fn main() {}
struct A {
    b: usize,
}
///Derived lenses for [`A`].
pub mod a_derived_lenses {
    ///Lens for the field `b` on [`A`](super::A).
    #[allow(non_camel_case_types)]
    pub struct b();
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::fmt::Debug for b {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "b")
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::marker::Copy for b {}
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::clone::Clone for b {
        #[inline]
        fn clone(&self) -> b {
            *self
        }
    }
    impl b {
        ///Creates a new lens for the field `b` on [`A`](super::A). Use [`A::b`](super::A::b) instead.
        pub const fn new() -> Self {
            Self()
        }
    }
}
impl druid::Lens<A, usize> for a_derived_lenses::b {
    fn with<V, F: FnOnce(&usize) -> V>(&self, data: &A, f: F) -> V {
        f(&data.b)
    }
    fn with_mut<V, F: FnOnce(&mut usize) -> V>(&self, data: &mut A, f: F) -> V {
        f(&mut data.b)
    }
}
#[allow(non_upper_case_globals)]
impl A {
    /// Lens for the corresponding field.
    pub const b: a_derived_lenses::b = a_derived_lenses::b::new();
}
