// A struct with named fields
struct Person {
    name: String,
    age: u8,
    likes_oranges: bool,
}

// A tuple struct
struct Point2D(u32, u32);

// A unit struct
#[derive(Debug)]
struct Unit;

fn main() {
    // Instantiate a classic struct, with named fields. Order does not matter.
    let person = Person {
        name: String::from("Adam"),
        likes_oranges: true,
        age: 25,
    };

    // Instantiate a tuple struct by passing the values in the same order as defined.
    let origin = Point2D(1, 3);

    println!("2D point: {}, {}", origin.0, origin.1);

    // Instantiate a unit struct.
    let unit = Unit;

    println!("unit struct: {:?}", unit);

    // Display the details of the person
    if person.likes_oranges {
        println!("{:?} is {:?} and likes oranges.", person.name, person.age);
    } else {
        println!(
            "{:?} is {:?} and doesn't like oranges.",
            person.name, person.age
        );
    }
}
