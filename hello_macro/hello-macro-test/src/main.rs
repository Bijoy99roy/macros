use hello_macro::HelloMacro;

trait HelloMacro {
    fn hello();
}

#[derive(HelloMacro)]
struct Anime;

fn main() {
    Anime::hello();
}
