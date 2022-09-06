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
