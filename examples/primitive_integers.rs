#![allow(unused_must_use)]

fn main() {
    let string = "hello world".to_owned();
    let slice = &string[0..1];
    dbg!(slice);
    dbg!(i32::from_str_radix(slice, 32));

    // rotating left on signed integers is strange. Don't do it
    let map: i8 = 0b1110000;
    dbg!(map);
    println!("{:b}, {}", map.rotate_left(1), 0b1100000);
    println!("{}, {}", map.rotate_left(1), 0b0100000);

    let max = i32::MAX;
    dbg!(max);
    dbg!(max.checked_add(1));
    // if it will overflow, don't add the number, just go to the maximum possible
    dbg!(max.saturating_add(1));
    // If it overflows, wrap around to the other extreme side of the type
    dbg!(max.wrapping_add(1)); // -2147483648

    // Return a tuple letting us know if we are overflowing, and if we are then wrap
    dbg!(max.overflowing_add(1)); // (-2147483648, true)
}
