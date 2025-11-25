use std::fs::read_to_string;

fn main() {
    let result = read_to_string("examples/a.txt");

    match result {
        Ok(data) => println!("File contents:\n{}", data),
        Err(e) => println!("Error reading file: {}", e),
    }
}
