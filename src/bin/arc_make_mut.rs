use std::sync::Arc;
fn main() {
    one();
    two();
}

pub fn one() {
    let mut data = Arc::new(5);
    *Arc::make_mut(&mut data) += 1; // Won't clone anything
    let mut other_data = Arc::clone(&data); // Won't clone inner data
    *Arc::make_mut(&mut data) += 1; // Clones inner data
    *Arc::make_mut(&mut data) += 1; // Won't clone anything
    *Arc::make_mut(&mut other_data) *= 2; // Won't clone anything
                                          // Now `data` and `other_data` point to different allocations.
    assert_eq!(*data, 8);
    assert_eq!(*other_data, 12);
}
pub fn two() {
    let mut data = Arc::new(5);
    *Arc::make_mut(&mut data) += 1; // Won't clone anything
    let other_data = Arc::clone(&data); // Won't clone inner data
    let other_two = data.clone();
    *Arc::make_mut(&mut data) += 1; // Clones inner data
    *Arc::make_mut(&mut data) += 1; // Won't clone anything
                                    // Now `data` and `other_data` point to different allocations.
    assert_eq!(*data, 8);
    assert_eq!(*other_data, 6);
    assert_eq!(*other_two, 6);
}
