#[derive(Debug, PartialEq)]
struct CustomNumber(i32);

struct Entity {
    id: u32,
    health: CustomNumber,
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.health == other.health
    }
}

fn main() {
    dbg!(() == ());
}
