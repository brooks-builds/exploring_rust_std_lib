use std::intrinsics::transmute;

#[allow(clippy::transmute_int_to_char)]
fn main() {
    print_size_of_type::<u32>();
    print_size_of_type::<f32>();
    print_size_of_type::<i32>();
    print_size_of_type::<f64>();
    print_size_of_type::<u64>();
    print_size_of_type::<char>();
    let result: char = unsafe { transmute(100_u32) };
    dbg!(result);
}

fn print_size_of_type<T>() {
    dbg!(std::mem::size_of::<T>());
}
