fn always_panics() {
    panic!("What did you expect?");
}

fn main() {
    match std::panic::catch_unwind(|| {
        always_panics();
    }) {
        Ok(_) => (),
        Err(error) => {
            dbg!(error);
        }
    }
    dbg!("hello");
}
