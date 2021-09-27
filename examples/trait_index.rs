use std::ops::Index;

#[derive(Debug, Default)]
struct Location {
    x: i32,
    y: i32,
}

impl Index<usize> for Location {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Invalid index"),
        }
    }
}

fn main() {
    let player_location = Location::default();
    dbg!(player_location[0]);
}
