#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use druid::Data;
fn main() {}
struct A {
    b: String,
}
impl ::druid::Data for A {
    fn same(&self, other: &Self) -> bool {
        druid::Data::same(&self.b, &other.b)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for A {
    #[inline]
    fn clone(&self) -> A {
        A {
            b: ::core::clone::Clone::clone(&self.b),
        }
    }
}
