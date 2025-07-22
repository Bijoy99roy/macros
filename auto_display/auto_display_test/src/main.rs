use auto_display::attribute_auto_display;

#[attribute_auto_display]
struct Person {
    name: String,
    age: u32,
}

#[attribute_auto_display]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Person {
        name: "Alice".into(),
        age: 30,
    };
    let pt = Point { x: 10, y: 20 };

    println!("{}", p);
    println!("{}", pt);
}
