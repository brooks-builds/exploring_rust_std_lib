#![allow(dead_code)]
use std::{fmt::Debug, mem::MaybeUninit, ptr::addr_of_mut};

#[derive(Debug)]
struct Location {
    pub x: f32,
    pub y: f32,
}

impl Drop for Location {
    fn drop(&mut self) {
        dbg!("I am dropped", self);
    }
}

#[derive(Debug)]
struct LocationBuilder {
    pub location: MaybeUninit<Location>,
}

impl LocationBuilder {
    pub fn new() -> Self {
        Self {
            location: MaybeUninit::uninit(),
        }
    }

    pub fn x(mut self, x: f32) -> Self {
        let pointer = self.location.as_mut_ptr();
        unsafe {
            addr_of_mut!((*pointer).x).write(x);
        }
        self
    }

    pub fn y(mut self, y: f32) -> Self {
        let pointer = self.location.as_mut_ptr();
        unsafe {
            addr_of_mut!((*pointer).y).write(y);
        }
        self
    }

    pub fn build(self) -> Location {
        unsafe { self.location.assume_init() }
    }
}

fn main() {
    // read_garbage_from_memory();
    // manually_initialize_memory();
    // intializing_array_element_by_element();
    // playing_with_builder();
    using_new();
}

fn read_garbage_from_memory() {
    #[allow(clippy::uninit_assumed_init)]
    let x: [char; 1000] = unsafe { MaybeUninit::uninit().assume_init() };
    x.iter().for_each(|char| {
        dbg!(*char as u32);
    });
}

fn manually_initialize_memory() {
    let mut x = MaybeUninit::<&i32>::uninit();
    x.write(&10);
    let x = unsafe { x.assume_init() };
    dbg!(x);
}

fn intializing_array_element_by_element() {
    let data: [u32; 10] = {
        let mut uninitialized_data: [MaybeUninit<u32>; 10] = unsafe {
            #[allow(clippy::uninit_assumed_init)]
            MaybeUninit::uninit().assume_init()
        };
        for element in &mut uninitialized_data[..] {
            element.write(42);
        }
        unsafe { std::mem::transmute(uninitialized_data) }
    };
    dbg!(data);
}

fn playing_with_builder() {
    let location = LocationBuilder::new();
    let location = location.y(20.0).build();
    dbg!(location.y);
}

enum EntityDataType {
    Unknown,
    F32,
    U64,
}

fn using_new() {
    let maybe_initialized_location = MaybeUninit::new(Location { x: 10.0, y: 20.0 });
    // let location = unsafe { maybe_initialized_location.assume_init() }; // this would cause the Location drop to run when function is completed
    dbg!(maybe_initialized_location);
}
