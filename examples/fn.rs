#![allow(clippy::eq_op, unused_variables)]
fn main() {
    // these are function pointers because we manually set the types
    let unsafely_cast_to_i: unsafe fn(f32) -> i32 = cast_to_i;
    let safely_cast_to_i: unsafe fn(f32) -> i32 = safely_cast_to_i;
    // this is a zero-sized unnamable type representing the function
    let c_abs = abs;
    // this is a reference to a zero-sized unnamable type. We never want this so don't do it
    let rand_i32 = rand;
    // other_rand has copied rand_i32 because all functions, includding ABI functions implement Copy
    let other_rand = rand_i32;
    unsafe {
        dbg!(unsafely_cast_to_i(10.0));
        dbg!(safely_cast_to_i(10.0));
        dbg!(c_abs(-5));
        dbg!(rand_i32());
    }

    // Rust functions implement Eq and PartialEq
    // FFI functions like rand do not implement Eq and PartialEq so cannot be compared
    dbg!(safely_cast_to_i == safely_cast_to_i);
}

unsafe fn cast_to_i(number: f32) -> i32 {
    number.to_int_unchecked()
}

// This implements Fn, FnMut, and FnOnce because it is a Rust function AND safe
fn safely_cast_to_i(number: f32) -> i32 {
    number as i32
}

// fn c_add_one(number: u32) -> u32 {
//     extern "C" {
//         number + 1
//     }
// }
extern "C" {
    fn abs(number: i32) -> i32;
    fn rand() -> i32;
}

// Cannot impl this ourselves because the std lib already does it for us.
// impl std::fmt::Debug for unsafe extern "C" fn() -> i32 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "You called a random number!")
//     }
// }
