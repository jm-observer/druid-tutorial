#![allow(dead_code)]

use druid::im::Vector;
fn main() {}
struct A {
    b: Vector<usize>,
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
#[allow(non_upper_case_globals)]
impl A {
    /// Lens for the corresponding field.
    pub const b: a_derived_lenses::b = a_derived_lenses::b::new();
}
