extern crate serde;
use prepend_todo::prepend_todo;

#[prepend_todo]
struct Trip {
    destination: String,
    total_people: u8,
}

fn main() {
    let t = Trip {
        destination: "Japan".to_string(),
        total_people: 3,
    };
    let json = serde_json::to_string_pretty(&t).unwrap();
    println!("{}", json);
}
