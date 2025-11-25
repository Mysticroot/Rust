use chrono::{Local, Utc};

fn main() {
    let now = Local::now();
    println!("Current date and time: {}", now);

    let utc = Utc::now();

    let formated = utc.format("%Y-%M-%D %H:%M:%S");

    println!("UTC date and time: {}", formated);
}
