#![warn(clippy::pedantic, clippy::all)] // Clippy is fine with it, you should not be however

use std::{fmt::Debug, mem::ManuallyDrop};

#[derive(Debug)]
struct Location {
    x: f32,
    y: f32,
}

impl Drop for Location {
    fn drop(&mut self) {
        self.x += 1.0;
        self.y += 1.0;
        println!(
            "I am being dropped, my values are x: {}, y: {}",
            self.x, self.y
        );
    }
}

fn main() {
    let mut manually_dropped_location = ManuallyDrop::new(Location { x: 10.0, y: 20.0 });

    // This location is a copy of the Location stored inside the ManuallyDrop
    // however it is not pointing to the same place in memory
    let mut location = unsafe { ManuallyDrop::take(&mut manually_dropped_location) };
    location.x += 10.0; // this won't change the location inside the ManuallyDropped instance
    loggit(&location); // { x: 20.0, y: 20.0 }
    loggit(&manually_dropped_location); // { x: 10.0, y: 20.0 }
    let another_location = unsafe {
        // We own ManuallyDrop, so we can call the drop static method which will call the drop
        // method of the contained item but then leave the item sitting in memory.
        ManuallyDrop::drop(&mut manually_dropped_location); // manually_dropped_location drops but does not get removed from memory
        ManuallyDrop::drop(&mut manually_dropped_location);
        ManuallyDrop::take(&mut manually_dropped_location) // yes we can take it again even after running drop
    };
    loggit(&another_location);
} // location drops and gets removed from memory

fn loggit<T: Debug>(thing: &T) {
    dbg!(thing);
}
