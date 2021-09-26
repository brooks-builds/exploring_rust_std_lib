#[derive(Debug, Clone, Copy)]
struct Size(u32);

#[derive(Default, Debug)]
struct Location {
    x: f32,
    y: f32,
}

fn main() {
    // play_with_arrays();
    array_maps();
}

fn play_with_arrays() {
    let names = ["John", "Jane", "Mary", "Patricia"];
    let zeros = [Size(0); 5];
    let locations = [
        Location::default(),
        Location::default(),
        Location::default(),
    ];
    dbg!(zeros);
    let [john, jane, mary, patricia] = names;
    dbg!(john);
    dbg!(names);

    let [location1, ..] = locations;
    let [.., location3] = locations;
    dbg!(location1);
    dbg!(location3);
    // dbg!(locations);
    let more_locations = <[Location; 3]>::default();
    dbg!(more_locations);
}

fn array_maps() {
    let a = [1, 2, 3, 4, 5];
    let changed_a = a.map(|x| x + 1).map(|x| x * 2);
    dbg!(changed_a);
}
