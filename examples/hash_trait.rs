use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::Hash,
};

#[derive(Debug, PartialEq, Eq)]
struct CustomNumber {
    number: i32,
}

impl std::hash::Hash for CustomNumber {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.number.hash(state);
    }
}

fn main() {
    // let mut h: HashMap<CustomNumber, u32> = HashMap::new();
    // h.insert(CustomNumber { number: 5 }, 10);
    // dbg!(h);
    let numbers = [1, 2, 3, 4, 5];
    let mut hasher = DefaultHasher::new();
    Hash::hash_slice(&numbers, &mut hasher);
    dbg!(hasher);
}
