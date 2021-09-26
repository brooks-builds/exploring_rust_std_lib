use std::{io::Write, thread, time::Duration};

fn main() {
    // println!("Hello {:5}!", 5);
    // println!("Hello {0:1$}!", "x", 9);
    // println!("Hello {1:0$}!", 5, "x");
    // println!("Hello {:width$}!", "x", width = 5);
    let loading_chars = ['/', '-', '\\', '|'];
    let mut i = 0;
    loop {
        i = (i + 1) % 4;
        // println!("{}", i);
        print!("{}\r", loading_chars[i]);
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(250));
        // print!("{:▪^-#width$.5?}!", (spaces, spaces as f32), width = 9);
    }
    // println!("{:e}", 10000.55);
}
