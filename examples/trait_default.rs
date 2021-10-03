#[derive(Debug, Default)]
struct Location {
    x: f32, // default will be 0.0
    y: f32, // default will be 0.0
}

#[derive(Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

impl Velocity {
    // This allows us to manully have default values for what is passed in. However it does
    // look pretty terrible. We can use the Into trait to make it a bit nicer.
    fn _new(x: Option<f32>, y: Option<f32>) -> Self {
        Self {
            x: x.unwrap_or(5.0),
            y: y.unwrap_or_default(),
        }
    }
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            x: 5.0,
            y: Default::default(),
        }
    }
}

#[derive(Debug)]
struct Entity {
    location: Location,
    velocity: Velocity,
    direction: Direction,
}

impl Entity {
    /// We have a bare new method here that doesn't take any arguments. Therefore we should
    /// also manually implement the default trait for the struct and have it run the new method.
    /// If the new method doesn't really do anything special, then it should just call the
    /// default method.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Entity {
    fn default() -> Self {
        // Infinite recursive loop here because we are calling new, which calls default.
        // One of them should actually construct the object.
        // Self::new()
        Self {
            location: Default::default(),
            velocity: Default::default(),
            direction: Direction::default(),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}

fn main() {
    let player = Entity::default();
    let enemy = Entity::new();
    dbg!(player, enemy);
}
