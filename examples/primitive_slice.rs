use std::mem::size_of;

fn main() {
    let x = [1, 2, 3, 4, 5];
    let size_of_thing = size_of::<[i32; 3]>(); // 12
    let size_of_slice = size_of::<[&i32; 3]>(); // 24
    dbg!(size_of_slice, size_of_thing);
}
