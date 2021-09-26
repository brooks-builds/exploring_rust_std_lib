fn main() {
    dbg!(1 == true.into());

    if false | false {
        dbg!("true & true");
    }

    let something = 5 > 3;

    // match something {
    //     true => todo!(),
    //     false => todo!(),
    // }

    assert!(false);

    let something_else = something.then(|| String::from("hello"));
}
