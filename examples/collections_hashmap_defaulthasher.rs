use std::collections::{hash_map::RandomState, HashMap};

fn main() {
    let mut data: HashMap<String, i32, RandomState> = HashMap::new();
    // let mut first = DefaultHasher::new();
    // first.write_i32(42);
    // dbg!(first.finish());
    data.insert("hello".into(), 5);
    dbg!(data);
}
