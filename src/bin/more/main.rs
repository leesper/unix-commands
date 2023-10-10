// more filename
// command | more
// more < filename
fn main() {
    println!("Hello, world!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn should_add_two_ints() {
    assert_eq!(add(1, 2), 3);
}