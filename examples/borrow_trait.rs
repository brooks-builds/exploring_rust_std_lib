use std::{borrow::Borrow, collections::HashMap};

#[derive(Debug, PartialEq, Eq, Hash)]
struct WrappedString {
    value: String,
}

impl Borrow<str> for WrappedString {
    fn borrow(&self) -> &str {
        self.value.as_str()
    }
}

fn main() {
    let mut entities: HashMap<WrappedString, u32> = HashMap::new();
    entities.insert(
        WrappedString {
            value: "foo".to_string(),
        },
        42,
    );
    // let foo = entities.get("foo");
    // dbg!(foo);
}
