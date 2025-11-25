fn main() {
    let mut s1 = String::from("Hello");

    // Try to create two mutable references at the same time
    let r1 = &mut s1;
    let r2 = &mut s1; // ERROR: cannot borrow `s1` as mutable more than once

    r1.push_str(" world");
    r2.push_str("!");

    println!("{}", s1);
}
