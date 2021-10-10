use std::borrow::Borrow;

#[derive(Debug)]
struct Location {
    data: [f32; 2],
}

impl Borrow<[f32; 2]> for Location {
    fn borrow(&self) -> &[f32; 2] {
        &self.data
    }
}

impl ToOwned for Location {
    type Owned = Location;

    fn to_owned(&self) -> Self::Owned {
        Self {
            data: [self.data[0], self.data[1]],
        }
    }
}

fn main() {
    let location = &Location { data: [10.0, 20.0] };
    let cloned_location = location.to_owned();
    dbg!(location, cloned_location);

    let _hello = "hello world".to_owned();
}
