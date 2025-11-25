
fn get_string_length(s: &str)->usize {
    s.chars().count()

}

fn main(){
 
 let  my_str= "Hello, world!";

    // let my_string: String = String::from("Hello, Rust!");
    let length = get_string_length(&my_str);
    println!("The length of '{}' is {}.", my_str, length);
}