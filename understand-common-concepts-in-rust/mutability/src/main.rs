fn main() {
    let mut a_number = 10;
    println!("The number is {}.", a_number);

    a_number = 15;
    // Error: cannot assign twice to immutable variable `a_number`
    println!("Now the number is {}.", a_number);
}
