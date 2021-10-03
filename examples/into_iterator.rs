const _LOCATION_SIZE: usize = 2;

#[derive(Debug)]
struct Location {
    x: i32,
    y: i32,
}

impl IntoIterator for &Location {
    type Item = i32;

    // this type must be a struct
    type IntoIter = std::array::IntoIter<Self::Item, 2>;

    // we are taking ownership of Self here
    fn into_iter(self) -> Self::IntoIter {
        std::array::IntoIter::new([self.x, self.y])
    }
}

fn main() {
    let location = Location { x: 0, y: 1 };
    for loc in &location {
        println!("{:?}", loc);
    }
    dbg!(location);
}
