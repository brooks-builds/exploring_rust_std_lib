#[derive(Debug, Default)]
struct Location {
    x: i32,
    y: i32,
}

impl Clone for Location {
    fn clone(&self) -> Self {
        Location {
            x: self.x,
            y: self.y,
        }
    }
}

fn main() {
    let mut player_location = Location { x: 0, y: 0 };
    player_location.x = 1;
    dbg!(&player_location);
    let mut a = Location::default();
    dbg!(&a);
    a.clone_from(&player_location);
    dbg!(&a);
}
