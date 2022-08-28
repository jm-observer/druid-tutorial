use druid::im::Vector;
use druid::Lens;
fn main() {}

#[derive(Lens)]
struct A {
    b: Vector<usize>,
}
