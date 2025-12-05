fn main() {
    let largest;
    let str1 = String::from("small");
    // {
    let str2 = String::from("longer");

    largest = bigger(&str1, &str2);
    //  } // str2 goes out of scope here, but largest has a reference to it
    print!("{}", largest);
}

fn bigger<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
