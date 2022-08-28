use druid::Lens;
fn main() {}

#[derive(Lens)]
struct A {
    b: B,
}
#[derive(Lens)]
struct B {
    c: usize,
}
