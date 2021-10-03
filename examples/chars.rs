fn main() {
    let _e_acute = 'é';
    // let e_latin = 'é';

    let hello = "Hello".to_owned(); // 5 bytes
    let _hello_chars = hello.chars(); // 20 bytes

    dbg!(char::MAX as u32); // 1114111
    let long_char = '\u{10FFFF}';
    dbg!(long_char);
}
