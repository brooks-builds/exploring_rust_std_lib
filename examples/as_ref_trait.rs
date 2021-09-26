struct Entity {
    name: String,
    health: i32,
}

impl AsRef<str> for Entity {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

impl AsRef<i32> for Entity {
    fn as_ref(&self) -> &i32 {
        &self.health
    }
}

fn print_name(entity_name: &str) {
    println!("{}", entity_name);
}

fn print_health(entity_health: &i32) {
    println!("{}", entity_health);
}

fn print_string(string: &str) {
    println!("{}", string);
}

fn ascii(string: &[u8]) {
    println!("{:?}", string);
}

fn main() {
    let name = String::from("hello");
    ascii(name.as_ref());
    ascii(name.as_bytes());
}
