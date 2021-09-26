use std::fmt::Debug;

#[derive(Debug)]
struct LocationWithDebug {
    x: i32,
    y: i32,
}

struct LocationWithoutDebug {
    x: i32,
    y: i32,
}

impl Debug for LocationWithoutDebug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocationWithoutDebug")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

fn main() {
    let point_with_debug = LocationWithDebug { x: 1, y: 2 };
    let point_without_debug = LocationWithoutDebug { x: 1, y: 2 };
    dbg!(point_with_debug);
    dbg!(point_without_debug);
}
