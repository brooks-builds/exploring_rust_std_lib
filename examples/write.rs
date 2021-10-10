fn main() {
    overwriting_memory();
    using_read();
    self_read();
    // different_sizes();
}

// This fails because dst and src must be the same type
// fn different_sizes() {
//     let mut large = [1, 2, 3, 4, 5];
//     unsafe {
//         let mut pointer = std::ptr::read(&large);
//         let pointer = [1, 2];
//         std::ptr::write(&mut large, pointer);
//     }
// }

fn self_read() {
    let mut hello = String::from("hello");
    dbg!(&hello);
    unsafe {
        let mut read_hello = std::ptr::read(&hello);
        read_hello.push('s');
        dbg!(&read_hello);
        std::ptr::write(&mut hello, read_hello);
    }
    dbg!(&hello);
}

fn using_read() {
    let mut hello = String::from("hello");
    let world = String::from("world");
    dbg!(&hello, &world);
    unsafe {
        let read_hello = std::ptr::read(&hello);
        dbg!(&read_hello);
        std::ptr::write(&mut hello, world);
    }
    dbg!(&hello);
}

fn overwriting_memory() {
    let mut hello = String::from("hello");
    dbg!(&hello); // "hello"
    let world = String::from("w");
    unsafe {
        std::ptr::write(&mut hello, world);
    }
    // we can no longer use world because it's moved
    dbg!(&hello); // "world"
}
