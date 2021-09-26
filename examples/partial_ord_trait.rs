#[derive(Debug, PartialEq)]
struct CustomNumber(i32);

impl PartialOrd for CustomNumber {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let x = self.0.partial_cmp(&other.0);
        dbg!(&x);
        x
    }
}

fn main() {
    dbg!(CustomNumber(5) < CustomNumber(10));
}
