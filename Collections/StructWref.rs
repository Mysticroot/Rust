struct User<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    let na = String::from("harkirat");

    let user1 = User { name: &na, age: 30 };

    println!("User: {}, Age: {}", user1.name, user1.age);
}
