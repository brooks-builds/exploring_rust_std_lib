#[derive(Debug, Default)]
struct Location {
    pub x: Scalar,
    pub y: Scalar,
}

#[derive(Debug, Default)]
struct Scalar {
    x: f32,
}

fn main() {
    bindings();
}

fn bindings() {
    // We can bind location, x, and y at the same time but only if it implements clone and copy or if we are getting by reference.
    let location @ Location { x, y } = &Location::default();
    dbg!(x, y);
    dbg!(location);
}
