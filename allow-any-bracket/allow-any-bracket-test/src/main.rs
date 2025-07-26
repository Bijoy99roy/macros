use allow_any_bracket::AnyBracket;

#[derive(AnyBracket)]
struct Test {
    #[attrib(seed = (1, 2, 3))]
    a: i32,
    #[attrib(seed = {1, 24, 3})]
    b: i32,
    #[attrib(seed = {4, 5, 6,})]
    c: i32,

    d: i32,
}

fn main() {
    let instance = Test {
        a: 10,
        b: 20,
        c: 0,
        d: 1,
    };

    instance.check_seeds();
}
