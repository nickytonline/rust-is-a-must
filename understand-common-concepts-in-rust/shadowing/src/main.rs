fn main() {
    // The first binding is created with the name "number"
    let number = 5;
    println!("This should print 5 => {}.", number);

    // A different binding shadows the name "number"
    let number = number + 5;
    println!("This should print 10 => {}.", number);

    // Again, another new binding is created
    let number = number * 2;
    println!("This should print 20 => {}.", number);
}
