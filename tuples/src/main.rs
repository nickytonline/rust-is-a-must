fn main() {
    let tuple = ("hello", 5, 'c');

    assert_eq!(tuple.0, "hello");
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, 'c');

    println!("{}", tuple.0); // prints "hello"
    println!("{}", tuple.1); // prints "5"
    println!("{}", tuple.2); // prints "c"
}
