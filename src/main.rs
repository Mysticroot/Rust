fn main() {
    println!("{}", is_even(4));
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 { true } else { false }
}
