struct Something {
    _thing: ThirdThing,
}

// This does not work
// impl Copy for Something {}

impl Clone for Something {
    fn clone(&self) -> Self {
        Self { _thing: ThirdThing }
    }
}

// #[derive(Copy, Clone)]
struct _SomethingElse {
    thing: ThirdThing,
}

struct ThirdThing;

#[derive(Copy, Clone)]
struct ThisCanBeCopied;

fn main() {
    let something = Something { _thing: ThirdThing };
    let _copy_of_something = something;
    // dbg!(something, copy_of_something);
}
