use std::num::ParseIntError;

#[derive(Debug)]
struct MyString {
    string: String,
}

impl From<&str> for MyString {
    fn from(string: &str) -> Self {
        Self {
            string: string.to_owned(),
        }
    }
}

impl From<i32> for MyString {
    fn from(input: i32) -> Self {
        Self {
            string: format!("You gave me {}", input),
        }
    }
}

#[derive(Debug)]
enum CustomErrors {
    SuperAwesomeCustomError,
}

impl From<ParseIntError> for CustomErrors {
    fn from(_error: ParseIntError) -> Self {
        Self::SuperAwesomeCustomError
    }
}

fn main() -> Result<(), CustomErrors> {
    let my_string = MyString::from("Hello world");
    dbg!(my_string);
    let my_custom_int: MyString = 10.into();
    dbg!(my_custom_int);

    let real_string = String::from("hello");
    let _converted = real_string.parse::<i32>()?;
    Ok(())
}
