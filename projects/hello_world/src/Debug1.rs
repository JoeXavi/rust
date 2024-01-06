#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    const name = "Peter";
    const age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}