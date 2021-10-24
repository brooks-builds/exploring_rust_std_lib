use std::{any::Any, fmt::Debug};

/// Location struct for displaying where we are?
#[derive(Debug)]
struct Location {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Locationy<'a> {
    location: &'a Location,
}

fn print_debug<T: Any + Debug>(thing: &T) {
    dbg!(thing, thing.type_id());
}

fn main() {
    // this is static, so can be sent to print_debug just fine
    let location = Location { x: 100.0, y: 200.0 };
    // &location is not static, so therefore cannot be sent to print_debug
    let _stringy = Locationy {
        location: &location,
    };
    print_debug(&location);
}
