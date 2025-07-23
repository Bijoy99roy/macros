use auto_debug::AutoDebug;

#[derive(AutoDebug)]
struct Person {
    #[debug(skip)]
    name: String,

    age: u8,
}

fn main() {
    let p = Person {
        name: "Bijoy".to_string(),
        age: 26,
    };
    println!("{:?}", p);
}
