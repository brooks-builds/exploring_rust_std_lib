use std::{cell::UnsafeCell, mem::ManuallyDrop};

#[derive(Debug, Default)]
struct NumberWrapper(i32);

fn main() {
    reading_from_unsafe_cell();
    dropping_a_copy();
}

fn dropping_a_copy() {
    let string = String::from("hello");
    let other = unsafe { std::ptr::read(&string) };
    println!("{} - {}", other, string);
    let s = ManuallyDrop::new(string);
    drop(other);
    dbg!(s); // getting garbage data from previously freed memory
}

fn reading_from_unsafe_cell() {
    let wrapper = UnsafeCell::<NumberWrapper>::default();
    let pointer = wrapper.get();
    let unwrapped = unsafe { std::ptr::read(pointer) };
    let unwrapped_again = unsafe { std::ptr::read(pointer) };
    println!("{:p} == {:p}", &unwrapped, &unwrapped_again);
}
