fn main() {
    // Create a String from a string literal
    let mut hello = String::from("Hello, ");

    // Push a character into our String
    hello.push('w');

    // Push a string literal into our String
    hello.push_str("orld!");

    println!("{}", hello)
}
