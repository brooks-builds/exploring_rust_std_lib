struct Something {
    thing: ThirdThing,
}

impl Copy for Something {}

impl Clone for Something {
    fn clone(&self) -> Self {
        Self { thing: ThirdThing }
    }
}

// #[derive(Copy, Clone)]
struct SomethingElse {
    thing: ThirdThing,
}

struct ThirdThing;

#[derive(Copy, Clone)]
struct ThisCanBeCopied;

fn main() {
    let something = Something { thing: ThirdThing };
    let copy_of_something = something;
    dbg!(something, copy_of_something);
}
