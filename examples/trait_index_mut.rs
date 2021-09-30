use std::ops::{Index, IndexMut};

#[derive(Debug)]
struct Location {
    x: f32,
    y: f32,
}

impl Index<char> for Location {
    type Output = f32;

    fn index(&self, index: char) -> &Self::Output {
        match index {
            'x' => &self.x,
            'y' => &self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<char> for Location {
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        match index {
            'x' => &mut self.x,
            'y' => &mut self.y,
            _ => panic!("Location index out of bounds"),
        }
    }
}

fn main() {
    let mut location = Location { x: 50.0, y: 100.0 };
    location['x'] += 1.0;

    dbg!(location);
}
